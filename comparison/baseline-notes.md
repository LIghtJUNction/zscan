# Baseline versions and correctness

Versions were queried from the registry on 2026-11-09, before optimization:

| Baseline | Version | Upstream commit | Registry checksum |
| --- | --- | --- | --- |
| [memchr](https://github.com/BurntSushi/memchr) | 2.8.3 | `5fdb40c054e1fff359a2f7bdf7f87a13b34b465d` | `cf8baf1c55e62ffcace7a9f06f4bd9cd3f0c4beb022d3b367256b91b87513d98` |
| [memx](https://github.com/aki-akaguma/memx) | 0.2.2 | `c52c59209caa169054a4b2f8533d9ad646c59dd4` | `b34799f4a1d8693a53c36c100211735052ea2193eabdd03c3aa304319a39978c` |

Commits were read from the published crates' `.cargo_vcs_info.json`. Checksums
are also in the independent comparison/Cargo.lock. memchr keeps its default
features and runtime SIMD detection; disabling std would weaken the baseline.

## memx reverse substring mismatch

The first correctness smoke run, before optimized zscan measurements, found that
memx 0.2.2 can omit the candidate at `haystack.len() - needle.len()`. Its
first-byte reverse path initializes `curr_idx` to that index but searches the
exclusive slice `[..curr_idx]`. When both slices have the same length, it skips
the search entirely. The issue appears in both its scalar and x86 paths.

Minimal reproduction with the pinned dependency:

```rust
let haystack = b"\xffA";
let needle = b"\xffA";
assert_eq!(memchr::memmem::rfind(haystack, needle), Some(0));
assert_eq!(zscan::rfind(haystack, needle), Some(0));
assert_eq!(memx::memrmem(haystack, needle), None); // comparator bug
```

The harness does not patch memx or remove cases. It retains memx/rfind timings
and records each result's `correct` flag, but marks **all** memx/rfind rows
`eligible=false` so that an incomplete search cannot win a speed comparison.
Other memx operations, zscan, and memchr must match independent reference loops
or the run stops. The remaining forward and byte memx results provide a second
optimized baseline; memchr remains the primary baseline for every API.

Source: [memx 0.2.2 reverse implementation](https://github.com/aki-akaguma/memx/blob/c52c59209caa169054a4b2f8533d9ad646c59dd4/src/arch/x86/x86_rmem.rs)
and [scalar implementation](https://github.com/aki-akaguma/memx/blob/c52c59209caa169054a4b2f8533d9ad646c59dd4/src/mem/mem_rmem.rs).

