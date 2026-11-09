# Recorded search comparison

These are observations on the machine recorded in each metadata.json. Shared/cloud hardware is not a reliable release performance gate. Ratios are the fastest valid competitor's median ns/op divided by zscan's median ns/op; above 1 means zscan is faster. Every sample is retained, including outliers, slower cases, and the diagnostic timings of an incorrect comparator.

memx 0.2.2's reverse substring implementation omits a valid final candidate on some inputs. All memx/rfind cells are ineligible for speed claims; their actual correctness and timing remain in samples.csv.gz. memchr is the strict reverse baseline. See [baseline-notes.md](../../baseline-notes.md).

## generic

Mode: `full`. RUSTFLAGS: `(empty)`. Retained 10926 engine/case cells; 630 raw samples returned a wrong comparator answer, and 4249 samples were excluded from ratios because the comparator operation is not semantics-equivalent. No zscan or memchr correctness failure is permitted in a completed run.

| Group | Operation | Cases | Geometric speedup | Minimum | Median | Maximum | At least 2x | Slower than best |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| adversarial | build_both | 24 | 1.160x | 0.461x | 1.232x | 1.799x | 0 | 8 |
| adversarial | byte_find | 24 | 0.640x | 0.419x | 0.635x | 0.983x | 0 | 24 |
| adversarial | byte_rfind | 24 | 0.728x | 0.547x | 0.753x | 0.919x | 0 | 24 |
| adversarial | find | 24 | 1.447x | 0.674x | 1.486x | 2.592x | 9 | 7 |
| adversarial | reuse_find | 24 | 1.985x | 0.850x | 1.545x | 8.522x | 6 | 2 |
| adversarial | reuse_rfind | 24 | 5.843x | 0.973x | 4.782x | 54.699x | 16 | 1 |
| adversarial | rfind | 24 | 10.184x | 0.866x | 23.120x | 74.637x | 16 | 3 |
| alignment | build_both | 64 | 1.191x | 0.640x | 1.101x | 2.869x | 4 | 22 |
| alignment | byte_find | 64 | 0.595x | 0.294x | 0.619x | 1.048x | 0 | 63 |
| alignment | byte_rfind | 64 | 0.793x | 0.493x | 0.790x | 1.455x | 0 | 60 |
| alignment | find | 64 | 1.066x | 0.338x | 0.925x | 4.081x | 10 | 35 |
| alignment | reuse_find | 64 | 0.828x | 0.229x | 0.685x | 2.353x | 1 | 40 |
| alignment | reuse_rfind | 64 | 1.348x | 0.355x | 1.464x | 4.524x | 14 | 17 |
| alignment | rfind | 64 | 2.108x | 0.783x | 2.324x | 4.716x | 41 | 5 |
| large | build_both | 6 | 1.059x | 0.773x | 0.966x | 1.569x | 0 | 4 |
| large | byte_find | 6 | 0.641x | 0.478x | 0.613x | 0.870x | 0 | 6 |
| large | byte_rfind | 6 | 0.915x | 0.730x | 0.843x | 1.508x | 0 | 5 |
| large | find | 6 | 1.160x | 0.969x | 1.130x | 1.492x | 0 | 2 |
| large | reuse_find | 6 | 1.274x | 0.886x | 1.395x | 1.541x | 0 | 1 |
| large | reuse_rfind | 6 | 5.385x | 1.464x | 6.368x | 15.940x | 5 | 0 |
| large | rfind | 6 | 9.782x | 4.026x | 10.352x | 27.670x | 6 | 0 |
| short | build_both | 238 | 1.618x | 0.414x | 1.742x | 4.042x | 87 | 41 |
| short | byte_find | 238 | 0.824x | 0.348x | 0.826x | 1.784x | 0 | 172 |
| short | byte_rfind | 238 | 0.975x | 0.309x | 0.988x | 2.266x | 2 | 121 |
| short | find | 238 | 1.082x | 0.316x | 1.105x | 3.385x | 22 | 89 |
| short | reuse_find | 238 | 0.899x | 0.358x | 0.858x | 4.000x | 20 | 142 |
| short | reuse_rfind | 238 | 1.206x | 0.399x | 1.112x | 7.261x | 31 | 78 |
| short | rfind | 238 | 1.538x | 0.343x | 1.379x | 5.960x | 80 | 56 |
| tail | build_both | 32 | 1.203x | 0.630x | 1.083x | 3.296x | 1 | 9 |
| tail | byte_find | 32 | 0.560x | 0.242x | 0.599x | 1.052x | 0 | 30 |
| tail | byte_rfind | 32 | 0.876x | 0.680x | 0.837x | 1.478x | 0 | 27 |
| tail | find | 32 | 0.928x | 0.411x | 0.764x | 2.388x | 5 | 19 |
| tail | reuse_find | 32 | 0.695x | 0.303x | 0.557x | 2.075x | 1 | 21 |
| tail | reuse_rfind | 32 | 1.242x | 0.413x | 1.404x | 2.909x | 7 | 9 |
| tail | rfind | 32 | 2.832x | 1.028x | 2.609x | 8.914x | 24 | 0 |
| text | build_both | 48 | 0.855x | 0.470x | 0.878x | 1.859x | 0 | 45 |
| text | byte_find | 48 | 0.709x | 0.355x | 0.657x | 1.310x | 0 | 43 |
| text | byte_rfind | 48 | 0.818x | 0.325x | 0.810x | 1.126x | 0 | 37 |
| text | find | 48 | 1.906x | 1.014x | 1.997x | 3.585x | 24 | 0 |
| text | reuse_find | 48 | 1.133x | 0.424x | 1.171x | 3.223x | 2 | 16 |
| text | reuse_rfind | 48 | 2.772x | 0.760x | 2.277x | 9.458x | 35 | 1 |
| text | rfind | 48 | 5.210x | 2.642x | 5.061x | 10.270x | 48 | 0 |
| typical | build_both | 195 | 1.248x | 0.598x | 1.252x | 3.344x | 17 | 55 |
| typical | byte_find | 195 | 0.871x | 0.442x | 0.873x | 1.640x | 0 | 161 |
| typical | byte_rfind | 195 | 0.880x | 0.398x | 0.891x | 1.492x | 0 | 158 |
| typical | find | 195 | 1.157x | 0.415x | 1.146x | 3.053x | 34 | 86 |
| typical | reuse_find | 195 | 1.650x | 0.395x | 1.485x | 8.238x | 62 | 48 |
| typical | reuse_rfind | 195 | 3.594x | 0.534x | 3.416x | 38.879x | 158 | 8 |
| typical | rfind | 195 | 8.122x | 1.343x | 7.625x | 79.364x | 191 | 0 |

### Ten lowest ratios (all remaining cases are in cells.csv)

| Case | Operation | zscan ns/op | Best baseline ns/op | Baseline | Speedup |
| --- | --- | ---: | ---: | --- | ---: |
| alignment-last-n42-m12-a10 | reuse_find | 53.824 | 12.330 | memchr | 0.229x |
| tail-none-n87-m10-a7 | byte_find | 12.830 | 3.103 | memchr | 0.242x |
| alignment-last-n51-m3-a52 | byte_find | 10.424 | 3.067 | memchr | 0.294x |
| tail-none-n66-m4-a7 | reuse_find | 19.400 | 5.869 | memchr | 0.303x |
| short-last-n17-m16-a33 | byte_rfind | 12.026 | 3.719 | memx | 0.309x |
| alignment-none-n61-m13-a62 | byte_find | 9.748 | 3.067 | memchr | 0.315x |
| short-none-n32-m4-a36 | find | 34.982 | 11.042 | memx | 0.316x |
| alignment-none-n55-m7-a56 | byte_find | 9.485 | 3.067 | memchr | 0.323x |
| text-log-last-n128-m11 | byte_rfind | 6.891 | 2.241 | memx | 0.325x |
| tail-none-n75-m13-a7 | reuse_find | 32.556 | 10.831 | memchr | 0.333x |

## native

Mode: `full`. RUSTFLAGS: `-C target-cpu=native`. Retained 10926 engine/case cells; 630 raw samples returned a wrong comparator answer, and 4249 samples were excluded from ratios because the comparator operation is not semantics-equivalent. No zscan or memchr correctness failure is permitted in a completed run.

| Group | Operation | Cases | Geometric speedup | Minimum | Median | Maximum | At least 2x | Slower than best |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| adversarial | build_both | 24 | 1.285x | 0.674x | 1.367x | 1.905x | 0 | 5 |
| adversarial | byte_find | 24 | 1.415x | 0.975x | 1.499x | 1.820x | 0 | 1 |
| adversarial | byte_rfind | 24 | 1.398x | 0.828x | 1.443x | 1.684x | 0 | 1 |
| adversarial | find | 24 | 1.398x | 0.703x | 1.481x | 2.812x | 6 | 7 |
| adversarial | reuse_find | 24 | 1.920x | 0.801x | 1.406x | 9.568x | 6 | 2 |
| adversarial | reuse_rfind | 24 | 6.276x | 0.813x | 5.784x | 59.018x | 16 | 2 |
| adversarial | rfind | 24 | 10.562x | 0.913x | 23.104x | 77.563x | 16 | 3 |
| alignment | build_both | 64 | 1.316x | 0.817x | 1.228x | 3.891x | 4 | 10 |
| alignment | byte_find | 64 | 1.071x | 0.354x | 1.191x | 2.004x | 2 | 23 |
| alignment | byte_rfind | 64 | 1.194x | 0.622x | 1.292x | 2.266x | 1 | 18 |
| alignment | find | 64 | 1.233x | 0.453x | 1.155x | 3.413x | 11 | 22 |
| alignment | reuse_find | 64 | 0.665x | 0.264x | 0.601x | 2.147x | 2 | 47 |
| alignment | reuse_rfind | 64 | 1.510x | 0.500x | 1.432x | 4.386x | 21 | 15 |
| alignment | rfind | 64 | 2.491x | 0.950x | 2.628x | 6.567x | 45 | 1 |
| large | build_both | 6 | 1.263x | 0.809x | 1.445x | 1.722x | 0 | 2 |
| large | byte_find | 6 | 0.647x | 0.445x | 0.614x | 1.126x | 0 | 5 |
| large | byte_rfind | 6 | 1.046x | 0.662x | 1.152x | 1.553x | 0 | 2 |
| large | find | 6 | 0.836x | 0.567x | 0.850x | 1.266x | 0 | 3 |
| large | reuse_find | 6 | 1.124x | 0.729x | 1.177x | 1.450x | 0 | 1 |
| large | reuse_rfind | 6 | 5.805x | 1.264x | 7.577x | 15.176x | 5 | 0 |
| large | rfind | 6 | 10.614x | 2.890x | 11.738x | 27.728x | 6 | 0 |
| short | build_both | 238 | 1.876x | 0.655x | 1.845x | 7.984x | 108 | 29 |
| short | byte_find | 238 | 1.250x | 0.310x | 1.245x | 4.010x | 25 | 54 |
| short | byte_rfind | 238 | 1.400x | 0.444x | 1.455x | 3.569x | 53 | 55 |
| short | find | 238 | 1.279x | 0.363x | 1.285x | 3.256x | 42 | 61 |
| short | reuse_find | 238 | 0.928x | 0.202x | 0.910x | 7.168x | 23 | 131 |
| short | reuse_rfind | 238 | 1.261x | 0.351x | 1.236x | 8.171x | 35 | 70 |
| short | rfind | 238 | 1.666x | 0.479x | 1.569x | 9.444x | 85 | 55 |
| tail | build_both | 32 | 1.358x | 0.879x | 1.262x | 3.136x | 5 | 6 |
| tail | byte_find | 32 | 0.926x | 0.469x | 0.965x | 1.529x | 0 | 19 |
| tail | byte_rfind | 32 | 1.257x | 0.811x | 1.266x | 1.945x | 0 | 8 |
| tail | find | 32 | 1.028x | 0.426x | 0.919x | 4.455x | 6 | 19 |
| tail | reuse_find | 32 | 0.595x | 0.289x | 0.500x | 1.804x | 0 | 23 |
| tail | reuse_rfind | 32 | 1.361x | 0.482x | 1.403x | 2.832x | 10 | 9 |
| tail | rfind | 32 | 3.029x | 1.427x | 2.868x | 8.247x | 28 | 0 |
| text | build_both | 48 | 1.026x | 0.688x | 0.990x | 2.087x | 1 | 27 |
| text | byte_find | 48 | 1.505x | 0.796x | 1.501x | 2.336x | 4 | 3 |
| text | byte_rfind | 48 | 1.431x | 0.594x | 1.443x | 2.360x | 4 | 4 |
| text | find | 48 | 1.812x | 0.841x | 1.849x | 3.947x | 19 | 3 |
| text | reuse_find | 48 | 1.093x | 0.542x | 1.148x | 2.088x | 1 | 15 |
| text | reuse_rfind | 48 | 2.905x | 1.212x | 2.315x | 7.998x | 36 | 0 |
| text | rfind | 48 | 6.058x | 2.386x | 5.996x | 15.246x | 48 | 0 |
| typical | build_both | 195 | 1.330x | 0.560x | 1.296x | 3.129x | 25 | 44 |
| typical | byte_find | 195 | 1.376x | 0.675x | 1.395x | 2.866x | 39 | 47 |
| typical | byte_rfind | 195 | 1.234x | 0.576x | 1.208x | 2.981x | 4 | 34 |
| typical | find | 195 | 1.328x | 0.428x | 1.279x | 5.056x | 43 | 66 |
| typical | reuse_find | 195 | 1.561x | 0.333x | 1.314x | 9.779x | 50 | 55 |
| typical | reuse_rfind | 195 | 3.715x | 0.687x | 3.689x | 65.459x | 161 | 7 |
| typical | rfind | 195 | 8.517x | 1.576x | 8.262x | 78.660x | 191 | 0 |

### Ten lowest ratios (all remaining cases are in cells.csv)

| Case | Operation | zscan ns/op | Best baseline ns/op | Baseline | Speedup |
| --- | --- | ---: | ---: | --- | ---: |
| short-last-n17-m3-a20 | reuse_find | 22.075 | 4.460 | memchr | 0.202x |
| short-last-n33-m4-a37 | reuse_find | 27.219 | 5.667 | memchr | 0.208x |
| short-none-n31-m8-a39 | reuse_find | 18.711 | 4.464 | memchr | 0.239x |
| alignment-last-n36-m6-a4 | reuse_find | 26.081 | 6.898 | memchr | 0.264x |
| alignment-last-n45-m15-a13 | reuse_find | 23.954 | 6.690 | memchr | 0.279x |
| short-none-n32-m2-a34 | reuse_find | 15.654 | 4.463 | memchr | 0.285x |
| alignment-none-n46-m16-a14 | reuse_find | 18.703 | 5.356 | memchr | 0.286x |
| tail-none-n66-m4-a7 | reuse_find | 16.958 | 4.902 | memchr | 0.289x |
| short-none-n33-m3-a36 | reuse_find | 15.150 | 4.463 | memchr | 0.295x |
| short-none-n31-m2-a33 | reuse_find | 15.059 | 4.459 | memchr | 0.296x |

## no-prefilter

Mode: `full`. RUSTFLAGS: `--cfg zscan_no_prefilter`. Retained 10926 engine/case cells; 630 raw samples returned a wrong comparator answer, and 4249 samples were excluded from ratios because the comparator operation is not semantics-equivalent. No zscan or memchr correctness failure is permitted in a completed run.

| Group | Operation | Cases | Geometric speedup | Minimum | Median | Maximum | At least 2x | Slower than best |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| adversarial | build_both | 24 | 1.332x | 0.749x | 1.347x | 2.378x | 2 | 4 |
| adversarial | byte_find | 24 | 0.744x | 0.563x | 0.749x | 1.238x | 0 | 22 |
| adversarial | byte_rfind | 24 | 0.647x | 0.349x | 0.746x | 0.788x | 0 | 24 |
| adversarial | find | 24 | 0.189x | 0.007x | 0.299x | 3.144x | 4 | 18 |
| adversarial | reuse_find | 24 | 0.278x | 0.005x | 0.524x | 7.456x | 8 | 13 |
| adversarial | reuse_rfind | 24 | 1.738x | 0.833x | 1.259x | 11.850x | 7 | 7 |
| adversarial | rfind | 24 | 2.918x | 0.835x | 1.303x | 84.464x | 7 | 7 |
| alignment | build_both | 64 | 1.263x | 0.657x | 1.144x | 3.248x | 7 | 16 |
| alignment | byte_find | 64 | 0.620x | 0.308x | 0.666x | 1.122x | 0 | 62 |
| alignment | byte_rfind | 64 | 0.750x | 0.391x | 0.750x | 1.188x | 0 | 57 |
| alignment | find | 64 | 0.405x | 0.084x | 0.200x | 3.398x | 11 | 43 |
| alignment | reuse_find | 64 | 0.347x | 0.098x | 0.189x | 2.246x | 3 | 44 |
| alignment | reuse_rfind | 64 | 0.451x | 0.078x | 0.321x | 3.504x | 12 | 42 |
| alignment | rfind | 64 | 0.717x | 0.287x | 0.446x | 5.078x | 13 | 43 |
| large | build_both | 6 | 1.334x | 0.920x | 1.444x | 1.731x | 0 | 2 |
| large | byte_find | 6 | 0.545x | 0.482x | 0.560x | 0.602x | 0 | 6 |
| large | byte_rfind | 6 | 0.690x | 0.623x | 0.693x | 0.768x | 0 | 6 |
| large | find | 6 | 0.024x | 0.016x | 0.025x | 0.030x | 0 | 6 |
| large | reuse_find | 6 | 0.031x | 0.027x | 0.032x | 0.035x | 0 | 6 |
| large | reuse_rfind | 6 | 0.745x | 0.110x | 0.762x | 5.695x | 2 | 3 |
| large | rfind | 6 | 1.490x | 0.116x | 1.843x | 37.606x | 3 | 3 |
| short | build_both | 238 | 1.677x | 0.422x | 1.825x | 3.345x | 90 | 40 |
| short | byte_find | 238 | 0.800x | 0.186x | 0.809x | 2.153x | 3 | 180 |
| short | byte_rfind | 238 | 0.964x | 0.264x | 0.977x | 2.207x | 13 | 127 |
| short | find | 238 | 0.977x | 0.058x | 1.111x | 3.567x | 24 | 87 |
| short | reuse_find | 238 | 0.806x | 0.097x | 0.873x | 5.223x | 18 | 135 |
| short | reuse_rfind | 238 | 1.005x | 0.087x | 1.050x | 4.719x | 24 | 94 |
| short | rfind | 238 | 1.453x | 0.164x | 1.307x | 8.114x | 75 | 52 |
| tail | build_both | 32 | 1.292x | 0.604x | 1.199x | 3.367x | 3 | 6 |
| tail | byte_find | 32 | 0.632x | 0.302x | 0.661x | 1.171x | 0 | 31 |
| tail | byte_rfind | 32 | 0.796x | 0.562x | 0.752x | 1.151x | 0 | 28 |
| tail | find | 32 | 0.364x | 0.090x | 0.154x | 4.115x | 10 | 21 |
| tail | reuse_find | 32 | 0.270x | 0.069x | 0.126x | 2.316x | 1 | 22 |
| tail | reuse_rfind | 32 | 0.354x | 0.068x | 0.228x | 2.750x | 3 | 21 |
| tail | rfind | 32 | 0.848x | 0.253x | 0.374x | 8.982x | 11 | 21 |
| text | build_both | 48 | 0.963x | 0.556x | 0.964x | 1.737x | 0 | 38 |
| text | byte_find | 48 | 0.766x | 0.299x | 0.749x | 1.182x | 0 | 40 |
| text | byte_rfind | 48 | 0.762x | 0.356x | 0.750x | 1.083x | 0 | 46 |
| text | find | 48 | 0.438x | 0.035x | 0.841x | 4.212x | 13 | 24 |
| text | reuse_find | 48 | 0.260x | 0.033x | 0.481x | 2.629x | 1 | 28 |
| text | reuse_rfind | 48 | 0.361x | 0.100x | 0.239x | 2.517x | 6 | 36 |
| text | rfind | 48 | 0.626x | 0.088x | 0.411x | 10.009x | 12 | 35 |
| typical | build_both | 195 | 1.281x | 0.437x | 1.285x | 3.306x | 18 | 49 |
| typical | byte_find | 195 | 0.971x | 0.206x | 0.955x | 2.142x | 1 | 114 |
| typical | byte_rfind | 195 | 0.870x | 0.257x | 0.899x | 1.427x | 0 | 145 |
| typical | find | 195 | 0.184x | 0.013x | 0.070x | 4.901x | 37 | 130 |
| typical | reuse_find | 195 | 0.271x | 0.015x | 0.161x | 10.336x | 41 | 117 |
| typical | reuse_rfind | 195 | 0.468x | 0.017x | 0.404x | 12.017x | 52 | 122 |
| typical | rfind | 195 | 0.939x | 0.028x | 0.566x | 91.029x | 68 | 123 |

### Ten lowest ratios (all remaining cases are in cells.csv)

| Case | Operation | zscan ns/op | Best baseline ns/op | Baseline | Speedup |
| --- | --- | ---: | ---: | --- | ---: |
| adversarial-low-entropy-neartrue-n65536-m128 | reuse_find | 410836.000 | 2148.911 | memchr | 0.005x |
| adversarial-low-entropy-neartrue-n65536-m32 | find | 293969.000 | 2082.133 | memchr | 0.007x |
| adversarial-low-entropy-neartrue-n65536-m32 | reuse_find | 273556.000 | 2071.319 | memchr | 0.008x |
| adversarial-low-entropy-neartrue-n65536-m8 | find | 223357.000 | 1831.500 | memchr | 0.008x |
| adversarial-low-entropy-neartrue-n65536-m128 | find | 323491.000 | 2698.657 | memchr | 0.008x |
| adversarial-low-entropy-neartrue-n65536-m8 | reuse_find | 197122.000 | 1811.833 | memchr | 0.009x |
| typical-last-n4096-m32-a0 | find | 6609.938 | 83.961 | memx | 0.013x |
| typical-none-n65536-m16-a0 | find | 80841.000 | 1173.698 | memx | 0.015x |
| typical-none-n4096-m127-a0 | reuse_find | 5267.429 | 76.680 | memchr | 0.015x |
| typical-last-n4096-m2-a0 | find | 9147.529 | 144.781 | memchr | 0.016x |

## no-short

Mode: `full`. RUSTFLAGS: `--cfg zscan_no_short`. Retained 10926 engine/case cells; 630 raw samples returned a wrong comparator answer, and 4249 samples were excluded from ratios because the comparator operation is not semantics-equivalent. No zscan or memchr correctness failure is permitted in a completed run.

| Group | Operation | Cases | Geometric speedup | Minimum | Median | Maximum | At least 2x | Slower than best |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| adversarial | build_both | 24 | 1.265x | 0.554x | 1.279x | 2.515x | 2 | 5 |
| adversarial | byte_find | 24 | 0.700x | 0.562x | 0.680x | 1.214x | 0 | 23 |
| adversarial | byte_rfind | 24 | 0.658x | 0.116x | 0.750x | 1.008x | 0 | 22 |
| adversarial | find | 24 | 1.034x | 0.260x | 1.083x | 2.868x | 3 | 10 |
| adversarial | reuse_find | 24 | 1.836x | 0.753x | 1.281x | 12.707x | 6 | 7 |
| adversarial | reuse_rfind | 24 | 4.808x | 0.451x | 6.402x | 52.033x | 14 | 7 |
| adversarial | rfind | 24 | 7.253x | 0.874x | 15.901x | 95.101x | 16 | 7 |
| alignment | build_both | 64 | 1.348x | 0.753x | 1.208x | 3.244x | 9 | 9 |
| alignment | byte_find | 64 | 0.600x | 0.269x | 0.607x | 1.194x | 0 | 60 |
| alignment | byte_rfind | 64 | 0.667x | 0.308x | 0.671x | 0.995x | 0 | 64 |
| alignment | find | 64 | 0.894x | 0.399x | 0.874x | 2.173x | 1 | 42 |
| alignment | reuse_find | 64 | 0.624x | 0.214x | 0.640x | 1.104x | 0 | 62 |
| alignment | reuse_rfind | 64 | 1.275x | 0.393x | 1.359x | 4.377x | 10 | 20 |
| alignment | rfind | 64 | 2.069x | 0.613x | 2.376x | 5.511x | 34 | 9 |
| large | build_both | 6 | 1.549x | 1.027x | 1.507x | 2.219x | 2 | 0 |
| large | byte_find | 6 | 0.526x | 0.381x | 0.561x | 0.607x | 0 | 6 |
| large | byte_rfind | 6 | 0.708x | 0.624x | 0.663x | 1.026x | 0 | 5 |
| large | find | 6 | 0.768x | 0.571x | 0.674x | 1.267x | 0 | 4 |
| large | reuse_find | 6 | 0.922x | 0.392x | 1.001x | 1.469x | 0 | 3 |
| large | reuse_rfind | 6 | 3.431x | 0.453x | 5.576x | 8.082x | 4 | 1 |
| large | rfind | 6 | 6.836x | 2.351x | 7.012x | 16.617x | 6 | 0 |
| short | build_both | 238 | 1.783x | 0.517x | 1.882x | 5.781x | 105 | 28 |
| short | byte_find | 238 | 0.901x | 0.209x | 0.909x | 2.629x | 13 | 140 |
| short | byte_rfind | 238 | 0.995x | 0.290x | 0.929x | 3.906x | 23 | 138 |
| short | find | 238 | 1.119x | 0.323x | 1.042x | 4.610x | 55 | 114 |
| short | reuse_find | 238 | 0.723x | 0.274x | 0.728x | 2.812x | 1 | 201 |
| short | reuse_rfind | 238 | 1.050x | 0.352x | 1.056x | 7.274x | 20 | 106 |
| short | rfind | 238 | 1.608x | 0.270x | 1.656x | 10.740x | 87 | 81 |
| tail | build_both | 32 | 1.319x | 0.625x | 1.190x | 3.063x | 7 | 6 |
| tail | byte_find | 32 | 0.647x | 0.417x | 0.627x | 1.154x | 0 | 30 |
| tail | byte_rfind | 32 | 0.776x | 0.484x | 0.772x | 1.230x | 0 | 29 |
| tail | find | 32 | 0.792x | 0.488x | 0.834x | 1.427x | 0 | 27 |
| tail | reuse_find | 32 | 0.588x | 0.319x | 0.591x | 1.207x | 0 | 30 |
| tail | reuse_rfind | 32 | 1.250x | 0.455x | 1.085x | 3.392x | 6 | 13 |
| tail | rfind | 32 | 2.957x | 1.518x | 2.997x | 6.057x | 29 | 0 |
| text | build_both | 48 | 0.972x | 0.538x | 0.992x | 1.812x | 0 | 27 |
| text | byte_find | 48 | 0.760x | 0.397x | 0.735x | 1.381x | 0 | 38 |
| text | byte_rfind | 48 | 0.758x | 0.224x | 0.751x | 1.001x | 0 | 46 |
| text | find | 48 | 1.275x | 0.611x | 1.221x | 2.933x | 9 | 16 |
| text | reuse_find | 48 | 0.797x | 0.398x | 0.743x | 1.809x | 0 | 36 |
| text | reuse_rfind | 48 | 2.470x | 1.069x | 2.227x | 7.730x | 25 | 0 |
| text | rfind | 48 | 5.016x | 2.138x | 5.140x | 9.386x | 48 | 0 |
| typical | build_both | 195 | 1.343x | 0.625x | 1.342x | 5.979x | 28 | 51 |
| typical | byte_find | 195 | 0.974x | 0.450x | 0.993x | 1.785x | 0 | 106 |
| typical | byte_rfind | 195 | 0.840x | 0.145x | 0.878x | 1.848x | 0 | 162 |
| typical | find | 195 | 1.082x | 0.287x | 1.093x | 2.879x | 7 | 81 |
| typical | reuse_find | 195 | 1.399x | 0.401x | 1.206x | 10.314x | 51 | 74 |
| typical | reuse_rfind | 195 | 3.036x | 0.443x | 2.909x | 54.916x | 140 | 19 |
| typical | rfind | 195 | 7.261x | 0.865x | 6.933x | 76.223x | 181 | 1 |

### Ten lowest ratios (all remaining cases are in cells.csv)

| Case | Operation | zscan ns/op | Best baseline ns/op | Baseline | Speedup |
| --- | --- | ---: | ---: | --- | ---: |
| adversarial-periodic-neartrue-n65536-m32 | byte_rfind | 14.600 | 1.687 | memx | 0.116x |
| typical-dense-n65536-m32-a0 | byte_rfind | 15.416 | 2.231 | memx | 0.145x |
| short-last-n63-m3-a2 | byte_find | 10.672 | 2.231 | memx | 0.209x |
| alignment-last-n42-m12-a10 | reuse_find | 44.738 | 9.576 | memchr | 0.214x |
| short-none-n63-m2-a1 | byte_find | 15.290 | 3.416 | memchr | 0.223x |
| text-json-none-n4096-m14 | byte_rfind | 9.385 | 2.098 | memx | 0.224x |
| short-last-n63-m4-a3 | byte_find | 8.910 | 2.234 | memx | 0.251x |
| short-empty-n63-m0-a63 | byte_find | 11.823 | 3.068 | memchr | 0.260x |
| adversarial-repeated-nearfalse-n65536-m32 | find | 37.294 | 9.701 | memx | 0.260x |
| adversarial-periodic-neartrue-n65536-m257 | byte_rfind | 6.333 | 1.696 | memx | 0.268x |

## scalar

Mode: `full`. RUSTFLAGS: `--cfg zscan_scalar`. Retained 10926 engine/case cells; 630 raw samples returned a wrong comparator answer, and 4249 samples were excluded from ratios because the comparator operation is not semantics-equivalent. No zscan or memchr correctness failure is permitted in a completed run.

| Group | Operation | Cases | Geometric speedup | Minimum | Median | Maximum | At least 2x | Slower than best |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| adversarial | build_both | 24 | 1.293x | 0.800x | 1.211x | 2.948x | 1 | 5 |
| adversarial | byte_find | 24 | 0.850x | 0.584x | 0.903x | 1.204x | 0 | 23 |
| adversarial | byte_rfind | 24 | 0.856x | 0.445x | 0.900x | 1.280x | 0 | 23 |
| adversarial | find | 24 | 0.493x | 0.101x | 0.463x | 3.009x | 4 | 16 |
| adversarial | reuse_find | 24 | 0.688x | 0.095x | 0.857x | 6.366x | 10 | 12 |
| adversarial | reuse_rfind | 24 | 2.849x | 0.662x | 3.103x | 12.308x | 16 | 4 |
| adversarial | rfind | 24 | 4.649x | 0.715x | 3.810x | 57.818x | 16 | 5 |
| alignment | build_both | 64 | 1.312x | 0.599x | 1.160x | 13.254x | 9 | 14 |
| alignment | byte_find | 64 | 0.578x | 0.259x | 0.542x | 1.240x | 0 | 60 |
| alignment | byte_rfind | 64 | 0.648x | 0.338x | 0.663x | 1.269x | 0 | 60 |
| alignment | find | 64 | 1.034x | 0.433x | 0.868x | 3.475x | 9 | 37 |
| alignment | reuse_find | 64 | 0.655x | 0.186x | 0.471x | 4.404x | 4 | 45 |
| alignment | reuse_rfind | 64 | 1.114x | 0.329x | 1.099x | 3.959x | 12 | 25 |
| alignment | rfind | 64 | 1.804x | 0.566x | 1.954x | 5.001x | 30 | 8 |
| large | build_both | 6 | 1.102x | 0.702x | 1.090x | 1.701x | 0 | 3 |
| large | byte_find | 6 | 0.128x | 0.114x | 0.131x | 0.133x | 0 | 6 |
| large | byte_rfind | 6 | 0.284x | 0.131x | 0.239x | 0.911x | 0 | 6 |
| large | find | 6 | 0.117x | 0.082x | 0.113x | 0.179x | 0 | 6 |
| large | reuse_find | 6 | 0.153x | 0.123x | 0.149x | 0.199x | 0 | 6 |
| large | reuse_rfind | 6 | 1.782x | 0.746x | 1.527x | 4.879x | 2 | 1 |
| large | rfind | 6 | 2.929x | 0.729x | 2.128x | 20.296x | 3 | 1 |
| short | build_both | 238 | 1.613x | 0.547x | 1.749x | 5.499x | 82 | 38 |
| short | byte_find | 238 | 0.902x | 0.219x | 0.913x | 3.027x | 15 | 138 |
| short | byte_rfind | 238 | 1.112x | 0.336x | 1.172x | 3.438x | 23 | 92 |
| short | find | 238 | 1.239x | 0.369x | 1.237x | 3.619x | 30 | 63 |
| short | reuse_find | 238 | 0.950x | 0.154x | 0.913x | 6.283x | 17 | 123 |
| short | reuse_rfind | 238 | 1.287x | 0.268x | 1.234x | 9.895x | 36 | 64 |
| short | rfind | 238 | 1.646x | 0.410x | 1.507x | 27.355x | 83 | 43 |
| tail | build_both | 32 | 1.176x | 0.481x | 1.108x | 2.363x | 4 | 12 |
| tail | byte_find | 32 | 0.525x | 0.246x | 0.500x | 1.171x | 0 | 31 |
| tail | byte_rfind | 32 | 0.495x | 0.248x | 0.471x | 0.931x | 0 | 32 |
| tail | find | 32 | 0.745x | 0.166x | 0.602x | 3.449x | 4 | 21 |
| tail | reuse_find | 32 | 0.559x | 0.134x | 0.347x | 2.242x | 3 | 21 |
| tail | reuse_rfind | 32 | 0.848x | 0.198x | 0.866x | 2.261x | 3 | 19 |
| tail | rfind | 32 | 2.231x | 0.795x | 1.830x | 8.124x | 15 | 2 |
| text | build_both | 48 | 0.957x | 0.179x | 0.942x | 1.644x | 0 | 37 |
| text | byte_find | 48 | 0.772x | 0.295x | 0.918x | 1.311x | 0 | 38 |
| text | byte_rfind | 48 | 0.707x | 0.293x | 0.862x | 1.346x | 0 | 40 |
| text | find | 48 | 0.907x | 0.170x | 1.250x | 6.834x | 15 | 21 |
| text | reuse_find | 48 | 0.535x | 0.127x | 0.611x | 3.190x | 2 | 28 |
| text | reuse_rfind | 48 | 1.131x | 0.454x | 1.054x | 2.876x | 6 | 21 |
| text | rfind | 48 | 2.083x | 0.590x | 2.586x | 6.244x | 27 | 13 |
| typical | build_both | 195 | 1.273x | 0.503x | 1.273x | 3.486x | 24 | 53 |
| typical | byte_find | 195 | 0.374x | 0.057x | 0.446x | 1.898x | 0 | 134 |
| typical | byte_rfind | 195 | 0.277x | 0.055x | 0.273x | 1.502x | 0 | 171 |
| typical | find | 195 | 0.516x | 0.079x | 0.499x | 4.937x | 33 | 130 |
| typical | reuse_find | 195 | 0.658x | 0.089x | 0.533x | 11.476x | 47 | 105 |
| typical | reuse_rfind | 195 | 1.396x | 0.173x | 1.398x | 13.564x | 80 | 64 |
| typical | rfind | 195 | 3.183x | 0.128x | 2.836x | 36.404x | 121 | 39 |

### Ten lowest ratios (all remaining cases are in cells.csv)

| Case | Operation | zscan ns/op | Best baseline ns/op | Baseline | Speedup |
| --- | --- | ---: | ---: | --- | ---: |
| typical-sparse-n4096-m256-a0 | byte_rfind | 519.366 | 28.666 | memchr | 0.055x |
| typical-none-n65536-m8-a0 | byte_find | 14168.667 | 805.953 | memchr | 0.057x |
| typical-last-n4096-m2-a0 | byte_find | 516.531 | 31.522 | memchr | 0.061x |
| typical-last-n65536-m31-a0 | byte_find | 10310.100 | 658.318 | memchr | 0.064x |
| typical-sparse-n65536-m2-a0 | byte_find | 1249.410 | 79.973 | memchr | 0.064x |
| typical-last-n4096-m32-a0 | byte_find | 444.580 | 31.450 | memchr | 0.071x |
| typical-none-n512-m32-a0 | byte_rfind | 82.577 | 5.851 | memchr | 0.071x |
| typical-none-n4096-m33-a0 | byte_rfind | 430.919 | 31.791 | memchr | 0.074x |
| typical-first-n4096-m33-a0 | byte_rfind | 435.134 | 32.106 | memchr | 0.074x |
| typical-first-n4096-m31-a0 | byte_rfind | 436.838 | 32.250 | memchr | 0.074x |

## Component observations

For ablations, the ratio is ablated zscan time / generic zscan time, so above 1 means the enabled component helped. These separate-process comparisons are sensitive to machine load. No interaction effects are removed.

| Ablation | Group | Operation | Cases | Geometric ratio |
| --- | --- | --- | ---: | ---: |
| scalar | adversarial | build_both | 24 | 1.002x |
| scalar | adversarial | byte_find | 24 | 0.912x |
| scalar | adversarial | byte_rfind | 24 | 0.983x |
| scalar | adversarial | find | 24 | 3.359x |
| scalar | adversarial | reuse_find | 24 | 3.311x |
| scalar | adversarial | reuse_rfind | 24 | 2.261x |
| scalar | adversarial | rfind | 24 | 2.238x |
| scalar | alignment | build_both | 64 | 0.762x |
| scalar | alignment | byte_find | 64 | 0.788x |
| scalar | alignment | byte_rfind | 64 | 0.864x |
| scalar | alignment | find | 64 | 0.848x |
| scalar | alignment | reuse_find | 64 | 0.913x |
| scalar | alignment | reuse_rfind | 64 | 0.972x |
| scalar | alignment | rfind | 64 | 0.916x |
| scalar | large | build_both | 6 | 0.560x |
| scalar | large | byte_find | 6 | 2.061x |
| scalar | large | byte_rfind | 6 | 1.362x |
| scalar | large | find | 6 | 5.075x |
| scalar | large | reuse_find | 6 | 5.390x |
| scalar | large | reuse_rfind | 6 | 2.002x |
| scalar | large | rfind | 6 | 2.096x |
| scalar | short | build_both | 238 | 0.728x |
| scalar | short | byte_find | 238 | 0.538x |
| scalar | short | byte_rfind | 238 | 0.548x |
| scalar | short | find | 238 | 0.604x |
| scalar | short | reuse_find | 238 | 0.592x |
| scalar | short | reuse_rfind | 238 | 0.610x |
| scalar | short | rfind | 238 | 0.621x |
| scalar | tail | build_both | 32 | 1.335x |
| scalar | tail | byte_find | 32 | 1.590x |
| scalar | tail | byte_rfind | 32 | 2.201x |
| scalar | tail | find | 32 | 1.744x |
| scalar | tail | reuse_find | 32 | 1.797x |
| scalar | tail | reuse_rfind | 32 | 1.924x |
| scalar | tail | rfind | 32 | 1.765x |
| scalar | text | build_both | 48 | 0.933x |
| scalar | text | byte_find | 48 | 0.939x |
| scalar | text | byte_rfind | 48 | 1.154x |
| scalar | text | find | 48 | 2.116x |
| scalar | text | reuse_find | 48 | 2.151x |
| scalar | text | reuse_rfind | 48 | 2.405x |
| scalar | text | rfind | 48 | 2.378x |
| scalar | typical | build_both | 195 | 0.597x |
| scalar | typical | byte_find | 195 | 1.208x |
| scalar | typical | byte_rfind | 195 | 1.719x |
| scalar | typical | find | 195 | 1.409x |
| scalar | typical | reuse_find | 195 | 1.428x |
| scalar | typical | reuse_rfind | 195 | 1.492x |
| scalar | typical | rfind | 195 | 1.470x |
| no-short | adversarial | build_both | 24 | 0.932x |
| no-short | adversarial | byte_find | 24 | 1.013x |
| no-short | adversarial | byte_rfind | 24 | 1.177x |
| no-short | adversarial | find | 24 | 1.472x |
| no-short | adversarial | reuse_find | 24 | 1.271x |
| no-short | adversarial | reuse_rfind | 24 | 1.160x |
| no-short | adversarial | rfind | 24 | 1.397x |
| no-short | alignment | build_both | 64 | 0.736x |
| no-short | alignment | byte_find | 64 | 0.774x |
| no-short | alignment | byte_rfind | 64 | 0.830x |
| no-short | alignment | find | 64 | 0.873x |
| no-short | alignment | reuse_find | 64 | 0.945x |
| no-short | alignment | reuse_rfind | 64 | 0.852x |
| no-short | alignment | rfind | 64 | 0.806x |
| no-short | large | build_both | 6 | 0.504x |
| no-short | large | byte_find | 6 | 0.475x |
| no-short | large | byte_rfind | 6 | 0.761x |
| no-short | large | find | 6 | 0.736x |
| no-short | large | reuse_find | 6 | 0.858x |
| no-short | large | reuse_rfind | 6 | 0.961x |
| no-short | large | rfind | 6 | 0.929x |
| no-short | short | build_both | 238 | 0.654x |
| no-short | short | byte_find | 238 | 0.589x |
| no-short | short | byte_rfind | 238 | 0.639x |
| no-short | short | find | 238 | 0.617x |
| no-short | short | reuse_find | 238 | 0.771x |
| no-short | short | reuse_rfind | 238 | 0.756x |
| no-short | short | rfind | 238 | 0.624x |
| no-short | tail | build_both | 32 | 0.759x |
| no-short | tail | byte_find | 32 | 0.709x |
| no-short | tail | byte_rfind | 32 | 0.817x |
| no-short | tail | find | 32 | 0.870x |
| no-short | tail | reuse_find | 32 | 0.928x |
| no-short | tail | reuse_rfind | 32 | 0.845x |
| no-short | tail | rfind | 32 | 0.806x |
| no-short | text | build_both | 48 | 0.911x |
| no-short | text | byte_find | 48 | 1.027x |
| no-short | text | byte_rfind | 48 | 1.065x |
| no-short | text | find | 48 | 1.391x |
| no-short | text | reuse_find | 48 | 1.393x |
| no-short | text | reuse_rfind | 48 | 1.111x |
| no-short | text | rfind | 48 | 1.061x |
| no-short | typical | build_both | 195 | 0.569x |
| no-short | typical | byte_find | 195 | 0.535x |
| no-short | typical | byte_rfind | 195 | 0.585x |
| no-short | typical | find | 195 | 0.689x |
| no-short | typical | reuse_find | 195 | 0.712x |
| no-short | typical | reuse_rfind | 195 | 0.669x |
| no-short | typical | rfind | 195 | 0.652x |
| no-prefilter | adversarial | build_both | 24 | 0.889x |
| no-prefilter | adversarial | byte_find | 24 | 0.942x |
| no-prefilter | adversarial | byte_rfind | 24 | 1.083x |
| no-prefilter | adversarial | find | 24 | 7.359x |
| no-prefilter | adversarial | reuse_find | 24 | 7.153x |
| no-prefilter | adversarial | reuse_rfind | 24 | 3.029x |
| no-prefilter | adversarial | rfind | 24 | 3.201x |
| no-prefilter | alignment | build_both | 64 | 0.725x |
| no-prefilter | alignment | byte_find | 64 | 0.694x |
| no-prefilter | alignment | byte_rfind | 64 | 0.752x |
| no-prefilter | alignment | find | 64 | 2.083x |
| no-prefilter | alignment | reuse_find | 64 | 1.721x |
| no-prefilter | alignment | reuse_rfind | 64 | 2.021x |
| no-prefilter | alignment | rfind | 64 | 2.329x |
| no-prefilter | large | build_both | 6 | 0.466x |
| no-prefilter | large | byte_find | 6 | 0.463x |
| no-prefilter | large | byte_rfind | 6 | 0.559x |
| no-prefilter | large | find | 6 | 26.544x |
| no-prefilter | large | reuse_find | 6 | 26.203x |
| no-prefilter | large | reuse_rfind | 6 | 4.227x |
| no-prefilter | large | rfind | 6 | 4.219x |
| no-prefilter | short | build_both | 238 | 0.683x |
| no-prefilter | short | byte_find | 238 | 0.648x |
| no-prefilter | short | byte_rfind | 238 | 0.623x |
| no-prefilter | short | find | 238 | 0.743x |
| no-prefilter | short | reuse_find | 238 | 0.722x |
| no-prefilter | short | reuse_rfind | 238 | 0.743x |
| no-prefilter | short | rfind | 238 | 0.731x |
| no-prefilter | tail | build_both | 32 | 0.700x |
| no-prefilter | tail | byte_find | 32 | 0.674x |
| no-prefilter | tail | byte_rfind | 32 | 0.783x |
| no-prefilter | tail | find | 32 | 2.079x |
| no-prefilter | tail | reuse_find | 32 | 1.920x |
| no-prefilter | tail | reuse_rfind | 32 | 2.341x |
| no-prefilter | tail | rfind | 32 | 2.442x |
| no-prefilter | text | build_both | 48 | 0.885x |
| no-prefilter | text | byte_find | 48 | 0.993x |
| no-prefilter | text | byte_rfind | 48 | 1.045x |
| no-prefilter | text | find | 48 | 4.542x |
| no-prefilter | text | reuse_find | 48 | 4.393x |
| no-prefilter | text | reuse_rfind | 48 | 6.774x |
| no-prefilter | text | rfind | 48 | 7.920x |
| no-prefilter | typical | build_both | 195 | 0.756x |
| no-prefilter | typical | byte_find | 195 | 0.668x |
| no-prefilter | typical | byte_rfind | 195 | 0.701x |
| no-prefilter | typical | find | 195 | 5.054x |
| no-prefilter | typical | reuse_find | 195 | 4.529x |
| no-prefilter | typical | reuse_rfind | 195 | 5.432x |
| no-prefilter | typical | rfind | 195 | 6.464x |

### Finder cache reuse

Ratio is one-shot / reuse time within the generic build. Construction is reported separately above as build_both and is not amortized into reuse. Ratios do not predict a break-even query count without including construction and needle length.

| Group | Engine | Direction | Cases | Geometric one-shot / reuse |
| --- | --- | --- | ---: | ---: |
| adversarial | memchr | find | 24 | 3.274x |
| adversarial | memchr | rfind | 24 | 1.729x |
| adversarial | zscan | find | 24 | 0.992x |
| adversarial | zscan | rfind | 24 | 0.992x |
| alignment | memchr | find | 64 | 3.177x |
| alignment | memchr | rfind | 64 | 1.722x |
| alignment | zscan | find | 64 | 1.110x |
| alignment | zscan | rfind | 64 | 1.101x |
| large | memchr | find | 6 | 1.038x |
| large | memchr | rfind | 6 | 1.793x |
| large | zscan | find | 6 | 1.013x |
| large | zscan | rfind | 6 | 0.987x |
| short | memchr | find | 238 | 2.095x |
| short | memchr | rfind | 238 | 1.356x |
| short | zscan | find | 238 | 1.061x |
| short | zscan | rfind | 238 | 1.063x |
| tail | memchr | find | 32 | 4.236x |
| tail | memchr | rfind | 32 | 2.459x |
| tail | zscan | find | 32 | 1.085x |
| tail | zscan | rfind | 32 | 1.079x |
| text | memchr | find | 48 | 3.124x |
| text | memchr | rfind | 48 | 2.040x |
| text | zscan | find | 48 | 1.019x |
| text | zscan | rfind | 48 | 1.085x |
| typical | memchr | find | 195 | 3.898x |
| typical | memchr | rfind | 195 | 2.410x |
| typical | zscan | find | 195 | 1.049x |
| typical | zscan | rfind | 195 | 1.066x |
