# zscan 0.1.0: verification and publication

## Published artifact

**zscan 0.1.0 was published to crates.io on 2026-11-09 at 16:51:40 UTC.**
The [GitHub Release](https://github.com/LIghtJUNction/zscan/releases/tag/v0.1.0)
was created after registry checksum confirmation.

- [crates.io version](https://crates.io/crates/zscan/0.1.0)
- Release commit: **104ed7b3472a986c075a4708c7a87833dd578f12**
- Downloaded crate: **21,694 bytes**
- SHA-256: **e6893b4d66228f42326189b34ff7bd72f243342eae084061d81a6f2f9cb562bd**
- [Successful release workflow](https://github.com/LIghtJUNction/zscan/actions/runs/42657996950)
- [Independent download verification](release-evidence/0.1.0.json)
- [Workflow and installed-package evidence](release-evidence/0.1.0-details.json)

A fresh download from the registry matched the registry checksum, the publication
log and the GitHub Release body. Its Cargo VCS metadata identifies the clean
release commit. All four production source files in the archive are byte-for-byte
identical to the optimized source measured in the round-2 benchmarks.

A separate consumer downloaded **zscan = "=0.1.0" from crates.io**, rather than
using a path dependency. Its library unconditionally declares no_std. Rust
1.70.0 successfully compiled it and passed the installed-package test covering
all four functions, Finder, binary bytes, empty needles and overlong needles.
The consumer's source, lockfile, exact commands, compiler identity and output
are retained in the publication evidence. Its dependency tree contains only
the consumer and zscan.

## Exact-commit checks

The [automation commit's CI](https://github.com/LIghtJUNction/zscan/actions/runs/42656990831)
passed all 13 jobs before the first release request. That request created an
empty release commit and its immutable v0.1.0 tag atomically. The release
workflow then ran the full matrix again with every checkout pinned to the
resolved release commit, not the triggering request commit.

The release workflow completed with **15 successful jobs and one intentionally
skipped tag-resolution job**: preparation, 13 verification jobs and publication.
The branch-request path uses preparation; the separate resolver is for tag pushes.

| Check | Observed result on the release commit |
| --- | --- |
| Stable, x86-64 and AArch64 | Debug/release tests, doctests and native builds passed |
| Rust 1.70.0, x86-64 and AArch64 | Debug/release tests, doctests and native builds passed |
| Scalar / AVX2 / NEON | Differential tests and guarded pages passed on supported hardware |
| Scalar Miri | Passed |
| Bare metal | thumbv7em-none-eabi and aarch64-unknown-none library builds passed |
| Formatting / Clippy | Passed, with Clippy warnings denied |
| Dependency audit | All dependency kinds empty; no build.rs; comparison isolated |
| Package / dry-run | Package listing, locked packaging and locked publication dry-run passed |
| Benchmark smoke | Generic/native on both architectures passed |
| Release tooling | 23 Python tests passed, including real local-Git ref-race tests |

The core suite has 16 unit/integration tests and one doctest. Exhaustive binary
and ternary tests cover 262,174 input pairs, supplemented by seeded random,
alignment, all-match-position, periodic, NUL, invalid UTF-8 and guarded-page
tests. Miri uses reduced sizes for the same scalar checks. Guard-page libc calls
are confined to native test executables.

## Implementation and complexity

The library always uses no_std and core, with no allocator, FFI, build script or
third-party dependency. Finder borrows its needle and stores fixed-size metadata
for both directions. It never copies or reverses the complete needle.

Byte search is O(n). Substring search is worst-case O(n + m), with O(1) extra
space. Candidate verification starts with a budget of 2m, gains credit only
from newly skipped positions, and pays m per candidate. Changing a probe does
not reset that budget. Exhaustion invokes one Two-Way search over the remaining
range. The short-haystack path has a fixed 32-byte bound.

Round 2 aligns the AVX2 byte-scanning bulk loop and replaces scalar tails with
complete overlapping vectors contained inside the original slice or legal
candidate interval. Overlap is restricted to already rejected positions, so
first/last match ordering is preserved. Scalar pair filtering uses the same
bounded-overlap approach with eight-byte words. The verification budget and
fallback are unchanged.

Generic x86-64 dispatch checks CPU features and OS XMM/YMM state. AArch64 NEON
requires the compilation target's guarantee. Explicit native builds require a
compatible deployment CPU and OS. Every unsafe block records the applicable
feature, access, alignment and aliasing conditions.

## Performance evidence

The [fixed protocol](comparison/cases.md) contains 607 cases. Baselines are
memchr 2.8.3 and memx 0.2.2, with exact versions, source commits and checksums
retained. The [round-2 review](comparison/results/round2-review.md) compares the
previous and optimized code with unchanged memchr controls and a full repeat.

All **1,070,832 raw observations** remain available: 14 full configurations
of 607 × 18 × 7 samples, plus 84 historical fresh-process calls. The results
tree contains 158 files. All 73 files from the first delivery remain unchanged.
Each full run includes raw samples, case fingerprints, source snapshots,
compile-input hashes, Cargo.lock, CPU/OS/compiler information, flags and logs.

Measurements used an Intel Xeon Platinum 8573C, Linux 6.18.57, Rust 1.99.1,
opt-level 3, one codegen unit and thin LTO. The following are median ns/op
from the generic build; the repeat uses unchanged optimized code.

| Targeted case | Previous zscan | Primary optimized run | Repeat | Previous / primary |
| --- | ---: | ---: | ---: | ---: |
| 87-byte absent-byte search | 8.3709 | 3.0673 | 3.0677 | 2.73x |
| 95-byte last-byte search | 9.7516 | 3.6263 | 3.6232 | 2.69x |
| 84-byte / 7-byte-needle reused forward search | 16.2119 | 7.8384 | 7.8082 | 2.07x |

The matched memchr controls were nearly unchanged for these cases. These are
improvements over the previous zscan implementation, not claims of the same
advantage over memchr. For example, memchr measured approximately 3.07 ns and
3.35 ns in the two byte cases.

**The broad 2x target remains unmet.** Primary-run geometric ratios against the
fastest eligible baseline were:

| Workload / operation | Generic | Native |
| --- | ---: | ---: |
| Short / forward substring | 1.205x | 1.244x |
| Short / reverse substring | 1.677x | 1.778x |
| Typical / forward substring | 1.338x | 1.339x |
| Typical / reverse substring | 7.665x | 8.499x |
| Structured text / forward substring | 1.922x | 1.892x |
| Structured text / reverse substring | 5.265x | 6.232x |
| Typical / forward byte | 0.981x | 1.444x |
| Typical / reverse byte | 0.909x | 1.284x |

All slower cases remain in the [primary full report](comparison/results/round2-after-calibrated/summary.md).
Two severe localized regressions did not reproduce in the unchanged repeat;
the original samples remain in the headline data. This establishes timing
instability, not its cause. Other variations remain. Shared-machine,
warm-cache measurements are observations, not a release speed guarantee.

The calibration fix takes the minimum of three warmup pilots solely to size
each batch; all seven measured samples remain unfiltered. Before and after use
the same corrected harness. The old calibration records and historical
component ablations remain labeled and preserved. The incorrect memx reverse
operation is retained diagnostically but excluded from speed claims.

## Version bumps and publication

The [Release workflow](.github/workflows/release.yml) supports current, patch,
minor, major, custom and existing-tag modes. A bump updates Cargo.toml and both
lockfiles, then pushes its version commit and tag in one non-force atomic
operation. Ordinary source pushes do not request publication; the explicit
request file or manual workflow selection does.

Packaging uses the tagged Rust/Cargo 1.99.1 pin. CARGO_REGISTRY_TOKEN is mapped
only into the publication step. The script invokes the real locked upload once,
checks ambiguous outcomes against the registry without blind re-upload, and
creates a GitHub Release only after checksum confirmation. Existing versions
and tags are never overwritten. See the [maintenance guide](scripts/README.md)
for the next bump and recovery of an existing tag.
