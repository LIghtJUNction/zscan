"""Version and atomic-ref tests use temporary local repositories, never GitHub."""

from contextlib import redirect_stdout
import io
import json
import os
from pathlib import Path
import subprocess
import tempfile
import tomllib
import unittest
from unittest.mock import patch

import prepare_release as prepare
import release


MANIFEST = '[package]\nname = "zscan"\nversion = "0.1.0" # keep comment\ndescription = "0.1.0 stays in prose"\n\n[lib]\npath = "src/lib.rs"\n'
LOCK = 'version = 3\n\n[[package]]\nname = "zscan"\nversion = "0.1.0"\n'
COMPARISON_LOCK = 'version = 3\n\n[[package]]\nname = "other"\nversion = "0.1.0"\nsource = "registry+https://example.test"\n\n[[package]]\nname = "zscan"\nversion = "0.1.0"\n\n[[package]]\nname = "comparison"\nversion = "0.0.0"\ndependencies = ["zscan 0.1.0"]\n'


class VersionTests(unittest.TestCase):
    def test_semver_bumps_and_custom_version(self):
        for mode, custom, expected in (
            ("current", "", "1.9.8"), ("patch", "", "1.9.9"),
            ("minor", "", "1.10.0"), ("major", "", "2.0.0"),
            ("custom", "1.10.7", "1.10.7"),
        ):
            self.assertEqual(prepare.next_version("1.9.8", mode, custom), expected)
        for mode, custom in (("custom", "1.9.8"), ("custom", "1.9.7"), ("custom", "v2.0.0"), ("patch", "2.0.0"), ("unknown", "")):
            with self.subTest(mode=mode, custom=custom), self.assertRaises(ValueError):
                prepare.next_version("1.9.8", mode, custom)

    def test_version_changes_preserve_unrelated_fields_and_lock_packages(self):
        manifest = prepare.update_manifest(MANIFEST, "0.1.0", "0.2.0")
        self.assertIn('version = "0.2.0" # keep comment', manifest)
        self.assertIn('description = "0.1.0 stays in prose"', manifest)
        lock = tomllib.loads(prepare.update_lock(COMPARISON_LOCK, "0.1.0", "0.2.0"))
        self.assertEqual(lock["package"][0]["version"], "0.1.0")
        self.assertEqual(lock["package"][1]["version"], "0.2.0")
        self.assertEqual(lock["package"][2]["dependencies"], ["zscan 0.2.0"])
        for malformed in (LOCK.replace('"0.1.0"', '"0.3.0"'), LOCK + LOCK[LOCK.index("[[package]]"):]):
            with self.assertRaises(ValueError):
                prepare.update_lock(malformed, "0.1.0", "0.2.0")

    def test_request_json_rejects_ambiguous_or_unsupported_fields(self):
        valid = {"mode": "current", "expected_version": "0.1.0"}
        self.assertEqual(prepare.request_fields(json.dumps(valid)), valid)
        invalid = (
            '[]', '{"mode":"current"}',
            '{"mode":"patch","mode":"major","expected_version":"0.1.0"}',
            '{"mode":"current","expected_version":"0.1.0","force":true}',
            '{"mode":"current","expected_version":0.1}',
            '{"mode":"current","expected_version":"01.2.3"}',
            '{"mode":"current","expected_version":"0.1.0","request_id":"../x"}',
        )
        for text in invalid:
            with self.subTest(text=text), self.assertRaises(ValueError):
                prepare.request_fields(text)


class LocalGitTests(unittest.TestCase):
    def setUp(self):
        temporary = self.enterContext(tempfile.TemporaryDirectory())
        self.root = Path(temporary) / "checkout"
        self.remote = Path(temporary) / "remote.git"
        self.root.mkdir()
        self.call("git", "init", "--bare", "-q", str(self.remote))
        self.git("init", "-q", "-b", "main")
        self.git("config", "user.name", "Release test")
        self.git("config", "user.email", "release-test@example.invalid")
        self.git("config", "commit.gpgsign", "false")
        for name, text in {
            "Cargo.toml": MANIFEST,
            "Cargo.lock": LOCK,
            "comparison/Cargo.lock": COMPARISON_LOCK,
            "scripts/release-toolchain.txt": "1.99.1\n",
        }.items():
            path = self.root / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(text)
        self.git("add", ".")
        self.git("commit", "-qm", "initial")
        self.base = self.git("rev-parse", "HEAD")
        self.git("remote", "add", "origin", str(self.remote))
        self.git("push", "-q", "origin", "main")
        self.enterContext(patch.object(release, "ROOT", self.root))
        self.enterContext(patch.object(release, "registry_version", return_value=None))
        self.enterContext(redirect_stdout(io.StringIO()))
        self.enterContext(patch.dict(os.environ, {
            "GITHUB_REPOSITORY": "LIghtJUNction/zscan", "GITHUB_REF": "refs/heads/main",
            "GITHUB_EVENT_NAME": "workflow_dispatch", "GITHUB_SHA": self.base,
            "RELEASE_MODE": "patch", "RELEASE_CUSTOM_VERSION": "", "RELEASE_EXISTING_TAG": "",
            "GH_TOKEN": "local-test-only-not-a-token",
        }))

    def call(self, *command, **kwargs):
        return subprocess.check_output(command, text=True, stderr=subprocess.PIPE, **kwargs).strip()

    def git(self, *args):
        return self.call("git", *args, cwd=self.root)

    def remote_git(self, *args):
        return self.call("git", "--git-dir=" + str(self.remote), *args)

    def test_patch_updates_all_locks_and_rerun_reuses_exact_commit(self):
        identity = prepare.prepare()
        self.assertEqual(identity["tag"], "v0.1.1")
        self.assertEqual(self.remote_git("rev-parse", "main"), identity["commit"])
        self.assertEqual(self.remote_git("rev-parse", "v0.1.1"), identity["commit"])
        self.assertEqual(set(self.git("diff", "--name-only", self.base, identity["commit"]).splitlines()), set(prepare.VERSION_FILES))
        for name in prepare.VERSION_FILES:
            parsed = tomllib.loads(self.git("show", identity["commit"] + ":" + name))
            package = parsed["package"]
            if isinstance(package, list):
                package = next(p for p in package if p["name"] == "zscan")
            self.assertEqual(package["version"], "0.1.1")
        # Rerun the original workflow's source commit, as GitHub's rerun does.
        self.git("reset", "--hard", self.base)
        with patch.object(prepare, "atomic_push") as pushed:
            self.assertEqual(prepare.prepare(), identity)
            pushed.assert_not_called()

    def test_current_release_creates_empty_commit_and_can_resume(self):
        with patch.dict(os.environ, {"RELEASE_MODE": "current"}):
            identity = prepare.prepare()
            self.assertNotEqual(identity["commit"], self.base)
            self.assertEqual(self.git("diff", "--name-only", self.base, identity["commit"]), "")
            self.git("reset", "--hard", identity["commit"])
            with patch.dict(os.environ, {"GITHUB_SHA": identity["commit"]}), patch.object(prepare, "atomic_push") as pushed:
                self.assertEqual(prepare.prepare(), identity)
                pushed.assert_not_called()

    def test_registry_conflict_stops_before_local_version_edits(self):
        with patch.object(release, "registry_version", return_value={"num": "0.1.1"}), patch.object(prepare, "candidate_commit") as candidate:
            with self.assertRaisesRegex(ValueError, "already exists on crates.io"):
                prepare.prepare()
            candidate.assert_not_called()
        self.assertEqual(self.remote_git("rev-parse", "main"), self.base)
        self.assertEqual((self.root / "Cargo.toml").read_text(), MANIFEST)

    def test_branch_race_cannot_leave_a_partial_tag(self):
        concurrent = self.remote_git(
            "-c", "user.name=Concurrent test", "-c", "user.email=concurrent@example.invalid",
            "commit-tree", self.git("rev-parse", "HEAD^{tree}"), "-p", self.base, "-m", "concurrent change"
        )
        original_run = subprocess.run

        def race(command, **kwargs):
            if list(command[:3]) == ["git", "push", "--atomic"]:
                self.remote_git("update-ref", "refs/heads/main", concurrent, self.base)
            return original_run(command, **kwargs)

        with patch.object(subprocess, "run", side_effect=race):
            with self.assertRaisesRegex(RuntimeError, "no retry or force push"):
                prepare.prepare()
        self.assertEqual(self.remote_git("rev-parse", "main"), concurrent)
        self.assertEqual(prepare.remote_refs("refs/tags/v0.1.1"), {})

    def test_tag_race_cannot_advance_main_or_overwrite_tag(self):
        original_run = subprocess.run

        def race(command, **kwargs):
            if list(command[:3]) == ["git", "push", "--atomic"]:
                self.remote_git("update-ref", "refs/tags/v0.1.1", self.base)
            return original_run(command, **kwargs)

        with patch.object(subprocess, "run", side_effect=race):
            with self.assertRaisesRegex(RuntimeError, "no retry or force push"):
                prepare.prepare()
        self.assertEqual(self.remote_git("rev-parse", "main"), self.base)
        self.assertEqual(self.remote_git("rev-parse", "v0.1.1"), self.base)

    def test_legacy_existing_tag_retry_does_not_bump_main(self):
        identity = prepare.prepare()
        self.git("reset", "--hard", self.base)
        with patch.dict(os.environ, {"RELEASE_MODE": "", "RELEASE_EXISTING_TAG": "v0.1.1"}), patch.object(prepare, "atomic_push") as pushed:
            self.assertEqual(prepare.prepare(), identity)
            pushed.assert_not_called()

    def test_explicit_request_ignores_dispatch_inputs_and_checks_expected_version(self):
        request = self.root / ".github/release-request.json"
        request.parent.mkdir()
        request.write_text('{"mode":"current","expected_version":"0.0.9"}')
        with patch.dict(os.environ, {"GITHUB_EVENT_NAME": "push", "RELEASE_MODE": "major"}):
            with self.assertRaisesRegex(ValueError, "stale release request"):
                prepare.selectors()
            request.write_text('{"mode":"current","expected_version":"0.1.0"}')
            self.assertEqual(prepare.selectors(), ("current", "", ""))


if __name__ == "__main__":
    unittest.main()
