# Fixed comparison protocol

This plan was written before optimized zscan measurements. The workload generator
in `src/main.rs` fixes the cases, seed, alignments, and lengths; `--list` prints
every case with its content fingerprint and expected answers. Keep failed and
slower cases in every report. A timing run may not silently discard a case.

## Baselines

Use crates.io's current stable releases, checked on 2026-11-09: `memchr 2.8.3`
(upstream commit `5fdb40c054e1fff359a2f7bdf7f87a13b34b465d`) and `memx 0.2.2`
(`c52c59209caa169054a4b2f8533d9ad646c59dd4`). Commits come from each published
crate's `.cargo_vcs_info.json`; Cargo.lock records package checksums. memchr's
default `std` feature remains enabled to permit its strongest runtime dispatch.
memx provides another independently maintained SSE2/AVX2 implementation. Both
operate on arbitrary bytes. Independent naive loops are correctness oracles,
not timed competitive baselines.

## Workloads, fixed before tuning

- Seed: `0x6a09e667f3bcc909`, xorshift64 (13, 7, 17).
- Short: haystack lengths 0, 1, 2, 3, 4, 7, 8, 15, 16, 17, 31, 32, 33, 63;
  needle lengths 0, 1, 2, 3, 4, 8, 16, 32. For fitting nonempty needles test
  first, last, and absent matches; empty and overlong needles are separate cases.
- Typical: haystack lengths 128, 512, 4,096, 65,536; needle lengths
  2, 4, 8, 16, 31, 32, 33, 64, 127, 256 where they fit. Test first, last,
  absent, dense, and sparse matches. The data between matches uses all byte
  values except the needle's sentinel, including NUL and invalid UTF-8.
- Large: 1,048,576-byte haystacks, needle lengths 4, 16, 64; last and absent.
- Alignment: every actual pointer residue modulo 64, not merely an allocation
  offset; lengths and needle lengths vary deterministically with the residue.
- SIMD tails: every tail length modulo 32, with a nonzero pointer residue.
- Adversarial: all-equal, periodic `ab`, and a two-byte low-entropy alphabet;
  matching and near-matching needles of lengths 8, 32, 128, 257 in 65,536-byte
  haystacks. A near miss changes a middle byte while preserving equal endpoints.

The primary short and typical result groups are fixed above. Report each
direction separately, and compare each zscan cell with the fastest
semantics-equivalent measured competitor in that cell. Report both medians and
the distribution of per-case ratios; a geometric mean does not establish that
every case is faster. The research target is at least 2x, not a claimed result.

## What is timed

`byte_find`, `byte_rfind`, `find`, and `rfind` call the public one-shot API inside
the timed loop, including that API's preprocessing and dispatch. Arguments and
results pass through `std::hint::black_box` on every iteration. The byte is the
first needle byte (255 for an empty needle).

`reuse_find` and `reuse_rfind` construct each library's borrowed searcher before
timing and include its public search method in timing. `build_both` compares one
bidirectional zscan Finder with the pair of memchr Finder and FinderRev needed
for the same two directions. No memx reuse rows are fabricated: it exposes only
the one-shot operations used here. Construction returns are black-boxed. The
timed loop uses statically specialized closures; algorithm selection happens
outside that loop.

Each cell has warmup and adaptive batching, followed by seven retained raw
samples. Engine order rotates between samples. The default target batch time is
100 microseconds; `--smoke` uses one small batch solely to exercise the harness.
Correctness is checked before timing, against independent forward/reverse loops
and both baseline libraries. Further fixed-seed differential inputs run first.
Only nanoseconds per operation are reported; no unscanned bytes are credited as
throughput.

The process has steady-state CPU dispatch caches after warmup. One-shot calls
still execute each API's dispatch path. `--cold` is a separate fresh-process,
single-call observation, including first-use dispatch; it is noisy and is not
used for the 2x target.

## Configurations and retained evidence

Run generic and `-C target-cpu=native`, then independent generic ablations
`--cfg zscan_scalar`, `--cfg zscan_no_short`, and `--cfg zscan_no_prefilter`.
These cfgs affect zscan only; the exact RUSTFLAGS are captured for every run.
Finder reuse versus one-shot isolates caching. This is an observational suite:
shared/cloud hardware is not a release performance gate.

`run.py` saves every CSV row (gzip), cases manifest, compiler/OS/CPU metadata,
Cargo metadata and lockfile, compile flags, git commit and worktree state,
source SHA-256 hashes, command lines, and stderr. `summarize.py` derives tables
without deleting outliers. Re-run the whole suite on fixed idle hardware before
using a timing difference as a product guarantee.


## Pre-timing addendum: structured text and baseline eligibility

Before any full timing run, add a separate `text` group with HTTP header, JSON,
and log-line fragments. Each has 128, 512, 4,096, and 65,536-byte haystacks with
first, last, absent, and dense matches, using ASCII needles `content-length:`,
`"request_id":"`, and `level=error`. These 48 cases supplement all 559 original
cases unchanged; they avoid relying only on synthetic sentinel-byte data. The
original short/typical groups remain separately visible in every summary.

Initial correctness-only smoke detected a memx 0.2.2 reverse substring bug.
Retain all memx reverse timing cells and their correctness flags, but mark the
entire operation ineligible for performance ratios. This prevents incomplete
searches from being counted as fast. See [baseline-notes.md](baseline-notes.md)
for the minimal reproduction and source. No zscan or memchr discrepancy is
accepted, and no case is removed or rewritten in response to this finding.
