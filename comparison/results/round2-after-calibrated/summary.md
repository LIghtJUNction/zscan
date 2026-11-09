# Recorded search comparison

These are observations on the machine recorded in each metadata.json. Shared/cloud hardware is not a reliable release performance gate. Ratios are the fastest valid competitor's median ns/op divided by zscan's median ns/op; above 1 means zscan is faster. Every sample is retained, including outliers, slower cases, and the diagnostic timings of an incorrect comparator.

memx 0.2.2's reverse substring implementation omits a valid final candidate on some inputs. All memx/rfind cells are ineligible for speed claims; their actual correctness and timing remain in samples.csv.gz. memchr is the strict reverse baseline. See [baseline-notes.md](../../baseline-notes.md).

## generic

Mode: `full`. RUSTFLAGS: `(empty)`. Retained 10926 engine/case cells; 630 raw samples returned a wrong comparator answer, and 4249 samples were excluded from ratios because the comparator operation is not semantics-equivalent. No zscan or memchr correctness failure is permitted in a completed run.

| Group | Operation | Cases | Geometric speedup | Minimum | Median | Maximum | At least 2x | Slower than best |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| adversarial | build_both | 24 | 1.360x | 0.569x | 1.472x | 2.509x | 4 | 4 |
| adversarial | byte_find | 24 | 0.766x | 0.512x | 0.822x | 0.934x | 0 | 24 |
| adversarial | byte_rfind | 24 | 0.764x | 0.443x | 0.750x | 1.264x | 0 | 20 |
| adversarial | find | 24 | 1.577x | 0.872x | 1.567x | 2.923x | 6 | 2 |
| adversarial | reuse_find | 24 | 2.246x | 0.695x | 1.768x | 13.418x | 10 | 4 |
| adversarial | reuse_rfind | 24 | 5.123x | 0.788x | 5.065x | 55.272x | 15 | 3 |
| adversarial | rfind | 24 | 8.695x | 0.895x | 16.644x | 47.799x | 16 | 6 |
| alignment | build_both | 64 | 1.265x | 0.461x | 1.124x | 4.123x | 9 | 18 |
| alignment | byte_find | 64 | 0.827x | 0.290x | 0.857x | 1.381x | 0 | 55 |
| alignment | byte_rfind | 64 | 1.046x | 0.443x | 1.111x | 2.037x | 1 | 23 |
| alignment | find | 64 | 1.292x | 0.600x | 1.235x | 3.090x | 8 | 17 |
| alignment | reuse_find | 64 | 0.875x | 0.403x | 0.776x | 2.710x | 4 | 43 |
| alignment | reuse_rfind | 64 | 1.416x | 0.471x | 1.540x | 3.932x | 15 | 16 |
| alignment | rfind | 64 | 2.588x | 0.877x | 2.769x | 7.090x | 49 | 4 |
| large | build_both | 6 | 1.128x | 0.419x | 1.287x | 2.310x | 1 | 2 |
| large | byte_find | 6 | 0.806x | 0.632x | 0.847x | 0.932x | 0 | 6 |
| large | byte_rfind | 6 | 0.828x | 0.550x | 0.818x | 1.184x | 0 | 4 |
| large | find | 6 | 0.821x | 0.322x | 1.057x | 1.180x | 0 | 2 |
| large | reuse_find | 6 | 0.989x | 0.545x | 1.102x | 1.212x | 0 | 2 |
| large | reuse_rfind | 6 | 3.975x | 0.934x | 4.445x | 13.815x | 5 | 1 |
| large | rfind | 6 | 9.270x | 2.867x | 12.129x | 21.018x | 6 | 0 |
| short | build_both | 238 | 1.829x | 0.478x | 1.994x | 11.463x | 118 | 39 |
| short | byte_find | 238 | 0.814x | 0.161x | 0.820x | 2.291x | 3 | 184 |
| short | byte_rfind | 238 | 0.993x | 0.206x | 0.999x | 2.777x | 5 | 124 |
| short | find | 238 | 1.205x | 0.345x | 1.170x | 4.042x | 25 | 60 |
| short | reuse_find | 238 | 0.955x | 0.227x | 0.937x | 4.666x | 22 | 127 |
| short | reuse_rfind | 238 | 1.149x | 0.174x | 1.075x | 6.246x | 42 | 90 |
| short | rfind | 238 | 1.677x | 0.282x | 1.486x | 11.416x | 94 | 63 |
| tail | build_both | 32 | 1.459x | 0.811x | 1.402x | 3.356x | 7 | 5 |
| tail | byte_find | 32 | 0.943x | 0.595x | 0.915x | 1.605x | 0 | 22 |
| tail | byte_rfind | 32 | 1.049x | 0.572x | 1.134x | 1.760x | 0 | 14 |
| tail | find | 32 | 1.314x | 0.547x | 1.078x | 3.906x | 12 | 16 |
| tail | reuse_find | 32 | 0.896x | 0.399x | 0.815x | 2.035x | 2 | 22 |
| tail | reuse_rfind | 32 | 1.497x | 0.545x | 1.509x | 3.686x | 10 | 6 |
| tail | rfind | 32 | 3.841x | 1.911x | 3.755x | 9.626x | 31 | 0 |
| text | build_both | 48 | 0.943x | 0.264x | 0.990x | 2.894x | 1 | 27 |
| text | byte_find | 48 | 0.761x | 0.394x | 0.777x | 1.118x | 0 | 42 |
| text | byte_rfind | 48 | 0.803x | 0.161x | 0.774x | 1.353x | 0 | 35 |
| text | find | 48 | 1.922x | 0.841x | 1.876x | 4.449x | 23 | 3 |
| text | reuse_find | 48 | 1.131x | 0.138x | 1.120x | 2.655x | 5 | 11 |
| text | reuse_rfind | 48 | 2.446x | 0.467x | 1.908x | 9.524x | 22 | 1 |
| text | rfind | 48 | 5.265x | 1.205x | 5.377x | 14.772x | 47 | 0 |
| typical | build_both | 195 | 1.326x | 0.503x | 1.285x | 5.093x | 28 | 54 |
| typical | byte_find | 195 | 0.981x | 0.323x | 0.997x | 1.997x | 0 | 108 |
| typical | byte_rfind | 195 | 0.909x | 0.437x | 0.951x | 2.221x | 1 | 131 |
| typical | find | 195 | 1.338x | 0.365x | 1.188x | 5.906x | 44 | 70 |
| typical | reuse_find | 195 | 1.719x | 0.514x | 1.333x | 16.944x | 68 | 38 |
| typical | reuse_rfind | 195 | 3.088x | 0.524x | 3.018x | 34.249x | 145 | 17 |
| typical | rfind | 195 | 7.665x | 1.396x | 7.974x | 60.406x | 186 | 0 |

### Ten lowest ratios (all remaining cases are in cells.csv)

| Case | Operation | zscan ns/op | Best baseline ns/op | Baseline | Speedup |
| --- | --- | ---: | ---: | --- | ---: |
| text-json-last-n4096-m14 | reuse_find | 829.548 | 114.757 | memchr | 0.138x |
| short-empty-n31-m0-a31 | byte_find | 21.917 | 3.518 | memchr | 0.161x |
| text-log-first-n512-m11 | byte_rfind | 20.621 | 3.319 | memx | 0.161x |
| short-overlong-n1-m16-a17 | reuse_rfind | 11.398 | 1.980 | memchr | 0.174x |
| short-last-n17-m3-a20 | byte_rfind | 10.857 | 2.232 | memx | 0.206x |
| short-first-n2-m1-a3 | byte_rfind | 12.142 | 2.513 | memchr | 0.207x |
| short-none-n32-m8-a40 | reuse_find | 25.727 | 5.853 | memchr | 0.227x |
| short-empty-n2-m0-a2 | reuse_find | 6.466 | 1.502 | memchr | 0.232x |
| short-overlong-n1-m32-a33 | byte_find | 9.545 | 2.220 | memchr | 0.233x |
| text-log-dense-n128-m11 | build_both | 231.467 | 61.154 | memchr | 0.264x |

## native

Mode: `full`. RUSTFLAGS: `-C target-cpu=native`. Retained 10926 engine/case cells; 630 raw samples returned a wrong comparator answer, and 4249 samples were excluded from ratios because the comparator operation is not semantics-equivalent. No zscan or memchr correctness failure is permitted in a completed run.

| Group | Operation | Cases | Geometric speedup | Minimum | Median | Maximum | At least 2x | Slower than best |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| adversarial | build_both | 24 | 1.272x | 0.672x | 1.262x | 2.600x | 2 | 6 |
| adversarial | byte_find | 24 | 1.613x | 0.459x | 1.619x | 3.795x | 5 | 1 |
| adversarial | byte_rfind | 24 | 1.418x | 0.898x | 1.459x | 2.224x | 1 | 2 |
| adversarial | find | 24 | 1.469x | 0.666x | 1.540x | 3.040x | 6 | 5 |
| adversarial | reuse_find | 24 | 2.282x | 0.399x | 1.822x | 20.778x | 10 | 2 |
| adversarial | reuse_rfind | 24 | 5.751x | 0.493x | 6.741x | 90.161x | 16 | 6 |
| adversarial | rfind | 24 | 9.515x | 0.716x | 21.458x | 83.832x | 16 | 5 |
| alignment | build_both | 64 | 1.225x | 0.208x | 1.161x | 3.438x | 6 | 15 |
| alignment | byte_find | 64 | 1.365x | 0.298x | 1.378x | 2.764x | 6 | 8 |
| alignment | byte_rfind | 64 | 1.393x | 0.305x | 1.469x | 2.562x | 5 | 7 |
| alignment | find | 64 | 1.395x | 0.184x | 1.409x | 4.102x | 13 | 11 |
| alignment | reuse_find | 64 | 0.895x | 0.343x | 0.787x | 3.291x | 6 | 43 |
| alignment | reuse_rfind | 64 | 1.717x | 0.540x | 1.641x | 4.533x | 26 | 11 |
| alignment | rfind | 64 | 3.133x | 0.577x | 3.557x | 23.580x | 51 | 4 |
| large | build_both | 6 | 1.117x | 0.619x | 1.154x | 1.642x | 0 | 2 |
| large | byte_find | 6 | 0.774x | 0.643x | 0.794x | 0.866x | 0 | 6 |
| large | byte_rfind | 6 | 1.022x | 0.427x | 1.046x | 1.759x | 0 | 2 |
| large | find | 6 | 0.871x | 0.585x | 0.879x | 1.353x | 0 | 3 |
| large | reuse_find | 6 | 1.343x | 1.014x | 1.325x | 1.856x | 0 | 0 |
| large | reuse_rfind | 6 | 6.122x | 1.142x | 7.363x | 13.598x | 5 | 0 |
| large | rfind | 6 | 10.255x | 2.859x | 11.048x | 28.962x | 6 | 0 |
| short | build_both | 238 | 1.515x | 0.540x | 1.445x | 4.639x | 73 | 38 |
| short | byte_find | 238 | 1.187x | 0.366x | 1.145x | 4.189x | 28 | 71 |
| short | byte_rfind | 238 | 1.263x | 0.265x | 1.311x | 3.660x | 42 | 64 |
| short | find | 238 | 1.244x | 0.315x | 1.284x | 4.310x | 34 | 73 |
| short | reuse_find | 238 | 0.934x | 0.136x | 0.953x | 4.921x | 24 | 125 |
| short | reuse_rfind | 238 | 1.239x | 0.251x | 1.196x | 10.530x | 44 | 75 |
| short | rfind | 238 | 1.778x | 0.499x | 1.585x | 18.372x | 93 | 46 |
| tail | build_both | 32 | 1.206x | 0.372x | 1.150x | 2.466x | 2 | 6 |
| tail | byte_find | 32 | 1.367x | 0.698x | 1.376x | 2.976x | 1 | 2 |
| tail | byte_rfind | 32 | 1.532x | 0.651x | 1.624x | 2.432x | 7 | 5 |
| tail | find | 32 | 1.472x | 1.001x | 1.403x | 2.894x | 5 | 0 |
| tail | reuse_find | 32 | 0.956x | 0.479x | 0.895x | 2.384x | 2 | 19 |
| tail | reuse_rfind | 32 | 1.842x | 0.589x | 1.852x | 6.874x | 16 | 3 |
| tail | rfind | 32 | 4.317x | 1.820x | 4.700x | 8.859x | 31 | 0 |
| text | build_both | 48 | 1.087x | 0.684x | 1.068x | 2.174x | 1 | 13 |
| text | byte_find | 48 | 1.649x | 1.145x | 1.565x | 2.669x | 12 | 0 |
| text | byte_rfind | 48 | 1.558x | 0.663x | 1.556x | 2.402x | 10 | 2 |
| text | find | 48 | 1.892x | 0.723x | 2.091x | 3.471x | 25 | 3 |
| text | reuse_find | 48 | 1.010x | 0.349x | 0.972x | 2.236x | 1 | 25 |
| text | reuse_rfind | 48 | 3.115x | 1.506x | 2.312x | 11.852x | 38 | 0 |
| text | rfind | 48 | 6.232x | 3.755x | 6.107x | 15.822x | 48 | 0 |
| typical | build_both | 195 | 1.285x | 0.409x | 1.258x | 8.707x | 19 | 41 |
| typical | byte_find | 195 | 1.444x | 0.300x | 1.404x | 5.592x | 73 | 49 |
| typical | byte_rfind | 195 | 1.284x | 0.267x | 1.292x | 3.992x | 37 | 38 |
| typical | find | 195 | 1.339x | 0.254x | 1.260x | 4.488x | 44 | 61 |
| typical | reuse_find | 195 | 1.755x | 0.254x | 1.399x | 21.191x | 70 | 61 |
| typical | reuse_rfind | 195 | 3.618x | 0.219x | 3.640x | 56.354x | 157 | 7 |
| typical | rfind | 195 | 8.499x | 0.402x | 8.454x | 92.080x | 186 | 1 |

### Ten lowest ratios (all remaining cases are in cells.csv)

| Case | Operation | zscan ns/op | Best baseline ns/op | Baseline | Speedup |
| --- | --- | ---: | ---: | --- | ---: |
| short-none-n31-m8-a39 | reuse_find | 33.398 | 4.546 | memchr | 0.136x |
| short-none-n32-m8-a40 | reuse_find | 24.823 | 4.541 | memchr | 0.183x |
| alignment-last-n39-m8-a40 | find | 45.445 | 8.379 | memx | 0.184x |
| alignment-last-n39-m8-a40 | build_both | 241.059 | 50.029 | memchr | 0.208x |
| short-none-n31-m2-a33 | reuse_find | 20.348 | 4.465 | memchr | 0.219x |
| typical-sparse-n128-m16-a0 | reuse_rfind | 173.328 | 38.041 | memchr | 0.219x |
| short-overlong-n1-m4-a5 | reuse_find | 4.963 | 1.117 | memchr | 0.225x |
| short-none-n32-m3-a35 | reuse_find | 21.651 | 4.951 | memchr | 0.229x |
| short-none-n32-m4-a36 | reuse_find | 19.500 | 4.464 | memchr | 0.229x |
| short-none-n31-m3-a34 | reuse_find | 19.743 | 4.526 | memchr | 0.229x |

## Component observations

For ablations, the ratio is ablated zscan time / generic zscan time, so above 1 means the enabled component helped. These separate-process comparisons are sensitive to machine load. No interaction effects are removed.

| Ablation | Group | Operation | Cases | Geometric ratio |
| --- | --- | --- | ---: | ---: |

### Finder cache reuse

Ratio is one-shot / reuse time within the generic build. Construction is reported separately above as build_both and is not amortized into reuse. Ratios do not predict a break-even query count without including construction and needle length.

| Group | Engine | Direction | Cases | Geometric one-shot / reuse |
| --- | --- | --- | ---: | ---: |
| adversarial | memchr | find | 24 | 3.044x |
| adversarial | memchr | rfind | 24 | 1.682x |
| adversarial | zscan | find | 24 | 1.019x |
| adversarial | zscan | rfind | 24 | 0.991x |
| alignment | memchr | find | 64 | 3.624x |
| alignment | memchr | rfind | 64 | 1.915x |
| alignment | zscan | find | 64 | 1.078x |
| alignment | zscan | rfind | 64 | 1.048x |
| large | memchr | find | 6 | 1.013x |
| large | memchr | rfind | 6 | 1.954x |
| large | zscan | find | 6 | 0.975x |
| large | zscan | rfind | 6 | 0.838x |
| short | memchr | find | 238 | 2.247x |
| short | memchr | rfind | 238 | 1.472x |
| short | zscan | find | 238 | 1.103x |
| short | zscan | rfind | 238 | 1.009x |
| tail | memchr | find | 32 | 4.530x |
| tail | memchr | rfind | 32 | 2.739x |
| tail | zscan | find | 32 | 1.102x |
| tail | zscan | rfind | 32 | 1.068x |
| text | memchr | find | 48 | 3.398x |
| text | memchr | rfind | 48 | 2.217x |
| text | zscan | find | 48 | 0.970x |
| text | zscan | rfind | 48 | 1.030x |
| typical | memchr | find | 195 | 4.051x |
| typical | memchr | rfind | 195 | 2.558x |
| typical | zscan | find | 195 | 1.072x |
| typical | zscan | rfind | 195 | 1.031x |
