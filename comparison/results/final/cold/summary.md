# Recorded search comparison

These are observations on the machine recorded in each metadata.json. Shared/cloud hardware is not a reliable release performance gate. Ratios are the fastest valid competitor's median ns/op divided by zscan's median ns/op; above 1 means zscan is faster. Every sample is retained, including outliers, slower cases, and the diagnostic timings of an incorrect comparator.

memx 0.2.2's reverse substring implementation omits a valid final candidate on some inputs. All memx/rfind cells are ineligible for speed claims; their actual correctness and timing remain in samples.csv.gz. memchr is the strict reverse baseline. See [baseline-notes.md](../../../baseline-notes.md).

## generic

Mode: `cold`. RUSTFLAGS: `(empty)`. Retained 12 engine/case cells; 0 raw samples returned a wrong comparator answer, and 7 samples were excluded from ratios because the comparator operation is not semantics-equivalent. No zscan or memchr correctness failure is permitted in a completed run.

| Group | Operation | Cases | Geometric speedup | Minimum | Median | Maximum | At least 2x | Slower than best |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| cold | byte_find | 1 | 0.893x | 0.893x | 0.893x | 0.893x | 0 | 1 |
| cold | byte_rfind | 1 | 0.783x | 0.783x | 0.783x | 0.783x | 0 | 1 |
| cold | find | 1 | 0.817x | 0.817x | 0.817x | 0.817x | 0 | 1 |
| cold | rfind | 1 | 0.069x | 0.069x | 0.069x | 0.069x | 0 | 1 |

### Ten lowest ratios (all remaining cases are in cells.csv)

| Case | Operation | zscan ns/op | Best baseline ns/op | Baseline | Speedup |
| --- | --- | ---: | ---: | --- | ---: |
| cold-none-n4096-m16-a17 | rfind | 24972.000 | 1734.000 | memchr | 0.069x |
| cold-none-n4096-m16-a17 | byte_rfind | 22595.000 | 17699.000 | memx | 0.783x |
| cold-none-n4096-m16-a17 | find | 23595.000 | 19280.000 | memx | 0.817x |
| cold-none-n4096-m16-a17 | byte_find | 23747.000 | 21212.000 | memx | 0.893x |

