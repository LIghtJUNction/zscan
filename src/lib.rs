#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]
#![warn(missing_docs)]
#![allow(unknown_lints, unexpected_cfgs)]
//! Allocation-free search in arbitrary byte slices.
//!
//! Indices refer to the original haystack. Empty needles match at zero when
//! searching forwards and at the haystack length when searching backwards.
//! Substring search takes worst-case O(n + m) time and O(1) extra space.
//!
//! ```
//! use zscan::{find, rfind, Finder};
//! assert_eq!(find(b"ab\0ab", b"\0a"), Some(2));
//! assert_eq!(rfind(b"ababa", b"aba"), Some(2));
//! let finder = Finder::new(b"ab");
//! assert_eq!(finder.find(b"xxab"), Some(2));
//! assert_eq!(finder.rfind(b"abxxab"), Some(4));
//! ```

#[cfg(test)]
extern crate std;

mod bytes;
mod prefilter;
mod twoway;

/// Finds the first occurrence of `byte` in `haystack`.
#[inline]
pub fn find_byte(byte: u8, haystack: &[u8]) -> Option<usize> {
    if haystack.len() < 32 {
        bytes::scalar_byte::<false>(byte, haystack)
    } else {
        bytes::Backend::detect().byte::<false>(byte, haystack)
    }
}

/// Finds the last occurrence of `byte` in `haystack`.
#[inline]
pub fn rfind_byte(byte: u8, haystack: &[u8]) -> Option<usize> {
    if haystack.len() < 32 {
        bytes::scalar_byte::<true>(byte, haystack)
    } else {
        bytes::Backend::detect().byte::<true>(byte, haystack)
    }
}

/// Finds the first occurrence of `needle` in `haystack`.
///
/// An empty needle returns `Some(0)`; an overlong needle returns `None`.
#[inline]
pub fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    search::<false>(haystack, needle, None)
}

/// Finds the last occurrence of `needle` in `haystack`.
///
/// An empty needle returns `Some(haystack.len())`.
#[inline]
pub fn rfind(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    search::<true>(haystack, needle, None)
}

/// A borrowed needle with fixed-size preprocessing for both search directions.
///
/// Construction takes O(m) time and O(1) space. Searches take O(n + m) time
/// in the worst case. The needle is never copied, and searching backwards
/// does not allocate or reverse either input.
#[derive(Clone, Debug)]
pub struct Finder<'n> {
    needle: &'n [u8],
    forward: twoway::TwoWay,
    reverse: twoway::TwoWay,
    filter: prefilter::Filter,
    backend: bytes::Backend,
}

impl<'n> Finder<'n> {
    /// Preprocesses a borrowed needle for repeated searches in either direction.
    #[inline]
    pub fn new(needle: &'n [u8]) -> Self {
        Self {
            needle,
            forward: twoway::TwoWay::new::<false>(needle),
            reverse: twoway::TwoWay::new::<true>(needle),
            filter: prefilter::Filter::new(needle),
            backend: bytes::Backend::detect(),
        }
    }

    /// Finds the first occurrence of this needle in `haystack`.
    #[inline]
    pub fn find(&self, haystack: &[u8]) -> Option<usize> {
        search::<false>(haystack, self.needle, Some(self))
    }

    /// Finds the last occurrence of this needle in `haystack`.
    #[inline]
    pub fn rfind(&self, haystack: &[u8]) -> Option<usize> {
        search::<true>(haystack, self.needle, Some(self))
    }
}

#[inline]
fn search<const REVERSE: bool>(h: &[u8], n: &[u8], cache: Option<&Finder<'_>>) -> Option<usize> {
    if n.len() > h.len() {
        return None;
    }
    if n.is_empty() {
        return Some(if REVERSE { h.len() } else { 0 });
    }
    if n.len() == 1 {
        if h.len() >= 32
            && let Some(finder) = cache
        {
            return finder.backend.byte::<REVERSE>(n[0], h);
        }
        return if REVERSE {
            rfind_byte(n[0], h)
        } else {
            find_byte(n[0], h)
        };
    }
    if !cfg!(zscan_no_short) {
        if h.len() <= 32 {
            return prefilter::short::<REVERSE>(h, n);
        }
        // A common first/last boundary hit needs neither factorization nor
        // runtime dispatch. One extra O(m) comparison keeps the linear bound.
        let boundary = if REVERSE { h.len() - n.len() } else { 0 };
        if h[boundary] == n[0] && bytes::equal(&h[boundary..boundary + n.len()], n) {
            return Some(boundary);
        }
    }
    match cache {
        Some(finder) => prefilter::find::<REVERSE>(
            h,
            n,
            finder.filter,
            finder.backend,
            Some(if REVERSE {
                &finder.reverse
            } else {
                &finder.forward
            }),
        ),
        None => prefilter::find::<REVERSE>(
            h,
            n,
            prefilter::Filter::new(n),
            bytes::Backend::detect(),
            None,
        ),
    }
}
