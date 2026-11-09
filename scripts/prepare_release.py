#!/usr/bin/env python3
"""Prepare version-only commits and immutable tags, or resume an existing tag."""

import base64
import json
import os
import re
import subprocess
import sys
import tomllib

import release

VERSION_FILES = ("Cargo.toml", "Cargo.lock", "comparison/Cargo.lock")
VERSION_LINE = re.compile(r'''(?m)^(\s*version\s*=\s*)(["'])([^"'\n]+)\2''')
MODES = ("current", "patch", "minor", "major", "custom", "existing-tag")


def request_fields(text):
    def unique_keys(pairs):
        result = {}
        for key, value in pairs:
            if key in result:
                raise ValueError("duplicate release-request key: " + key)
            result[key] = value
        return result

    request = json.loads(text, object_pairs_hook=unique_keys)
    allowed = {"mode", "expected_version", "version", "tag", "request_id"}
    if not isinstance(request, dict) or set(request) - allowed:
        raise ValueError("release request must be an object with only documented fields")
    if not all(isinstance(value, str) and value for value in request.values()):
        raise ValueError("release-request values must be nonempty strings")
    if request.get("mode") not in MODES or "expected_version" not in request:
        raise ValueError("release request requires mode and expected_version")
    release.version_from_tag("v" + request["expected_version"])
    if "request_id" in request and not re.fullmatch(r"[A-Za-z0-9._-]{1,80}", request["request_id"]):
        raise ValueError("request_id must be 1-80 letters, digits, dots, underscores or hyphens")
    return request


def selectors():
    if os.environ.get("GITHUB_EVENT_NAME") == "push":
        path = release.ROOT / ".github/release-request.json"
        if path.is_symlink() or not path.is_file() or path.stat().st_size > 4096:
            raise ValueError("release request must be a regular JSON file of at most 4096 bytes")
        request = request_fields(path.read_text())
        current = tomllib.loads((release.ROOT / "Cargo.toml").read_text())["package"]["version"]
        if request["expected_version"] != current:
            raise ValueError("stale release request: expected_version does not match the checked source")
        return request["mode"], request.get("version", ""), request.get("tag", "")
    return (
        os.environ.get("RELEASE_MODE") or "current",
        os.environ.get("RELEASE_CUSTOM_VERSION", ""),
        os.environ.get("RELEASE_EXISTING_TAG", ""),
    )


def next_version(current, mode, custom=""):
    release.version_from_tag("v" + current)
    numbers = tuple(int(part) for part in current.split("."))
    if mode not in ("current", "patch", "minor", "major", "custom"):
        raise ValueError("unknown release mode")
    if custom and mode != "custom":
        raise ValueError("custom version is only accepted in custom mode")
    if mode == "current":
        return current
    if mode == "custom":
        release.version_from_tag("v" + custom)
        if tuple(int(part) for part in custom.split(".")) <= numbers:
            raise ValueError("custom version must be greater than the current version")
        return custom
    major, minor, patch = numbers
    return {
        "patch": f"{major}.{minor}.{patch + 1}",
        "minor": f"{major}.{minor + 1}.0",
        "major": f"{major + 1}.0.0",
    }[mode]


def replace_version_line(block, old, new):
    matches = list(VERSION_LINE.finditer(block))
    if len(matches) != 1 or matches[0].group(3) != old:
        raise ValueError("expected exactly one matching package version")
    match = matches[0]
    return block[:match.start(3)] + new + block[match.end(3):]


def update_manifest(text, old, new):
    manifest = tomllib.loads(text)
    if manifest["package"]["name"] != "zscan" or manifest["package"]["version"] != old:
        raise ValueError("unexpected root package/version")
    start = re.search(r"(?m)^\[package\][ \t]*(?:#.*)?$", text)
    if start is None:
        raise ValueError("root Cargo.toml must have an explicit [package] table")
    following = re.search(r"(?m)^\[", text[start.end():])
    end = start.end() + following.start() if following else len(text)
    return text[:start.end()] + replace_version_line(text[start.end():end], old, new) + text[end:]


def update_lock(text, old, new):
    blocks = re.split(r"(?m)(?=^\[\[package\]\][ \t]*$)", text)
    matches = 0
    for index, block in enumerate(blocks):
        entries = tomllib.loads(block).get("package", [])
        if entries and entries[0]["name"] == "zscan" and "source" not in entries[0]:
            matches += 1
            if entries[0]["version"] != old:
                raise ValueError("path zscan lock entry disagrees with Cargo.toml")
            blocks[index] = replace_version_line(block, old, new)
    if matches != 1:
        raise ValueError("each lockfile must contain exactly one path zscan package")
    updated = "".join(blocks)
    # Cargo can qualify a path dependency when another package has the same name.
    return updated.replace('"zscan ' + old + '"', '"zscan ' + new + '"')


def version_updates(old, new):
    result = {}
    for name in VERSION_FILES:
        text = (release.ROOT / name).read_text()
        update = update_manifest if name == "Cargo.toml" else update_lock
        result[name] = update(text, old, new)
    return result


def remote_refs(*refs):
    # Empty output means absence; a failed connection raises, never means absent.
    output = release.git("ls-remote", "origin", *refs)
    return {ref: sha for sha, ref in (line.split("\t", 1) for line in output.splitlines())}


def tag_commit(refs, tag):
    name = "refs/tags/" + tag
    return refs.get(name + "^{}", refs.get(name))


def fetch_tag(tag):
    name = "refs/tags/" + tag
    release.git("fetch", "--no-tags", "origin", name + ":" + name)
    return release.resolve_tag(tag)


def candidate_commit(base, version, updates):
    for name, content in updates.items():
        (release.ROOT / name).write_text(content)
    release.git("add", "--", *VERSION_FILES)
    staged = release.git("diff", "--cached", "--name-only").splitlines()
    if any(name not in VERSION_FILES for name in staged):
        raise ValueError("release commit contains an unrelated staged change")
    print(release.git("diff", "--cached", "--", *VERSION_FILES), flush=True)
    tree = release.git("write-tree")
    # Identical input commits and version choices produce identical Git objects.
    timestamp = str(int(release.git("show", "-s", "--format=%ct", base)) + 1) + " +0000"
    env = dict(
        os.environ,
        GIT_AUTHOR_NAME="github-actions[bot]",
        GIT_AUTHOR_EMAIL="41898282+github-actions[bot]@users.noreply.github.com",
        GIT_COMMITTER_NAME="github-actions[bot]",
        GIT_COMMITTER_EMAIL="41898282+github-actions[bot]@users.noreply.github.com",
        GIT_AUTHOR_DATE=timestamp,
        GIT_COMMITTER_DATE=timestamp,
    )
    return subprocess.check_output(
        ["git", "-c", "commit.gpgsign=false", "commit-tree", tree, "-p", base],
        input="chore: release zscan v" + version + "\n",
        cwd=release.ROOT,
        env=env,
        text=True,
    ).strip()


def atomic_push(base, commit, tag):
    branch = "refs/heads/main"
    name = "refs/tags/" + tag
    before = remote_refs(branch, name, name + "^{}")
    if before.get(branch) != base:
        raise ValueError("main changed during preparation; no refs were pushed")
    if tag_commit(before, tag) is not None:
        raise ValueError("release tag appeared during preparation; no refs were pushed")
    token = os.environ.get("GH_TOKEN", "")
    if not token:
        raise ValueError("missing GitHub token for the version commit/tag")
    env = dict(os.environ, GIT_TERMINAL_PROMPT="0", GIT_TRACE="0", GIT_TRACE_CURL="0", GIT_CURL_VERBOSE="0")
    index = int(env.get("GIT_CONFIG_COUNT", "0"))
    env["GIT_CONFIG_COUNT"] = str(index + 1)
    env["GIT_CONFIG_KEY_" + str(index)] = "http.https://github.com/.extraheader"
    env["GIT_CONFIG_VALUE_" + str(index)] = "AUTHORIZATION: basic " + base64.b64encode(
        ("x-access-token:" + token).encode()
    ).decode()
    # One atomic, non-force push: either both refs advance, or neither does.
    result = subprocess.run(
        ["git", "push", "--atomic", "origin", commit + ":" + branch, commit + ":" + name],
        cwd=release.ROOT,
        env=env,
        check=False,
    )
    after = remote_refs(branch, name, name + "^{}")
    if after.get(branch) != commit or tag_commit(after, tag) != commit:
        raise RuntimeError(
            "release refs were not confirmed (git exit " + str(result.returncode)
            + "); no retry or force push was attempted. Inspect main and the tag."
        )
    if result.returncode != 0:
        print("::warning::Git reported an error, but both remote refs confirm the exact release commit.")


def prepare():
    if os.environ.get("GITHUB_REPOSITORY") != "LIghtJUNction/zscan":
        raise ValueError("release preparation is only intended for LIghtJUNction/zscan")
    mode, custom, existing = selectors()
    # Retain the old workflow_dispatch {tag: ...} interface without ambiguity.
    if existing or mode == "existing-tag":
        if mode not in ("current", "existing-tag") or custom:
            raise ValueError("existing tag cannot be combined with a version bump")
        release.version_from_tag(existing)
        identity = fetch_tag(existing)
        release.registry_version(identity["version"])
        return identity
    if os.environ.get("GITHUB_REF") != "refs/heads/main":
        raise ValueError("select the main branch to prepare a new release")
    base = release.git("rev-parse", "HEAD")
    if base != os.environ.get("GITHUB_SHA"):
        raise ValueError("checkout does not match the triggering request commit")
    if release.git("status", "--porcelain"):
        raise ValueError("release preparation requires a clean checkout")
    current = tomllib.loads((release.ROOT / "Cargo.toml").read_text())["package"]["version"]
    version = next_version(current, mode, custom)
    tag = "v" + version
    name = "refs/tags/" + tag
    refs = remote_refs("refs/heads/main", name, name + "^{}")
    remote_tag = tag_commit(refs, tag)
    record = release.registry_version(version)
    if record is not None and remote_tag is None:
        raise ValueError("version already exists on crates.io without a release tag; no refs changed")
    if remote_tag == base and mode == "current":
        return fetch_tag(tag)
    if remote_tag is None and refs.get("refs/heads/main") != base:
        raise ValueError("main advanced after the request; restart from its current commit")
    updates = version_updates(current, version)
    candidate = candidate_commit(base, version, updates)
    if remote_tag is not None:
        if remote_tag != candidate:
            raise ValueError("release tag already points elsewhere; choose existing-tag or a new version")
        return fetch_tag(tag)
    atomic_push(base, candidate, tag)
    identity = fetch_tag(tag)
    if identity["commit"] != candidate:
        raise ValueError("release tag changed after the atomic push")
    return identity


if __name__ == "__main__":
    try:
        release.output_identity(prepare())
    except (ValueError, RuntimeError, KeyError, OSError, subprocess.CalledProcessError) as error:
        print("::error::" + str(error), file=sys.stderr)
        sys.exit(1)
