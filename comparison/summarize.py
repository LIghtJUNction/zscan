#!/usr/bin/env python3
"""Summarize all recorded cells; failed comparator answers never become speed wins."""

import argparse
from collections import defaultdict
import csv
import gzip
import hashlib
import json
import math
import os
from pathlib import Path
import statistics


def geometric_mean(values):
    return math.exp(statistics.mean(math.log(value) for value in values))


def incompatible_ablation(base_metadata, other_metadata, variant):
    problems = []
    if "compile_inputs_before" not in base_metadata or "compile_inputs_before" not in other_metadata:
        problems.append("missing before/after compile-input evidence")
    else:
        for state in ["compile_inputs_before", "compile_inputs_after_build", "compile_inputs_after_run"]:
            if base_metadata.get(state) != other_metadata.get(state):
                problems.append(f"different {state}")
    for field in ["cases_sha256", "compile_profile", "mode"]:
        if base_metadata.get(field) != other_metadata.get(field):
            problems.append(f"different {field}")
    if base_metadata["rustc"]["stdout"] != other_metadata["rustc"]["stdout"]:
        problems.append("different compiler")
    expected_delta = {
        "scalar": "--cfg zscan_scalar",
        "no-short": "--cfg zscan_no_short",
        "no-prefilter": "--cfg zscan_no_prefilter",
    }[variant]
    expected = (base_metadata["rustflags"] + " " + expected_delta).strip()
    if other_metadata["rustflags"] != expected:
        problems.append("RUSTFLAGS differ beyond intended ablation")
    return problems


def load(directory):
    metadata = json.loads((directory / "metadata.json").read_text())
    if not metadata["complete"]:
        raise ValueError(f"incomplete run: {directory}")
    if "cases_sha256" in metadata and metadata["cases_sha256"] != hashlib.sha256((directory / "cases.csv").read_bytes()).hexdigest():
        raise ValueError(f"case manifest changed after capture: {directory}")
    cells = defaultdict(list)
    detail = {}
    mismatches = 0
    ineligible = 0
    with gzip.open(directory / "samples.csv.gz", "rt", newline="") as source:
        for row in csv.DictReader(source):
            key = (row["case"], row["operation"], row["engine"])
            detail[key] = row
            cells[key].append(float(row["ns_per_op"]))
            mismatches += row["correct"] != "true"
            ineligible += row["eligible"] != "true"
    expected_samples = 1 if metadata["mode"] == "smoke" else 7
    for key, values in cells.items():
        if len(values) != expected_samples:
            raise ValueError(f"missing or duplicate samples at {key}: {len(values)}")
        if min(values) <= 0:
            raise ValueError(f"timer resolution invalid at {key}")
    medians = {key: statistics.median(values) for key, values in cells.items()}
    rows = []
    for (case, operation, engine), zscan_ns in medians.items():
        if engine != "zscan":
            continue
        row = detail[(case, operation, engine)]
        peers = {
            peer: medians[(case, operation, peer)]
            for peer in ["memchr", "memx"]
            if (case, operation, peer) in medians
            and detail[(case, operation, peer)]["correct"] == "true"
            and detail[(case, operation, peer)]["eligible"] == "true"
        }
        if not peers:
            raise ValueError(f"no valid competitor: {case}/{operation}")
        best = min(peers, key=peers.get)
        rows.append({
            "case": case,
            "group": row["group"],
            "operation": operation,
            "haystack_len": row["haystack_len"],
            "needle_len": row["needle_len"],
            "zscan_ns": zscan_ns,
            "memchr_ns": peers.get("memchr", ""),
            "memx_ns": peers.get("memx", ""),
            "best_baseline": best,
            "best_baseline_ns": peers[best],
            "speedup": peers[best] / zscan_ns,
        })
    with (directory / "cells.csv").open("w", newline="") as target:
        writer = csv.DictWriter(target, fieldnames=rows[0])
        writer.writeheader()
        writer.writerows(rows)
    return metadata, rows, medians, mismatches, ineligible


def summary_table(rows):
    lines = [
        "| Group | Operation | Cases | Geometric speedup | Minimum | Median | Maximum | At least 2x | Slower than best |",
        "| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |",
    ]
    grouped = defaultdict(list)
    for row in rows:
        grouped[(row["group"], row["operation"])].append(row["speedup"])
    for (group, operation), values in sorted(grouped.items()):
        lines.append(
            f"| {group} | {operation} | {len(values)} | {geometric_mean(values):.3f}x | "
            f"{min(values):.3f}x | {statistics.median(values):.3f}x | {max(values):.3f}x | "
            f"{sum(v >= 2 for v in values)} | {sum(v < 1 for v in values)} |"
        )
    return lines


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("run_root", type=Path)
    args = parser.parse_args()
    baseline_link = os.path.relpath(Path(__file__).resolve().parent / "baseline-notes.md", args.run_root.resolve())
    reports = {}
    metadatas = {}
    lines = [
        "# Recorded search comparison",
        "",
        "These are observations on the machine recorded in each metadata.json. Shared/cloud hardware "
        "is not a reliable release performance gate. Ratios are the fastest valid competitor's median "
        "ns/op divided by zscan's median ns/op; above 1 means zscan is faster. Every sample is retained, "
        "including outliers, slower cases, and the diagnostic timings of an incorrect comparator.",
        "",
        "memx 0.2.2's reverse substring implementation omits a valid final candidate on some inputs. "
        "All memx/rfind cells are ineligible for speed claims; their actual correctness and timing remain "
        f"in samples.csv.gz. memchr is the strict reverse baseline. See [baseline-notes.md]({baseline_link}).",
        "",
    ]
    for directory in sorted(args.run_root.iterdir()):
        if not (directory / "metadata.json").is_file():
            continue
        metadata, rows, medians, mismatches, ineligible = load(directory)
        reports[directory.name] = (rows, medians)
        metadatas[directory.name] = metadata
        lines += [
            f"## {directory.name}",
            "",
            f"Mode: `{metadata['mode']}`. RUSTFLAGS: `{metadata['rustflags'] or '(empty)'}`. "
            f"Retained {len(medians)} engine/case cells; {mismatches} raw samples returned a wrong "
            f"comparator answer, and {ineligible} samples were excluded from ratios because the "
            "comparator operation is not semantics-equivalent. No zscan or memchr correctness "
            "failure is permitted in a completed run.",
            "",
            *summary_table(rows),
            "",
            "### Ten lowest ratios (all remaining cases are in cells.csv)",
            "",
            "| Case | Operation | zscan ns/op | Best baseline ns/op | Baseline | Speedup |",
            "| --- | --- | ---: | ---: | --- | ---: |",
        ]
        for row in sorted(rows, key=lambda r: r["speedup"])[:10]:
            lines.append(
                f"| {row['case']} | {row['operation']} | {row['zscan_ns']:.3f} | "
                f"{row['best_baseline_ns']:.3f} | {row['best_baseline']} | {row['speedup']:.3f}x |"
            )
        lines.append("")
    if "generic" in reports and any(key[1] == "reuse_find" for key in reports["generic"][1]):
        _, base = reports["generic"]
        lines += [
            "## Component observations",
            "",
            "For ablations, the ratio is ablated zscan time / generic zscan time, so above 1 "
            "means the enabled component helped. These separate-process comparisons are sensitive "
            "to machine load. No interaction effects are removed.",
            "",
            "| Ablation | Group | Operation | Cases | Geometric ratio |",
            "| --- | --- | --- | ---: | ---: |",
        ]
        for variant in ["scalar", "no-short", "no-prefilter"]:
            if variant not in reports:
                continue
            incompatible = incompatible_ablation(metadatas["generic"], metadatas[variant], variant)
            if incompatible:
                lines.append(f"| {variant} | Not computed | {'; '.join(incompatible)} | — | — |")
                continue
            _, ablated = reports[variant]
            groups = defaultdict(list)
            for key, value in base.items():
                case, operation, engine = key
                if engine == "zscan":
                    if key not in ablated:
                        raise ValueError(f"missing ablation case {variant}/{key}")
                    groups[(case.split("-", 1)[0], operation)].append(ablated[key] / value)
            for (group, operation), values in sorted(groups.items()):
                lines.append(f"| {variant} | {group} | {operation} | {len(values)} | {geometric_mean(values):.3f}x |")
        lines += [
            "",
            "### Finder cache reuse",
            "",
            "Ratio is one-shot / reuse time within the generic build. Construction is reported "
            "separately above as build_both and is not amortized into reuse. Ratios do not predict "
            "a break-even query count without including construction and needle length.",
            "",
            "| Group | Engine | Direction | Cases | Geometric one-shot / reuse |",
            "| --- | --- | --- | ---: | ---: |",
        ]
        groups = defaultdict(list)
        for (case, operation, engine), value in base.items():
            if operation in ["find", "rfind"] and engine in ["zscan", "memchr"]:
                reused = base[(case, "reuse_" + operation, engine)]
                groups[(case.split("-", 1)[0], engine, operation)].append(value / reused)
        for (group, engine, direction), values in sorted(groups.items()):
            lines.append(f"| {group} | {engine} | {direction} | {len(values)} | {geometric_mean(values):.3f}x |")
    (args.run_root / "summary.md").write_text("\n".join(lines) + "\n")
    print(args.run_root / "summary.md")


if __name__ == "__main__":
    main()

