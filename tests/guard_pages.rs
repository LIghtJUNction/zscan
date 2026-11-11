//! Hardware fault boundaries catch reads outside slices at either page edge.
//! libc is used only by this test executable; zscan itself has no FFI.
#![cfg(all(
    target_os = "linux",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    not(miri)
))]

use std::ffi::c_void;
use std::ops::Range;
use std::os::raw::{c_int, c_long};
use std::ptr::NonNull;
use zscan::{Finder, find, find_byte, rfind, rfind_byte};

const PROT_NONE: c_int = 0;
const PROT_READ_WRITE: c_int = 1 | 2;
const MAP_PRIVATE_ANONYMOUS: c_int = 2 | 0x20;
const SC_PAGESIZE: c_int = 30;

extern "C" {
    fn mmap(
        address: *mut c_void,
        length: usize,
        protection: c_int,
        flags: c_int,
        fd: c_int,
        offset: i64,
    ) -> *mut c_void;
    fn mprotect(address: *mut c_void, length: usize, protection: c_int) -> c_int;
    fn munmap(address: *mut c_void, length: usize) -> c_int;
    fn sysconf(name: c_int) -> c_long;
}

struct GuardedPage {
    base: NonNull<u8>,
    page_length: usize,
}

impl GuardedPage {
    fn new() -> Self {
        // SAFETY: sysconf has no pointer arguments, memory access or CPU
        // feature requirements. SC_PAGESIZE is the Linux libc selector.
        let page_length = unsafe { sysconf(SC_PAGESIZE) };
        assert!(
            page_length > 0,
            "sysconf: {}",
            std::io::Error::last_os_error()
        );
        let page_length = page_length as usize;
        let total = page_length.checked_mul(3).expect("mapping size overflow");
        // SAFETY: a null address requests a fresh page-aligned mapping; fd=-1
        // and offset=0 select anonymous zero-initialized storage. No Rust
        // allocation or reference aliases this mapping. All three pages
        // initially prohibit access, so no guard bytes are dereferenced.
        let raw = unsafe {
            mmap(
                std::ptr::null_mut(),
                total,
                PROT_NONE,
                MAP_PRIVATE_ANONYMOUS,
                -1,
                0,
            )
        };
        assert_ne!(
            raw as isize,
            -1,
            "mmap: {}",
            std::io::Error::last_os_error()
        );
        let base = match NonNull::new(raw.cast::<u8>()) {
            Some(base) => base,
            None => {
                // SAFETY: even a successful mapping at address zero is owned
                // by this call and has exactly `total` bytes. It cannot be
                // represented as a Rust slice; release it before failing.
                assert_eq!(unsafe { munmap(raw, total) }, 0);
                panic!("mmap returned an address unsuitable for Rust slices");
            }
        };
        let page = Self { base, page_length };
        // SAFETY: mmap returned a page-aligned three-page allocation. The
        // one-page displacement stays inside it, and the one-page range is
        // exactly the middle page. No references exist yet. This changes
        // permissions without reading inaccessible bytes or using SIMD.
        let result = unsafe {
            mprotect(
                page.base.as_ptr().add(page_length).cast(),
                page_length,
                PROT_READ_WRITE,
            )
        };
        assert_eq!(result, 0, "mprotect: {}", std::io::Error::last_os_error());
        page
    }

    fn bytes_mut(&mut self) -> &mut [u8] {
        // SAFETY: only the initialized, writable middle page is exposed.
        // Its pointer is non-null, page-aligned (hence u8-aligned), and the
        // one-page displacement plus length remain in this mapping. `&mut
        // self` prevents another live borrow of the page. The slice cannot
        // outlive the owner or survive munmap; no CPU features are required.
        unsafe {
            std::slice::from_raw_parts_mut(
                self.base.as_ptr().add(self.page_length),
                self.page_length,
            )
        }
    }

    fn bytes(&self, range: Range<usize>) -> &[u8] {
        // SAFETY: mmap zero-initialized the readable middle page, and the
        // pointer displacement and length stay within that page. The owner
        // remains borrowed, so no mutation or munmap can overlap these shared
        // references. Byte alignment is one; no vector instruction is used.
        let page = unsafe {
            std::slice::from_raw_parts(self.base.as_ptr().add(self.page_length), self.page_length)
        };
        &page[range]
    }
}

impl Drop for GuardedPage {
    fn drop(&mut self) {
        // SAFETY: this owner created the entire three-page mapping and is
        // never cloned. Exclusive destruction guarantees every slice borrow
        // has ended. munmap receives the original aligned base and exact
        // mapping size; it does not dereference the two inaccessible pages.
        let result = unsafe {
            munmap(
                self.base.as_ptr().cast(),
                self.page_length.checked_mul(3).unwrap(),
            )
        };
        assert_eq!(result, 0, "munmap: {}", std::io::Error::last_os_error());
    }
}

fn region(page_length: usize, length: usize, trailing: bool, gap: usize) -> Range<usize> {
    assert!(length <= page_length && gap <= page_length - length);
    let start = if trailing {
        page_length - length - gap
    } else {
        gap
    };
    start..start + length
}

fn reference(haystack: &[u8], needle: &[u8], reverse: bool) -> Option<usize> {
    if needle.len() > haystack.len() {
        return None;
    }
    let candidates = 0..=haystack.len() - needle.len();
    let matches = |&i: &usize| (0..needle.len()).all(|j| haystack[i + j] == needle[j]);
    if reverse {
        candidates.rev().find(matches)
    } else {
        candidates.clone().find(matches)
    }
}

fn check(haystack: &[u8], needle: &[u8]) {
    let first = reference(haystack, needle, false);
    let last = reference(haystack, needle, true);
    let finder = Finder::new(needle);
    assert_eq!(find(haystack, needle), first);
    assert_eq!(rfind(haystack, needle), last);
    assert_eq!(finder.find(haystack), first);
    assert_eq!(finder.rfind(haystack), last);
}

fn set_needle(page: &mut GuardedPage, range: Range<usize>) {
    let bytes = &mut page.bytes_mut()[range];
    for (i, byte) in bytes.iter_mut().enumerate() {
        *byte = 0x80 + (i % 127) as u8;
    }
}

fn exercise(
    haystack_page: &mut GuardedPage,
    haystack: Range<usize>,
    needle_page: &GuardedPage,
    needle: Range<usize>,
    position: Option<usize>,
) {
    let needle = needle_page.bytes(needle);
    let buffer = &mut haystack_page.bytes_mut()[haystack.clone()];
    buffer.fill(0x11);
    if let Some(position) = position {
        buffer[position..position + needle.len()].copy_from_slice(needle);
    }
    check(haystack_page.bytes(haystack), needle);
}

#[test]
fn byte_search_guarded_edges_and_offsets() {
    let mut page = GuardedPage::new();
    let mut lengths: Vec<usize> = (0..=129).collect();
    lengths.extend([
        255,
        256,
        257,
        511,
        512,
        513,
        page.page_length - 1,
        page.page_length,
    ]);
    for length in lengths {
        for gap in 0..64.min(page.page_length - length + 1) {
            for trailing in [false, true] {
                let range = region(page.page_length, length, trailing, gap);
                for position in [None, Some(0), length.checked_sub(1)] {
                    let buffer = &mut page.bytes_mut()[range.clone()];
                    buffer.fill(0x11);
                    if let Some(position) = position.filter(|&position| position < length) {
                        buffer[position] = 0xe7;
                    }
                    let haystack = page.bytes(range.clone());
                    assert_eq!(
                        find_byte(0xe7, haystack),
                        haystack.iter().position(|&b| b == 0xe7)
                    );
                    assert_eq!(
                        rfind_byte(0xe7, haystack),
                        haystack.iter().rposition(|&b| b == 0xe7)
                    );
                }
            }
        }
    }
}

#[test]
fn substring_search_at_both_guard_pages() {
    let mut h_page = GuardedPage::new();
    let mut n_page = GuardedPage::new();
    let mut h_lengths: Vec<usize> = (0..=129).collect();
    h_lengths.extend([
        255,
        256,
        257,
        511,
        512,
        513,
        h_page.page_length - 1,
        h_page.page_length,
    ]);
    let n_lengths = [
        0,
        1,
        2,
        3,
        4,
        7,
        8,
        15,
        16,
        17,
        31,
        32,
        33,
        63,
        64,
        65,
        127,
        128,
        129,
        255,
        n_page.page_length,
    ];
    for h_length in h_lengths {
        for n_length in n_lengths {
            for h_trailing in [false, true] {
                for n_trailing in [false, true] {
                    let h = region(h_page.page_length, h_length, h_trailing, 0);
                    let n = region(n_page.page_length, n_length, n_trailing, 0);
                    set_needle(&mut n_page, n.clone());
                    exercise(&mut h_page, h.clone(), &n_page, n.clone(), None);
                    if n_length <= h_length {
                        exercise(&mut h_page, h.clone(), &n_page, n.clone(), Some(0));
                        exercise(&mut h_page, h, &n_page, n, Some(h_length - n_length));
                    }
                }
            }
        }
    }
}

#[test]
fn substring_alignment_and_simd_tails() {
    let mut h_page = GuardedPage::new();
    let mut n_page = GuardedPage::new();
    for offset in 0..64 {
        for h_length in [
            0, 1, 2, 7, 15, 16, 17, 31, 32, 33, 63, 64, 65, 95, 96, 97, 127, 128, 129, 255, 256,
            257, 511, 512, 513,
        ] {
            for n_length in [
                0, 1, 2, 3, 4, 7, 8, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129,
            ] {
                for trailing in [false, true] {
                    let h = region(h_page.page_length, h_length, trailing, offset);
                    // Multiplication by an odd number permutes all 64 offsets.
                    let n = region(n_page.page_length, n_length, trailing, (offset * 17) % 64);
                    set_needle(&mut n_page, n.clone());
                    exercise(&mut h_page, h.clone(), &n_page, n.clone(), None);
                    if n_length <= h_length {
                        exercise(&mut h_page, h.clone(), &n_page, n.clone(), Some(0));
                        exercise(&mut h_page, h, &n_page, n, Some(h_length - n_length));
                    }
                }
            }
        }
    }
}

#[test]
fn every_short_match_position_at_guard_edges() {
    let mut h_page = GuardedPage::new();
    let mut n_page = GuardedPage::new();
    for h_length in 0..=48 {
        for n_length in 0..=h_length.min(16) {
            for h_trailing in [false, true] {
                for n_trailing in [false, true] {
                    let h = region(h_page.page_length, h_length, h_trailing, 0);
                    let n = region(n_page.page_length, n_length, n_trailing, 0);
                    set_needle(&mut n_page, n.clone());
                    for position in 0..=h_length - n_length {
                        exercise(&mut h_page, h.clone(), &n_page, n.clone(), Some(position));
                    }
                }
            }
        }
    }
}
