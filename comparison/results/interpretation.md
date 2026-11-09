# Interpretation of the recorded runs

The predeclared short/typical target of a general 2x speedup was **not met**.
All 607 cases, both directions, all seven samples, and slower results remain in
the raw files. These measurements used an Intel Xeon Platinum 8573C virtual
machine, Rust 1.99.1, memchr 2.8.3, and memx 0.2.2. This shared machine is not a
fixed-hardware performance acceptance environment.

## Observed final results

Ratios are the fastest correct, eligible competitor median divided by zscan's
median, then geometrically averaged across the entire named group. Above 1 is
faster. One-shot includes preprocessing and the warm dispatch path.

| Group / operation | Generic | Native |
| --- | ---: | ---: |
| Short / find | 1.082x | 1.279x |
| Short / rfind | 1.538x | 1.666x |
| Typical / find | 1.157x | 1.328x |
| Typical / rfind | 8.122x | 8.517x |
| Structured text / find | 1.906x | 1.812x |
| Structured text / rfind | 5.210x | 6.058x |
| Typical / byte find | 0.871x | 1.376x |
| Typical / byte rfind | 0.880x | 1.234x |

The full tables include Finder construction, reuse, alignments, vector tails,
large inputs, and adversarial cases. For example, generic reuse-find on one
42-byte, 12-byte-needle alignment case was only 0.229x the baseline. A large
reverse-search advantage does not establish a universal advantage.

## Measured optimization and limits of attribution

The initial adaptive-filter hotspot was a 65,536-byte, two-symbol haystack with
a 32-byte near-matching needle. Forward search took 367,667 ns initially and
1,372 ns in the final generic run; the latter was 1.48x memchr. The first
candidate's mismatch now identifies an informative probe byte. The initial
source and raw data are retained separately, so the improvement is inspectable.

The scalar, no-short-path, and no-prefilter runs have identical compile inputs,
case fingerprints, compiler, and profile; only their intended cfg differs.
Nevertheless, unrelated memchr control timings also shifted across these runs.
For example, its typical byte-find geometric time ratio in no-short/generic was
0.581, while zscan's was 0.535, although the short-path cfg does not change byte
search. Thus raw cross-build ratios are not clean causal estimates. The short
path's independent gain is **not established** by this noisy run. The full
ablation tables are observations and must be repeated on fixed idle hardware.

| Component / observation | zscan ablated / enabled | Unchanged memchr control ratio |
| --- | ---: | ---: |
| SIMD disabled, typical find | 1.409 | 0.598 |
| Prefilter disabled, typical find | 5.054 | 0.748 |
| Short path disabled, short find | 0.617 | 0.622 |
| Short path disabled, typical byte find | 0.535 | 0.581 |

The last row is a negative control: that cfg does not alter byte-search logic.
It shows why source-equivalent ablation builds do not by themselves establish a
clean performance effect. The short-path row is unresolved because its shift is
comparable to the unchanged baseline's shift.

Cold measurements use seven fresh processes for each public operation, with no
searcher warmup. zscan's observed medians were approximately 23–25 microseconds;
memchr reverse substring search was approximately 1.7 microseconds. First-use
CPU detection and cold code are real costs and are not hidden in the warm
one-shot measurements. These single-call observations include timer overhead
and are not used to claim the 2x target.
They measure the whole first operation, not isolated CPUID instruction cost.

memx reverse substring search has a separately reproduced correctness defect.
Its timings and actual answers remain in every run but its entire reverse
operation is ineligible for speed claims. memchr remains a strict baseline in
all directions; no zscan or memchr mismatch was accepted.

