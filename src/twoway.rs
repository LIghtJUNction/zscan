//! Constant-space critical factorization and Two-Way search.
//!
//! The algorithm follows Crochemore and Perrin, "Two-Way String-Matching",
//! JACM 38(3), 1991, DOI: 10.1145/116825.116845. Both alphabet orders are
//! factored in O(m); the later maximal suffix supplies a critical position.
//! Search compares the right factor first. A right-factor mismatch pays for
//! its own shift; a left-factor mismatch advances by a period (remembering the
//! overlap) or by at least half the pattern. Thus search takes O(n), including
//! on periodic inputs. Reversing indexing, rather than the bytes, preserves
//! these arguments and uses no buffer.

#[derive(Clone, Copy, Debug)]
pub(crate) struct TwoWay {
    critical: usize,
    shift: usize,
    memory: usize,
}

impl TwoWay {
    pub(crate) fn new<const REVERSE: bool>(needle: &[u8]) -> Self {
        if needle.is_empty() {
            return Self {
                critical: 0,
                shift: 1,
                memory: 0,
            };
        }
        let a = maximal_suffix::<REVERSE, false>(needle);
        let b = maximal_suffix::<REVERSE, true>(needle);
        let (critical, period) = if a.0 > b.0 { a } else { b };
        let periodic = period <= needle.len() - critical
            && (0..critical)
                .all(|i| byte::<REVERSE>(needle, i) == byte::<REVERSE>(needle, period + i));
        if periodic {
            Self {
                critical,
                shift: period,
                memory: needle.len() - period,
            }
        } else {
            // critical is nonzero: an empty left factor always passed the
            // periodic test. This conservative lower bound on the period
            // still advances by at least half the needle after a full right
            // factor comparison. It is at most needle.len().
            Self {
                critical,
                shift: critical.saturating_sub(1).max(needle.len() - critical) + 1,
                memory: 0,
            }
        }
    }

    /// Metadata must have been constructed for this needle and direction.
    /// Returns offsets in the original haystack, even for reverse search.
    pub(crate) fn find<const REVERSE: bool>(
        &self,
        haystack: &[u8],
        needle: &[u8],
    ) -> Option<usize> {
        let length = needle.len();
        if length == 0 {
            return Some(if REVERSE { haystack.len() } else { 0 });
        }
        if length > haystack.len() {
            return None;
        }
        let last = haystack.len() - length;
        let mut position = 0;
        let mut memory = 0;
        while position <= last {
            let mut i = self.critical.max(memory);
            while i < length
                && byte::<REVERSE>(needle, i) == byte::<REVERSE>(haystack, position + i)
            {
                i += 1;
            }
            if i < length {
                // At most length, so position + this shift is at most the
                // haystack length. No wrapping arithmetic is needed.
                position += i - self.critical + 1;
                memory = 0;
                continue;
            }
            i = self.critical;
            while i > memory
                && byte::<REVERSE>(needle, i - 1) == byte::<REVERSE>(haystack, position + i - 1)
            {
                i -= 1;
            }
            if i <= memory {
                return Some(if REVERSE { last - position } else { position });
            }
            position += self.shift;
            // For periodic patterns this avoids comparing the overlapping
            // prefix again. For nonperiodic patterns memory remains zero.
            memory = self.memory;
        }
        None
    }
}

#[inline(always)]
fn byte<const REVERSE: bool>(bytes: &[u8], index: usize) -> u8 {
    bytes[if REVERSE {
        bytes.len() - 1 - index
    } else {
        index
    }]
}

/// Returns (maximal-suffix start, its candidate period).
/// `critical` represents the usual signed maximal-suffix index plus one,
/// avoiding a -1 sentinel and all wrapping index arithmetic.
fn maximal_suffix<const REVERSE: bool, const INVERT_ORDER: bool>(needle: &[u8]) -> (usize, usize) {
    let mut critical = 0;
    let mut candidate = 0;
    let mut offset = 1;
    let mut period = 1;
    while offset < needle.len() - candidate {
        let a = byte::<REVERSE>(needle, candidate + offset);
        let b = byte::<REVERSE>(needle, critical + offset - 1);
        if a == b {
            if offset == period {
                candidate += period;
                offset = 1;
            } else {
                offset += 1;
            }
        } else if (a < b) != INVERT_ORDER {
            candidate += offset;
            offset = 1;
            period = candidate + 1 - critical;
        } else {
            critical = candidate + 1;
            candidate = critical;
            offset = 1;
            period = 1;
        }
    }
    (critical, period)
}

#[cfg(test)]
mod tests {
    use super::TwoWay;

    fn reference(h: &[u8], n: &[u8], reverse: bool) -> Option<usize> {
        if n.len() > h.len() {
            return None;
        }
        let mut found = None;
        for start in 0..=h.len() - n.len() {
            if (0..n.len()).all(|i| h[start + i] == n[i]) {
                found = Some(start);
                if !reverse {
                    break;
                }
            }
        }
        found
    }

    #[test]
    fn twoway_exhaustive_binary() {
        let max_h = if cfg!(miri) { 5 } else { 9 };
        let max_n = if cfg!(miri) { 3 } else { 6 };
        for h_len in 0..=max_h {
            for h_bits in 0..1usize << h_len {
                let mut h = [0; 9];
                for (i, value) in h[..h_len].iter_mut().enumerate() {
                    *value = ((h_bits >> i) & 1) as u8;
                }
                for n_len in 0..=max_n {
                    for n_bits in 0..1usize << n_len {
                        let mut n = [0; 6];
                        for (i, value) in n[..n_len].iter_mut().enumerate() {
                            *value = ((n_bits >> i) & 1) as u8;
                        }
                        let h = &h[..h_len];
                        let n = &n[..n_len];
                        assert_eq!(
                            TwoWay::new::<false>(n).find::<false>(h, n),
                            reference(h, n, false),
                            "forward h={h:?} n={n:?}"
                        );
                        assert_eq!(
                            TwoWay::new::<true>(n).find::<true>(h, n),
                            reference(h, n, true),
                            "reverse h={h:?} n={n:?}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn twoway_long_periodic_and_nonperiodic() {
        let size = if cfg!(miri) { 257 } else { 8193 };
        let mut haystack = std::vec![b'a'; size];
        for period in [1, 2, 3, 5, 16, 31] {
            for (i, byte) in haystack.iter_mut().enumerate() {
                *byte = (i % period) as u8;
            }
            for length in [2, 3, 7, 32, 65, 128] {
                let mut needle = haystack[..length].to_vec();
                for mismatch in [None, Some(0), Some(length / 2), Some(length - 1)] {
                    if let Some(index) = mismatch {
                        needle[index] = 255;
                    }
                    assert_eq!(
                        TwoWay::new::<false>(&needle).find::<false>(&haystack, &needle),
                        reference(&haystack, &needle, false)
                    );
                    assert_eq!(
                        TwoWay::new::<true>(&needle).find::<true>(&haystack, &needle),
                        reference(&haystack, &needle, true)
                    );
                    needle.copy_from_slice(&haystack[..length]);
                }
            }
        }
    }
}
