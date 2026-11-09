#!/usr/bin/env python3
"""Record reproducible, unfiltered comparison runs; never overwrite a run."""

import argparse
import csv
import datetime
import gzip
import hashlib
import io
import json
import os
from pathlib import Path
import platform
import shutil
import subprocess
import tarfile
import time


ROOT = Path(__file__).resolve().parent.parent
CONFIGS = {
    "generic": "",
    "native": "-C target-cpu=native",
    "scalar": "--cfg zscan_scalar",
    "no-short": "--cfg zscan_no_short",
    "no-prefilter": "--cfg zscan_no_prefilter",
}


def capture(command, env):
    process = subprocess.run(command, cwd=ROOT, env=env, text=True, capture_output=True)
    return {
        "command": command,
        "returncode": process.returncode,
        "stdout": process.stdout,
        "stderr": process.stderr,
    }


def sources():
    entries = subprocess.check_output(
        ["git", "ls-files", "-co", "--exclude-standard", "-z"], cwd=ROOT
    )
    return sorted(
        ROOT / path
        for path in set(entries.decode().split("\0"))
        if path
        and (ROOT / path).is_file()
        and "results" not in Path(path).parts
        and "target" not in Path(path).parts
        and "__pycache__" not in Path(path).parts
    )


def compile_inputs():
    paths = set((ROOT / "src").rglob("*.rs")) | set((ROOT / "comparison/src").rglob("*.rs"))
    for filename in [
        "Cargo.toml", "Cargo.lock", "comparison/Cargo.toml", "comparison/Cargo.lock",
        "rust-toolchain", "rust-toolchain.toml", ".cargo/config", ".cargo/config.toml",
    ]:
        path = ROOT / filename
        if path.is_file():
            paths.add(path)
    return {
        str(path.relative_to(ROOT)): hashlib.sha256(path.read_bytes()).hexdigest()
        for path in sorted(paths)
    }


def cpu_information():
    try:
        blocks = Path("/proc/cpuinfo").read_text().strip().split("\n\n")
        # CPU flags and topology are public machine metadata; do not dump env.
        return {"logical_cpu_entries": len(blocks), "first_cpu": blocks[0]}
    except OSError as error:
        return {"unavailable": str(error)}


def execute(config, args, output_root):
    output = output_root / config
    output.mkdir(parents=True, exist_ok=False)
    env = os.environ.copy()
    env["PATH"] = str(Path.home() / ".cargo/bin") + os.pathsep + env.get("PATH", "")
    env["RUSTFLAGS"] = " ".join(
        flag for flag in [env.get("RUSTFLAGS", ""), CONFIGS[config]] if flag
    )
    env["CARGO_TARGET_DIR"] = str(ROOT / "comparison/target")
    cargo = shutil.which("cargo", path=env["PATH"])
    if not cargo:
        raise SystemExit("cargo is not available")
    build = [
        cargo,
        "build",
        "--release",
        "--locked",
        "--manifest-path",
        "comparison/Cargo.toml",
    ]
    binary = Path(env["CARGO_TARGET_DIR"]) / "release/zscan-comparison"
    metadata = {
        "started_utc": datetime.datetime.now(datetime.timezone.utc).isoformat(),
        "config": config,
        "mode": args.mode,
        "complete": False,
        "rustflags": env["RUSTFLAGS"],
        "compile_profile": "release: opt-level=3, codegen-units=1, lto=thin",
        "uname": platform.uname()._asdict(),
        "cpu": cpu_information(),
        "affinity": sorted(os.sched_getaffinity(0)) if hasattr(os, "sched_getaffinity") else None,
        "rustc": capture(["rustc", "--version", "--verbose"], env),
        "cargo": capture([cargo, "--version", "--verbose"], env),
        "git_commit": capture(["git", "rev-parse", "HEAD"], env),
        "git_status": capture(["git", "status", "--short"], env),
        "baseline_commits": {
            "memchr-2.8.3": "5fdb40c054e1fff359a2f7bdf7f87a13b34b465d",
            "memx-0.2.2": "c52c59209caa169054a4b2f8533d9ad646c59dd4",
        },
        "build_command": build,
        "source_sha256": {},
        "compile_inputs_before": compile_inputs(),
    }
    with tarfile.open(output / "source.tar.gz", "w:gz") as archive:
        for path in sources():
            relative = str(path.relative_to(ROOT))
            content = path.read_bytes()
            metadata["source_sha256"][relative] = hashlib.sha256(content).hexdigest()
            entry = tarfile.TarInfo(relative)
            entry.size = len(content)
            archive.addfile(entry, io.BytesIO(content))
    shutil.copyfile(ROOT / "comparison/Cargo.lock", output / "Cargo.lock")
    metadata_path = output / "metadata.json"
    metadata_path.write_text(json.dumps(metadata, indent=2) + "\n")
    built = capture(build, env)
    (output / "build.stdout").write_text(built["stdout"])
    (output / "build.stderr").write_text(built["stderr"])
    metadata["build_returncode"] = built["returncode"]
    metadata["compile_inputs_after_build"] = compile_inputs()
    if built["returncode"]:
        metadata_path.write_text(json.dumps(metadata, indent=2) + "\n")
        raise SystemExit(f"build failed; preserved logs in {output}")
    if metadata["compile_inputs_after_build"] != metadata["compile_inputs_before"]:
        metadata["error"] = "compile inputs changed during build; no timing performed"
        metadata_path.write_text(json.dumps(metadata, indent=2) + "\n")
        raise SystemExit(metadata["error"])
    dependency_metadata = capture(
        [cargo, "metadata", "--locked", "--format-version", "1", "--manifest-path", "comparison/Cargo.toml"],
        env,
    )
    if dependency_metadata["returncode"]:
        (output / "dependency-metadata-error.json").write_text(json.dumps(dependency_metadata, indent=2))
        raise SystemExit("cargo metadata failed")
    (output / "cargo-metadata.json").write_text(dependency_metadata["stdout"])
    with (output / "cases.csv").open("w") as manifest:
        subprocess.run([str(binary), "--list"], cwd=ROOT, env=env, stdout=manifest, check=True)
    metadata["cases_sha256"] = hashlib.sha256((output / "cases.csv").read_bytes()).hexdigest()
    start = time.monotonic()
    if args.mode == "cold":
        commands = []
        with gzip.open(output / "samples.csv.gz", "wt", newline="") as saved:
            writer = None
            for sample in range(7):
                for operation in ["byte_find", "byte_rfind", "find", "rfind"]:
                    for engine in ["zscan", "memchr", "memx"]:
                        command = [str(binary), "--cold", operation, engine]
                        commands.append(command)
                        result = capture(command, env)
                        if result["returncode"]:
                            (output / "cold-failure.json").write_text(json.dumps(result, indent=2))
                            raise SystemExit("cold observation failed")
                        reader = csv.DictReader(io.StringIO(result["stdout"]))
                        if writer is None:
                            writer = csv.DictWriter(saved, fieldnames=reader.fieldnames)
                            writer.writeheader()
                        for row in reader:
                            row["sample"] = sample
                            writer.writerow(row)
        metadata["benchmark_commands"] = commands
        returncode = 0
    else:
        command = [str(binary)] + (["--smoke"] if args.mode == "smoke" else [])
        metadata["benchmark_commands"] = [command]
        with (output / "benchmark.stderr").open("wb") as errors:
            process = subprocess.Popen(command, cwd=ROOT, env=env, stdout=subprocess.PIPE, stderr=errors)
            with gzip.open(output / "samples.csv.gz", "wb") as saved:
                shutil.copyfileobj(process.stdout, saved)
            process.stdout.close()
            returncode = process.wait()
    metadata["benchmark_seconds"] = time.monotonic() - start
    metadata["benchmark_returncode"] = returncode
    metadata["compile_inputs_after_run"] = compile_inputs()
    unchanged = metadata["compile_inputs_after_run"] == metadata["compile_inputs_before"]
    metadata["complete"] = returncode == 0 and unchanged
    if not unchanged:
        metadata["error"] = "compile inputs changed while timing; cannot attribute samples to one source state"
    metadata["finished_utc"] = datetime.datetime.now(datetime.timezone.utc).isoformat()
    metadata_path.write_text(json.dumps(metadata, indent=2) + "\n")
    if returncode:
        raise SystemExit(f"benchmark failed; retained partial output and logs in {output}")
    if not unchanged:
        raise SystemExit(metadata["error"])
    print(f"{config}: saved every sample in {output}", flush=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--config", choices=CONFIGS, nargs="+", default=list(CONFIGS))
    parser.add_argument("--mode", choices=["full", "smoke", "cold"], default="full")
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    output = args.output or ROOT / "comparison/results" / datetime.datetime.now(datetime.timezone.utc).strftime("%Y%m%dT%H%M%SZ")
    for config in args.config:
        execute(config, args, output.resolve())


if __name__ == "__main__":
    main()

