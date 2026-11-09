#!/usr/bin/env python3
"""Verify the published crate has no dependencies or custom build step."""

import argparse
import json
from pathlib import Path
import re
import subprocess
import tomllib


def check_metadata(metadata):
    packages = metadata["packages"]
    if len(packages) != 1 or packages[0]["name"] != "zscan":
        raise ValueError("the root workspace must contain only zscan")
    package = packages[0]
    # Cargo includes normal, build, dev, optional and target-specific entries here.
    if package["dependencies"]:
        raise ValueError("all dependency kinds must be empty: " + repr(package["dependencies"]))
    if any("custom-build" in target["kind"] for target in package["targets"]):
        raise ValueError("custom build scripts are forbidden")
    if len(metadata["workspace_members"]) != 1:
        raise ValueError("comparison must not be a member of the root workspace")


def check_package_list(names):
    for name in names:
        parts = Path(name).parts
        if "comparison" in parts or "build.rs" in parts:
            raise ValueError("forbidden package entry: " + name)
    for required in ("Cargo.toml", "Cargo.lock", "src/lib.rs", "README.md"):
        if required not in names:
            raise ValueError("missing package entry: " + required)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--package-list", type=Path)
    args = parser.parse_args()
    root = Path(__file__).resolve().parent.parent
    manifest = tomllib.loads((root / "Cargo.toml").read_text())
    if manifest["package"].get("build", False) is not False or (root / "build.rs").exists():
        raise ValueError("build.rs and custom package.build are forbidden")
    if not re.search(r"^\s*#!\[no_std\]", (root / "src/lib.rs").read_text(), re.MULTILINE):
        raise ValueError("src/lib.rs must unconditionally declare #![no_std]")
    metadata = subprocess.check_output(
        ["cargo", "metadata", "--format-version", "1", "--no-deps", "--locked"],
        cwd=root,
        text=True,
    )
    check_metadata(json.loads(metadata))
    if args.package_list:
        check_package_list(args.package_list.read_text().splitlines())
    print("PASS: unconditional no_std, no third-party dependencies of any kind, no build script")


if __name__ == "__main__":
    main()

