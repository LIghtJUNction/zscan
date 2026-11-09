#!/usr/bin/env python3
"""Exercise private backend controls without weakening safe CPU detection."""

import os
from pathlib import Path
import platform
import subprocess


def run(cfg):
    env = dict(os.environ, RUSTFLAGS="--cfg " + cfg)
    print("Testing " + cfg, flush=True)
    subprocess.run(["cargo", "test", "--release", "--locked"], env=env, check=True)


def main():
    run("zscan_scalar")
    run("zscan_no_short")
    run("zscan_no_prefilter")
    machine = platform.machine()
    if machine in ("x86_64", "amd64"):
        # Linux advertises AVX2 only when the kernel supports its extended state.
        if "avx2" in Path("/proc/cpuinfo").read_text().split():
            run("zscan_force_avx2")
        else:
            print("AVX2 unavailable on this runner; the generic path was tested")
    elif machine in ("aarch64", "arm64"):
        run("zscan_force_neon")
    else:
        raise RuntimeError("this job expects an x86-64 or AArch64 runner")


if __name__ == "__main__":
    main()
