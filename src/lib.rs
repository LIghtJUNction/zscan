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

mod twoway;

/// Finds the first occurrence of `byte` in `haystack`.
#[inline]
pub fn find_byte(byte: u8, haystack: &[u8]) -> Option<usize> {
    haystack.iter().position(|&b| b == byte)
}

/// Finds the last occurrence of `byte` in `haystack`.
#[inline]
pub fn rfind_byte(byte: u8, haystack: &[u8]) -> Option<usize> {
    haystack.iter().rposition(|&b| b == byte)
}

/// Finds the first occurrence of `needle` in `haystack`.
///
/// An empty needle returns `Some(0)`; an overlong needle returns `None`.
#[inline]
pub fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.len() > haystack.len() {
        return None;
    }
    match needle.len() {
        0 => Some(0),
        1 => find_byte(needle[0], haystack),
        _ => twoway::TwoWay::new::<false>(needle).find::<false>(haystack, needle),
    }
}

/// Finds the last occurrence of `needle` in `haystack`.
///
/// An empty needle returns `Some(haystack.len())`.
#[inline]
pub fn rfind(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.len() > haystack.len() {
        return None;
    }
    match needle.len() {
        0 => Some(haystack.len()),
        1 => rfind_byte(needle[0], haystack),
        _ => twoway::TwoWay::new::<true>(needle).find::<true>(haystack, needle),
    }
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
}

impl<'n> Finder<'n> {
    /// Preprocesses a borrowed needle for repeated searches in either direction.
    #[inline]
    pub fn new(needle: &'n [u8]) -> Self {
        Self {
            needle,
            forward: twoway::TwoWay::new::<false>(needle),
            reverse: twoway::TwoWay::new::<true>(needle),
        }
    }

    /// Finds the first occurrence of this needle in `haystack`.
    #[inline]
    pub fn find(&self, haystack: &[u8]) -> Option<usize> {
        if self.needle.len() > haystack.len() {
            return None;
        }
        match self.needle.len() {
            0 => Some(0),
            1 => find_byte(self.needle[0], haystack),
            _ => self.forward.find::<false>(haystack, self.needle),
        }
    }

    /// Finds the last occurrence of this needle in `haystack`.
    #[inline]
    pub fn rfind(&self, haystack: &[u8]) -> Option<usize> {
        if self.needle.len() > haystack.len() {
            return None;
        }
        match self.needle.len() {
            0 => Some(haystack.len()),
            1 => rfind_byte(self.needle[0], haystack),
            _ => self.reverse.find::<true>(haystack, self.needle),
        }
    }
}
