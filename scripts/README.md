# Verification and release maintenance

The scripts use Python 3.11+ and its standard library. They are excluded from the
published crate. `audit.py` reads Cargo metadata, including normal, build, dev,
optional, and target-specific dependencies; every entry must be absent.

```sh
python3 scripts/audit.py
python3 -m unittest discover -s scripts -p 'test_*.py' -v
python3 scripts/test_backends.py
```

## Release

Set the repository Secret `CARGO_REGISTRY_TOKEN`, then open **Actions → Release
→ Run workflow** on `main`. Choose one mode:

| Mode | Effect |
| --- | --- |
| `current` | Release the version already in Cargo.toml; use this for the first release. |
| `patch` | For example, 0.1.0 → 0.1.1. |
| `minor` | For example, 0.1.3 → 0.2.0. |
| `major` | For example, 0.1.3 → 1.0.0. |
| `custom` | Set the optional `version` field to a greater plain X.Y.Z version. |
| `existing-tag` | Set `tag` to an existing vX.Y.Z tag to resume its release. |

Leave unused fields empty. Older dispatch clients supplying only `tag` still
work. Plain release versions are required; leading zeroes and prerelease
suffixes are rejected. New release preparation is restricted to `main`.

### Release through a committed request

When workflow dispatch is unavailable, commit a deliberate change to
`.github/release-request.json` on `main`. For the first 0.1.0 release:

```json
{"mode":"current","expected_version":"0.1.0"}
```

Only changes to this exact file initiate branch-based publication. Ordinary
source pushes do not. `expected_version` must match the request commit's
Cargo.toml, preventing an old request from causing a different bump. The JSON
accepts only `mode`, `expected_version`, optional `version` or `tag` as above,
and an optional `request_id` of 1–80 letters, digits, dots, underscores or hyphens.
A fresh `request_id` allows a deliberate retry commit. Unknown or duplicate
keys, empty or non-string values, conflicting selectors, symlinks, and requests
over 4 KiB are rejected. To resume an existing release, for example:

```json
{"mode":"existing-tag","expected_version":"0.1.0","tag":"v0.1.0","request_id":"retry-1"}
```

Here `expected_version` refers to the branch source, even when the selected tag
is older. Create or edit the request file only when intending to initiate a
release.

### Version commit, checks, and publication

Preparation updates only `Cargo.toml`, `Cargo.lock`, and the path zscan entry in
`comparison/Cargo.lock`. It prints the diff and builds a deterministic release
commit with a fixed bot identity and the source commit's timestamp plus one
second. `current` makes an empty release commit so the branch and tag can be
advanced in one atomic operation. The script checks registry availability before
changing refs, then performs one non-force atomic push of `main` and `vX.Y.Z`.
Concurrent branch or tag changes cannot partially advance these refs; no tag or
published version is overwritten. A matching existing release commit/tag can be
resumed without another push.

GitHub-token pushes do not launch new Actions runs. This workflow explicitly
calls the full CI matrix on the resolved commit and then publishes in the same
run. Direct pushes of existing-commit `vX.Y.Z` tags remain supported. Every CI
checkout uses the tag's exact commit. Publication checks the local HEAD, tag,
remote tag, manifest version, clean tracked files, and archive VCS identity
again. The publication job starts only after the full CI matrix succeeds, then
repeats packaging and dry-run before uploading.

Release packaging and publication use pinned Rust/Cargo **1.99.1**, recorded in
`scripts/release-toolchain.txt`. The resolver reads this file from the tagged
commit, so a manual run from a newer branch still uses the older tag's pin.
Regular CI continues to check rolling stable and the MSRV. This prevents a later
stable update from silently changing normalized manifests or archive bytes
during manual recovery. The publication script checks the actual
`rustc --version --verbose` and `cargo --version --verbose` identities and records
them in `target/release-evidence.json`, without credentials. For a previously
published archive, these identify the current verification tools, not the
historical publisher's tools. Checksum equality remains mandatory; matching
source commits alone never establish archive equality.

Only the publication step receives `CARGO_REGISTRY_TOKEN`. It invokes
`cargo publish --locked` once. A missing Secret, failed check, unavailable registry
preflight, or conflicting published checksum stops the run. No token is passed in
arguments or written to a file.

After an upload error, read-only registry checks determine whether the exact
archive was published. The workflow does not upload again automatically. If
confirmation remains unavailable, inspect crates.io before manually rerunning the
same tag. A rerun recognizes an identical existing archive by SHA256 and proceeds
without republishing. A different archive or a yanked version requires manual
investigation. GitHub Release creation happens only after the registry confirms
the matching crate, version, and checksum.

`test_release.py` exercises publication failures with isolated mocks.
`test_prepare_release.py` checks version changes and uses temporary local Git
remotes to exercise branch/tag races, atomic pushes and idempotent recovery.
Neither test suite contacts GitHub or uploads a crate. Configuring this workflow
or passing its tests is not evidence of a real publication.

## Action pins

The official tag refs were checked on 2026-11-09:

| Action | Tag | Full commit |
| --- | --- | --- |
| [actions/checkout](https://api.github.com/repos/actions/checkout/git/ref/tags/v7.0.1) | v7.0.1 | `3d3c42e5aac5ba805825da76410c181273ba90b1` |
| [actions/upload-artifact](https://api.github.com/repos/actions/upload-artifact/git/ref/tags/v7.0.2) | v7.0.2 | `cf430e030ddbb5b0abf93d22962f4752f3646cd9` |

The crates.io version/checksum response schema was checked against its
[official response snapshot](https://github.com/rust-lang/crates.io/blob/main/src/tests/routes/crates/versions/snapshots/integration__routes__crates__versions__read__show_by_crate_name_and_version.snap).
Updates should resolve and review a new full commit before changing either pin.
Hosted-runner benchmark smoke results are observations, not a performance gate.
