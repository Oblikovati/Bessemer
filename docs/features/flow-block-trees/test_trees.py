"""Literal full-tree invariants, depth-wrap and huge inactive-grid boundaries."""
import gzip
import hashlib
import json
import unittest
from pathlib import Path
from tempfile import TemporaryDirectory
from freeze_trees import ROOT, UNIT, FULL, owned_decoded, native_cases, freeze, header, flowblocktrees
from swarchive import ArchiveError


class TreeFixtures(unittest.TestCase):
    def test_native_full_eight_child_forest(self) -> None:
        native, audit = native_cases()
        self.assertEqual([row["present"] for row in audit], [False, True, False])
        parsed = flowblocktrees.read_decoded(native[0]["decoded"])
        stats = parsed["stats"]
        self.assertEqual((parsed["grid_count"], parsed["present_roots"]), (792, 792))
        self.assertEqual((stats["wire_nodes"], stats["wire_splits"], stats["wire_leaves"]), (4440, 456, 3984))
        self.assertEqual(stats["wire_nodes"], parsed["present_roots"] + 8 * stats["wire_splits"])
        self.assertEqual(stats["wire_leaves"], parsed["present_roots"] + 7 * stats["wire_splits"])

    def test_depth_256_native_level_wrap_and_prefix_is_not_presence(self) -> None:
        case = next(case for case in owned_decoded() if case[0] == "depth256" and case[2] is None)
        parsed = flowblocktrees.read_decoded(case[1], max_depth=256)
        self.assertEqual(parsed["stats"]["max_wire_depth"], 256)
        node = parsed["roots"][0]["tree"]
        for _ in range(256):
            self.assertEqual(node["children"][0]["discarded_prefix_byte"], 0)
            node = node["children"][0]["node"]
        self.assertEqual((node["depth"], node["native_level_byte"]), (256, 0))
        with self.assertRaises(ArchiveError):
            flowblocktrees.read_decoded(case[1], max_depth=255)

    def test_inactive_u128_grid_and_active_delegation_are_distinct(self) -> None:
        parsed = flowblocktrees.read_decoded(header(4, FULL) + b"\0")
        self.assertEqual(parsed["grid_count"], 1 << 96)
        self.assertIsNone(parsed["roots"])
        with self.assertRaises(ArchiveError):
            flowblocktrees.read_decoded(header(4, UNIT) + b"\1")
        with self.assertRaises(ArchiveError):
            flowblocktrees.read_decoded(header(3, UNIT), legacy_bounds=UNIT)

    def test_temporary_regeneration_preserves_frozen_case_packet(self) -> None:
        original = (ROOT / "tests/fixtures/flow-block-trees/trees.bin.gz").read_bytes()
        with TemporaryDirectory(prefix="bessemer-trees-") as temporary:
            staging = Path(temporary)
            freeze(staging)
            self.assertEqual((staging / "tests/fixtures/flow-block-trees/trees.bin.gz").read_bytes(), original)
            report = json.loads((staging / "docs/features/flow-block-trees/fixture-verification.json").read_text())
        self.assertEqual(report["cases"], 81)
        self.assertEqual(hashlib.sha256(gzip.decompress(original)).hexdigest(), report["sha256"])


if __name__ == "__main__":
    unittest.main()
