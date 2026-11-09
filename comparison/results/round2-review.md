# Round 2: bounded vector tails and byte scanning

The bounded-tail changes improved the targeted medium-size cases, while the broad 2x short/typical performance target remains unmet. The headline results below use the primary corrected after run. A second unchanged run diagnoses severe localized regressions; it does not replace the primary run or supply a selectively better headline.

All 607 predeclared cases, every slower result, and all seven measurement samples are retained. Each full configuration has 76,482 raw samples. memchr 2.8.3 is always a strict baseline; memx 0.2.2 contributes eligible byte/forward results. Its known incorrect reverse operation is retained but excluded from speed ratios.

## Calibration correction and source control

The original single-pilot calibration could size a fast operation at one iteration after a scheduler pause during warmup. For example, the diagnostic native text-http-first-n65536-m15 byte_rfind run used one iteration per measurement and reported 73–196 ns, dominated by timer overhead. The original round2-before directory and all round-1 data remain intact. They are historical/diagnostic observations; round-1 ablations were not rerun on the new code.

Corrected calibration uses the minimum of three warmup pilots at each candidate batch size. This only sizes the batch: none of the seven actual measurement samples is filtered. The corrected before production source is an isolated checkout of fbe6bcb91989e45d4bf6805c6888863c57554cb5. Before, primary after, and repeat use the identical corrected harness and identical case fingerprints, compiler, profile, and per-configuration RUSTFLAGS. The only production source change is src/bytes.rs. Complete source snapshots and hashes accompany every run.

| Build | Before minimum median batch ns | After minimum median batch ns | Repeat minimum median batch ns |
| --- | ---: | ---: | ---: |
| generic | 13514 | 9267 | 14245 |
| native | 6785 | 7985 | 4275 |

Across the corrected runs, the smallest engine/case median batch was 4.275 microseconds. The three-pilot correction prevents one unusually slow pilot from determining the batch size; it does not guarantee a minimum batch duration or eliminate shared-machine timing instability. These data are observations, not a fixed-hardware release gate.

## Complete group comparison

Best-baseline speedup is fastest eligible competitor median / zscan median, geometrically averaged across every case in the group. Above 1 favors zscan. The own-improvement column is before zscan / primary-after zscan. The unchanged memchr control uses before memchr / primary-after memchr. Control-adjusted improvement divides those two ratios; it is a diagnostic comparison, not proof of a causal effect.

### generic

| Group | Operation | Cases | Before / best | Primary after / best | Own improvement | memchr control | Control-adjusted | Repeat / best |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| adversarial | build_both | 24 | 1.197x | 1.360x | 1.231x | 1.083x | 1.136x | 1.375x |
| adversarial | byte_find | 24 | 0.734x | 0.766x | 1.079x | 1.009x | 1.070x | 0.740x |
| adversarial | byte_rfind | 24 | 0.801x | 0.764x | 0.999x | 0.853x | 1.170x | 0.739x |
| adversarial | find | 24 | 1.575x | 1.577x | 1.025x | 1.089x | 0.942x | 1.667x |
| adversarial | reuse_find | 24 | 2.462x | 2.246x | 1.015x | 1.112x | 0.912x | 2.474x |
| adversarial | reuse_rfind | 24 | 5.314x | 5.123x | 1.002x | 1.039x | 0.964x | 5.338x |
| adversarial | rfind | 24 | 8.275x | 8.695x | 1.116x | 1.062x | 1.051x | 8.142x |
| alignment | build_both | 64 | 1.279x | 1.265x | 0.967x | 0.978x | 0.989x | 1.302x |
| alignment | byte_find | 64 | 0.548x | 0.827x | 1.485x | 0.946x | 1.570x | 0.818x |
| alignment | byte_rfind | 64 | 0.782x | 1.046x | 1.318x | 0.930x | 1.418x | 1.025x |
| alignment | find | 64 | 1.093x | 1.292x | 1.171x | 0.959x | 1.221x | 1.288x |
| alignment | reuse_find | 64 | 0.726x | 0.875x | 1.228x | 1.019x | 1.205x | 0.933x |
| alignment | reuse_rfind | 64 | 1.262x | 1.416x | 1.142x | 1.018x | 1.122x | 1.396x |
| alignment | rfind | 64 | 2.370x | 2.588x | 1.103x | 1.010x | 1.092x | 2.633x |
| large | build_both | 6 | 1.517x | 1.128x | 0.792x | 1.065x | 0.744x | 1.624x |
| large | byte_find | 6 | 0.651x | 0.806x | 1.466x | 1.185x | 1.237x | 0.856x |
| large | byte_rfind | 6 | 0.652x | 0.828x | 1.307x | 0.897x | 1.456x | 0.658x |
| large | find | 6 | 0.923x | 0.821x | 0.934x | 1.174x | 0.796x | 0.841x |
| large | reuse_find | 6 | 1.257x | 0.989x | 0.857x | 1.090x | 0.787x | 1.098x |
| large | reuse_rfind | 6 | 4.761x | 3.975x | 0.932x | 1.116x | 0.835x | 5.194x |
| large | rfind | 6 | 11.103x | 9.270x | 1.079x | 1.293x | 0.835x | 9.473x |
| short | build_both | 238 | 1.783x | 1.829x | 0.981x | 0.957x | 1.025x | 2.011x |
| short | byte_find | 238 | 0.713x | 0.814x | 1.131x | 0.985x | 1.148x | 0.795x |
| short | byte_rfind | 238 | 0.909x | 0.993x | 1.085x | 0.973x | 1.115x | 0.965x |
| short | find | 238 | 1.069x | 1.205x | 1.125x | 0.979x | 1.149x | 1.191x |
| short | reuse_find | 238 | 0.830x | 0.955x | 1.166x | 1.014x | 1.150x | 0.937x |
| short | reuse_rfind | 238 | 1.165x | 1.149x | 1.031x | 1.045x | 0.987x | 1.135x |
| short | rfind | 238 | 1.522x | 1.677x | 1.062x | 0.964x | 1.102x | 1.657x |
| tail | build_both | 32 | 1.421x | 1.459x | 0.944x | 0.920x | 1.027x | 1.451x |
| tail | byte_find | 32 | 0.559x | 0.943x | 1.502x | 0.896x | 1.677x | 0.837x |
| tail | byte_rfind | 32 | 0.765x | 1.049x | 1.313x | 0.867x | 1.515x | 1.081x |
| tail | find | 32 | 0.930x | 1.314x | 1.340x | 0.996x | 1.346x | 1.254x |
| tail | reuse_find | 32 | 0.664x | 0.896x | 1.362x | 1.010x | 1.348x | 0.976x |
| tail | reuse_rfind | 32 | 1.167x | 1.497x | 1.277x | 0.995x | 1.283x | 1.581x |
| tail | rfind | 32 | 3.217x | 3.841x | 1.214x | 1.016x | 1.194x | 3.935x |
| text | build_both | 48 | 1.080x | 0.943x | 0.979x | 1.121x | 0.873x | 1.012x |
| text | byte_find | 48 | 0.682x | 0.761x | 1.155x | 1.026x | 1.126x | 0.786x |
| text | byte_rfind | 48 | 0.840x | 0.803x | 0.968x | 0.827x | 1.170x | 0.792x |
| text | find | 48 | 1.698x | 1.922x | 1.130x | 0.941x | 1.201x | 1.878x |
| text | reuse_find | 48 | 1.067x | 1.131x | 1.049x | 0.988x | 1.061x | 1.146x |
| text | reuse_rfind | 48 | 2.599x | 2.446x | 1.045x | 1.110x | 0.941x | 2.617x |
| text | rfind | 48 | 5.460x | 5.265x | 1.043x | 1.082x | 0.964x | 5.178x |
| typical | build_both | 195 | 1.347x | 1.326x | 1.004x | 1.020x | 0.984x | 1.353x |
| typical | byte_find | 195 | 0.857x | 0.981x | 1.131x | 0.989x | 1.144x | 0.946x |
| typical | byte_rfind | 195 | 0.960x | 0.909x | 1.014x | 0.952x | 1.065x | 0.890x |
| typical | find | 195 | 1.290x | 1.338x | 1.059x | 1.075x | 0.985x | 1.353x |
| typical | reuse_find | 195 | 1.692x | 1.719x | 1.083x | 1.066x | 1.016x | 1.777x |
| typical | reuse_rfind | 195 | 3.347x | 3.088x | 0.943x | 1.023x | 0.922x | 3.129x |
| typical | rfind | 195 | 8.650x | 7.665x | 0.961x | 1.085x | 0.886x | 7.614x |

### native

| Group | Operation | Cases | Before / best | Primary after / best | Own improvement | memchr control | Control-adjusted | Repeat / best |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| adversarial | build_both | 24 | 1.085x | 1.272x | 1.192x | 1.016x | 1.173x | 1.248x |
| adversarial | byte_find | 24 | 1.462x | 1.613x | 1.045x | 0.963x | 1.085x | 1.660x |
| adversarial | byte_rfind | 24 | 1.202x | 1.418x | 1.246x | 1.024x | 1.217x | 1.401x |
| adversarial | find | 24 | 1.218x | 1.469x | 1.323x | 1.036x | 1.276x | 1.376x |
| adversarial | reuse_find | 24 | 2.036x | 2.282x | 1.181x | 1.054x | 1.121x | 2.588x |
| adversarial | reuse_rfind | 24 | 6.118x | 5.751x | 1.054x | 1.121x | 0.940x | 5.901x |
| adversarial | rfind | 24 | 10.402x | 9.515x | 1.041x | 1.139x | 0.915x | 9.353x |
| alignment | build_both | 64 | 1.230x | 1.225x | 1.104x | 1.108x | 0.996x | 1.157x |
| alignment | byte_find | 64 | 1.082x | 1.365x | 1.380x | 1.032x | 1.338x | 1.429x |
| alignment | byte_rfind | 64 | 1.057x | 1.393x | 1.420x | 0.978x | 1.451x | 1.507x |
| alignment | find | 64 | 1.147x | 1.395x | 1.236x | 1.070x | 1.155x | 1.493x |
| alignment | reuse_find | 64 | 0.746x | 0.895x | 1.330x | 1.107x | 1.201x | 0.858x |
| alignment | reuse_rfind | 64 | 1.429x | 1.717x | 1.397x | 1.162x | 1.202x | 1.599x |
| alignment | rfind | 64 | 2.687x | 3.133x | 1.206x | 1.034x | 1.166x | 3.096x |
| large | build_both | 6 | 1.382x | 1.117x | 0.979x | 1.212x | 0.808x | 1.183x |
| large | byte_find | 6 | 0.583x | 0.774x | 1.358x | 1.023x | 1.328x | 0.809x |
| large | byte_rfind | 6 | 0.902x | 1.022x | 1.080x | 0.887x | 1.217x | 1.049x |
| large | find | 6 | 0.965x | 0.871x | 0.993x | 1.047x | 0.948x | 0.996x |
| large | reuse_find | 6 | 1.170x | 1.343x | 1.104x | 0.962x | 1.148x | 1.168x |
| large | reuse_rfind | 6 | 5.606x | 6.122x | 1.045x | 0.957x | 1.092x | 3.931x |
| large | rfind | 6 | 10.908x | 10.255x | 1.043x | 1.110x | 0.940x | 10.523x |
| short | build_both | 238 | 1.614x | 1.515x | 0.955x | 1.017x | 0.939x | 1.556x |
| short | byte_find | 238 | 1.244x | 1.187x | 0.982x | 1.035x | 0.948x | 1.171x |
| short | byte_rfind | 238 | 1.358x | 1.263x | 0.873x | 0.872x | 1.001x | 1.275x |
| short | find | 238 | 1.216x | 1.244x | 0.984x | 0.970x | 1.015x | 1.286x |
| short | reuse_find | 238 | 0.932x | 0.934x | 1.070x | 1.067x | 1.003x | 0.942x |
| short | reuse_rfind | 238 | 1.266x | 1.239x | 1.002x | 1.024x | 0.979x | 1.280x |
| short | rfind | 238 | 1.765x | 1.778x | 0.954x | 0.947x | 1.007x | 1.766x |
| tail | build_both | 32 | 1.233x | 1.206x | 1.007x | 1.030x | 0.978x | 1.385x |
| tail | byte_find | 32 | 0.924x | 1.367x | 1.509x | 0.868x | 1.739x | 1.408x |
| tail | byte_rfind | 32 | 1.180x | 1.532x | 1.394x | 1.107x | 1.259x | 1.460x |
| tail | find | 32 | 1.123x | 1.472x | 1.395x | 1.106x | 1.261x | 1.542x |
| tail | reuse_find | 32 | 0.644x | 0.956x | 1.555x | 1.048x | 1.483x | 0.896x |
| tail | reuse_rfind | 32 | 1.212x | 1.842x | 1.698x | 1.117x | 1.520x | 1.616x |
| tail | rfind | 32 | 3.240x | 4.317x | 1.508x | 1.132x | 1.332x | 4.554x |
| text | build_both | 48 | 1.012x | 1.087x | 1.086x | 1.011x | 1.074x | 1.097x |
| text | byte_find | 48 | 1.540x | 1.649x | 1.137x | 0.923x | 1.232x | 1.485x |
| text | byte_rfind | 48 | 1.296x | 1.558x | 1.187x | 0.962x | 1.234x | 1.502x |
| text | find | 48 | 1.749x | 1.892x | 1.116x | 1.101x | 1.014x | 1.782x |
| text | reuse_find | 48 | 1.077x | 1.010x | 1.198x | 1.278x | 0.937x | 1.162x |
| text | reuse_rfind | 48 | 3.013x | 3.115x | 1.181x | 1.143x | 1.034x | 3.206x |
| text | rfind | 48 | 5.440x | 6.232x | 1.246x | 1.088x | 1.146x | 6.204x |
| typical | build_both | 195 | 1.262x | 1.285x | 1.022x | 1.003x | 1.019x | 1.283x |
| typical | byte_find | 195 | 1.327x | 1.444x | 1.045x | 0.960x | 1.089x | 1.572x |
| typical | byte_rfind | 195 | 1.249x | 1.284x | 0.971x | 0.939x | 1.034x | 1.301x |
| typical | find | 195 | 1.294x | 1.339x | 1.083x | 0.965x | 1.122x | 1.352x |
| typical | reuse_find | 195 | 1.654x | 1.755x | 1.098x | 1.035x | 1.061x | 1.712x |
| typical | reuse_rfind | 195 | 3.815x | 3.618x | 1.000x | 1.054x | 0.948x | 3.629x |
| typical | rfind | 195 | 9.196x | 8.499x | 0.963x | 1.042x | 0.924x | 8.433x |

## Stable-control examples and localized regressions

All entries are median ns/op from seven retained samples. The columns show both libraries in all three runs; none is substituted for another.

| Build | Case / operation | zscan before | zscan primary after | zscan repeat | memchr before | memchr primary after | memchr repeat |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| generic | short-last-n63-m3-a2 / byte_find | 10.2916 | 2.8578 | 2.7857 | 2.7873 | 3.2366 | 2.7889 |
| generic | tail-none-n87-m10-a7 / byte_find | 8.3709 | 3.0673 | 3.0677 | 3.0702 | 3.0672 | 3.0657 |
| generic | tail-last-n95-m3-a7 / byte_find | 9.7516 | 3.6263 | 3.6232 | 3.3489 | 3.3491 | 3.3445 |
| generic | tail-none-n84-m7-a7 / reuse_find | 16.2119 | 7.8384 | 7.8082 | 6.4124 | 6.4346 | 6.4112 |
| generic | text-json-last-n4096-m14 / reuse_find | 102.1734 | 829.5483 | 91.4453 | 113.0418 | 114.7571 | 113.2743 |
| generic | typical-sparse-n128-m16-a0 / reuse_rfind | 10.8867 | 9.1806 | 13.8364 | 35.7868 | 31.2289 | 31.2116 |
| generic | text-log-first-n65536-m11 / rfind | 1952.3725 | 7514.5306 | 1953.5098 | 14268.6364 | 9058.1000 | 12587.9091 |
| generic | typical-sparse-n512-m2-a0 / rfind | 14.5145 | 13.7268 | 9.9105 | 27.3651 | 26.5631 | 26.7588 |
| native | short-last-n63-m3-a2 / byte_find | 7.4269 | 2.0040 | 1.7279 | 3.0711 | 3.0693 | 3.0693 |
| native | tail-none-n87-m10-a7 / byte_find | 6.1308 | 2.5096 | 5.1339 | 4.0140 | 7.4683 | 6.0914 |
| native | tail-last-n95-m3-a7 / byte_find | 6.9732 | 3.9328 | 2.7907 | 5.0091 | 6.1301 | 3.6332 |
| native | tail-none-n84-m7-a7 / reuse_find | 15.7366 | 6.9899 | 7.2422 | 32.6411 | 5.5768 | 5.5728 |
| native | text-json-last-n4096-m14 / reuse_find | 201.2641 | 84.3568 | 456.2100 | 89.7020 | 88.5080 | 264.2781 |
| native | typical-sparse-n128-m16-a0 / reuse_rfind | 8.4394 | 173.3283 | 9.1940 | 40.1867 | 38.0413 | 32.1153 |
| native | text-log-first-n65536-m11 / rfind | 1538.5500 | 1489.6167 | 5971.5088 | 18329.6000 | 9866.1818 | 18033.1667 |
| native | typical-sparse-n512-m2-a0 / rfind | 8.3643 | 68.3552 | 9.3501 | 35.1287 | 27.4727 | 29.9435 |

The generic 87-byte absent-byte case improved from 8.3709 to 3.0673 ns, then measured 3.0677 ns in the repeat; memchr stayed at approximately 3.07 ns. The 95-byte last-byte case improved from 9.7516 to 3.6263 ns, then 3.6232 ns, while memchr stayed at 3.35 ns. These are scoped, repeated improvements relative to the preceding zscan implementation.

The severe generic JSON reuse-find regression (102.1734 → 829.5483 ns) did not reproduce: the unchanged repeat measured 91.4453 ns, with memchr approximately 113–115 ns throughout. The severe native sparse reverse regression (8.4394 → 173.3283 ns) also did not reproduce: the repeat measured 9.1940 ns. This establishes instability, not its cause. Other localized variations remain, including the native JSON/log repeat rows above. A fixed idle machine is needed before treating these timings as a performance guarantee.

A source walkthrough reconstructed the exact sparse-case fingerprint 27412016fbc842be: the match at 18 is reached by three complete 32-candidate blocks (81–112, 49–80, 17–48), without the new overlap tail, scalar path, or fallback. For JSON fingerprint 6a9ba0dc1e838c74, candidate 10 mismatches at index 1, the probe changes, and the only remaining hit is 4082 after 127 full vector blocks. The old eight-candidate scalar tail becomes a bounded overlapping vector tail; the candidate sequence and fallback behavior are unchanged. These traces narrow the algorithmic explanation but do not diagnose microarchitectural or scheduling effects.

## Evidence and reproduction

The round2-before directory is the original diagnostic run with the old calibration. round2-before-calibrated, round2-after-calibrated, and round2-after-repeat contain all corrected records. Each configuration includes samples.csv.gz, cases.csv, cells.csv, metadata.json, Cargo.lock, source.tar.gz, and logs. The earlier initial-optimized/final directories and round-1 ablations remain historical evidence.

Regenerate the per-run summaries with comparison/summarize.py, then run `python3 comparison/compare_runs.py` to regenerate this review. The script checks source stability, matching harness/compiler/cases/flags, the expected production source delta, and complete sample counts before computing tables.
