# Recorded search comparison

These are observations on the machine recorded in each metadata.json. Shared/cloud hardware is not a reliable release performance gate. Ratios are the fastest valid competitor's median ns/op divided by zscan's median ns/op; above 1 means zscan is faster. Every sample is retained, including outliers, slower cases, and the diagnostic timings of an incorrect comparator.

memx 0.2.2's reverse substring implementation omits a valid final candidate on some inputs. All memx/rfind cells are ineligible for speed claims; their actual correctness and timing remain in samples.csv.gz. memchr is the strict reverse baseline. See [baseline-notes.md](../../baseline-notes.md).

## generic

Mode: `full`. RUSTFLAGS: `(empty)`. Retained 10926 engine/case cells; 630 raw samples returned a wrong comparator answer, and 4249 samples were excluded from ratios because the comparator operation is not semantics-equivalent. No zscan or memchr correctness failure is permitted in a completed run.

| Group | Operation | Cases | Geometric speedup | Minimum | Median | Maximum | At least 2x | Slower than best |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| adversarial | build_both | 24 | 1.170x | 0.444x | 1.094x | 1.875x | 0 | 8 |
| adversarial | byte_find | 24 | 0.651x | 0.416x | 0.660x | 1.005x | 0 | 23 |
| adversarial | byte_rfind | 24 | 0.677x | 0.443x | 0.666x | 1.023x | 0 | 23 |
| adversarial | find | 24 | 0.285x | 0.006x | 1.000x | 2.626x | 4 | 12 |
| adversarial | reuse_find | 24 | 0.401x | 0.005x | 1.401x | 10.929x | 10 | 12 |
| adversarial | reuse_rfind | 24 | 2.391x | 0.813x | 1.195x | 39.767x | 10 | 7 |
| adversarial | rfind | 24 | 4.581x | 0.924x | 1.509x | 74.433x | 10 | 1 |
| alignment | build_both | 64 | 1.205x | 0.438x | 1.083x | 3.923x | 8 | 24 |
| alignment | byte_find | 64 | 0.574x | 0.143x | 0.601x | 1.338x | 0 | 62 |
| alignment | byte_rfind | 64 | 0.684x | 0.235x | 0.679x | 1.179x | 0 | 62 |
| alignment | find | 64 | 0.822x | 0.195x | 0.831x | 3.093x | 12 | 40 |
| alignment | reuse_find | 64 | 0.600x | 0.051x | 0.622x | 21.728x | 4 | 41 |
| alignment | reuse_rfind | 64 | 1.154x | 0.103x | 1.128x | 10.312x | 11 | 23 |
| alignment | rfind | 64 | 1.945x | 0.461x | 2.293x | 4.539x | 38 | 6 |
| large | build_both | 6 | 1.157x | 0.388x | 1.385x | 1.652x | 0 | 1 |
| large | byte_find | 6 | 0.477x | 0.251x | 0.547x | 0.777x | 0 | 6 |
| large | byte_rfind | 6 | 0.704x | 0.534x | 0.691x | 1.025x | 0 | 5 |
| large | find | 6 | 0.750x | 0.536x | 0.718x | 1.259x | 0 | 5 |
| large | reuse_find | 6 | 1.126x | 0.942x | 1.182x | 1.323x | 0 | 2 |
| large | reuse_rfind | 6 | 6.550x | 1.192x | 5.611x | 54.741x | 5 | 0 |
| large | rfind | 6 | 8.996x | 2.965x | 9.666x | 30.822x | 6 | 0 |
| short | build_both | 238 | 1.601x | 0.498x | 1.660x | 4.323x | 83 | 45 |
| short | byte_find | 238 | 0.798x | 0.257x | 0.845x | 2.106x | 1 | 181 |
| short | byte_rfind | 238 | 0.910x | 0.344x | 0.901x | 2.473x | 6 | 140 |
| short | find | 238 | 0.992x | 0.235x | 1.001x | 2.841x | 19 | 110 |
| short | reuse_find | 238 | 0.762x | 0.197x | 0.774x | 4.280x | 19 | 160 |
| short | reuse_rfind | 238 | 1.078x | 0.289x | 1.063x | 6.534x | 24 | 88 |
| short | rfind | 238 | 1.417x | 0.228x | 1.218x | 5.222x | 68 | 78 |
| tail | build_both | 32 | 1.183x | 0.627x | 1.085x | 3.016x | 2 | 13 |
| tail | byte_find | 32 | 0.581x | 0.280x | 0.615x | 1.405x | 0 | 30 |
| tail | byte_rfind | 32 | 0.768x | 0.388x | 0.777x | 1.640x | 0 | 29 |
| tail | find | 32 | 0.793x | 0.263x | 0.566x | 4.893x | 7 | 21 |
| tail | reuse_find | 32 | 0.531x | 0.211x | 0.377x | 2.192x | 1 | 22 |
| tail | reuse_rfind | 32 | 1.048x | 0.343x | 1.106x | 3.990x | 4 | 15 |
| tail | rfind | 32 | 2.610x | 0.895x | 2.231x | 9.486x | 19 | 2 |
| text | build_both | 48 | 0.942x | 0.654x | 0.921x | 1.733x | 0 | 38 |
| text | byte_find | 48 | 0.676x | 0.254x | 0.666x | 0.962x | 0 | 48 |
| text | byte_rfind | 48 | 0.747x | 0.331x | 0.779x | 1.045x | 0 | 47 |
| text | find | 48 | 1.314x | 0.187x | 1.570x | 3.231x | 12 | 10 |
| text | reuse_find | 48 | 0.840x | 0.153x | 1.052x | 2.108x | 2 | 20 |
| text | reuse_rfind | 48 | 2.030x | 0.605x | 1.938x | 9.804x | 21 | 7 |
| text | rfind | 48 | 4.256x | 1.012x | 4.538x | 10.708x | 40 | 0 |
| typical | build_both | 195 | 1.306x | 0.467x | 1.298x | 6.786x | 30 | 55 |
| typical | byte_find | 195 | 0.673x | 0.175x | 0.771x | 1.412x | 0 | 175 |
| typical | byte_rfind | 195 | 0.737x | 0.299x | 0.779x | 1.345x | 0 | 181 |
| typical | find | 195 | 1.256x | 0.264x | 1.154x | 4.489x | 45 | 76 |
| typical | reuse_find | 195 | 1.619x | 0.264x | 1.398x | 18.340x | 66 | 49 |
| typical | reuse_rfind | 195 | 3.304x | 0.567x | 3.133x | 42.464x | 151 | 11 |
| typical | rfind | 195 | 7.695x | 0.749x | 7.493x | 79.776x | 183 | 1 |

### Ten lowest ratios (all remaining cases are in cells.csv)

| Case | Operation | zscan ns/op | Best baseline ns/op | Baseline | Speedup |
| --- | --- | ---: | ---: | --- | ---: |
| adversarial-low-entropy-neartrue-n65536-m32 | reuse_find | 462437.000 | 2102.814 | memchr | 0.005x |
| adversarial-low-entropy-neartrue-n65536-m32 | find | 367667.000 | 2132.630 | memchr | 0.006x |
| adversarial-low-entropy-neartrue-n65536-m128 | reuse_find | 330805.000 | 2135.289 | memchr | 0.006x |
| adversarial-low-entropy-neartrue-n65536-m8 | reuse_find | 255360.000 | 1805.667 | memchr | 0.007x |
| adversarial-low-entropy-neartrue-n65536-m128 | find | 322173.000 | 2690.303 | memchr | 0.008x |
| adversarial-low-entropy-neartrue-n65536-m8 | find | 217675.000 | 1845.167 | memchr | 0.008x |
| adversarial-repeated-neartrue-n65536-m257 | reuse_find | 73055.000 | 2027.956 | memchr | 0.028x |
| adversarial-periodic-neartrue-n65536-m257 | reuse_find | 55200.000 | 1766.679 | memchr | 0.032x |
| adversarial-periodic-neartrue-n65536-m8 | reuse_find | 55532.000 | 1829.250 | memchr | 0.033x |
| adversarial-periodic-neartrue-n65536-m32 | reuse_find | 55385.000 | 1826.769 | memchr | 0.033x |

## Component observations

For ablations, the ratio is ablated zscan time / generic zscan time, so above 1 means the enabled component helped. These separate-process comparisons are sensitive to machine load. No interaction effects are removed.

| Ablation | Group | Operation | Cases | Geometric ratio |
| --- | --- | --- | ---: | ---: |

### Finder cache reuse

Ratio is one-shot / reuse time within the generic build. Construction is reported separately above as build_both and is not amortized into reuse. Ratios do not predict a break-even query count without including construction and needle length.

| Group | Engine | Direction | Cases | Geometric one-shot / reuse |
| --- | --- | --- | ---: | ---: |
| adversarial | memchr | find | 24 | 3.416x |
| adversarial | memchr | rfind | 24 | 1.796x |
| adversarial | zscan | find | 24 | 1.044x |
| adversarial | zscan | rfind | 24 | 0.937x |
| alignment | memchr | find | 64 | 3.273x |
| alignment | memchr | rfind | 64 | 1.733x |
| alignment | zscan | find | 64 | 1.060x |
| alignment | zscan | rfind | 64 | 1.027x |
| large | memchr | find | 6 | 1.000x |
| large | memchr | rfind | 6 | 1.395x |
| large | zscan | find | 6 | 1.201x |
| large | zscan | rfind | 6 | 1.016x |
| short | memchr | find | 238 | 2.158x |
| short | memchr | rfind | 238 | 1.359x |
| short | zscan | find | 238 | 1.092x |
| short | zscan | rfind | 238 | 1.034x |
| tail | memchr | find | 32 | 4.643x |
| tail | memchr | rfind | 32 | 2.454x |
| tail | zscan | find | 32 | 1.086x |
| tail | zscan | rfind | 32 | 0.985x |
| text | memchr | find | 48 | 3.131x |
| text | memchr | rfind | 48 | 2.158x |
| text | zscan | find | 48 | 1.049x |
| text | zscan | rfind | 48 | 1.029x |
| typical | memchr | find | 195 | 4.372x |
| typical | memchr | rfind | 195 | 2.443x |
| typical | zscan | find | 195 | 1.093x |
| typical | zscan | rfind | 195 | 1.049x |
