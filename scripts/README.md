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

Set the repository Secret `CARGO_REGISTRY_TOKEN`, commit the intended version,
then push its existing-commit `vX.Y.Z` tag. The Release workflow also accepts an
existing tag through **Run workflow**. Only plain release versions are accepted;
leading zeroes and prerelease suffixes are rejected.

The workflow resolves the tag first and passes its full commit SHA to every CI
checkout. A manual run checks the tagged source even when the workflow is started
from a different branch. Publication checks the local HEAD, tag, remote tag,
manifest version, clean tracked files, and archive VCS identity again. The
publication job starts only after the full CI matrix succeeds, then repeats
packaging and dry-run before uploading.

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

`test_release.py` exercises these failure and recovery cases with isolated mocks;
it never uploads a crate. Configuring this workflow or passing its tests is not
evidence of a real publication.

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
