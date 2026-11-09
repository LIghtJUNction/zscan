# zscan 0.1.0 verification record

## Delivered and checked

The production code and automation were verified at commit
[`a37969d369ef01c87304ec61519aaf224dbd47f9`](https://github.com/LIghtJUNction/zscan/commit/a37969d369ef01c87304ec61519aaf224dbd47f9).
All 13 jobs in [GitHub Actions run 42648070615](https://github.com/LIghtJUNction/zscan/actions/runs/42648070615)
passed on 2026-11-09. Subsequent delivery commits add measurement evidence and
documentation; the four production Rust source files remain identical to the
checked code and the final benchmark source snapshots.

| Check | Observed result |
| --- | --- |
| Stable, x86-64 and AArch64 | Debug/release, doctests and native builds passed |
| Rust 1.70.0, x86-64 and AArch64 | Debug/release, doctests and native builds passed |
| Scalar / AVX2 / NEON | Differential and guarded-page checks passed on supported hardware |
| Scalar Miri | Interpreter checks passed |
| Bare metal | `thumbv7em-none-eabi` and `aarch64-unknown-none` library builds passed |
| Formatting and Clippy | Passed, with warnings denied by Clippy |
| Dependency audit | Every dependency kind empty; no build.rs; comparison isolated |
| Package list / package / publish dry-run | Passed locally and in CI |
| Benchmark smoke | Generic/native on both architectures passed |
| Release script unit tests | 13 passed; no registry upload is performed by these tests |

The core tests contain 16 unit/integration tests and one doctest. Exhaustive
binary and ternary cases cover 262,174 input pairs before the seeded randomized,
alignment, all-match-position, periodic, binary and guarded-page cases. Miri uses
smaller versions of the same scalar tests. Guarded pages are checked by native
Linux test executables; their libc calls are outside the library.

## Scope and complexity

The public API is `find_byte`, `rfind_byte`, `find`, `rfind`, and a borrowed
`Finder` with `new`, `find`, and `rfind`. Empty needles return zero forwards and
the haystack length backwards. Matches use original byte offsets, including
NUL and invalid UTF-8. A longer needle returns `None`.

All search paths use O(1) extra space. Byte scans are O(n). Two-Way
preprocessing is O(m), and candidate verification has a single budget initially
equal to 2m. Only newly skipped positions add credit; each complete candidate
verification costs m. Changing a probe after a mismatch never resets that
budget. On exhaustion, one Two-Way search completes the remaining range. Thus
the complete substring operation is worst-case O(n + m), including low entropy
and periodic inputs. The bounded short-haystack path does at most 32 byte
comparisons per candidate.

Every explicit vector load is bounded by the slice or the number of legal
candidate starts. x86 generic dispatch checks CPU AVX/XSAVE/OSXSAVE/AVX2 and OS
XMM/YMM state. AArch64 NEON requires the compilation target's guarantee. Explicit
native builds require a compatible deployment CPU and OS.

## Measurement evidence

The [fixed protocol](comparison/cases.md) contains 607 cases and was established
before full measurements. Baselines are exact-pinned memchr 2.8.3 and memx 0.2.2;
their published source commits and checksums are recorded in
[baseline-notes.md](comparison/baseline-notes.md). Timings use black_box, warmup,
seven retained samples, rotating engine order and correctness checks. One-shot
calls include preprocessing and dispatch; reusable searches exclude construction
for both implementations. Construction compares one bidirectional zscan Finder
with a memchr Finder/FinderRev pair.

All 458,976 raw observations are retained in [comparison/results](comparison/results):
the initial measurement, five final configurations, and 84 fresh-process calls.
Each full configuration contains 607 × 18 × 7 = 76,482 rows. Each run includes
source snapshots, compile-input hashes, Cargo.lock, CPU/OS/compiler information,
case fingerprints, command lines, logs, and all slow or incorrect comparator
results. Final runs check for source changes during building and measurement;
cross-build summaries verify matching inputs and intended flag differences.

The machine reports Intel Xeon Platinum 8573C, Linux 6.18.57 x86-64 and Rust
1.99.1 (LLVM 23.1.1). The benchmark profile uses opt-level 3, one codegen unit
and thin LTO. Data is repeatedly searched after warmup; these are warm-cache
observations on shared hardware.

Ratios below divide each case's fastest correct baseline median ns/op by zscan's
median, then take the geometric mean over every case in the named group.

| Workload / operation | Generic | Native |
| --- | ---: | ---: |
| Short / forward substring | 1.082x | 1.279x |
| Short / reverse substring | 1.538x | 1.666x |
| Typical / forward substring | 1.157x | 1.328x |
| Typical / reverse substring | 8.122x | 8.517x |
| HTTP / JSON / logs / forward substring | 1.906x | 1.812x |
| HTTP / JSON / logs / reverse substring | 5.210x | 6.058x |
| Typical / forward byte | 0.871x | 1.376x |
| Typical / reverse byte | 0.880x | 1.234x |

**The general 2x target was not achieved.** All 195 typical reverse-substring
cases were faster in the generic observations, with 191 at least 2x; all 48
structured-text reverse cases were at least 2x. Short forward searches and byte
searches did not reach the group target. Some reused searches and alignments
were slower. The [full report](comparison/results/final/summary.md) includes
every group, low ratios and per-case files.

The adaptive probe addressed a measured low-entropy hotspot. A 65,536-byte
haystack with a 32-byte near-matching needle dropped from 367,667 ns to 1,372 ns
in the forward one-shot observations, reaching 1.48x memchr. Both versions and
all measurements remain in the evidence.

## Remaining costs and limits of attribution

Source inspection and disassembly confirm these remaining costs:

- Generic byte searches read the runtime detection cache and enter a separate
  AVX2 function. Native builds inline more code and, on this CPU, LLVM also emits
  AVX-512 mask instructions. Native improvements therefore do not isolate the
  cost of runtime dispatch.
- Medium candidate tails cross another function boundary into SWAR and then
  scalar scanning. A 42-byte haystack / 12-byte needle has 31 legal starts:
  16 SIMD candidates, eight SWAR candidates, then seven scalar candidates.
  The retained alignment-last case measured 53.824 ns for reused forward search
  versus memchr's 12.330 ns. Finder stores probe indices, while the search still
  constructs broadcast vectors for each call.
- Both libraries use four-vector byte loops. memchr aligns its bulk pointer;
  zscan preserves the slice's input alignment. The 1 MiB cases use an actual
  pointer residue of 17 modulo 64, so two of every four zscan vector loads cross
  cache-line boundaries. zscan also finishes with SWAR/scalar tails. These are
  observed implementation differences; hardware-counter attribution was not
  measured.

The [interpretation note](comparison/results/interpretation.md) includes
unchanged memchr control timings beside component ablations. Those controls
also moved between runs. In particular, the independent short-path gain remains
unresolved where control drift is comparable. Fixed hardware is needed for a
reliable causal estimate; the recorded cloud results are not a release speed gate.

[Fresh-process observations](comparison/results/final/cold/summary.md) measured
the entire first call on a 4 KiB absent-match case: roughly 23–25 microseconds for
zscan. The first path includes three CPUID instructions and XGETBV, but the
whole-call timing must not be presented as isolated feature-detection time.
memchr's reverse substring observation was about 1.7 microseconds.

memx 0.2.2's reverse substring routine returned an incorrect result on a minimal
same-length match. Its raw answers and timings remain recorded, while every
memx reverse-substring row is ineligible for speed claims. memchr remains the
strict reverse baseline. All zscan, memchr and eligible memx results were checked
against independent reference loops.

## Publication state

No release tag was created and no actual crates.io upload or GitHub Release was
performed during this implementation. `cargo publish --dry-run --locked` passed;
the registry lookup for zscan returned HTTP 404 at verification time.

The workflow is configured for existing `vX.Y.Z` tags, with the exact tagged
commit checked in every job. Packaging uses the toolchain recorded in that tag
(currently 1.99.1). Only the publication step receives `CARGO_REGISTRY_TOKEN`.
Missing secrets, failed checks, changed tags and conflicting archive checksums
stop publication. Ambiguous uploads are checked against the registry without an
automatic second upload. A GitHub Release follows only a confirmed matching
archive. See [release maintenance](scripts/README.md).
