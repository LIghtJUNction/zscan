//! These references deliberately do no preprocessing, filtering or word loads.
use zscan::{Finder, find, find_byte, rfind, rfind_byte};

fn reference(haystack: &[u8], needle: &[u8], reverse: bool) -> Option<usize> {
    if needle.len() > haystack.len() {
        return None;
    }
    let mut answer = None;
    for position in 0..=haystack.len() - needle.len() {
        let mut equal = true;
        for i in 0..needle.len() {
            if haystack[position + i] != needle[i] {
                equal = false;
                break;
            }
        }
        if equal {
            answer = Some(position);
            if !reverse {
                break;
            }
        }
    }
    answer
}

fn check(haystack: &[u8], needle: &[u8]) {
    let first = reference(haystack, needle, false);
    let last = reference(haystack, needle, true);
    let finder = Finder::new(needle);
    assert_eq!(find(haystack, needle), first, "h={haystack:?} n={needle:?}");
    assert_eq!(rfind(haystack, needle), last, "h={haystack:?} n={needle:?}");
    assert_eq!(finder.find(haystack), first, "h={haystack:?} n={needle:?}");
    assert_eq!(finder.rfind(haystack), last, "h={haystack:?} n={needle:?}");
    if needle.len() == 1 {
        assert_eq!(find_byte(needle[0], haystack), first);
        assert_eq!(rfind_byte(needle[0], haystack), last);
    }
}

fn words(max_length: usize) -> Vec<Vec<u8>> {
    let mut words = Vec::new();
    for length in 0..=max_length {
        for mut code in 0..3usize.pow(length as u32) {
            let mut word = vec![0; length];
            for byte in &mut word {
                *byte = [0, 1, 255][code % 3];
                code /= 3;
            }
            words.push(word);
        }
    }
    words
}

#[test]
fn exhaustive_ternary() {
    let haystacks = words(if cfg!(miri) { 3 } else { 6 });
    let needles = words(if cfg!(miri) { 2 } else { 4 });
    for haystack in &haystacks {
        for needle in &needles {
            check(haystack, needle);
        }
    }
}

struct Random(u64);

impl Random {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn bytes(&mut self, length: usize, mask: u8) -> Vec<u8> {
        (0..length).map(|_| self.next() as u8 & mask).collect()
    }
}

#[test]
fn fixed_seed_random() {
    let mut random = Random(0xe891_73f5_64ab_02cd);
    let rounds = if cfg!(miri) { 80 } else { 10_000 };
    let max_length = if cfg!(miri) { 100 } else { 2048 };
    for round in 0..rounds {
        let h_len = random.next() as usize % max_length;
        let n_len = random.next() as usize % 257;
        let mask = [1, 3, 15, 255][round % 4];
        let mut haystack = random.bytes(h_len, mask);
        let needle = random.bytes(n_len, mask);
        if round % 2 == 0 && n_len <= h_len {
            let position = random.next() as usize % (h_len - n_len + 1);
            haystack[position..position + n_len].copy_from_slice(&needle);
        }
        check(&haystack, &needle);
        let byte = random.next() as u8;
        assert_eq!(
            find_byte(byte, &haystack),
            haystack.iter().position(|&b| b == byte)
        );
        assert_eq!(
            rfind_byte(byte, &haystack),
            haystack.iter().rposition(|&b| b == byte)
        );
    }
}

#[test]
fn all_match_positions_and_offsets() {
    let alignments = if cfg!(miri) { 3 } else { 64 };
    let haystack_length = if cfg!(miri) { 35 } else { 129 };
    for offset in 0..alignments {
        for length in [0, 1, 2, 3, 4, 7, 8, 15, 16, 17, 31, 32, 33, 63, 64, 65] {
            if length > haystack_length {
                continue;
            }
            let needle_offset = (offset * 17) % 64;
            let mut needle_storage = vec![0x33; needle_offset + length + 64];
            let needle = &mut needle_storage[needle_offset..needle_offset + length];
            for (i, byte) in needle.iter_mut().enumerate() {
                *byte = 0x80 + i as u8;
            }
            for position in 0..=haystack_length - length {
                let mut storage = vec![0x11; offset + haystack_length + 64];
                let haystack = &mut storage[offset..offset + haystack_length];
                haystack[position..position + length].copy_from_slice(needle);
                check(haystack, needle);
            }
        }
    }
}

#[test]
fn periodic_and_low_entropy() {
    let sizes = if cfg!(miri) {
        &[32, 65, 129][..]
    } else {
        &[64, 257, 1024, 16_384][..]
    };
    for &size in sizes {
        for period in [1, 2, 3, 7, 16, 31] {
            let haystack: Vec<u8> = (0..size).map(|i| (i % period) as u8).collect();
            for length in [1, 2, 3, 8, 15, 32, 64, 127, 256, 1023] {
                if length > size {
                    continue;
                }
                let mut needle = haystack[..length].to_vec();
                check(&haystack, &needle);
                for index in [0, length / 2, length - 1] {
                    needle[index] = 255;
                    check(&haystack, &needle);
                    // Also force a late match after many plausible prefixes.
                    let mut with_match = haystack.clone();
                    with_match[size - length..].copy_from_slice(&needle);
                    check(&with_match, &needle);
                    needle.copy_from_slice(&haystack[..length]);
                }
            }
        }
    }
}

#[test]
fn binary_and_empty_semantics() {
    for haystack in [&[][..], b"a", b"\0", b"\xff\xfe\0\xc0\x80\0\xff"] {
        assert_eq!(find(haystack, b""), Some(0));
        assert_eq!(rfind(haystack, b""), Some(haystack.len()));
        check(haystack, b"");
        for needle in [b"\0".as_slice(), b"\xff\xfe", b"\xc0\x80", b"\0\xff"] {
            check(haystack, needle);
        }
    }
    check(b"", b"x");
    check(b"abc", b"abcd");
    check(b"\0abc\0abc\0", b"abc\0");
    check(b"aaaaaaa", b"aaa");
    check(b"abababababa", b"ababa");
    let all_bytes: Vec<u8> = (0..=255).collect();
    for value in 0..=255 {
        check(&all_bytes, &[value]);
    }
    for length in 0..=all_bytes.len() {
        check(&all_bytes, &all_bytes[..length]);
        check(&all_bytes, &all_bytes[all_bytes.len() - length..]);
    }
}

#[test]
fn reusable_finder() {
    let needles = [b"".as_slice(), b"a", b"abc", b"abababa", b"\0\xff\0"];
    let haystacks = [
        b"".as_slice(),
        b"a",
        b"---abc---abc",
        b"abababababababa",
        b"\0\xff\0\xff\0",
        b"no match",
    ];
    for needle in needles {
        let finder = Finder::new(needle);
        // Interleave directions and unrelated buffers: no mutable per-search
        // state may leak from one call into the next.
        for haystack in haystacks.into_iter().cycle().take(36) {
            assert_eq!(finder.rfind(haystack), reference(haystack, needle, true));
            assert_eq!(finder.find(haystack), reference(haystack, needle, false));
        }
    }
}

#[test]
fn pair_filter_anchor_choices_and_candidate_tails() {
    let offsets = if cfg!(miri) {
        &[1][..]
    } else {
        &[0, 1, 7, 15, 31, 63][..]
    };
    let lengths = if cfg!(miri) {
        &[2, 33, 65][..]
    } else {
        &[2, 3, 8, 16, 17, 31, 32, 33, 64, 65, 96, 129][..]
    };
    let tails = if cfg!(miri) {
        &[32][..]
    } else {
        &[31, 32, 33, 63, 64, 65][..]
    };
    for &offset in offsets {
        for &length in lengths {
            for variant in 0..4 {
                let mut needle = vec![0x80; length];
                match variant {
                    // Distinct endpoint probes.
                    0 => needle[length - 1] = 0xfe,
                    // Equal endpoints with a different probe near the front.
                    1 => needle[(length - 1).min(17)] = 0x81,
                    // The differing byte is beyond the bounded probe search
                    // for long needles: dense false positives must fall back.
                    2 => needle[length / 2] = 0x82,
                    // Repeated patterns make both filters and full candidate
                    // verification encounter overlapping plausible matches.
                    _ => {
                        for (i, byte) in needle.iter_mut().enumerate() {
                            *byte += (i % 3 == 1) as u8;
                        }
                    }
                }
                for &extra in tails {
                    let haystack_length = length + extra;
                    // All inputs exceed the short-haystack cutoff, while the
                    // number of legal starts straddles SIMD block boundaries.
                    assert!(haystack_length > 32);
                    let mut original = vec![0x80; haystack_length];
                    if variant == 1 {
                        for i in (3..haystack_length).step_by(7) {
                            original[i] = 0x81;
                        }
                    }
                    for position in [None, Some(0), Some(1), Some(31), Some(32), Some(extra)] {
                        if position.is_some_and(|position| position > extra) {
                            continue;
                        }
                        let mut storage = vec![0x53; offset + haystack_length + 64];
                        let haystack = &mut storage[offset..offset + haystack_length];
                        haystack.copy_from_slice(&original);
                        if let Some(position) = position {
                            haystack[position..position + length].copy_from_slice(&needle);
                        }
                        check(haystack, &needle);
                    }
                }
            }
        }
    }
}
