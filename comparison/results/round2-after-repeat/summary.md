# Recorded search comparison

These are observations on the machine recorded in each metadata.json. Shared/cloud hardware is not a reliable release performance gate. Ratios are the fastest valid competitor's median ns/op divided by zscan's median ns/op; above 1 means zscan is faster. Every sample is retained, including outliers, slower cases, and the diagnostic timings of an incorrect comparator.

memx 0.2.2's reverse substring implementation omits a valid final candidate on some inputs. All memx/rfind cells are ineligible for speed claims; their actual correctness and timing remain in samples.csv.gz. memchr is the strict reverse baseline. See [baseline-notes.md](../../baseline-notes.md).

## generic

Mode: `full`. RUSTFLAGS: `(empty)`. Retained 10926 engine/case cells; 630 raw samples returned a wrong comparator answer, and 4249 samples were excluded from ratios because the comparator operation is not semantics-equivalent. No zscan or memchr correctness failure is permitted in a completed run.

| Group | Operation | Cases | Geometric speedup | Minimum | Median | Maximum | At least 2x | Slower than best |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| adversarial | build_both | 24 | 1.375x | 0.745x | 1.441x | 2.174x | 2 | 3 |
| adversarial | byte_find | 24 | 0.740x | 0.278x | 0.768x | 1.154x | 0 | 23 |
| adversarial | byte_rfind | 24 | 0.739x | 0.470x | 0.752x | 1.067x | 0 | 23 |
| adversarial | find | 24 | 1.667x | 0.957x | 1.615x | 3.918x | 5 | 1 |
| adversarial | reuse_find | 24 | 2.474x | 0.947x | 1.862x | 13.237x | 10 | 2 |
| adversarial | reuse_rfind | 24 | 5.338x | 0.786x | 5.643x | 47.953x | 17 | 5 |
| adversarial | rfind | 24 | 8.142x | 0.793x | 19.076x | 59.262x | 16 | 7 |
| alignment | build_both | 64 | 1.302x | 0.673x | 1.240x | 3.114x | 6 | 11 |
| alignment | byte_find | 64 | 0.818x | 0.184x | 0.844x | 1.182x | 0 | 54 |
| alignment | byte_rfind | 64 | 1.025x | 0.530x | 1.096x | 1.549x | 0 | 23 |
| alignment | find | 64 | 1.288x | 0.250x | 1.240x | 3.728x | 10 | 17 |
| alignment | reuse_find | 64 | 0.933x | 0.365x | 0.821x | 2.114x | 4 | 36 |
| alignment | reuse_rfind | 64 | 1.396x | 0.514x | 1.400x | 3.843x | 13 | 14 |
| alignment | rfind | 64 | 2.633x | 0.802x | 2.782x | 6.683x | 47 | 5 |
| large | build_both | 6 | 1.624x | 0.995x | 1.661x | 2.942x | 2 | 1 |
| large | byte_find | 6 | 0.856x | 0.578x | 0.903x | 1.006x | 0 | 5 |
| large | byte_rfind | 6 | 0.658x | 0.345x | 0.681x | 0.991x | 0 | 6 |
| large | find | 6 | 0.841x | 0.458x | 0.986x | 1.134x | 0 | 3 |
| large | reuse_find | 6 | 1.098x | 0.654x | 1.252x | 1.293x | 0 | 1 |
| large | reuse_rfind | 6 | 5.194x | 1.044x | 6.253x | 16.382x | 5 | 0 |
| large | rfind | 6 | 9.473x | 2.827x | 9.337x | 27.167x | 6 | 0 |
| short | build_both | 238 | 2.011x | 0.236x | 2.157x | 14.233x | 132 | 31 |
| short | byte_find | 238 | 0.795x | 0.179x | 0.820x | 2.548x | 3 | 182 |
| short | byte_rfind | 238 | 0.965x | 0.191x | 1.000x | 2.275x | 8 | 120 |
| short | find | 238 | 1.191x | 0.195x | 1.159x | 4.876x | 33 | 71 |
| short | reuse_find | 238 | 0.937x | 0.224x | 0.946x | 6.777x | 21 | 128 |
| short | reuse_rfind | 238 | 1.135x | 0.201x | 1.060x | 6.449x | 35 | 95 |
| short | rfind | 238 | 1.657x | 0.398x | 1.448x | 7.457x | 91 | 57 |
| tail | build_both | 32 | 1.451x | 0.765x | 1.296x | 3.152x | 4 | 1 |
| tail | byte_find | 32 | 0.837x | 0.536x | 0.916x | 1.115x | 0 | 29 |
| tail | byte_rfind | 32 | 1.081x | 0.569x | 1.121x | 1.552x | 0 | 10 |
| tail | find | 32 | 1.254x | 0.461x | 1.221x | 3.474x | 5 | 12 |
| tail | reuse_find | 32 | 0.976x | 0.466x | 0.851x | 2.334x | 3 | 19 |
| tail | reuse_rfind | 32 | 1.581x | 0.808x | 1.545x | 3.336x | 10 | 4 |
| tail | rfind | 32 | 3.935x | 2.158x | 3.843x | 7.859x | 32 | 0 |
| text | build_both | 48 | 1.012x | 0.389x | 1.036x | 1.656x | 0 | 19 |
| text | byte_find | 48 | 0.786x | 0.356x | 0.826x | 1.227x | 0 | 43 |
| text | byte_rfind | 48 | 0.792x | 0.217x | 0.771x | 1.489x | 0 | 39 |
| text | find | 48 | 1.878x | 0.750x | 1.907x | 3.130x | 21 | 2 |
| text | reuse_find | 48 | 1.146x | 0.390x | 1.169x | 2.340x | 4 | 15 |
| text | reuse_rfind | 48 | 2.617x | 0.932x | 2.239x | 7.584x | 26 | 2 |
| text | rfind | 48 | 5.178x | 2.706x | 5.167x | 9.231x | 48 | 0 |
| typical | build_both | 195 | 1.353x | 0.257x | 1.315x | 4.309x | 28 | 47 |
| typical | byte_find | 195 | 0.946x | 0.405x | 0.967x | 1.555x | 0 | 116 |
| typical | byte_rfind | 195 | 0.890x | 0.204x | 0.926x | 1.707x | 0 | 139 |
| typical | find | 195 | 1.353x | 0.511x | 1.229x | 6.097x | 44 | 64 |
| typical | reuse_find | 195 | 1.777x | 0.505x | 1.426x | 13.550x | 69 | 35 |
| typical | reuse_rfind | 195 | 3.129x | 0.213x | 2.939x | 49.352x | 149 | 11 |
| typical | rfind | 195 | 7.614x | 1.456x | 7.528x | 88.338x | 192 | 0 |

### Ten lowest ratios (all remaining cases are in cells.csv)

| Case | Operation | zscan ns/op | Best baseline ns/op | Baseline | Speedup |
| --- | --- | ---: | ---: | --- | ---: |
| short-overlong-n0-m4-a4 | byte_find | 9.511 | 1.699 | memchr | 0.179x |
| alignment-last-n39-m8-a40 | byte_find | 8.852 | 1.632 | memx | 0.184x |
| short-last-n31-m16-a47 | byte_find | 11.063 | 2.072 | memchr | 0.187x |
| short-first-n7-m4-a11 | byte_rfind | 26.864 | 5.132 | memchr | 0.191x |
| short-last-n16-m2-a18 | find | 45.723 | 8.927 | memx | 0.195x |
| short-last-n1-m1-a2 | reuse_rfind | 18.081 | 3.630 | memchr | 0.201x |
| typical-first-n65536-m64-a0 | byte_rfind | 2979.644 | 608.134 | memchr | 0.204x |
| typical-dense-n65536-m2-a0 | reuse_rfind | 27.667 | 5.892 | memchr | 0.213x |
| text-log-last-n4096-m11 | byte_rfind | 7.721 | 1.674 | memx | 0.217x |
| short-none-n31-m3-a34 | find | 36.478 | 8.109 | memx | 0.222x |

## native

Mode: `full`. RUSTFLAGS: `-C target-cpu=native`. Retained 10926 engine/case cells; 630 raw samples returned a wrong comparator answer, and 4249 samples were excluded from ratios because the comparator operation is not semantics-equivalent. No zscan or memchr correctness failure is permitted in a completed run.

| Group | Operation | Cases | Geometric speedup | Minimum | Median | Maximum | At least 2x | Slower than best |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| adversarial | build_both | 24 | 1.248x | 0.458x | 1.220x | 2.980x | 1 | 5 |
| adversarial | byte_find | 24 | 1.660x | 1.007x | 1.615x | 2.805x | 2 | 0 |
| adversarial | byte_rfind | 24 | 1.401x | 0.490x | 1.505x | 2.143x | 2 | 1 |
| adversarial | find | 24 | 1.376x | 0.772x | 1.192x | 4.237x | 2 | 3 |
| adversarial | reuse_find | 24 | 2.588x | 0.830x | 2.149x | 13.333x | 13 | 2 |
| adversarial | reuse_rfind | 24 | 5.901x | 0.678x | 7.077x | 59.867x | 15 | 6 |
| adversarial | rfind | 24 | 9.353x | 0.782x | 20.732x | 91.853x | 16 | 6 |
| alignment | build_both | 64 | 1.157x | 0.607x | 1.099x | 2.446x | 5 | 18 |
| alignment | byte_find | 64 | 1.429x | 0.703x | 1.500x | 2.404x | 6 | 6 |
| alignment | byte_rfind | 64 | 1.507x | 0.633x | 1.556x | 3.277x | 9 | 5 |
| alignment | find | 64 | 1.493x | 0.647x | 1.422x | 3.958x | 13 | 11 |
| alignment | reuse_find | 64 | 0.858x | 0.412x | 0.772x | 3.261x | 5 | 48 |
| alignment | reuse_rfind | 64 | 1.599x | 0.568x | 1.639x | 4.681x | 19 | 8 |
| alignment | rfind | 64 | 3.096x | 0.966x | 3.316x | 8.011x | 53 | 1 |
| large | build_both | 6 | 1.183x | 0.881x | 1.205x | 1.741x | 0 | 2 |
| large | byte_find | 6 | 0.809x | 0.463x | 0.870x | 1.036x | 0 | 5 |
| large | byte_rfind | 6 | 1.049x | 0.746x | 0.975x | 1.466x | 0 | 3 |
| large | find | 6 | 0.996x | 0.561x | 1.121x | 1.443x | 0 | 2 |
| large | reuse_find | 6 | 1.168x | 0.621x | 1.348x | 1.697x | 0 | 2 |
| large | reuse_rfind | 6 | 3.931x | 0.886x | 4.019x | 10.135x | 5 | 1 |
| large | rfind | 6 | 10.523x | 3.672x | 9.874x | 27.359x | 6 | 0 |
| short | build_both | 238 | 1.556x | 0.462x | 1.494x | 5.643x | 78 | 33 |
| short | byte_find | 238 | 1.171x | 0.207x | 1.167x | 3.001x | 22 | 71 |
| short | byte_rfind | 238 | 1.275x | 0.324x | 1.331x | 3.785x | 39 | 64 |
| short | find | 238 | 1.286x | 0.378x | 1.286x | 4.114x | 36 | 58 |
| short | reuse_find | 238 | 0.942x | 0.101x | 1.000x | 5.202x | 21 | 119 |
| short | reuse_rfind | 238 | 1.280x | 0.267x | 1.187x | 7.339x | 45 | 68 |
| short | rfind | 238 | 1.766x | 0.300x | 1.597x | 8.728x | 89 | 43 |
| tail | build_both | 32 | 1.385x | 0.546x | 1.287x | 5.689x | 5 | 5 |
| tail | byte_find | 32 | 1.408x | 0.855x | 1.382x | 3.230x | 2 | 3 |
| tail | byte_rfind | 32 | 1.460x | 0.154x | 1.540x | 2.687x | 5 | 4 |
| tail | find | 32 | 1.542x | 0.952x | 1.595x | 2.561x | 6 | 3 |
| tail | reuse_find | 32 | 0.896x | 0.500x | 0.796x | 3.121x | 2 | 24 |
| tail | reuse_rfind | 32 | 1.616x | 0.150x | 1.913x | 4.498x | 16 | 4 |
| tail | rfind | 32 | 4.554x | 2.086x | 4.945x | 9.910x | 32 | 0 |
| text | build_both | 48 | 1.097x | 0.441x | 1.052x | 3.587x | 3 | 16 |
| text | byte_find | 48 | 1.485x | 0.296x | 1.583x | 2.943x | 8 | 3 |
| text | byte_rfind | 48 | 1.502x | 0.459x | 1.529x | 3.126x | 8 | 4 |
| text | find | 48 | 1.782x | 0.311x | 1.878x | 3.789x | 20 | 4 |
| text | reuse_find | 48 | 1.162x | 0.512x | 1.046x | 7.857x | 8 | 22 |
| text | reuse_rfind | 48 | 3.206x | 0.692x | 2.588x | 18.304x | 38 | 1 |
| text | rfind | 48 | 6.204x | 1.906x | 5.789x | 53.549x | 47 | 0 |
| typical | build_both | 195 | 1.283x | 0.441x | 1.299x | 3.128x | 17 | 43 |
| typical | byte_find | 195 | 1.572x | 0.637x | 1.447x | 3.197x | 77 | 22 |
| typical | byte_rfind | 195 | 1.301x | 0.429x | 1.267x | 4.192x | 35 | 33 |
| typical | find | 195 | 1.352x | 0.399x | 1.304x | 8.840x | 45 | 59 |
| typical | reuse_find | 195 | 1.712x | 0.410x | 1.273x | 26.840x | 67 | 60 |
| typical | reuse_rfind | 195 | 3.629x | 0.609x | 3.542x | 53.456x | 153 | 6 |
| typical | rfind | 195 | 8.433x | 1.627x | 7.691x | 82.000x | 193 | 0 |

### Ten lowest ratios (all remaining cases are in cells.csv)

| Case | Operation | zscan ns/op | Best baseline ns/op | Baseline | Speedup |
| --- | --- | ---: | ---: | --- | ---: |
| short-overlong-n7-m16-a23 | reuse_find | 11.342 | 1.145 | memchr | 0.101x |
| short-none-n32-m8-a40 | reuse_find | 47.972 | 5.510 | memchr | 0.115x |
| tail-none-n81-m4-a7 | reuse_rfind | 101.287 | 15.159 | memchr | 0.150x |
| tail-last-n80-m3-a7 | byte_rfind | 10.872 | 1.679 | memx | 0.154x |
| short-none-n32-m4-a36 | reuse_find | 26.875 | 4.480 | memchr | 0.167x |
| short-last-n31-m1-a32 | byte_find | 16.173 | 3.348 | memchr | 0.207x |
| short-none-n32-m2-a34 | reuse_find | 20.618 | 4.481 | memchr | 0.217x |
| short-none-n31-m2-a33 | reuse_find | 20.336 | 4.463 | memchr | 0.219x |
| short-none-n32-m3-a35 | reuse_find | 20.334 | 4.479 | memchr | 0.220x |
| short-none-n31-m3-a34 | reuse_find | 19.790 | 4.461 | memchr | 0.225x |

## Component observations

For ablations, the ratio is ablated zscan time / generic zscan time, so above 1 means the enabled component helped. These separate-process comparisons are sensitive to machine load. No interaction effects are removed.

| Ablation | Group | Operation | Cases | Geometric ratio |
| --- | --- | --- | ---: | ---: |

### Finder cache reuse

Ratio is one-shot / reuse time within the generic build. Construction is reported separately above as build_both and is not amortized into reuse. Ratios do not predict a break-even query count without including construction and needle length.

| Group | Engine | Direction | Cases | Geometric one-shot / reuse |
| --- | --- | --- | ---: | ---: |
| adversarial | memchr | find | 24 | 2.999x |
| adversarial | memchr | rfind | 24 | 1.620x |
| adversarial | zscan | find | 24 | 1.040x |
| adversarial | zscan | rfind | 24 | 1.062x |
| alignment | memchr | find | 64 | 3.445x |
| alignment | memchr | rfind | 64 | 1.990x |
| alignment | zscan | find | 64 | 1.094x |
| alignment | zscan | rfind | 64 | 1.055x |
| large | memchr | find | 6 | 0.901x |
| large | memchr | rfind | 6 | 1.916x |
| large | zscan | find | 6 | 0.956x |
| large | zscan | rfind | 6 | 1.051x |
| short | memchr | find | 238 | 2.164x |
| short | memchr | rfind | 238 | 1.457x |
| short | zscan | find | 238 | 1.071x |
| short | zscan | rfind | 238 | 0.998x |
| tail | memchr | find | 32 | 4.526x |
| tail | memchr | rfind | 32 | 2.708x |
| tail | zscan | find | 32 | 1.166x |
| tail | zscan | rfind | 32 | 1.088x |
| text | memchr | find | 48 | 3.194x |
| text | memchr | rfind | 48 | 2.210x |
| text | zscan | find | 48 | 0.981x |
| text | zscan | rfind | 48 | 1.117x |
| typical | memchr | find | 195 | 3.928x |
| typical | memchr | rfind | 195 | 2.513x |
| typical | zscan | find | 195 | 1.072x |
| typical | zscan | rfind | 195 | 1.033x |
