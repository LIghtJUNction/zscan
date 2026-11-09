"""Release failure tests; no network request or real upload is made."""

from contextlib import redirect_stdout
import io
import json
import os
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch
from urllib.error import URLError

import audit
import release


class ContractTests(unittest.TestCase):
    def test_dependency_kinds_are_all_rejected(self):
        metadata = {
            "packages": [{"name": "zscan", "dependencies": [], "targets": [{"kind": ["lib"]}]}],
            "workspace_members": ["zscan"],
        }
        audit.check_metadata(metadata)
        for kind in (None, "dev", "build"):
            for target in (None, "cfg(target_arch = \"aarch64\")"):
                metadata["packages"][0]["dependencies"] = [
                    {"name": "external", "kind": kind, "target": target, "optional": True}
                ]
                with self.assertRaises(ValueError):
                    audit.check_metadata(metadata)

    def test_packaging_excludes_comparison_and_build_scripts(self):
        required = ["Cargo.toml", "Cargo.lock", "src/lib.rs", "README.md"]
        audit.check_package_list(required)
        for path in ("comparison/Cargo.toml", "comparison/src/main.rs", "build.rs"):
            with self.assertRaises(ValueError):
                audit.check_package_list(required + [path])

    def test_tags_cannot_inject_options_or_select_non_release_refs(self):
        for value in ("main", "v01.2.3", "v1.2", "v1.2.3-rc1", "v1.2.3\n", "--all", "v1.2.3;id"):
            with self.subTest(value=value), self.assertRaises(ValueError):
                release.version_from_tag(value)
        self.assertEqual(release.version_from_tag("v0.1.0"), "0.1.0")

    def test_tag_version_must_match_tagged_manifest(self):
        with patch.object(release, "git", side_effect=["a" * 40, '[package]\nname="zscan"\nversion="0.2.0"']):
            with self.assertRaises(ValueError):
                release.resolve_tag("v0.1.0")

    def test_release_toolchain_checks_actual_binaries(self):
        outputs = ["rustc 1.99.1\nrelease: 1.99.1", "cargo 1.99.1\nrelease: 1.99.1"]
        with patch.dict(os.environ, {"RUSTUP_TOOLCHAIN": "1.99.1"}):
            with patch.object(release.subprocess, "check_output", side_effect=outputs):
                info = release.toolchain_information()
                self.assertEqual(info["requested"], "1.99.1")
                self.assertIn("release: 1.99.1", info["rustc_verbose"])
                self.assertIn("release: 1.99.1", info["cargo_verbose"])
            with patch.object(release.subprocess, "check_output", return_value="rustc 1.100.0\nrelease: 1.100.0"):
                with self.assertRaisesRegex(ValueError, "does not match"):
                    release.toolchain_information()
        with patch.dict(os.environ, {"RUSTUP_TOOLCHAIN": "stable"}):
            with self.assertRaisesRegex(ValueError, "requires RUSTUP_TOOLCHAIN"):
                release.toolchain_information()

    def test_manual_resolution_reads_toolchain_from_tagged_source(self):
        values = ["a" * 40, '[package]\nname="zscan"\nversion="0.1.0"', "1.99.1"]
        with patch.object(release, "git", side_effect=values):
            self.assertEqual(release.resolve_tag("v0.1.0")["toolchain"], "1.99.1")
        values[-1] = "stable"
        with patch.object(release, "git", side_effect=values):
            with self.assertRaisesRegex(ValueError, "fixed stable"):
                release.resolve_tag("v0.1.0")

    def test_manual_release_checks_tagged_commit_not_dispatch_branch(self):
        identity = {"tag": "v0.1.0", "version": "0.1.0", "commit": "a" * 40, "toolchain": "1.99.1"}
        env = {"RELEASE_TAG": "v0.1.0", "RELEASE_VERSION": "0.1.0", "RELEASE_COMMIT": "a" * 40, "RUSTUP_TOOLCHAIN": "1.99.1"}
        tag_refs = "b" * 40 + "\trefs/tags/v0.1.0\n" + "a" * 40 + "\trefs/tags/v0.1.0^{}"
        with patch.dict(os.environ, env), patch.object(release, "resolve_tag", return_value=identity):
            with patch.object(release, "git", side_effect=["a" * 40, "", tag_refs]):
                self.assertEqual(release.checked_identity(), identity)
            with patch.object(release, "git", return_value="c" * 40):
                with self.assertRaisesRegex(ValueError, "not the verified release commit"):
                    release.checked_identity()
            with patch.object(release, "git", side_effect=["a" * 40, " M src/lib.rs"]):
                with self.assertRaisesRegex(ValueError, "tracked files changed"):
                    release.checked_identity()
            with patch.object(release, "git", side_effect=["a" * 40, "", "c" * 40 + "\trefs/tags/v0.1.0"]):
                with self.assertRaisesRegex(ValueError, "remote tag moved"):
                    release.checked_identity()


class PublishTests(unittest.TestCase):
    identity = {"tag": "v0.1.0", "version": "0.1.0", "commit": "a" * 40, "toolchain": "1.99.1"}
    checksum = "b" * 64

    def setUp(self):
        directory = tempfile.TemporaryDirectory()
        self.addCleanup(directory.cleanup)
        self.evidence = Path(directory.name) / "evidence.json"
        self.enterContext(redirect_stdout(io.StringIO()))
        for patcher in (
            patch.dict(os.environ, {"CARGO_REGISTRY_TOKEN": "test-not-a-real-token"}),
            patch.object(release, "checked_identity", return_value=self.identity),
            patch.object(release, "package_checksum", return_value=self.checksum),
            patch.object(release, "toolchain_information", return_value={
                "requested": "1.99.1", "rustc_verbose": "rustc test", "cargo_verbose": "cargo test"
            }),
            patch.object(release, "EVIDENCE", self.evidence),
        ):
            self.enterContext(patcher)

    def record(self, checksum=None):
        return {"crate": "zscan", "num": "0.1.0", "checksum": checksum or self.checksum, "yanked": False}

    def test_missing_secret_stops_before_upload(self):
        with patch.dict(os.environ, {"CARGO_REGISTRY_TOKEN": ""}), patch.object(release.subprocess, "run") as upload:
            with self.assertRaisesRegex(ValueError, "missing CARGO_REGISTRY_TOKEN"):
                release.publish()
            upload.assert_not_called()

    def test_existing_identical_version_recovers_without_upload(self):
        with patch.object(release, "registry_version", return_value=self.record()), patch.object(release.subprocess, "run") as upload:
            release.publish()
            upload.assert_not_called()
        self.assertEqual(json.loads(self.evidence.read_text())["outcome"], "already-published-identical")
        self.assertEqual(json.loads(self.evidence.read_text())["verification_toolchain"]["requested"], "1.99.1")

    def test_different_existing_archive_stops(self):
        with patch.object(release, "registry_version", return_value=self.record("c" * 64)), patch.object(release.subprocess, "run") as upload:
            with self.assertRaisesRegex(ValueError, "different bytes"):
                release.publish()
            upload.assert_not_called()
        self.assertFalse(self.evidence.exists())

    def test_registry_outage_is_not_interpreted_as_missing_version(self):
        with patch.object(release, "registry_version", side_effect=URLError("offline")), patch.object(release.subprocess, "run") as upload:
            with self.assertRaises(URLError):
                release.publish()
            upload.assert_not_called()

    def test_post_upload_error_can_be_confirmed_without_second_upload(self):
        result = subprocess.CompletedProcess(["cargo", "publish", "--locked"], 101)
        with patch.object(release, "registry_version", side_effect=[None, self.record()]), patch.object(release.subprocess, "run", return_value=result) as upload:
            release.publish()
            upload.assert_called_once_with(["cargo", "publish", "--locked"], cwd=release.ROOT, check=False)
        self.assertEqual(json.loads(self.evidence.read_text())["outcome"], "published-confirmed")

    def test_unconfirmed_upload_never_retries(self):
        result = subprocess.CompletedProcess(["cargo", "publish", "--locked"], 101)
        with patch.object(release, "registry_version", return_value=None), patch.object(release.subprocess, "run", return_value=result) as upload, patch.object(release.time, "monotonic", side_effect=[0, 121]):
            with self.assertRaisesRegex(RuntimeError, "No second upload"):
                release.publish()
            self.assertEqual(upload.call_count, 1)
        self.assertFalse(self.evidence.exists())


if __name__ == "__main__":
    unittest.main()
