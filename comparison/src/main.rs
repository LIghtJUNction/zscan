use std::hint::black_box as bb;
use std::io::{self, BufWriter, Write};
use std::time::{Duration, Instant};

const SEED: u64 = 0x6a09_e667_f3bc_c909;
const ENGINES: [(&str, &str); 18] = [
    ("byte_find", "zscan"),
    ("byte_find", "memchr"),
    ("byte_find", "memx"),
    ("byte_rfind", "zscan"),
    ("byte_rfind", "memchr"),
    ("byte_rfind", "memx"),
    ("find", "zscan"),
    ("find", "memchr"),
    ("find", "memx"),
    ("rfind", "zscan"),
    ("rfind", "memchr"),
    ("rfind", "memx"),
    ("reuse_find", "zscan"),
    ("reuse_find", "memchr"),
    ("reuse_rfind", "zscan"),
    ("reuse_rfind", "memchr"),
    ("build_both", "zscan"),
    ("build_both", "memchr"),
];

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}

struct Case {
    name: String,
    group: &'static str,
    storage: Vec<u8>,
    start: usize,
    len: usize,
    needle: Vec<u8>,
}

impl Case {
    fn new(
        name: String,
        group: &'static str,
        haystack: Vec<u8>,
        needle: Vec<u8>,
        alignment: usize,
    ) -> Self {
        let mut storage = vec![0xa7; haystack.len() + 64];
        let start = (alignment + 64 - storage.as_ptr() as usize % 64) % 64;
        storage[start..start + haystack.len()].copy_from_slice(&haystack);
        let result = Self {
            name,
            group,
            storage,
            start,
            len: haystack.len(),
            needle,
        };
        assert_eq!(result.haystack().as_ptr() as usize % 64, alignment);
        result
    }

    fn haystack(&self) -> &[u8] {
        &self.storage[self.start..self.start + self.len]
    }

    fn byte(&self) -> u8 {
        self.needle.first().copied().unwrap_or(255)
    }

    fn fingerprint(&self) -> u64 {
        self.haystack()
            .iter()
            .chain(self.needle.iter())
            .fold(0xcbf2_9ce4_8422_2325, |h, &byte| {
                (h ^ u64::from(byte)).wrapping_mul(0x100_0000_01b3)
            })
    }
}

fn ordinary(
    group: &'static str,
    n: usize,
    m: usize,
    pattern: &str,
    alignment: usize,
    rng: &mut Rng,
) -> Case {
    let mut h: Vec<u8> = (0..n).map(|_| (rng.next() % 255) as u8).collect();
    let mut needle: Vec<u8> = (0..m).map(|_| rng.next() as u8).collect();
    if m > 0 {
        needle[0] = 255;
    }
    if m > 0 && m <= n {
        match pattern {
            "first" => h[..m].copy_from_slice(&needle),
            "last" => h[n - m..].copy_from_slice(&needle),
            "dense" => {
                for pos in (0..=n - m).step_by(m) {
                    h[pos..pos + m].copy_from_slice(&needle);
                }
            }
            "sparse" => {
                for pos in ((n / 7).min(n - m)..=n - m).step_by((m * 13).max(64)) {
                    h[pos..pos + m].copy_from_slice(&needle);
                }
            }
            "none" | "empty" | "overlong" => {}
            _ => unreachable!(),
        }
    }
    Case::new(
        format!("{group}-{pattern}-n{n}-m{m}-a{alignment}"),
        group,
        h,
        needle,
        alignment,
    )
}

fn cases() -> Vec<Case> {
    let mut rng = Rng(SEED);
    let mut result = Vec::new();
    for n in [0, 1, 2, 3, 4, 7, 8, 15, 16, 17, 31, 32, 33, 63] {
        for m in [0, 1, 2, 3, 4, 8, 16, 32] {
            let patterns: &[&str] = if m == 0 {
                &["empty"]
            } else if m > n {
                &["overlong"]
            } else {
                &["first", "last", "none"]
            };
            for pattern in patterns {
                result.push(ordinary("short", n, m, pattern, (n + m) % 64, &mut rng));
            }
        }
    }
    for n in [128, 512, 4096, 65536] {
        for m in [2, 4, 8, 16, 31, 32, 33, 64, 127, 256] {
            if m <= n {
                for pattern in ["first", "last", "none", "dense", "sparse"] {
                    result.push(ordinary("typical", n, m, pattern, 0, &mut rng));
                }
            }
        }
    }
    for m in [4, 16, 64] {
        for pattern in ["last", "none"] {
            result.push(ordinary("large", 1 << 20, m, pattern, 17, &mut rng));
        }
    }
    for alignment in 0..64 {
        result.push(ordinary(
            "alignment",
            32 + alignment % 33,
            2 + alignment % 17,
            ["first", "last", "none"][alignment % 3],
            alignment,
            &mut rng,
        ));
    }
    for tail in 0..32 {
        result.push(ordinary(
            "tail",
            64 + tail,
            2 + tail % 15,
            ["first", "last", "none"][tail % 3],
            7,
            &mut rng,
        ));
    }
    for kind in ["repeated", "periodic", "low-entropy"] {
        for m in [8, 32, 128, 257] {
            let h: Vec<u8> = (0..65536)
                .map(|i| match kind {
                    "repeated" => 0xa5,
                    "periodic" => b'a' + (i % 2) as u8,
                    _ => (rng.next() % 2) as u8,
                })
                .collect();
            for near_miss in [false, true] {
                let mut needle = h[..m].to_vec();
                if near_miss {
                    // Equal endpoints create frequent prefilter false positives.
                    needle[m / 2] = 0x7f;
                }
                result.push(Case::new(
                    format!("adversarial-{kind}-near{near_miss}-n65536-m{m}"),
                    "adversarial",
                    h.clone(),
                    needle,
                    31,
                ));
            }
        }
    }
    // Added before any full timing. Preserve all 559 earlier cases unchanged.
    let texts: [(&str, &[u8], &[u8]); 3] = [
        (
            "http",
            b"GET /v1/items HTTP/1.1\r\nhost: api.example.test\r\naccept: application/json\r\nconnection: keep-alive\r\n",
            b"content-length:",
        ),
        (
            "json",
            b"{\"status\":\"ok\",\"items\":[{\"id\":123,\"name\":\"sample\",\"active\":true}],\"count\":1}\n",
            b"\"request_id\":\"",
        ),
        (
            "log",
            b"2026-11-09T00:00:00Z level=info service=worker request=582 action=read duration_ms=3\n",
            b"level=error",
        ),
    ];
    for (kind_index, &(kind, template, needle)) in texts.iter().enumerate() {
        for n in [128, 512, 4096, 65536] {
            for pattern in ["first", "last", "none", "dense"] {
                let mut h: Vec<u8> = (0..n).map(|i| template[i % template.len()]).collect();
                let m = needle.len();
                match pattern {
                    "first" => h[..m].copy_from_slice(needle),
                    "last" => h[n - m..].copy_from_slice(needle),
                    "dense" => {
                        for pos in (0..=n - m).step_by(m + 17) {
                            h[pos..pos + m].copy_from_slice(needle);
                        }
                    }
                    "none" => {}
                    _ => unreachable!(),
                }
                result.push(Case::new(
                    format!("text-{kind}-{pattern}-n{n}-m{m}"),
                    "text",
                    h,
                    needle.to_vec(),
                    kind_index * 17,
                ));
            }
        }
    }
    result
}

fn reference(h: &[u8], n: &[u8], reverse: bool) -> Option<usize> {
    if n.len() > h.len() {
        return None;
    }
    let end = h.len() - n.len();
    for step in 0..=end {
        let pos = if reverse { end - step } else { step };
        let mut matched = true;
        for j in 0..n.len() {
            if h[pos + j] != n[j] {
                matched = false;
                break;
            }
        }
        if matched {
            return Some(pos);
        }
    }
    None
}

fn check(h: &[u8], n: &[u8], byte: u8, label: &str) -> bool {
    let forward = reference(h, n, false);
    let backward = reference(h, n, true);
    let finder = zscan::Finder::new(n);
    for (name, actual) in [
        ("zscan/find", zscan::find(h, n)),
        ("zscan/reuse_find", finder.find(h)),
        ("memchr/find", memchr::memmem::find(h, n)),
        ("memchr/reuse_find", memchr::memmem::Finder::new(n).find(h)),
        ("memx/find", memx::memmem(h, n)),
    ] {
        assert_eq!(actual, forward, "{label}: {name}");
    }
    for (name, actual) in [
        ("zscan/rfind", zscan::rfind(h, n)),
        ("zscan/reuse_rfind", finder.rfind(h)),
        ("memchr/rfind", memchr::memmem::rfind(h, n)),
        (
            "memchr/reuse_rfind",
            memchr::memmem::FinderRev::new(n).rfind(h),
        ),
    ] {
        assert_eq!(actual, backward, "{label}: {name}");
    }
    let memx_reverse = memx::memrmem(h, n);
    if memx_reverse != backward {
        eprintln!(
            "known-baseline-mismatch,{label},memx/rfind,expected={backward:?},actual={memx_reverse:?}"
        );
    }
    let bf = h.iter().position(|&v| v == byte);
    let br = h.iter().rposition(|&v| v == byte);
    for (name, actual) in [
        ("zscan/byte_find", zscan::find_byte(byte, h)),
        ("memchr/byte_find", memchr::memchr(byte, h)),
        ("memx/byte_find", memx::memchr(h, byte)),
    ] {
        assert_eq!(actual, bf, "{label}: {name}");
    }
    for (name, actual) in [
        ("zscan/byte_rfind", zscan::rfind_byte(byte, h)),
        ("memchr/byte_rfind", memchr::memrchr(byte, h)),
        ("memx/byte_rfind", memx::memrchr(h, byte)),
    ] {
        assert_eq!(actual, br, "{label}: {name}");
    }
    memx_reverse == backward
}

fn differential(smoke: bool) {
    let mut rng = Rng(SEED ^ 0xbb67_ae85_84ca_a73b);
    for i in 0..if smoke { 200 } else { 4000 } {
        let size = (rng.next() % 2049) as usize;
        let m = (rng.next() % 270) as usize;
        let alphabet = [2, 4, 16, 256][i % 4];
        let mut h: Vec<u8> = (0..size).map(|_| (rng.next() % alphabet) as u8).collect();
        let n: Vec<u8> = (0..m).map(|_| (rng.next() % alphabet) as u8).collect();
        if m <= size && i % 2 == 0 {
            let pos = (rng.next() % (size - m + 1) as u64) as usize;
            h[pos..pos + m].copy_from_slice(&n);
        }
        let c = Case::new(format!("random-{i}"), "random", h, n, i % 64);
        check(c.haystack(), &c.needle, rng.next() as u8, &c.name);
    }
}

fn batch<T>(iterations: u64, f: &mut impl FnMut() -> T) -> Duration {
    let start = Instant::now();
    for _ in 0..iterations {
        bb(f());
    }
    start.elapsed()
}

fn measure<T>(iterations: &mut u64, smoke: bool, mut f: impl FnMut() -> T) -> Duration {
    if *iterations == 0 {
        if smoke {
            *iterations = 4;
            batch(4, &mut f);
        } else {
            let mut count = 16;
            loop {
                // A scheduler pause in one pilot must not turn a nanosecond
                // operation into a one-iteration, timer-dominated batch. Use
                // the least interrupted of three warmup pilots for sizing.
                // All seven *measurement* samples remain unfiltered.
                let elapsed = (0..3)
                    .map(|_| batch(count, &mut f).as_nanos())
                    .min()
                    .unwrap();
                if elapsed >= 20_000 || count >= 1 << 22 {
                    *iterations =
                        ((u128::from(count) * 100_000 / elapsed.max(1)).clamp(1, 1 << 24)) as u64;
                    break;
                }
                count *= 2;
            }
        }
    }
    batch(*iterations, &mut f)
}

struct Searchers<'a> {
    zscan: zscan::Finder<'a>,
    memchr: memchr::memmem::Finder<'a>,
    memchr_rev: memchr::memmem::FinderRev<'a>,
}

fn time_engine(
    engine: usize,
    c: &Case,
    searchers: &Searchers<'_>,
    iterations: &mut u64,
    smoke: bool,
) -> Duration {
    let h = c.haystack();
    let n = c.needle.as_slice();
    let b = c.byte();
    match engine {
        0 => measure(iterations, smoke, || zscan::find_byte(bb(b), bb(h))),
        1 => measure(iterations, smoke, || memchr::memchr(bb(b), bb(h))),
        2 => measure(iterations, smoke, || memx::memchr(bb(h), bb(b))),
        3 => measure(iterations, smoke, || zscan::rfind_byte(bb(b), bb(h))),
        4 => measure(iterations, smoke, || memchr::memrchr(bb(b), bb(h))),
        5 => measure(iterations, smoke, || memx::memrchr(bb(h), bb(b))),
        6 => measure(iterations, smoke, || zscan::find(bb(h), bb(n))),
        7 => measure(iterations, smoke, || memchr::memmem::find(bb(h), bb(n))),
        8 => measure(iterations, smoke, || memx::memmem(bb(h), bb(n))),
        9 => measure(iterations, smoke, || zscan::rfind(bb(h), bb(n))),
        10 => measure(iterations, smoke, || memchr::memmem::rfind(bb(h), bb(n))),
        11 => measure(iterations, smoke, || memx::memrmem(bb(h), bb(n))),
        12 => measure(iterations, smoke, || bb(&searchers.zscan).find(bb(h))),
        13 => measure(iterations, smoke, || bb(&searchers.memchr).find(bb(h))),
        14 => measure(iterations, smoke, || bb(&searchers.zscan).rfind(bb(h))),
        15 => measure(iterations, smoke, || bb(&searchers.memchr_rev).rfind(bb(h))),
        16 => measure(iterations, smoke, || zscan::Finder::new(bb(n))),
        17 => measure(iterations, smoke, || {
            (
                memchr::memmem::Finder::new(bb(n)),
                memchr::memmem::FinderRev::new(bb(n)),
            )
        }),
        _ => unreachable!(),
    }
}

fn cold(operation: &str, engine: &str) {
    let c = ordinary("cold", 4096, 16, "none", 17, &mut Rng(SEED));
    let h = c.haystack();
    let n = c.needle.as_slice();
    let b = c.byte();
    // Nothing above this line calls a searcher or initializes its CPU cache.
    let start = Instant::now();
    let answer = match (operation, engine) {
        ("byte_find", "zscan") => zscan::find_byte(bb(b), bb(h)),
        ("byte_find", "memchr") => memchr::memchr(bb(b), bb(h)),
        ("byte_find", "memx") => memx::memchr(bb(h), bb(b)),
        ("byte_rfind", "zscan") => zscan::rfind_byte(bb(b), bb(h)),
        ("byte_rfind", "memchr") => memchr::memrchr(bb(b), bb(h)),
        ("byte_rfind", "memx") => memx::memrchr(bb(h), bb(b)),
        ("find", "zscan") => zscan::find(bb(h), bb(n)),
        ("find", "memchr") => memchr::memmem::find(bb(h), bb(n)),
        ("find", "memx") => memx::memmem(bb(h), bb(n)),
        ("rfind", "zscan") => zscan::rfind(bb(h), bb(n)),
        ("rfind", "memchr") => memchr::memmem::rfind(bb(h), bb(n)),
        ("rfind", "memx") => memx::memrmem(bb(h), bb(n)),
        _ => panic!("cold supports byte_find/byte_rfind/find/rfind and zscan/memchr/memx"),
    };
    bb(answer);
    let elapsed = start.elapsed().as_nanos();
    assert_eq!(answer, None);
    println!(
        "case,group,operation,engine,haystack_len,needle_len,alignment,sample,iterations,elapsed_ns,ns_per_op,correct,eligible"
    );
    println!(
        "{name},cold,{operation},{engine},4096,16,17,0,1,{elapsed},{elapsed},true,{}",
        !(operation == "rfind" && engine == "memx"),
        name = c.name
    );
}

fn main() -> io::Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("--cold") {
        assert_eq!(args.len(), 3, "usage: --cold OPERATION ENGINE");
        cold(&args[1], &args[2]);
        return Ok(());
    }
    assert!(
        args.iter().all(|a| a == "--smoke" || a == "--list"),
        "usage: [--smoke | --list | --cold OPERATION ENGINE]"
    );
    let smoke = args.iter().any(|a| a == "--smoke");
    let cases = cases();
    let mut out = BufWriter::new(io::stdout().lock());
    if args.iter().any(|a| a == "--list") {
        writeln!(
            out,
            "case,group,haystack_len,needle_len,alignment,fingerprint,forward,reverse"
        )?;
        for c in &cases {
            writeln!(
                out,
                "{},{},{},{},{},{:016x},{:?},{:?}",
                c.name,
                c.group,
                c.len,
                c.needle.len(),
                c.haystack().as_ptr() as usize % 64,
                c.fingerprint(),
                reference(c.haystack(), &c.needle, false),
                reference(c.haystack(), &c.needle, true),
            )?;
        }
        return Ok(());
    }
    differential(smoke);
    let validity: Vec<bool> = cases
        .iter()
        .map(|c| check(c.haystack(), &c.needle, c.byte(), &c.name))
        .collect();
    eprintln!(
        "{} fixed cases and seeded random differential checks passed",
        cases.len()
    );
    writeln!(
        out,
        "case,group,operation,engine,haystack_len,needle_len,alignment,sample,iterations,elapsed_ns,ns_per_op,correct,eligible"
    )?;
    for (case_index, c) in cases.iter().enumerate() {
        let searchers = Searchers {
            zscan: zscan::Finder::new(&c.needle),
            memchr: memchr::memmem::Finder::new(&c.needle),
            memchr_rev: memchr::memmem::FinderRev::new(&c.needle),
        };
        let mut iterations = [0; ENGINES.len()];
        for sample in 0..if smoke { 1 } else { 7 } {
            for slot in 0..ENGINES.len() {
                let engine = (slot + sample * 5) % ENGINES.len();
                let elapsed = time_engine(engine, c, &searchers, &mut iterations[engine], smoke);
                let (operation, implementation) = ENGINES[engine];
                writeln!(
                    out,
                    "{},{},{},{},{},{},{},{},{},{},{:.4},{},{}",
                    c.name,
                    c.group,
                    operation,
                    implementation,
                    c.len,
                    c.needle.len(),
                    c.haystack().as_ptr() as usize % 64,
                    sample,
                    iterations[engine],
                    elapsed.as_nanos(),
                    elapsed.as_nanos() as f64 / iterations[engine] as f64,
                    engine != 11 || validity[case_index],
                    engine != 11,
                )?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn pinned_memx_reverse_regression_is_ineligible() {
        let input = b"\xffA";
        assert_eq!(super::reference(input, input, true), Some(0));
        assert_eq!(zscan::rfind(input, input), Some(0));
        assert_eq!(memchr::memmem::rfind(input, input), Some(0));
        // Record the pinned baseline defect; do not turn an incomplete search
        // into a speed claim or silently remove the failing workload.
        assert_eq!(memx::memrmem(input, input), None);
    }
}
