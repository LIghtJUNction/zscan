# zscan

Byte and substring search with no dependencies, no allocation and an always
`no_std` library. Rust 1.70 or newer; MIT licensed.

The API consists of four functions and one borrowed searcher:

```rust
use zscan::{find, rfind, find_byte, rfind_byte, Finder};

assert_eq!(find_byte(0, b"ab\0cd"), Some(2));
assert_eq!(rfind_byte(b'a', b"aba"), Some(2));
assert_eq!(find(b"ababa", b"aba"), Some(0));
assert_eq!(rfind(b"ababa", b"aba"), Some(2));
let finder = Finder::new(b"aba");
assert_eq!(finder.find(b"xxaba"), Some(2));
assert_eq!(finder.rfind(b"ababa"), Some(2));
```

All inputs are byte slices, including NUL and invalid UTF-8. Returned positions
are byte offsets in the original haystack. An empty needle matches at `0`
forwards and `haystack.len()` backwards. An overlong needle returns `None`.

## Complexity and implementation

Byte search takes O(n) time. Substring search takes worst-case O(n + m) time
and O(1) extra space, where n and m are the haystack and needle lengths.
`Finder::new` takes O(m) time, borrows its needle, and caches fixed-size metadata
for both directions. It never copies or reverses the input. A Finder can be
shared between threads; searches do not mutate it.

Short haystacks avoid preprocessing. Longer searches filter candidate starts
using two needle bytes, then verify the full candidate. A failed verification
can replace an unhelpful probe with the first mismatching byte. Verification
spends a single search-wide budget: initially 2m, replenished only by newly
skipped positions, with m charged per candidate. When that budget is exhausted,
search falls back once to [Two-Way](https://doi.org/10.1145/116825.116845).
This bounds verification work by O(n + m), including repeated and periodic data.

## CPU support and safety

| Target | Generic build |
| --- | --- |
| x86-64 | AVX2 after cached CPUID, OSXSAVE and XCR0 checks; otherwise SWAR |
| AArch64 with target-guaranteed NEON | NEON |
| Other targets | Portable SWAR/scalar |

The x86 AVX2 path also uses SSE2 for 16-byte tails. No vector load crosses either
end of a supplied slice; unaligned slices are supported. Safe public functions
require no caller feature detection. A build using `-C target-cpu=native` or
explicit target features must run on a compatible CPU and OS, as with any Rust
binary built with those flags. No `std` feature detector, allocation, FFI,
build script or third-party dependency is used by the library.

Unsafe code is confined to architecture intrinsics and their private checked
dispatch; each block records its feature and access preconditions. Linux
protection-page tests use libc FFI in the test executable only.

## Verification

```sh
cargo test --locked
cargo test --release --locked
cargo test --doc --locked
cargo +1.70.0 test --locked
python3 scripts/audit.py
python3 scripts/test_backends.py
```

Tests include independent naive references, exhaustive small alphabets, seeded
random inputs, every short match position, binary data, offsets, SIMD tails,
direct scalar/vector comparisons, and guarded pages. CI runs stable and MSRV on
x86-64 and AArch64, fmt/Clippy, native builds, scalar Miri, and bare-metal
`thumbv7em-none-eabi` / `aarch64-unknown-none` library builds.

## Benchmarks

The isolated [comparison project](comparison/README.md) pins `memchr 2.8.3`
and `memx 0.2.2`. The [fixed protocol](comparison/cases.md) includes 607 short,
binary, structured-text, alignment, tail, and adversarial workloads. One-shot
timing includes preprocessing and dispatch; Finder construction and reuse are
measured separately. All measurements are ns/op, including early returns.
Raw samples, source snapshots, hashes, build settings and failed comparator
answers are retained. See [baseline correctness notes](comparison/baseline-notes.md).

On the recorded Xeon Platinum 8573C / Rust 1.99.1 generic build, the geometric
mean speedups against the fastest correct baseline in each case were:

| Workloads | One-shot substring forward | One-shot substring reverse |
| --- | ---: | ---: |
| Short (238 cases) | 1.08x | 1.54x |
| Typical binary (195 cases) | 1.16x | 8.12x |
| HTTP / JSON / logs (48 cases) | 1.91x | 5.21x |

**The overall 2x research target was not met.** Typical byte searches were
0.87x/0.88x the fastest forward/reverse baseline, and some short/reused searches
regressed. These repeated-buffer, warm-cache measurements on shared hardware
are observations, not a universal performance guarantee. The [complete report](comparison/results/final/summary.md)
includes all cases, slower results, native builds and component ablations.

## Release

The [Release workflow](.github/workflows/release.yml) accepts a `vX.Y.Z` tag or a
manual selection of an existing tag. It checks the tag's exact commit through
the full CI matrix, verifies the package contents, packages and dry-runs before
publishing. Only the publish step receives the repository Secret
`CARGO_REGISTRY_TOKEN`. An exact crates.io archive checksum must be confirmed
before a GitHub Release is created. [Maintenance details](scripts/README.md)
cover ambiguous upload errors and safe manual recovery.
