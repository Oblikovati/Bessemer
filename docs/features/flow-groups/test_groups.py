"""Literal separate/interleaved layouts and temporary-root fixture regeneration."""
import gzip
import hashlib
import json
import unittest
from pathlib import Path
from tempfile import TemporaryDirectory
from freeze_groups import ROOT, cases, decoded_case, decoded_wire, freeze, group_wire, owned_version_cases


class GroupFixtureTests(unittest.TestCase):
    def test_literal_separate_and_interleaved_members_have_no_padding(self) -> None:
        members = [(0x11223344, 0x0102030405060708, 0xAA),
                   (0x55667788, 0x1112131415161718, 0xBB)]
        separate = bytes.fromhex("02000000443322118877665508070605040302011817161514131211aabb")
        interleaved = bytes.fromhex("02000000443322110807060504030201aa887766551817161514131211bb")
        self.assertEqual(group_wire(5, 0, members), separate)
        self.assertEqual(group_wire(6, 0, members), interleaved)
        self.assertEqual(len(separate), 4 + 2 * 13)
        self.assertEqual(group_wire(4, -0x80000000, []), bytes.fromhex("0000000000000080"))
        self.assertEqual(decoded_wire(3, []), bytes.fromhex("0300000000000000"))

    def test_all_owned_versions_keep_explicit_context_and_optional_wire_fields(self) -> None:
        for version in range(3, 7):
            profiles = owned_version_cases(version)
            self.assertEqual(len(profiles), 6)
            for case in profiles:
                decoded = decoded_case(case)
                self.assertEqual(decoded["version"], version)
                self.assertEqual(case.wire[:2], b"\x08\x00")
                for group in decoded["groups"]:
                    self.assertEqual("raw_auxiliary" in group, version == 4)
                    self.assertEqual(group["flags"] is None, version < 5)

    def test_regeneration_preserves_reviewed_packet_without_writing_its_root(self) -> None:
        reviewed = ROOT / "tests/fixtures/flow-groups/groups.bin.gz"
        before = reviewed.read_bytes()
        with TemporaryDirectory(prefix="bessemer-group-fixture-") as temporary:
            staging = Path(temporary)
            (staging / "tests/fixtures/flow-groups").mkdir(parents=True)
            (staging / "docs/features/flow-groups").mkdir(parents=True)
            freeze(staging)
            self.assertEqual((staging / "tests/fixtures/flow-groups/groups.bin.gz").read_bytes(), before)
            proof = json.loads((staging / "docs/features/flow-groups/fixture-verification.json").read_text())
        self.assertEqual(hashlib.sha256(gzip.decompress(before)).hexdigest(), proof["sha256"])
        self.assertEqual(len(cases()), 26)
        self.assertEqual(proof["unique_native_payloads"], 1)


if __name__ == "__main__":
    unittest.main()
