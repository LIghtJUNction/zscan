//! Exact-width scalar and vector scans. Every load stays inside the slice.

#[derive(Clone, Copy, Debug)]
pub(crate) enum Backend {
    Scalar,
    #[cfg(all(target_arch = "x86_64", not(any(miri, zscan_scalar))))]
    Avx2,
    #[cfg(all(
        target_arch = "aarch64",
        target_feature = "neon",
        not(any(miri, zscan_scalar))
    ))]
    Neon,
}

impl Backend {
    #[inline]
    pub(crate) fn detect() -> Self {
        #[cfg(all(target_arch = "x86_64", not(any(miri, zscan_scalar))))]
        if x86::available() {
            return Self::Avx2;
        }
        #[cfg(all(
            target_arch = "aarch64",
            target_feature = "neon",
            not(any(miri, zscan_scalar))
        ))]
        return Self::Neon;
        #[allow(unreachable_code)]
        Self::Scalar
    }

    #[inline]
    pub(crate) fn byte<const REVERSE: bool>(self, byte: u8, haystack: &[u8]) -> Option<usize> {
        match self {
            Self::Scalar => scalar_byte::<REVERSE>(byte, haystack),
            #[cfg(all(target_arch = "x86_64", not(any(miri, zscan_scalar))))]
            Self::Avx2 => {
                // SAFETY: this variant is constructed only after CPUID and
                // XCR0 checks, or with AVX2 guaranteed by the compilation
                // target. The callee bounds every unaligned, read-only load.
                unsafe { x86::byte::<REVERSE>(byte, haystack) }
            }
            #[cfg(all(
                target_arch = "aarch64",
                target_feature = "neon",
                not(any(miri, zscan_scalar))
            ))]
            Self::Neon => {
                // SAFETY: the compilation target guarantees NEON support.
                // The callee bounds all read-only loads and needs no alignment.
                unsafe { neon::byte::<REVERSE>(byte, haystack) }
            }
        }
    }

    /// Scan candidate starts in [lo, hi), where hi <= h.len() - n.len() + 1.
    #[inline]
    pub(crate) fn pair<const REVERSE: bool>(
        self,
        h: &[u8],
        n: &[u8],
        a: usize,
        b: usize,
        lo: usize,
        hi: usize,
    ) -> Option<usize> {
        debug_assert!(a < n.len() && b < n.len());
        debug_assert!(lo <= hi && n.len() <= h.len());
        debug_assert!(hi <= h.len() - n.len() + 1);
        match self {
            Self::Scalar => scalar_pair::<REVERSE>(h, n, a, b, lo, hi),
            #[cfg(all(target_arch = "x86_64", not(any(miri, zscan_scalar))))]
            Self::Avx2 => {
                // SAFETY: this private variant records checked AVX2+OS support.
                // a,b are in n; every scanned start fits the complete needle.
                // Thus the callee's two unaligned loads fit h independently.
                unsafe { x86::pair::<REVERSE>(h, n, a, b, lo, hi) }
            }
            #[cfg(all(
                target_arch = "aarch64",
                target_feature = "neon",
                not(any(miri, zscan_scalar))
            ))]
            Self::Neon => {
                // SAFETY: target guarantees NEON; a,b and candidate starts
                // satisfy the same checked, read-only range invariant above.
                unsafe { neon::pair::<REVERSE>(h, n, a, b, lo, hi) }
            }
        }
    }
}

#[inline]
pub(crate) fn scalar_byte<const REVERSE: bool>(byte: u8, h: &[u8]) -> Option<usize> {
    let repeated = u64::from(byte) * 0x0101_0101_0101_0101;
    let mut lo = 0;
    let mut hi = h.len();
    while hi - lo >= 8 {
        let at = if REVERSE { hi - 8 } else { lo };
        let mask = zero_bytes(word(&h[at..at + 8]) ^ repeated);
        if mask != 0 {
            let bit = if REVERSE {
                63 - mask.leading_zeros()
            } else {
                mask.trailing_zeros()
            };
            return Some(at + bit as usize / 8);
        }
        if REVERSE {
            hi -= 8;
        } else {
            lo += 8;
        }
    }
    while lo < hi {
        let at = if REVERSE { hi - 1 } else { lo };
        if h[at] == byte {
            return Some(at);
        }
        if REVERSE {
            hi -= 1;
        } else {
            lo += 1;
        }
    }
    None
}

// Unlike the subtract-and-borrow test, this has no false positive bits in
// adjacent 0x01 lanes. Both forward and reverse bit extraction are exact.
#[inline]
fn zero_bytes(x: u64) -> u64 {
    const LOW: u64 = 0x7f7f_7f7f_7f7f_7f7f;
    !(((x & LOW).wrapping_add(LOW)) | x | LOW)
}

#[inline]
fn word(bytes: &[u8]) -> u64 {
    u64::from_le_bytes(bytes[..8].try_into().unwrap())
}

/// Equality without calling a C/FFI comparison routine. Bounds are checked
/// on exact 8-byte chunks; the tail is compared byte by byte.
#[inline]
pub(crate) fn equal(a: &[u8], b: &[u8]) -> bool {
    mismatch(a, b).is_none()
}

/// First different byte, used to replace an unhelpful filter probe.
#[inline]
pub(crate) fn mismatch(a: &[u8], b: &[u8]) -> Option<usize> {
    debug_assert_eq!(a.len(), b.len());
    let mut i = 0;
    while a.len() - i >= 8 {
        let different = word(&a[i..i + 8]) ^ word(&b[i..i + 8]);
        if different != 0 {
            return Some(i + different.trailing_zeros() as usize / 8);
        }
        i += 8;
    }
    while i < a.len() {
        if a[i] != b[i] {
            return Some(i);
        }
        i += 1;
    }
    None
}

#[inline]
fn scalar_pair<const REVERSE: bool>(
    h: &[u8],
    n: &[u8],
    a: usize,
    b: usize,
    start: usize,
    end: usize,
) -> Option<usize> {
    // Each anchor slice has exactly one byte per legal candidate. Establishing
    // these ranges once lets the word loop retain simple relative bounds.
    let left = &h[a..][start..end];
    let right = &h[b..][start..end];
    let first = u64::from(n[a]) * 0x0101_0101_0101_0101;
    let second = u64::from(n[b]) * 0x0101_0101_0101_0101;
    let length = left.len();
    let mut lo = 0;
    let mut hi = length;
    while hi - lo >= 8 {
        let at = if REVERSE { hi - 8 } else { lo };
        let mask = zero_bytes(word(&left[at..at + 8]) ^ first)
            & zero_bytes(word(&right[at..at + 8]) ^ second);
        if mask != 0 {
            let bit = if REVERSE {
                63 - mask.leading_zeros()
            } else {
                mask.trailing_zeros()
            };
            return Some(start + at + bit as usize / 8);
        }
        if REVERSE {
            hi -= 8;
        } else {
            lo += 8;
        }
    }
    if length >= 8 && lo < hi {
        // The complete boundary word stays inside both anchor slices. Its
        // overlap was already proved not to match, so it cannot change order.
        let at = if REVERSE { 0 } else { length - 8 };
        let mask = zero_bytes(word(&left[at..at + 8]) ^ first)
            & zero_bytes(word(&right[at..at + 8]) ^ second);
        if mask != 0 {
            let bit = if REVERSE {
                63 - mask.leading_zeros()
            } else {
                mask.trailing_zeros()
            };
            return Some(start + at + bit as usize / 8);
        }
        return None;
    }
    while lo < hi {
        let at = if REVERSE { hi - 1 } else { lo };
        if left[at] == n[a] && right[at] == n[b] {
            return Some(start + at);
        }
        if REVERSE {
            hi -= 1;
        } else {
            lo += 1;
        }
    }
    None
}

#[cfg(all(target_arch = "x86_64", not(any(miri, zscan_scalar))))]
#[allow(unused_unsafe)] // Register intrinsics were unsafe on Rust 1.70.
mod x86 {
    use core::arch::x86_64::*;
    use core::sync::atomic::{AtomicU8, Ordering};

    #[inline]
    pub(super) fn available() -> bool {
        if cfg!(target_feature = "avx2") {
            return true;
        }
        // Only a derived boolean is shared: Relaxed needs no publication fence.
        static DETECTED: AtomicU8 = AtomicU8::new(0);
        match DETECTED.load(Ordering::Relaxed) {
            1 => false,
            2 => true,
            _ => {
                let supported = detect();
                DETECTED.store(if supported { 2 } else { 1 }, Ordering::Relaxed);
                supported
            }
        }
    }

    #[cold]
    #[allow(unused_unsafe)] // CPUID became safe after the MSRV.
    fn detect() -> bool {
        // SAFETY: CPUID is guaranteed on x86-64. Leaf zero queries the largest
        // basic leaf. No pointers, aliases, alignment or shifts are involved.
        if unsafe { __cpuid(0).eax } < 7 {
            return false;
        }
        // SAFETY: CPUID exists and leaf 1 is supported after the leaf-0 check.
        let features = unsafe { __cpuid(1).ecx };
        const XSAVE_OSXSAVE_AVX: u32 = (1 << 26) | (1 << 27) | (1 << 28);
        if features & XSAVE_OSXSAVE_AVX != XSAVE_OSXSAVE_AVX {
            return false;
        }
        // SAFETY: XSAVE and OSXSAVE are checked above, enabling XGETBV.
        // XCR0 is valid index 0. Both XMM and YMM state must be OS-managed.
        if unsafe { _xgetbv(0) } & 6 != 6 {
            return false;
        }
        // SAFETY: leaf 7 exists; subleaf 0 always exists for leaf 7.
        unsafe { __cpuid_count(7, 0).ebx & (1 << 5) != 0 }
    }

    #[target_feature(enable = "avx2")]
    pub(super) unsafe fn byte<const REVERSE: bool>(value: u8, h: &[u8]) -> Option<usize> {
        if h.len() < 32 {
            return super::scalar_byte::<REVERSE>(value, h);
        }
        // SAFETY: caller established AVX2; register-only broadcast, no memory.
        let target = unsafe { _mm256_set1_epi8(value as i8) };
        let nearest = if REVERSE { h.len() - 32 } else { 0 };
        // SAFETY: h has at least 32 bytes and nearest selects its first/last
        // full vector. loadu permits byte alignment and shared read-only
        // aliases. The caller guarantees AVX2 and OS support.
        let mask = unsafe {
            _mm256_movemask_epi8(_mm256_cmpeq_epi8(
                _mm256_loadu_si256(h.as_ptr().add(nearest).cast()),
                target,
            ))
        } as u32;
        if mask != 0 {
            let lane = if REVERSE {
                31 - mask.leading_zeros()
            } else {
                mask.trailing_zeros()
            };
            return Some(nearest + lane as usize);
        }
        // Move to the next aligned start/end strictly inside the first vector
        // examined. The overlap is already known not to match. Only low five
        // address bits are used; wrapping computes alignment, never a pointer
        // for dereferencing. Every actual load still uses bounded slice offsets.
        let (mut lo, mut hi) = if REVERSE {
            let back = ((h.as_ptr() as usize).wrapping_add(h.len()).wrapping_sub(1) & 31) + 1;
            (0, h.len() - back)
        } else {
            (32 - (h.as_ptr() as usize & 31), h.len())
        };
        while hi - lo >= 128 {
            let at = if REVERSE { hi - 128 } else { lo };
            // SAFETY: the loop guarantees at..at+128 lies within h. All four
            // loadu operations allow arbitrary byte alignment and only read
            // the shared slice. AVX2 is active; no pointer escapes this block.
            let (a, b, c, d) = unsafe {
                (
                    _mm256_cmpeq_epi8(_mm256_loadu_si256(h.as_ptr().add(at).cast()), target),
                    _mm256_cmpeq_epi8(_mm256_loadu_si256(h.as_ptr().add(at + 32).cast()), target),
                    _mm256_cmpeq_epi8(_mm256_loadu_si256(h.as_ptr().add(at + 64).cast()), target),
                    _mm256_cmpeq_epi8(_mm256_loadu_si256(h.as_ptr().add(at + 96).cast()), target),
                )
            };
            // SAFETY: AVX2 is active; combining masks uses registers only.
            let any = unsafe {
                _mm256_movemask_epi8(_mm256_or_si256(
                    _mm256_or_si256(a, b),
                    _mm256_or_si256(c, d),
                ))
            };
            if any != 0 {
                let blocks = if REVERSE { [d, c, b, a] } else { [a, b, c, d] };
                for (i, block) in blocks.into_iter().enumerate() {
                    // SAFETY: AVX2 is active; extracting a register mask has
                    // no memory operands. Nonzero masks yield a lane in 0..32.
                    let mask = unsafe { _mm256_movemask_epi8(block) } as u32;
                    if mask != 0 {
                        let lane = if REVERSE {
                            31 - mask.leading_zeros()
                        } else {
                            mask.trailing_zeros()
                        };
                        let block_index = if REVERSE { 3 - i } else { i };
                        return Some(at + block_index * 32 + lane as usize);
                    }
                }
            }
            if REVERSE {
                hi -= 128;
            } else {
                lo += 128;
            }
        }
        while hi - lo >= 32 {
            let at = if REVERSE { hi - 32 } else { lo };
            // SAFETY: at..at+32 is within h. loadu permits byte alignment;
            // this shared slice is read only, and the pointer is not retained.
            let block = unsafe { _mm256_loadu_si256(h.as_ptr().add(at).cast()) };
            // SAFETY: AVX2 is active; these operations use registers only.
            let mask = unsafe { _mm256_movemask_epi8(_mm256_cmpeq_epi8(block, target)) } as u32;
            if mask != 0 {
                let lane = if REVERSE {
                    31 - mask.leading_zeros()
                } else {
                    mask.trailing_zeros()
                };
                return Some(at + lane as usize);
            }
            if REVERSE {
                hi -= 32;
            } else {
                lo += 32;
            }
        }
        if lo == hi {
            return None;
        }
        let at = if REVERSE { 0 } else { h.len() - 32 };
        // SAFETY: h.len() >= 32, so this entire boundary vector is inside h.
        // It overlaps only bytes already proved not to match. Thus selecting
        // the first/last set bit cannot return a previously skipped hit.
        // The read is unaligned, shared, and executed with AVX2 active.
        let mask = unsafe {
            _mm256_movemask_epi8(_mm256_cmpeq_epi8(
                _mm256_loadu_si256(h.as_ptr().add(at).cast()),
                target,
            ))
        } as u32;
        if mask == 0 {
            None
        } else {
            let lane = if REVERSE {
                31 - mask.leading_zeros()
            } else {
                mask.trailing_zeros()
            };
            Some(at + lane as usize)
        }
    }

    #[target_feature(enable = "avx2")]
    pub(super) unsafe fn pair<const REVERSE: bool>(
        h: &[u8],
        n: &[u8],
        a: usize,
        b: usize,
        mut lo: usize,
        mut hi: usize,
    ) -> Option<usize> {
        let start = lo;
        let end = hi;
        if end - start < 16 {
            return super::scalar_pair::<REVERSE>(h, n, a, b, lo, hi);
        }
        // SAFETY: caller established AVX2; both broadcasts use registers only.
        let (first, second) =
            unsafe { (_mm256_set1_epi8(n[a] as i8), _mm256_set1_epi8(n[b] as i8)) };
        while hi - lo >= 32 {
            let at = if REVERSE { hi - 32 } else { lo };
            // SAFETY: all 32 candidate starts fit a complete needle; a,b are
            // below n.len(). Thus at+a..at+a+32 and at+b..at+b+32 are within
            // h. loadu accepts arbitrary alignment, reads only shared bytes,
            // and AVX2 plus OS state support was checked before entry.
            let (x, y) = unsafe {
                (
                    _mm256_loadu_si256(h.as_ptr().add(at + a).cast()),
                    _mm256_loadu_si256(h.as_ptr().add(at + b).cast()),
                )
            };
            // SAFETY: AVX2 is active; comparison, AND and mask use registers.
            let mask = unsafe {
                _mm256_movemask_epi8(_mm256_and_si256(
                    _mm256_cmpeq_epi8(x, first),
                    _mm256_cmpeq_epi8(y, second),
                ))
            } as u32;
            if mask != 0 {
                let lane = if REVERSE {
                    31 - mask.leading_zeros()
                } else {
                    mask.trailing_zeros()
                };
                return Some(at + lane as usize);
            }
            if REVERSE {
                hi -= 32;
            } else {
                lo += 32;
            }
        }
        if end - start >= 32 {
            if lo == hi {
                return None;
            }
            let at = if REVERSE { start } else { end - 32 };
            // SAFETY: the original range has at least 32 legal starts, so
            // this full boundary block stays inside it. With a,b<n.len(),
            // both entire probe loads stay inside h. The overlap was tested
            // with these same probes and had no match, preserving direction.
            // loadu permits unaligned shared reads; AVX2 is active.
            let mask = unsafe {
                _mm256_movemask_epi8(_mm256_and_si256(
                    _mm256_cmpeq_epi8(_mm256_loadu_si256(h.as_ptr().add(at + a).cast()), first),
                    _mm256_cmpeq_epi8(_mm256_loadu_si256(h.as_ptr().add(at + b).cast()), second),
                ))
            } as u32;
            return if mask == 0 {
                None
            } else {
                let lane = if REVERSE {
                    31 - mask.leading_zeros()
                } else {
                    mask.trailing_zeros()
                };
                Some(at + lane as usize)
            };
        }
        // AVX2 implies SSE2. Use a complete 16-byte vector for medium tails;
        // this is still wholly in bounds, even when the haystack is unaligned.
        // Here the original range has 16..31 starts. Its two boundary vectors
        // cover the entire range, overlapping only already rejected starts.
        let (first, second) = unsafe {
            // SAFETY: AVX2 includes SSE2; these casts only select registers.
            (
                _mm256_castsi256_si128(first),
                _mm256_castsi256_si128(second),
            )
        };
        {
            let at = if REVERSE { hi - 16 } else { lo };
            // SAFETY: all 16 candidate starts fit the full needle. Both loads
            // therefore fit h, permit unaligned shared reads and use SSE2,
            // which is present whenever this AVX2 function is entered.
            let mask = unsafe {
                _mm_movemask_epi8(_mm_and_si128(
                    _mm_cmpeq_epi8(_mm_loadu_si128(h.as_ptr().add(at + a).cast()), first),
                    _mm_cmpeq_epi8(_mm_loadu_si128(h.as_ptr().add(at + b).cast()), second),
                ))
            } as u32;
            if mask != 0 {
                let lane = if REVERSE {
                    31 - mask.leading_zeros()
                } else {
                    mask.trailing_zeros()
                };
                return Some(at + lane as usize);
            }
            if end - start == 16 {
                return None;
            }
        }
        let at = if REVERSE { start } else { end - 16 };
        // SAFETY: the complete 16-start boundary block is in [start,end).
        // Each anchor offset is smaller than n.len(), so both loads fit h.
        // Overlapping starts were rejected above with identical probes.
        // SSE2 is active via AVX2, and reads are shared and unaligned.
        let mask = unsafe {
            _mm_movemask_epi8(_mm_and_si128(
                _mm_cmpeq_epi8(_mm_loadu_si128(h.as_ptr().add(at + a).cast()), first),
                _mm_cmpeq_epi8(_mm_loadu_si128(h.as_ptr().add(at + b).cast()), second),
            ))
        } as u32;
        if mask == 0 {
            None
        } else {
            let lane = if REVERSE {
                31 - mask.leading_zeros()
            } else {
                mask.trailing_zeros()
            };
            Some(at + lane as usize)
        }
    }
}

#[cfg(all(
    target_arch = "aarch64",
    target_feature = "neon",
    not(any(miri, zscan_scalar))
))]
#[allow(unused_unsafe)] // Register intrinsics were unsafe on Rust 1.70.
mod neon {
    use core::arch::aarch64::*;

    #[target_feature(enable = "neon")]
    pub(super) unsafe fn byte<const REVERSE: bool>(value: u8, h: &[u8]) -> Option<usize> {
        // SAFETY: NEON is active; broadcast has no memory operands.
        let target = unsafe { vdupq_n_u8(value) };
        let mut lo = 0;
        let mut hi = h.len();
        while hi - lo >= 16 {
            let at = if REVERSE { hi - 16 } else { lo };
            // SAFETY: at..at+16 is inside h. vld1q_u8 permits unaligned
            // shared reads. NEON is guaranteed by the compilation target.
            let block = unsafe { vld1q_u8(h.as_ptr().add(at)) };
            // SAFETY: NEON is active; comparison and reduction use registers.
            if unsafe { vmaxvq_u8(vceqq_u8(block, target)) } != 0 {
                return super::scalar_byte::<REVERSE>(value, &h[at..at + 16]).map(|i| at + i);
            }
            if REVERSE {
                hi -= 16;
            } else {
                lo += 16;
            }
        }
        super::scalar_byte::<REVERSE>(value, &h[lo..hi]).map(|i| lo + i)
    }

    #[target_feature(enable = "neon")]
    pub(super) unsafe fn pair<const REVERSE: bool>(
        h: &[u8],
        n: &[u8],
        a: usize,
        b: usize,
        mut lo: usize,
        mut hi: usize,
    ) -> Option<usize> {
        // SAFETY: NEON is active; broadcasts have no memory operands.
        let (first, second) = unsafe { (vdupq_n_u8(n[a]), vdupq_n_u8(n[b])) };
        while hi - lo >= 16 {
            let at = if REVERSE { hi - 16 } else { lo };
            // SAFETY: each candidate fits n; a,b<n.len(), so both complete
            // 16-byte loads are within h. Arbitrary alignment and shared
            // aliasing are valid for these read-only loads. NEON is guaranteed.
            let (x, y) = unsafe {
                (
                    vld1q_u8(h.as_ptr().add(at + a)),
                    vld1q_u8(h.as_ptr().add(at + b)),
                )
            };
            // SAFETY: NEON is active; all operations below are register-only.
            if unsafe { vmaxvq_u8(vandq_u8(vceqq_u8(x, first), vceqq_u8(y, second))) } != 0 {
                return super::scalar_pair::<REVERSE>(h, n, a, b, at, at + 16);
            }
            if REVERSE {
                hi -= 16;
            } else {
                lo += 16;
            }
        }
        super::scalar_pair::<REVERSE>(h, n, a, b, lo, hi)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_backends() {
        let backend = Backend::detect();
        let mut storage = [17; 384];
        let offsets = if cfg!(miri) { 3 } else { 64 };
        for offset in 0..offsets {
            for len in [0, 1, 2, 7, 8, 15, 16, 17, 31, 32, 33, 63, 64, 65, 129, 257] {
                if cfg!(miri) && len > 33 {
                    continue;
                }
                let h = &mut storage[offset..offset + len];
                h.fill(17);
                for hit in 0..=len {
                    if hit < len {
                        h[hit] = 255;
                    }
                    let first = h.iter().position(|&b| b == 255);
                    let last = h.iter().rposition(|&b| b == 255);
                    assert_eq!(scalar_byte::<false>(255, h), first);
                    assert_eq!(scalar_byte::<true>(255, h), last);
                    assert_eq!(backend.byte::<false>(255, h), first);
                    assert_eq!(backend.byte::<true>(255, h), last);
                    if hit < len {
                        h[hit] = 17;
                    }
                }
            }
        }
        // Exercise the SWAR borrow trap in both endianness-independent scans.
        for value in 0..=255 {
            for zero in 0..8 {
                let mut h = [value ^ 1; 8];
                h[zero] = value;
                assert_eq!(scalar_byte::<false>(value, &h), Some(zero));
                assert_eq!(scalar_byte::<true>(value, &h), Some(zero));
            }
        }
    }

    #[test]
    fn forced_backend_is_exercised() {
        #[cfg(zscan_force_avx2)]
        {
            assert!(std::is_x86_feature_detected!("avx2"));
            assert!(matches!(Backend::detect(), Backend::Avx2));
        }
        #[cfg(zscan_force_neon)]
        assert!(matches!(Backend::detect(), Backend::Neon));
    }

    #[test]
    fn pair_backends() {
        let backend = Backend::detect();
        let mut h = [19u8; 352];
        let mut n = [19u8; 65];
        for length in [2, 3, 8, 17, 32, 33, 65] {
            if cfg!(miri) && length > 8 {
                continue;
            }
            for a in [0, length / 2] {
                for b in [length - 1, 1] {
                    n[..length].fill(19);
                    n[a] = 71;
                    n[b] = 97;
                    for offset in 0..if cfg!(miri) { 2 } else { 32 } {
                        let haystack = &mut h[offset..offset + 257];
                        for at in [0, 1, 15, 16, 31, 32, 63, 128] {
                            haystack.fill(19);
                            haystack[at + a] = n[a];
                            haystack[at + b] = n[b];
                            let end = haystack.len() - length + 1;
                            for (lo, hi) in [(0, end), (at, end), (0, at + 1), (at + 1, end)] {
                                assert_eq!(
                                    backend.pair::<false>(haystack, &n[..length], a, b, lo, hi),
                                    scalar_pair::<false>(haystack, &n[..length], a, b, lo, hi)
                                );
                                assert_eq!(
                                    backend.pair::<true>(haystack, &n[..length], a, b, lo, hi),
                                    scalar_pair::<true>(haystack, &n[..length], a, b, lo, hi)
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}
