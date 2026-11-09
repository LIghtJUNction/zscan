# Recorded search comparison

These are observations on the machine recorded in each metadata.json. Shared/cloud hardware is not a reliable release performance gate. Ratios are the fastest valid competitor's median ns/op divided by zscan's median ns/op; above 1 means zscan is faster. Every sample is retained, including outliers, slower cases, and the diagnostic timings of an incorrect comparator.

memx 0.2.2's reverse substring implementation omits a valid final candidate on some inputs. All memx/rfind cells are ineligible for speed claims; their actual correctness and timing remain in samples.csv.gz. memchr is the strict reverse baseline. See [baseline-notes.md](../../baseline-notes.md).

## generic

Mode: `full`. RUSTFLAGS: `(empty)`. Retained 10926 engine/case cells; 630 raw samples returned a wrong comparator answer, and 4249 samples were excluded from ratios because the comparator operation is not semantics-equivalent. No zscan or memchr correctness failure is permitted in a completed run.

| Group | Operation | Cases | Geometric speedup | Minimum | Median | Maximum | At least 2x | Slower than best |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| adversarial | build_both | 24 | 1.339x | 0.831x | 1.313x | 2.401x | 2 | 4 |
| adversarial | byte_find | 24 | 0.630x | 0.150x | 0.660x | 0.832x | 0 | 24 |
| adversarial | byte_rfind | 24 | 0.672x | 0.216x | 0.755x | 1.053x | 0 | 23 |
| adversarial | find | 24 | 1.535x | 0.534x | 1.674x | 3.594x | 6 | 5 |
| adversarial | reuse_find | 24 | 1.996x | 0.877x | 1.561x | 8.017x | 9 | 2 |
| adversarial | reuse_rfind | 24 | 5.505x | 0.467x | 6.335x | 47.735x | 16 | 4 |
| adversarial | rfind | 24 | 9.666x | 0.845x | 18.246x | 74.926x | 16 | 4 |
| alignment | build_both | 64 | 1.328x | 0.501x | 1.216x | 4.426x | 7 | 13 |
| alignment | byte_find | 64 | 0.573x | 0.187x | 0.613x | 1.000x | 0 | 64 |
| alignment | byte_rfind | 64 | 0.827x | 0.313x | 0.821x | 1.358x | 0 | 53 |
| alignment | find | 64 | 1.215x | 0.532x | 1.063x | 3.252x | 13 | 30 |
| alignment | reuse_find | 64 | 0.796x | 0.105x | 0.694x | 9.446x | 4 | 42 |
| alignment | reuse_rfind | 64 | 1.337x | 0.232x | 1.444x | 3.732x | 19 | 21 |
| alignment | rfind | 64 | 2.236x | 0.652x | 2.359x | 18.603x | 37 | 5 |
| large | build_both | 6 | 1.403x | 0.901x | 1.580x | 1.918x | 0 | 2 |
| large | byte_find | 6 | 0.591x | 0.546x | 0.590x | 0.652x | 0 | 6 |
| large | byte_rfind | 6 | 0.708x | 0.610x | 0.744x | 0.778x | 0 | 6 |
| large | find | 6 | 0.874x | 0.590x | 0.868x | 1.258x | 0 | 4 |
| large | reuse_find | 6 | 1.149x | 0.885x | 1.160x | 1.518x | 0 | 1 |
| large | reuse_rfind | 6 | 5.300x | 1.493x | 6.084x | 12.535x | 5 | 0 |
| large | rfind | 6 | 8.134x | 2.900x | 8.374x | 24.081x | 6 | 0 |
| short | build_both | 238 | 1.920x | 0.135x | 2.070x | 11.506x | 124 | 30 |
| short | byte_find | 238 | 0.833x | 0.196x | 0.863x | 2.102x | 6 | 167 |
| short | byte_rfind | 238 | 0.986x | 0.144x | 0.970x | 2.911x | 15 | 131 |
| short | find | 238 | 1.128x | 0.209x | 1.138x | 4.093x | 23 | 74 |
| short | reuse_find | 238 | 0.888x | 0.327x | 0.882x | 5.027x | 20 | 142 |
| short | reuse_rfind | 238 | 1.241x | 0.327x | 1.147x | 10.222x | 37 | 70 |
| short | rfind | 238 | 1.490x | 0.297x | 1.427x | 18.875x | 81 | 73 |
| tail | build_both | 32 | 1.370x | 0.805x | 1.240x | 3.244x | 5 | 5 |
| tail | byte_find | 32 | 0.552x | 0.315x | 0.581x | 1.127x | 0 | 30 |
| tail | byte_rfind | 32 | 0.901x | 0.544x | 0.848x | 1.601x | 0 | 21 |
| tail | find | 32 | 1.063x | 0.336x | 0.939x | 2.725x | 7 | 17 |
| tail | reuse_find | 32 | 0.723x | 0.302x | 0.533x | 2.153x | 1 | 22 |
| tail | reuse_rfind | 32 | 1.252x | 0.356x | 1.440x | 3.221x | 6 | 11 |
| tail | rfind | 32 | 2.954x | 1.301x | 2.486x | 11.848x | 23 | 0 |
| text | build_both | 48 | 0.990x | 0.501x | 0.975x | 2.118x | 1 | 31 |
| text | byte_find | 48 | 0.680x | 0.388x | 0.649x | 1.430x | 0 | 44 |
| text | byte_rfind | 48 | 0.822x | 0.481x | 0.752x | 1.377x | 0 | 39 |
| text | find | 48 | 1.943x | 1.075x | 1.961x | 3.663x | 24 | 0 |
| text | reuse_find | 48 | 1.205x | 0.496x | 1.243x | 3.486x | 4 | 14 |
| text | reuse_rfind | 48 | 2.828x | 0.946x | 2.332x | 7.637x | 35 | 1 |
| text | rfind | 48 | 5.149x | 2.177x | 4.913x | 14.705x | 48 | 0 |
| typical | build_both | 195 | 1.336x | 0.327x | 1.355x | 3.678x | 22 | 38 |
| typical | byte_find | 195 | 0.935x | 0.277x | 0.957x | 2.670x | 1 | 118 |
| typical | byte_rfind | 195 | 0.894x | 0.090x | 0.907x | 2.053x | 1 | 133 |
| typical | find | 195 | 1.266x | 0.369x | 1.196x | 8.226x | 37 | 69 |
| typical | reuse_find | 195 | 1.586x | 0.292x | 1.361x | 9.697x | 59 | 44 |
| typical | reuse_rfind | 195 | 3.648x | 0.552x | 3.400x | 71.805x | 155 | 9 |
| typical | rfind | 195 | 8.392x | 1.682x | 7.739x | 68.508x | 189 | 0 |

### Ten lowest ratios (all remaining cases are in cells.csv)

| Case | Operation | zscan ns/op | Best baseline ns/op | Baseline | Speedup |
| --- | --- | ---: | ---: | --- | ---: |
| typical-sparse-n65536-m31-a0 | byte_rfind | 61.333 | 5.512 | memx | 0.090x |
| alignment-first-n62-m15-a30 | reuse_find | 62.971 | 6.589 | memchr | 0.105x |
| short-overlong-n8-m16-a24 | build_both | 567.191 | 76.677 | memchr | 0.135x |
| short-first-n63-m32-a31 | byte_rfind | 36.847 | 5.301 | memx | 0.144x |
| adversarial-periodic-nearfalse-n65536-m128 | byte_find | 11.429 | 1.716 | memx | 0.150x |
| alignment-none-n64-m17-a32 | byte_find | 18.272 | 3.414 | memchr | 0.187x |
| short-last-n63-m3-a2 | byte_find | 11.534 | 2.259 | memx | 0.196x |
| short-last-n3-m1-a4 | find | 29.141 | 6.099 | memchr | 0.209x |
| adversarial-periodic-neartrue-n65536-m257 | byte_rfind | 8.278 | 1.789 | memx | 0.216x |
| alignment-none-n37-m7-a5 | reuse_rfind | 29.985 | 6.970 | memchr | 0.232x |

## native

Mode: `full`. RUSTFLAGS: `-C target-cpu=native`. Retained 10926 engine/case cells; 630 raw samples returned a wrong comparator answer, and 4249 samples were excluded from ratios because the comparator operation is not semantics-equivalent. No zscan or memchr correctness failure is permitted in a completed run.

| Group | Operation | Cases | Geometric speedup | Minimum | Median | Maximum | At least 2x | Slower than best |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| adversarial | build_both | 24 | 1.261x | 0.658x | 1.266x | 3.124x | 2 | 8 |
| adversarial | byte_find | 24 | 1.529x | 1.059x | 1.501x | 2.073x | 2 | 0 |
| adversarial | byte_rfind | 24 | 1.100x | 0.266x | 1.436x | 1.675x | 0 | 7 |
| adversarial | find | 24 | 1.483x | 0.627x | 1.517x | 6.691x | 7 | 4 |
| adversarial | reuse_find | 24 | 1.845x | 0.843x | 1.370x | 7.764x | 7 | 3 |
| adversarial | reuse_rfind | 24 | 6.366x | 0.418x | 6.470x | 66.176x | 16 | 4 |
| adversarial | rfind | 24 | 9.967x | 0.374x | 23.478x | 73.830x | 16 | 4 |
| alignment | build_both | 64 | 1.121x | 0.646x | 1.018x | 2.221x | 3 | 27 |
| alignment | byte_find | 64 | 1.086x | 0.438x | 1.419x | 2.234x | 3 | 27 |
| alignment | byte_rfind | 64 | 1.168x | 0.427x | 1.261x | 2.290x | 2 | 20 |
| alignment | find | 64 | 1.217x | 0.463x | 1.150x | 3.535x | 10 | 23 |
| alignment | reuse_find | 64 | 0.698x | 0.285x | 0.612x | 2.192x | 3 | 45 |
| alignment | reuse_rfind | 64 | 1.491x | 0.487x | 1.455x | 11.450x | 22 | 16 |
| alignment | rfind | 64 | 2.391x | 0.585x | 2.564x | 5.668x | 41 | 2 |
| large | build_both | 6 | 1.267x | 0.738x | 1.449x | 1.831x | 0 | 2 |
| large | byte_find | 6 | 0.599x | 0.557x | 0.579x | 0.735x | 0 | 6 |
| large | byte_rfind | 6 | 0.866x | 0.422x | 0.840x | 1.506x | 0 | 4 |
| large | find | 6 | 0.927x | 0.687x | 0.887x | 1.538x | 0 | 4 |
| large | reuse_find | 6 | 1.307x | 1.039x | 1.233x | 1.940x | 0 | 0 |
| large | reuse_rfind | 6 | 5.595x | 1.470x | 5.820x | 19.566x | 5 | 0 |
| large | rfind | 6 | 9.985x | 2.728x | 10.346x | 28.854x | 6 | 0 |
| short | build_both | 238 | 1.616x | 0.554x | 1.562x | 12.325x | 77 | 41 |
| short | byte_find | 238 | 1.223x | 0.174x | 1.204x | 4.673x | 35 | 62 |
| short | byte_rfind | 238 | 1.391x | 0.276x | 1.448x | 3.674x | 55 | 54 |
| short | find | 238 | 1.231x | 0.171x | 1.259x | 4.538x | 33 | 60 |
| short | reuse_find | 238 | 0.922x | 0.171x | 0.999x | 5.736x | 23 | 119 |
| short | reuse_rfind | 238 | 1.296x | 0.258x | 1.224x | 9.684x | 41 | 60 |
| short | rfind | 238 | 1.677x | 0.352x | 1.492x | 23.283x | 79 | 53 |
| tail | build_both | 32 | 1.063x | 0.220x | 1.095x | 2.208x | 5 | 13 |
| tail | byte_find | 32 | 0.974x | 0.410x | 1.039x | 2.435x | 1 | 15 |
| tail | byte_rfind | 32 | 1.236x | 0.606x | 1.277x | 2.607x | 2 | 8 |
| tail | find | 32 | 1.038x | 0.396x | 0.916x | 3.295x | 7 | 18 |
| tail | reuse_find | 32 | 0.604x | 0.281x | 0.452x | 1.842x | 0 | 25 |
| tail | reuse_rfind | 32 | 1.324x | 0.557x | 1.397x | 2.645x | 8 | 8 |
| tail | rfind | 32 | 3.315x | 1.520x | 2.966x | 9.021x | 28 | 0 |
| text | build_both | 48 | 0.994x | 0.577x | 0.923x | 8.451x | 1 | 34 |
| text | byte_find | 48 | 1.487x | 0.550x | 1.499x | 2.187x | 6 | 3 |
| text | byte_rfind | 48 | 1.156x | 0.014x | 1.441x | 2.700x | 1 | 7 |
| text | find | 48 | 1.873x | 0.702x | 1.876x | 4.793x | 21 | 1 |
| text | reuse_find | 48 | 1.154x | 0.476x | 1.228x | 5.909x | 5 | 14 |
| text | reuse_rfind | 48 | 2.802x | 1.037x | 2.256x | 9.412x | 34 | 0 |
| text | rfind | 48 | 5.860x | 2.685x | 5.808x | 10.505x | 48 | 0 |
| typical | build_both | 195 | 1.228x | 0.491x | 1.248x | 4.013x | 16 | 54 |
| typical | byte_find | 195 | 1.375x | 0.553x | 1.357x | 2.757x | 29 | 38 |
| typical | byte_rfind | 195 | 1.249x | 0.609x | 1.210x | 2.549x | 3 | 28 |
| typical | find | 195 | 1.354x | 0.394x | 1.349x | 4.169x | 45 | 64 |
| typical | reuse_find | 195 | 1.528x | 0.367x | 1.264x | 11.259x | 48 | 61 |
| typical | reuse_rfind | 195 | 3.861x | 0.595x | 3.619x | 41.984x | 158 | 8 |
| typical | rfind | 195 | 8.652x | 1.736x | 8.195x | 74.710x | 194 | 0 |

### Ten lowest ratios (all remaining cases are in cells.csv)

| Case | Operation | zscan ns/op | Best baseline ns/op | Baseline | Speedup |
| --- | --- | ---: | ---: | --- | ---: |
| text-http-first-n65536-m15 | byte_rfind | 142.000 | 2.008 | memchr | 0.014x |
| text-http-none-n4096-m15 | byte_rfind | 39.000 | 1.952 | memchr | 0.050x |
| short-overlong-n3-m32-a35 | find | 13.211 | 2.256 | memx | 0.171x |
| short-overlong-n1-m8-a9 | reuse_find | 6.593 | 1.129 | memchr | 0.171x |
| short-none-n7-m1-a8 | byte_find | 35.333 | 6.156 | memx | 0.174x |
| short-none-n31-m2-a33 | reuse_find | 23.525 | 4.459 | memchr | 0.190x |
| short-none-n4-m4-a8 | reuse_find | 23.489 | 4.831 | memchr | 0.206x |
| tail-first-n67-m5-a7 | build_both | 160.225 | 35.271 | memchr | 0.220x |
| short-last-n31-m3-a34 | reuse_find | 22.915 | 5.576 | memchr | 0.243x |
| short-none-n7-m2-a9 | reuse_find | 33.000 | 8.148 | memchr | 0.247x |

## Component observations

For ablations, the ratio is ablated zscan time / generic zscan time, so above 1 means the enabled component helped. These separate-process comparisons are sensitive to machine load. No interaction effects are removed.

| Ablation | Group | Operation | Cases | Geometric ratio |
| --- | --- | --- | ---: | ---: |

### Finder cache reuse

Ratio is one-shot / reuse time within the generic build. Construction is reported separately above as build_both and is not amortized into reuse. Ratios do not predict a break-even query count without including construction and needle length.

| Group | Engine | Direction | Cases | Geometric one-shot / reuse |
| --- | --- | --- | ---: | ---: |
| adversarial | memchr | find | 24 | 3.289x |
| adversarial | memchr | rfind | 24 | 1.720x |
| adversarial | zscan | find | 24 | 0.937x |
| adversarial | zscan | rfind | 24 | 0.979x |
| alignment | memchr | find | 64 | 3.269x |
| alignment | memchr | rfind | 64 | 1.920x |
| alignment | zscan | find | 64 | 0.996x |
| alignment | zscan | rfind | 64 | 1.148x |
| large | memchr | find | 6 | 1.003x |
| large | memchr | rfind | 6 | 1.687x |
| large | zscan | find | 6 | 1.023x |
| large | zscan | rfind | 6 | 1.099x |
| short | memchr | find | 238 | 2.159x |
| short | memchr | rfind | 238 | 1.341x |
| short | zscan | find | 238 | 1.057x |
| short | zscan | rfind | 238 | 1.117x |
| tail | memchr | find | 32 | 4.611x |
| tail | memchr | rfind | 32 | 2.545x |
| tail | zscan | find | 32 | 1.064x |
| tail | zscan | rfind | 32 | 1.079x |
| text | memchr | find | 48 | 2.927x |
| text | memchr | rfind | 48 | 2.013x |
| text | zscan | find | 48 | 1.021x |
| text | zscan | rfind | 48 | 1.106x |
| typical | memchr | find | 195 | 4.205x |
| typical | memchr | rfind | 195 | 2.483x |
| typical | zscan | find | 195 | 1.034x |
| typical | zscan | rfind | 195 | 1.079x |
