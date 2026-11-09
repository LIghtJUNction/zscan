# Independent comparison project

This project is excluded from the published crate and has its own workspace,
dependencies, and lockfile. Read [cases.md](cases.md) for the fixed protocol and
[baseline-notes.md](baseline-notes.md) for a discovered comparator correctness
failure. The production crate has no dependency on any of this code.

```sh
cargo run --release --manifest-path comparison/Cargo.toml --locked -- --smoke
python3 comparison/run.py --config generic native scalar no-short no-prefilter --output comparison/results/experiment-1
python3 comparison/summarize.py comparison/results/experiment-1
```

Run one timing process at a time on an idle machine. `run.py` refuses to overwrite
an existing configuration directory. It preserves raw samples (gzip), every
case's fingerprint, exact source snapshot and SHA-256 hashes, compiler and CPU
metadata, commands, build logs, and correctness diagnostics. Set CPU affinity
outside the script when the machine permits it. Shared CI timings are only
observations; the smoke job validates the harness and results, not a speed gate.

`--list` prints the fixed case manifest without timing. `--cold OP ENGINE`
observes the first public call in a fresh process, including CPU initialization;
the clock and process startup state make it noisy. Capture seven fresh-process
observations for all one-shot functions with:

```sh
python3 comparison/run.py --mode cold --config generic --output comparison/results/cold-1
python3 comparison/summarize.py comparison/results/cold-1
```

All latency is ns/op. The summarizer uses the median of seven retained samples,
then divides the fastest correct, eligible baseline median by zscan's median.
The full per-case table remains next to each raw run. Construction compares one
bidirectional zscan Finder with a forward/reverse memchr Finder pair. Reuse
measurements keep construction outside both timed loops. Each one-shot loop
includes its API's preprocessing and CPU dispatch.

