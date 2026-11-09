#!/usr/bin/env python3
"""Compare the retained round-2 runs without selecting a better repeat."""

from collections import defaultdict
import csv
import gzip
import hashlib
import json
import math
from pathlib import Path
import statistics


ROOT = Path(__file__).resolve().parent / "results"
RUNS = ["round2-before-calibrated", "round2-after-calibrated", "round2-after-repeat"]


def gm(values):
    return math.exp(statistics.mean(math.log(value) for value in values))


def read(run, config):
    directory = ROOT / run / config
    metadata = json.loads((directory / "metadata.json").read_text())
    assert metadata["complete"], directory
    assert metadata["compile_inputs_before"] == metadata["compile_inputs_after_build"] == metadata["compile_inputs_after_run"]
    assert metadata["cases_sha256"] == hashlib.sha256((directory / "cases.csv").read_bytes()).hexdigest()
    with (directory / "cells.csv").open() as source:
        rows = {(row["case"], row["operation"]): row for row in csv.DictReader(source)}
    batches = defaultdict(list)
    with gzip.open(directory / "samples.csv.gz", "rt") as source:
        for row in csv.DictReader(source):
            batches[(row["case"], row["operation"], row["engine"])].append(int(row["elapsed_ns"]))
    assert len(batches) == 607 * 18
    assert all(len(values) == 7 for values in batches.values())
    return metadata, rows, min(statistics.median(values) for values in batches.values())


def main():
    lines = [
        "# Round 2: bounded vector tails and byte scanning",
        "",
        "The bounded-tail changes improved the targeted medium-size cases, while the broad 2x "
        "short/typical performance target remains unmet. The headline results below use the "
        "primary corrected after run. A second unchanged run diagnoses severe localized "
        "regressions; it does not replace the primary run or supply a selectively better headline.",
        "",
        "All 607 predeclared cases, every slower result, and all seven measurement samples are "
        "retained. Each full configuration has 76,482 raw samples. memchr 2.8.3 is always a "
        "strict baseline; memx 0.2.2 contributes eligible byte/forward results. Its known "
        "incorrect reverse operation is retained but excluded from speed ratios.",
        "",
        "## Calibration correction and source control",
        "",
        "The original single-pilot calibration could size a fast operation at one iteration "
        "after a scheduler pause during warmup. For example, the diagnostic native "
        "text-http-first-n65536-m15 byte_rfind run used one iteration per measurement and "
        "reported 73–196 ns, dominated by timer overhead. The original round2-before directory "
        "and all round-1 data remain intact. They are historical/diagnostic observations; "
        "round-1 ablations were not rerun on the new code.",
        "",
        "Corrected calibration uses the minimum of three warmup pilots at each candidate "
        "batch size. This only sizes the batch: none of the seven actual measurement samples "
        "is filtered. The corrected before production source is an isolated checkout of "
        "fbe6bcb91989e45d4bf6805c6888863c57554cb5. Before, primary after, and repeat use the "
        "identical corrected harness and identical case fingerprints, compiler, profile, and "
        "per-configuration RUSTFLAGS. The only production source change is src/bytes.rs. "
        "Complete source snapshots and hashes accompany every run.",
        "",
        "| Build | Before minimum median batch ns | After minimum median batch ns | Repeat minimum median batch ns |",
        "| --- | ---: | ---: | ---: |",
    ]
    all_data = {}
    minimum_batch_ns = float("inf")
    for config in ["generic", "native"]:
        loaded = [read(run, config) for run in RUNS]
        metas = [item[0] for item in loaded]
        data = [item[1] for item in loaded]
        assert data[0].keys() == data[1].keys() == data[2].keys()
        for field in ["cases_sha256", "compile_profile", "rustflags", "baseline_commits"]:
            assert metas[0][field] == metas[1][field] == metas[2][field], field
        assert len({m["rustc"]["stdout"] for m in metas}) == 1
        assert len({m["compile_inputs_before"]["comparison/src/main.rs"] for m in metas}) == 1
        assert metas[0]["compile_inputs_before"].keys() == metas[1]["compile_inputs_before"].keys()
        changed = {
            name for name in metas[0]["compile_inputs_before"]
            if metas[0]["compile_inputs_before"][name] != metas[1]["compile_inputs_before"][name]
        }
        assert changed == {"src/bytes.rs"}, changed
        assert metas[1]["compile_inputs_before"] == metas[2]["compile_inputs_before"]
        lines.append(f"| {config} | {loaded[0][2]} | {loaded[1][2]} | {loaded[2][2]} |")
        minimum_batch_ns = min(minimum_batch_ns, *(item[2] for item in loaded))
        all_data[config] = data
    lines += [
        "",
        f"Across the corrected runs, the smallest engine/case median batch was {minimum_batch_ns / 1000:.3f} "
        "microseconds. The three-pilot correction prevents one unusually slow pilot from "
        "determining the batch size; it does not guarantee a minimum batch duration or eliminate "
        "shared-machine timing instability. These data are observations, not a fixed-hardware release gate.",
        "",
        "## Complete group comparison",
        "",
        "Best-baseline speedup is fastest eligible competitor median / zscan median, "
        "geometrically averaged across every case in the group. Above 1 favors zscan. "
        "The own-improvement column is before zscan / primary-after zscan. The unchanged "
        "memchr control uses before memchr / primary-after memchr. Control-adjusted improvement "
        "divides those two ratios; it is a diagnostic comparison, not proof of a causal effect.",
        "",
    ]
    for config, (before, after, repeat) in all_data.items():
        lines += [
            f"### {config}",
            "",
            "| Group | Operation | Cases | Before / best | Primary after / best | Own improvement | memchr control | Control-adjusted | Repeat / best |",
            "| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |",
        ]
        groups = sorted({(row["group"], row["operation"]) for row in before.values()})
        for group, operation in groups:
            keys = [key for key, row in before.items() if (row["group"], row["operation"]) == (group, operation)]
            own = gm([float(before[k]["zscan_ns"]) / float(after[k]["zscan_ns"]) for k in keys])
            control = gm([float(before[k]["memchr_ns"]) / float(after[k]["memchr_ns"]) for k in keys])
            ratios = [gm([float(rows[k]["speedup"]) for k in keys]) for rows in [before, after, repeat]]
            lines.append(
                f"| {group} | {operation} | {len(keys)} | {ratios[0]:.3f}x | {ratios[1]:.3f}x | "
                f"{own:.3f}x | {control:.3f}x | {own / control:.3f}x | {ratios[2]:.3f}x |"
            )
        lines.append("")
    lines += [
        "## Stable-control examples and localized regressions",
        "",
        "All entries are median ns/op from seven retained samples. The columns show both "
        "libraries in all three runs; none is substituted for another.",
        "",
        "| Build | Case / operation | zscan before | zscan primary after | zscan repeat | memchr before | memchr primary after | memchr repeat |",
        "| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |",
    ]
    selected = [
        ("short-last-n63-m3-a2", "byte_find"),
        ("tail-none-n87-m10-a7", "byte_find"),
        ("tail-last-n95-m3-a7", "byte_find"),
        ("tail-none-n84-m7-a7", "reuse_find"),
        ("text-json-last-n4096-m14", "reuse_find"),
        ("typical-sparse-n128-m16-a0", "reuse_rfind"),
        ("text-log-first-n65536-m11", "rfind"),
        ("typical-sparse-n512-m2-a0", "rfind"),
    ]
    for config, data in all_data.items():
        for key in selected:
            values = [float(rows[key][name]) for name in ["zscan_ns", "memchr_ns"] for rows in data]
            lines.append(f"| {config} | {key[0]} / {key[1]} | " + " | ".join(f"{value:.4f}" for value in values) + " |")
    lines += [
        "",
        "The generic 87-byte absent-byte case improved from 8.3709 to 3.0673 ns, then measured "
        "3.0677 ns in the repeat; memchr stayed at approximately 3.07 ns. The 95-byte last-byte "
        "case improved from 9.7516 to 3.6263 ns, then 3.6232 ns, while memchr stayed at 3.35 ns. "
        "These are scoped, repeated improvements relative to the preceding zscan implementation.",
        "",
        "The severe generic JSON reuse-find regression (102.1734 → 829.5483 ns) did not "
        "reproduce: the unchanged repeat measured 91.4453 ns, with memchr approximately "
        "113–115 ns throughout. The severe native sparse reverse regression "
        "(8.4394 → 173.3283 ns) also did not reproduce: the repeat measured 9.1940 ns. "
        "This establishes instability, not its cause. Other localized variations remain, "
        "including the native JSON/log repeat rows above. A fixed idle machine is needed "
        "before treating these timings as a performance guarantee.",
        "",
        "A source walkthrough reconstructed the exact sparse-case fingerprint "
        "27412016fbc842be: the match at 18 is reached by three complete 32-candidate blocks "
        "(81–112, 49–80, 17–48), without the new overlap tail, scalar path, or fallback. "
        "For JSON fingerprint 6a9ba0dc1e838c74, candidate 10 mismatches at index 1, the "
        "probe changes, and the only remaining hit is 4082 after 127 full vector blocks. "
        "The old eight-candidate scalar tail becomes a bounded overlapping vector tail; "
        "the candidate sequence and fallback behavior are unchanged. These traces narrow "
        "the algorithmic explanation but do not diagnose microarchitectural or scheduling effects.",
        "",
        "## Evidence and reproduction",
        "",
        "The round2-before directory is the original diagnostic run with the old calibration. "
        "round2-before-calibrated, round2-after-calibrated, and round2-after-repeat contain "
        "all corrected records. Each configuration includes samples.csv.gz, cases.csv, "
        "cells.csv, metadata.json, Cargo.lock, source.tar.gz, and logs. The earlier "
        "initial-optimized/final directories and round-1 ablations remain historical evidence.",
        "",
        "Regenerate the per-run summaries with comparison/summarize.py, then run "
        "`python3 comparison/compare_runs.py` to regenerate this review. The script checks "
        "source stability, matching harness/compiler/cases/flags, the expected production "
        "source delta, and complete sample counts before computing tables.",
    ]
    target = ROOT / "round2-review.md"
    target.write_text("\n".join(lines) + "\n")
    print(target)


if __name__ == "__main__":
    main()
