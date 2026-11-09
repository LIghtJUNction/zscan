#!/usr/bin/env python3
"""Validate a tag, publish once, and create a Release after registry confirmation."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import tarfile
import time
import tomllib
from urllib.error import HTTPError, URLError
from urllib.request import Request, urlopen

ROOT = Path(__file__).resolve().parent.parent
EVIDENCE = ROOT / "target/release-evidence.json"
RELEASE_TOOLCHAIN = (ROOT / "scripts/release-toolchain.txt").read_text().strip()
TAG_PATTERN = re.compile(r"v(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\Z")
SHA_PATTERN = re.compile(r"[0-9a-f]{40}\Z")
TOOLCHAIN_PATTERN = re.compile(r"1\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\Z")


def git(*args):
    return subprocess.check_output(["git", *args], cwd=ROOT, text=True).strip()


def version_from_tag(tag):
    if not TAG_PATTERN.fullmatch(tag):
        raise ValueError("release tag must be vX.Y.Z with no leading zeroes or suffix")
    return tag[1:]


def resolve_tag(tag):
    version = version_from_tag(tag)
    commit = git("rev-parse", "--verify", "refs/tags/" + tag + "^{commit}")
    if not SHA_PATTERN.fullmatch(commit):
        raise ValueError("tag must resolve to an existing full commit SHA")
    manifest = tomllib.loads(git("show", commit + ":Cargo.toml"))["package"]
    if manifest["name"] != "zscan" or manifest["version"] != version:
        raise ValueError("tag and the tagged Cargo.toml package/version disagree")
    toolchain = git("show", commit + ":scripts/release-toolchain.txt")
    if not TOOLCHAIN_PATTERN.fullmatch(toolchain):
        raise ValueError("tagged release toolchain must be a fixed stable 1.X.Y version")
    return {"tag": tag, "version": version, "commit": commit, "toolchain": toolchain}


def checked_identity():
    identity = resolve_tag(os.environ["RELEASE_TAG"])
    if identity["commit"] != os.environ["RELEASE_COMMIT"]:
        raise ValueError("release tag moved after verification; refusing publication")
    if identity["version"] != os.environ["RELEASE_VERSION"]:
        raise ValueError("resolved release version changed")
    if identity["toolchain"] != os.environ.get("RUSTUP_TOOLCHAIN"):
        raise ValueError("packaging toolchain does not match the release tag's fixed version")
    if git("rev-parse", "HEAD") != identity["commit"]:
        raise ValueError("checkout is not the verified release commit")
    if git("status", "--porcelain", "--untracked-files=no"):
        raise ValueError("tracked files changed after checkout")
    tag_ref = "refs/tags/" + identity["tag"]
    remote_refs = {}
    for line in git("ls-remote", "--exit-code", "origin", tag_ref, tag_ref + "^{}").splitlines():
        sha, ref = line.split("\t", 1)
        remote_refs[ref] = sha
    remote_commit = remote_refs.get(tag_ref + "^{}", remote_refs.get(tag_ref))
    if remote_commit != identity["commit"]:
        raise ValueError("remote tag moved after verification; refusing publication")
    return identity


def package_checksum(identity):
    path = ROOT / "target/package" / ("zscan-" + identity["version"] + ".crate")
    # No extraction: validate the VCS identity recorded inside the archive itself.
    with tarfile.open(path, "r:gz") as archive:
        member = archive.extractfile("zscan-" + identity["version"] + "/.cargo_vcs_info.json")
        if member is None:
            raise ValueError("package does not record its Git identity")
        vcs = json.load(member)["git"]
    if vcs["sha1"] != identity["commit"] or vcs.get("dirty", False):
        raise ValueError("packaged source is not the clean, verified commit")
    return hashlib.sha256(path.read_bytes()).hexdigest()


def toolchain_information():
    info = {"requested": os.environ.get("RUSTUP_TOOLCHAIN", "")}
    if info["requested"] != RELEASE_TOOLCHAIN:
        raise ValueError("release packaging requires RUSTUP_TOOLCHAIN=" + RELEASE_TOOLCHAIN)
    for command in ("rustc", "cargo"):
        output = subprocess.check_output(
            [command, "--version", "--verbose"], cwd=ROOT, text=True
        ).strip()
        if "release: " + RELEASE_TOOLCHAIN not in output.splitlines():
            raise ValueError(command + " does not match the pinned release toolchain")
        info[command + "_verbose"] = output
    return info


def request_json(url, *, token=None, body=None, allow_missing=False):
    headers = {"User-Agent": "zscan-release/1.0 (https://github.com/LIghtJUNction/zscan)"}
    headers["Accept"] = "application/json"
    if token:
        headers["Authorization"] = "Bearer " + token
        headers["X-GitHub-Api-Version"] = "2022-11-28"
    data = None
    if body is not None:
        headers["Content-Type"] = "application/json"
        data = json.dumps(body).encode()
    request = Request(url, data=data, headers=headers)
    try:
        with urlopen(request, timeout=20) as response:
            return json.load(response)
    except HTTPError as error:
        # Only a definite 404 establishes absence. Network/auth errors never do.
        if allow_missing and error.code == 404:
            return None
        raise


def registry_version(version):
    result = request_json(
        "https://crates.io/api/v1/crates/zscan/" + version, allow_missing=True
    )
    return result["version"] if result is not None else None


def confirm_version(record, version, checksum):
    if record["crate"] != "zscan" or record["num"] != version:
        raise ValueError("registry returned a different crate or version")
    if record["checksum"] != checksum:
        raise ValueError("version already exists with different bytes; never overwrite or retry")
    if record["yanked"]:
        raise ValueError("published version is yanked; inspect manually before making a Release")


def publish():
    # The workflow maps this secret only into this step, never args/files/logs.
    if not os.environ.get("CARGO_REGISTRY_TOKEN", "").strip():
        raise ValueError("missing CARGO_REGISTRY_TOKEN repository Secret; publication stopped")
    identity = checked_identity()
    toolchain = toolchain_information()
    checksum = package_checksum(identity)
    record = registry_version(identity["version"])
    if record is not None:
        confirm_version(record, identity["version"], checksum)
        outcome = "already-published-identical"
        print("Registry already contains this exact archive; no upload attempted.")
    else:
        # Exactly one upload attempt. A post-upload failure can still mean success.
        result = subprocess.run(["cargo", "publish", "--locked"], cwd=ROOT, check=False)
        deadline = time.monotonic() + 120
        last_error = "version not yet visible"
        while True:
            try:
                record = registry_version(identity["version"])
                if record is not None:
                    confirm_version(record, identity["version"], checksum)
                    break
            except (HTTPError, URLError, TimeoutError) as error:
                last_error = str(error)
            if time.monotonic() >= deadline:
                raise RuntimeError(
                    "publication state is unconfirmed (cargo exit " + str(result.returncode)
                    + "; " + last_error + "). No second upload was attempted. Inspect crates.io; "
                    "a manual rerun first checks the existing version's checksum."
                )
            time.sleep(4)
        outcome = "published-confirmed"
        if result.returncode != 0:
            print("::warning::cargo publish failed after upload, but the registry confirms the exact archive.")
    evidence = dict(
        identity,
        checksum=checksum,
        outcome=outcome,
        registry=record,
        verification_toolchain=toolchain,
    )
    EVIDENCE.parent.mkdir(parents=True, exist_ok=True)
    EVIDENCE.write_text(json.dumps(evidence, indent=2) + "\n")
    print("Confirmed zscan " + identity["version"] + " SHA256 " + checksum)


def github_release():
    identity = checked_identity()
    evidence = json.loads(EVIDENCE.read_text())
    for key, value in identity.items():
        if evidence[key] != value:
            raise ValueError("registry evidence does not match the checked commit")
    checksum = package_checksum(identity)
    if evidence["checksum"] != checksum:
        raise ValueError("archive changed after registry confirmation")
    record = registry_version(identity["version"])
    if record is None:
        raise ValueError("registry no longer confirms this version; refusing GitHub Release")
    confirm_version(record, identity["version"], checksum)
    repository = os.environ["GITHUB_REPOSITORY"]
    if repository != "LIghtJUNction/zscan":
        raise ValueError("release workflow is only intended for LIghtJUNction/zscan")
    token = os.environ["GH_TOKEN"]
    base = "https://api.github.com/repos/" + repository
    remote = request_json(base + "/git/ref/tags/" + identity["tag"], token=token)["object"]
    for _ in range(10):
        if remote["type"] != "tag":
            break
        remote = request_json(base + "/git/tags/" + remote["sha"], token=token)["object"]
    if remote["type"] != "commit" or remote["sha"] != identity["commit"]:
        raise ValueError("remote tag moved after verification; refusing GitHub Release")
    release = request_json(
        base + "/releases/tags/" + identity["tag"], token=token, allow_missing=True
    )
    if release is not None:
        if release["draft"] or release["prerelease"]:
            raise ValueError("existing Release is a draft/prerelease; inspect it manually")
        print("Release already exists: " + release["html_url"])
        return
    release = request_json(
        base + "/releases",
        token=token,
        body={
            "tag_name": identity["tag"],
            "target_commitish": identity["commit"],
            "name": "zscan " + identity["version"],
            "body": (
                "Published to [crates.io](https://crates.io/crates/zscan/"
                + identity["version"] + ").\n\nVerified commit: `" + identity["commit"]
                + "`\n\nCrate SHA256: `" + checksum + "`"
            ),
            "generate_release_notes": True,
            "draft": False,
            "prerelease": False,
            "make_latest": "legacy",
        },
    )
    print("Created Release: " + release["html_url"])


def output_identity(identity):
    if output := os.environ.get("GITHUB_OUTPUT"):
        with open(output, "a") as stream:
            for key, value in identity.items():
                stream.write(key + "=" + value + "\n")
    print(json.dumps(identity))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", choices=("resolve", "check", "publish", "github"))
    command = parser.parse_args().command
    if command == "resolve":
        identity = resolve_tag(os.environ["RELEASE_TAG"])
        if os.environ.get("GITHUB_EVENT_NAME") == "push" and git("rev-parse", "HEAD") != identity["commit"]:
            raise ValueError("tag moved since the push event; refusing a different commit")
        output_identity(identity)
    elif command == "check":
        print(json.dumps(checked_identity()))
    elif command == "publish":
        publish()
    else:
        github_release()


if __name__ == "__main__":
    try:
        main()
    except (ValueError, RuntimeError, KeyError, OSError, subprocess.CalledProcessError) as error:
        print("::error::" + str(error), file=sys.stderr)
        sys.exit(1)
