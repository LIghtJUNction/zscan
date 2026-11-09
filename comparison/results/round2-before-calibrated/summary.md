# Recorded search comparison

These are observations on the machine recorded in each metadata.json. Shared/cloud hardware is not a reliable release performance gate. Ratios are the fastest valid competitor's median ns/op divided by zscan's median ns/op; above 1 means zscan is faster. Every sample is retained, including outliers, slower cases, and the diagnostic timings of an incorrect comparator.

memx 0.2.2's reverse substring implementation omits a valid final candidate on some inputs. All memx/rfind cells are ineligible for speed claims; their actual correctness and timing remain in samples.csv.gz. memchr is the strict reverse baseline. See [baseline-notes.md](../../baseline-notes.md).

## generic

Mode: `full`. RUSTFLAGS: `(empty)`. Retained 10926 engine/case cells; 630 raw samples returned a wrong comparator answer, and 4249 samples were excluded from ratios because the comparator operation is not semantics-equivalent. No zscan or memchr correctness failure is permitted in a completed run.

| Group | Operation | Cases | Geometric speedup | Minimum | Median | Maximum | At least 2x | Slower than best |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| adversarial | build_both | 24 | 1.197x | 0.250x | 1.267x | 2.830x | 2 | 8 |
| adversarial | byte_find | 24 | 0.734x | 0.532x | 0.749x | 0.968x | 0 | 24 |
| adversarial | byte_rfind | 24 | 0.801x | 0.461x | 0.765x | 1.254x | 0 | 22 |
| adversarial | find | 24 | 1.575x | 0.659x | 1.638x | 3.354x | 6 | 2 |
| adversarial | reuse_find | 24 | 2.462x | 0.867x | 1.570x | 23.429x | 8 | 4 |
| adversarial | reuse_rfind | 24 | 5.314x | 0.657x | 6.099x | 56.706x | 17 | 4 |
| adversarial | rfind | 24 | 8.275x | 0.307x | 17.884x | 68.311x | 16 | 6 |
| alignment | build_both | 64 | 1.279x | 0.643x | 1.178x | 3.720x | 7 | 13 |
| alignment | byte_find | 64 | 0.548x | 0.294x | 0.593x | 0.889x | 0 | 64 |
| alignment | byte_rfind | 64 | 0.782x | 0.326x | 0.816x | 1.185x | 0 | 57 |
| alignment | find | 64 | 1.093x | 0.232x | 1.008x | 3.136x | 11 | 31 |
| alignment | reuse_find | 64 | 0.726x | 0.144x | 0.651x | 2.707x | 4 | 43 |
| alignment | reuse_rfind | 64 | 1.262x | 0.319x | 1.333x | 3.506x | 14 | 21 |
| alignment | rfind | 64 | 2.370x | 0.532x | 2.516x | 8.473x | 43 | 3 |
| large | build_both | 6 | 1.517x | 1.071x | 1.496x | 2.949x | 1 | 0 |
| large | byte_find | 6 | 0.651x | 0.579x | 0.608x | 0.989x | 0 | 6 |
| large | byte_rfind | 6 | 0.652x | 0.558x | 0.657x | 0.750x | 0 | 6 |
| large | find | 6 | 0.923x | 0.592x | 0.891x | 1.832x | 0 | 3 |
| large | reuse_find | 6 | 1.257x | 1.022x | 1.196x | 1.718x | 0 | 0 |
| large | reuse_rfind | 6 | 4.761x | 1.714x | 5.606x | 8.126x | 5 | 0 |
| large | rfind | 6 | 11.103x | 3.983x | 12.826x | 22.775x | 6 | 0 |
| short | build_both | 238 | 1.783x | 0.056x | 1.861x | 6.017x | 108 | 16 |
| short | byte_find | 238 | 0.713x | 0.137x | 0.749x | 3.304x | 5 | 195 |
| short | byte_rfind | 238 | 0.909x | 0.078x | 0.897x | 2.671x | 9 | 147 |
| short | find | 238 | 1.069x | 0.294x | 1.091x | 4.360x | 21 | 103 |
| short | reuse_find | 238 | 0.830x | 0.207x | 0.791x | 5.347x | 26 | 145 |
| short | reuse_rfind | 238 | 1.165x | 0.259x | 1.080x | 6.506x | 41 | 91 |
| short | rfind | 238 | 1.522x | 0.257x | 1.421x | 5.691x | 91 | 83 |
| tail | build_both | 32 | 1.421x | 0.796x | 1.314x | 2.832x | 5 | 3 |
| tail | byte_find | 32 | 0.559x | 0.192x | 0.589x | 0.874x | 0 | 32 |
| tail | byte_rfind | 32 | 0.765x | 0.496x | 0.750x | 1.187x | 0 | 28 |
| tail | find | 32 | 0.930x | 0.394x | 0.768x | 2.938x | 5 | 19 |
| tail | reuse_find | 32 | 0.664x | 0.342x | 0.480x | 2.595x | 2 | 22 |
| tail | reuse_rfind | 32 | 1.167x | 0.532x | 1.120x | 3.106x | 5 | 13 |
| tail | rfind | 32 | 3.217x | 1.559x | 2.903x | 9.770x | 26 | 0 |
| text | build_both | 48 | 1.080x | 0.638x | 0.998x | 2.611x | 1 | 24 |
| text | byte_find | 48 | 0.682x | 0.408x | 0.720x | 0.985x | 0 | 48 |
| text | byte_rfind | 48 | 0.840x | 0.504x | 0.806x | 1.393x | 0 | 41 |
| text | find | 48 | 1.698x | 0.911x | 1.670x | 3.299x | 12 | 2 |
| text | reuse_find | 48 | 1.067x | 0.448x | 1.114x | 1.986x | 0 | 18 |
| text | reuse_rfind | 48 | 2.599x | 1.206x | 2.210x | 11.856x | 34 | 0 |
| text | rfind | 48 | 5.460x | 2.295x | 5.100x | 18.517x | 48 | 0 |
| typical | build_both | 195 | 1.347x | 0.181x | 1.366x | 3.713x | 24 | 28 |
| typical | byte_find | 195 | 0.857x | 0.281x | 0.875x | 1.930x | 0 | 148 |
| typical | byte_rfind | 195 | 0.960x | 0.327x | 0.993x | 2.187x | 1 | 117 |
| typical | find | 195 | 1.290x | 0.241x | 1.219x | 5.395x | 41 | 75 |
| typical | reuse_find | 195 | 1.692x | 0.372x | 1.315x | 18.121x | 61 | 53 |
| typical | reuse_rfind | 195 | 3.347x | 0.569x | 3.185x | 53.130x | 145 | 10 |
| typical | rfind | 195 | 8.650x | 1.371x | 8.106x | 98.421x | 182 | 0 |

### Ten lowest ratios (all remaining cases are in cells.csv)

| Case | Operation | zscan ns/op | Best baseline ns/op | Baseline | Speedup |
| --- | --- | ---: | ---: | --- | ---: |
| short-overlong-n3-m32-a35 | build_both | 2402.286 | 133.616 | memchr | 0.056x |
| short-first-n63-m4-a3 | byte_rfind | 50.267 | 3.904 | memchr | 0.078x |
| short-last-n7-m1-a8 | byte_find | 30.473 | 4.187 | memx | 0.137x |
| alignment-last-n36-m6-a4 | reuse_find | 40.865 | 5.873 | memchr | 0.144x |
| short-first-n63-m4-a3 | byte_find | 9.349 | 1.678 | memx | 0.180x |
| typical-none-n128-m8-a0 | build_both | 285.824 | 51.657 | memchr | 0.181x |
| tail-last-n89-m12-a7 | byte_find | 14.879 | 2.857 | memx | 0.192x |
| short-none-n32-m8-a40 | reuse_find | 27.003 | 5.593 | memchr | 0.207x |
| short-none-n31-m2-a33 | reuse_find | 24.610 | 5.313 | memchr | 0.216x |
| short-none-n17-m2-a19 | reuse_find | 20.276 | 4.461 | memchr | 0.220x |

## native

Mode: `full`. RUSTFLAGS: `-C target-cpu=native`. Retained 10926 engine/case cells; 630 raw samples returned a wrong comparator answer, and 4249 samples were excluded from ratios because the comparator operation is not semantics-equivalent. No zscan or memchr correctness failure is permitted in a completed run.

| Group | Operation | Cases | Geometric speedup | Minimum | Median | Maximum | At least 2x | Slower than best |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| adversarial | build_both | 24 | 1.085x | 0.144x | 1.354x | 2.091x | 2 | 7 |
| adversarial | byte_find | 24 | 1.462x | 0.562x | 1.500x | 2.410x | 3 | 2 |
| adversarial | byte_rfind | 24 | 1.202x | 0.336x | 1.415x | 3.054x | 1 | 6 |
| adversarial | find | 24 | 1.218x | 0.324x | 1.569x | 3.965x | 5 | 8 |
| adversarial | reuse_find | 24 | 2.036x | 0.433x | 1.594x | 11.163x | 9 | 4 |
| adversarial | reuse_rfind | 24 | 6.118x | 0.445x | 5.291x | 72.823x | 16 | 4 |
| adversarial | rfind | 24 | 10.402x | 0.690x | 23.557x | 92.096x | 16 | 2 |
| alignment | build_both | 64 | 1.230x | 0.332x | 1.266x | 3.167x | 2 | 14 |
| alignment | byte_find | 64 | 1.082x | 0.472x | 1.091x | 2.842x | 3 | 27 |
| alignment | byte_rfind | 64 | 1.057x | 0.271x | 1.104x | 2.400x | 3 | 22 |
| alignment | find | 64 | 1.147x | 0.272x | 1.094x | 4.068x | 11 | 25 |
| alignment | reuse_find | 64 | 0.746x | 0.170x | 0.655x | 2.578x | 4 | 42 |
| alignment | reuse_rfind | 64 | 1.429x | 0.277x | 1.483x | 7.499x | 21 | 18 |
| alignment | rfind | 64 | 2.687x | 0.965x | 2.720x | 8.845x | 49 | 2 |
| large | build_both | 6 | 1.382x | 0.659x | 1.428x | 2.280x | 1 | 1 |
| large | byte_find | 6 | 0.583x | 0.528x | 0.566x | 0.664x | 0 | 6 |
| large | byte_rfind | 6 | 0.902x | 0.503x | 0.951x | 1.685x | 0 | 3 |
| large | find | 6 | 0.965x | 0.671x | 0.838x | 1.813x | 0 | 4 |
| large | reuse_find | 6 | 1.170x | 0.735x | 1.072x | 2.590x | 1 | 2 |
| large | reuse_rfind | 6 | 5.606x | 1.742x | 6.572x | 14.891x | 5 | 0 |
| large | rfind | 6 | 10.908x | 3.493x | 10.496x | 45.482x | 6 | 0 |
| short | build_both | 238 | 1.614x | 0.388x | 1.640x | 5.348x | 93 | 34 |
| short | byte_find | 238 | 1.244x | 0.289x | 1.243x | 3.812x | 36 | 57 |
| short | byte_rfind | 238 | 1.358x | 0.148x | 1.417x | 3.933x | 44 | 50 |
| short | find | 238 | 1.216x | 0.093x | 1.309x | 3.600x | 32 | 68 |
| short | reuse_find | 238 | 0.932x | 0.096x | 0.963x | 10.438x | 31 | 127 |
| short | reuse_rfind | 238 | 1.266x | 0.398x | 1.224x | 10.105x | 37 | 75 |
| short | rfind | 238 | 1.765x | 0.264x | 1.615x | 10.061x | 87 | 37 |
| tail | build_both | 32 | 1.233x | 0.534x | 1.255x | 3.703x | 5 | 10 |
| tail | byte_find | 32 | 0.924x | 0.404x | 0.970x | 2.000x | 0 | 16 |
| tail | byte_rfind | 32 | 1.180x | 0.714x | 1.184x | 2.285x | 1 | 9 |
| tail | find | 32 | 1.123x | 0.300x | 1.026x | 4.155x | 7 | 15 |
| tail | reuse_find | 32 | 0.644x | 0.141x | 0.473x | 2.101x | 3 | 21 |
| tail | reuse_rfind | 32 | 1.212x | 0.297x | 1.397x | 2.745x | 8 | 11 |
| tail | rfind | 32 | 3.240x | 0.538x | 3.135x | 16.621x | 28 | 1 |
| text | build_both | 48 | 1.012x | 0.305x | 0.987x | 3.627x | 3 | 29 |
| text | byte_find | 48 | 1.540x | 0.491x | 1.500x | 2.756x | 9 | 3 |
| text | byte_rfind | 48 | 1.296x | 0.402x | 1.448x | 2.128x | 2 | 8 |
| text | find | 48 | 1.749x | 0.459x | 1.748x | 9.138x | 20 | 6 |
| text | reuse_find | 48 | 1.077x | 0.217x | 1.127x | 10.545x | 7 | 18 |
| text | reuse_rfind | 48 | 3.013x | 0.814x | 2.606x | 11.736x | 32 | 1 |
| text | rfind | 48 | 5.440x | 1.146x | 5.573x | 22.263x | 45 | 0 |
| typical | build_both | 195 | 1.262x | 0.122x | 1.296x | 10.167x | 34 | 50 |
| typical | byte_find | 195 | 1.327x | 0.173x | 1.371x | 3.998x | 46 | 56 |
| typical | byte_rfind | 195 | 1.249x | 0.103x | 1.257x | 3.231x | 10 | 44 |
| typical | find | 195 | 1.294x | 0.122x | 1.367x | 6.413x | 46 | 69 |
| typical | reuse_find | 195 | 1.654x | 0.203x | 1.397x | 14.068x | 60 | 52 |
| typical | reuse_rfind | 195 | 3.815x | 0.500x | 3.503x | 63.159x | 158 | 11 |
| typical | rfind | 195 | 9.196x | 1.453x | 8.448x | 159.582x | 192 | 0 |

### Ten lowest ratios (all remaining cases are in cells.csv)

| Case | Operation | zscan ns/op | Best baseline ns/op | Baseline | Speedup |
| --- | --- | ---: | ---: | --- | ---: |
| short-last-n33-m8-a41 | find | 129.423 | 12.054 | memx | 0.093x |
| short-none-n33-m16-a49 | reuse_find | 52.217 | 5.023 | memchr | 0.096x |
| typical-last-n512-m256-a0 | byte_rfind | 59.058 | 6.098 | memchr | 0.103x |
| short-none-n31-m2-a33 | reuse_find | 37.485 | 4.469 | memchr | 0.119x |
| typical-sparse-n128-m31-a0 | find | 78.016 | 9.482 | memx | 0.122x |
| typical-none-n4096-m32-a0 | build_both | 1257.983 | 153.858 | memchr | 0.122x |
| tail-none-n66-m4-a7 | reuse_find | 35.964 | 5.088 | memchr | 0.141x |
| adversarial-repeated-neartrue-n65536-m32 | build_both | 1090.685 | 156.627 | memchr | 0.144x |
| short-last-n32-m1-a33 | byte_rfind | 11.910 | 1.762 | memx | 0.148x |
| short-none-n31-m2-a33 | find | 61.578 | 9.475 | memx | 0.154x |

## Component observations

For ablations, the ratio is ablated zscan time / generic zscan time, so above 1 means the enabled component helped. These separate-process comparisons are sensitive to machine load. No interaction effects are removed.

| Ablation | Group | Operation | Cases | Geometric ratio |
| --- | --- | --- | ---: | ---: |

### Finder cache reuse

Ratio is one-shot / reuse time within the generic build. Construction is reported separately above as build_both and is not amortized into reuse. Ratios do not predict a break-even query count without including construction and needle length.

| Group | Engine | Direction | Cases | Geometric one-shot / reuse |
| --- | --- | --- | ---: | ---: |
| adversarial | memchr | find | 24 | 2.980x |
| adversarial | memchr | rfind | 24 | 1.719x |
| adversarial | zscan | find | 24 | 1.030x |
| adversarial | zscan | rfind | 24 | 1.104x |
| alignment | memchr | find | 64 | 3.409x |
| alignment | memchr | rfind | 64 | 1.901x |
| alignment | zscan | find | 64 | 1.028x |
| alignment | zscan | rfind | 64 | 1.012x |
| large | memchr | find | 6 | 1.090x |
| large | memchr | rfind | 6 | 2.264x |
| large | zscan | find | 6 | 1.062x |
| large | zscan | rfind | 6 | 0.971x |
| short | memchr | find | 238 | 2.170x |
| short | memchr | rfind | 238 | 1.358x |
| short | zscan | find | 238 | 1.063x |
| short | zscan | rfind | 238 | 1.039x |
| tail | memchr | find | 32 | 4.466x |
| tail | memchr | rfind | 32 | 2.798x |
| tail | zscan | find | 32 | 1.085x |
| tail | zscan | rfind | 32 | 1.015x |
| text | memchr | find | 48 | 3.235x |
| text | memchr | rfind | 48 | 2.160x |
| text | zscan | find | 48 | 1.045x |
| text | zscan | rfind | 48 | 1.028x |
| typical | memchr | find | 195 | 4.086x |
| typical | memchr | rfind | 195 | 2.714x |
| typical | zscan | find | 195 | 1.048x |
| typical | zscan | rfind | 195 | 1.050x |
