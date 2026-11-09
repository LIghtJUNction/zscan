//! Bounded candidate verification followed by a single Two-Way fallback.

use crate::{bytes, twoway::TwoWay};

#[derive(Clone, Copy, Debug)]
pub(crate) struct Filter {
    a: usize,
    b: usize,
}

impl Filter {
    pub(crate) fn new(needle: &[u8]) -> Self {
        let a = 0;
        let mut b = needle.len().saturating_sub(1);
        if !needle.is_empty() && needle[a] == needle[b] {
            // At most 32 bytes of probe selection, even for huge needles.
            // Equal endpoints are common in periodic or structured data.
            for i in (1..needle.len().min(33)).rev() {
                if needle[i] != needle[a] {
                    b = i;
                    break;
                }
            }
        }
        Self { a, b }
    }
}

pub(crate) fn find<const REVERSE: bool>(
    h: &[u8],
    n: &[u8],
    mut filter: Filter,
    backend: bytes::Backend,
    cached: Option<&TwoWay>,
) -> Option<usize> {
    debug_assert!(n.len() >= 2 && n.len() <= h.len());
    if cfg!(zscan_no_prefilter) {
        return fallback::<REVERSE>(h, n, cached);
    }
    let mut lo = 0;
    let mut hi = h.len() - n.len() + 1;
    // Each failed full comparison is charged its maximum cost, n.len().
    // Only newly skipped candidates replenish this search-wide budget. Total
    // verification work is <= 2*m + n; a single O(n+m) Two-Way fallback
    // completes the search. Saturation only underestimates mathematical credit.
    let mut budget = n.len().saturating_mul(2);
    while lo < hi {
        let candidate = backend.pair::<REVERSE>(h, n, filter.a, filter.b, lo, hi)?;
        let skipped = if REVERSE {
            hi - 1 - candidate
        } else {
            candidate - lo
        };
        budget = budget.saturating_add(skipped);
        if budget < n.len() {
            return if REVERSE {
                fallback::<true>(&h[..candidate + n.len()], n, cached)
            } else {
                fallback::<false>(&h[candidate..], n, cached).map(|i| candidate + i)
            };
        }
        budget -= n.len();
        match bytes::mismatch(&h[candidate..candidate + n.len()], n) {
            None => return Some(candidate),
            Some(index) => {
                // The failed comparison identifies a useful third byte for
                // this haystack. Replace the second probe, without restarting
                // the scan or resetting its budget. This skips repeated false
                // positives at SIMD speed while preserving the linear proof.
                filter.b = index;
            }
        }
        if REVERSE {
            hi = candidate;
        } else {
            lo = candidate + 1;
        }
    }
    None
}

#[inline]
fn fallback<const REVERSE: bool>(h: &[u8], n: &[u8], cached: Option<&TwoWay>) -> Option<usize> {
    match cached {
        Some(metadata) => metadata.find::<REVERSE>(h, n),
        None => TwoWay::new::<REVERSE>(n).find::<REVERSE>(h, n),
    }
}

/// With at most 32 haystack bytes, naive verification has a fixed <=32
/// comparisons per candidate. No preprocessing or CPU dispatch is needed.
#[inline]
pub(crate) fn short<const REVERSE: bool>(h: &[u8], n: &[u8]) -> Option<usize> {
    let mut lo = 0;
    let mut hi = h.len() - n.len() + 1;
    while lo < hi {
        let at = if REVERSE { hi - 1 } else { lo };
        if h[at] == n[0] && bytes::equal(&h[at..at + n.len()], n) {
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
