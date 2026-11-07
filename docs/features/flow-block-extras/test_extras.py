"""Independent flag gates, width transitions and explicit runtime context."""
import gzip
import struct
import tempfile
import unittest
from pathlib import Path
import freeze_extras as baseline


class ExtraBaselineTests(unittest.TestCase):
    def test_regeneration_matches_complete_frozen_packet(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            baseline.freeze(root)
            relative = Path("tests/fixtures/flow-block-extras/extras.bin.gz")
            self.assertEqual(gzip.decompress((root / relative).read_bytes()),
                             gzip.decompress((baseline.ROOT / relative).read_bytes()))

    def test_minimal_fixed_hex_has_no_implicit_nested_or_final_flags(self) -> None:
        raw = bytes.fromhex("00000000 ffffffff 0000000002")
        parsed = baseline.flowblockextras.read_decoded(raw, leaf_count=1, enabled=True)
        body = parsed["blocks"][0]["body"]
        self.assertEqual((body["start"], body["end"], body["outer_presence_byte"]), (8, 13, 2))
        for key in ("nested_presence_byte", "nested", "final_presence_byte", "final", "extra_presence_byte", "extra", "last_byte"):
            self.assertIsNone(body[key])

    def test_mixed_parallel_and_interleaved_layouts_have_fixed_hex_oracles(self) -> None:
        parallel = bytes.fromhex("02000000 00000080 00000000 0000000000000080 341200000000f87f")
        interleaved = bytes.fromhex("02000000 00000080 0000000000000080 00000000 341200000000f87f")
        self.assertEqual(baseline.mixed_wire(7, 2), parallel)
        self.assertEqual(baseline.mixed_wire(8, 2), interleaved)

    def test_exact_one_and_nonzero_flags_remain_distinct(self) -> None:
        raw, leaves = baseline.owned_case(7, "flags-two")
        parsed = baseline.flowblockextras.read_decoded(raw, leaf_count=leaves, enabled=True)
        body = parsed["blocks"][0]["body"]
        self.assertEqual((body["outer_presence_byte"], body["nested_presence_byte"], body["final_presence_byte"]), (1, 2, 2))
        self.assertIsNone(body["nested"])
        self.assertIsNone(body["final"])
        self.assertIsNotNone(body["extra"])
        self.assertEqual(len(body["first"][0]["additional_i32"]), 6)
        self.assertEqual(body["first"][0]["additional_presence_byte"], 2)

    def test_byte_array_absence_skips_the_additional_presence_field(self) -> None:
        for version in (6, 7, 8):
            raw, leaves = baseline.owned_case(version, "bytes-empty")
            item = baseline.flowblockextras.read_decoded(raw, leaf_count=leaves, enabled=True)["blocks"][0]["body"]["first"][0]
            self.assertEqual(item["byte_array"], b"")
            self.assertIsNone(item["additional_presence_byte"])
            self.assertEqual(item["additional_i32"], ())

    def test_nested_count_width_and_stored_indices_have_separate_version_gates(self) -> None:
        for version, count in ((4, 255), (5, 300), (6, 300), (8, 300)):
            raw, leaves = baseline.owned_case(version, "nested-max")
            row = baseline.flowblockextras.read_decoded(raw, leaf_count=leaves, enabled=True)["blocks"][0]["body"]["nested"][0]
            self.assertEqual(len(row), count)
            self.assertEqual(row[-1]["native_index"], count - 1 if version < 6 else -2147483648 + count - 1)
            self.assertEqual(row[-1]["stored_index"], None if version < 6 else -2147483648 + count - 1)

    def test_enabled_and_leaf_count_context_are_not_inferred_from_payload(self) -> None:
        raw, leaves = baseline.owned_case(4, "rich")
        for enabled in (False, None, 1):
            with self.assertRaises(baseline.flowblockextras.ArchiveError):
                baseline.flowblockextras.read_decoded(raw, leaf_count=leaves, enabled=enabled)
        for count in (-1, True, 0, 1, 3, 2147483648):
            with self.assertRaises(baseline.flowblockextras.ArchiveError):
                baseline.flowblockextras.read_decoded(raw, leaf_count=count, enabled=True)
        self.assertEqual(baseline.flowblockextras.read_decoded(struct.pack("<i", 8), leaf_count=0, enabled=True)["blocks"], [])

    def test_all_four_byte_counted_lists_accept_255_elements(self) -> None:
        for version in (0, 4, 8):
            raw, leaves = baseline.owned_case(version, "list-max")
            body = baseline.flowblockextras.read_decoded(raw, leaf_count=leaves, enabled=True)["blocks"][0]["body"]
            self.assertEqual([len(body[key]) for key in ("first", "second", "third")], [255, 255, 255])
            self.assertEqual((body["fourth_count"], len(body["fourth_wide_bits"]), len(body["fourth_bytes"])), (255, 765, 255))


if __name__ == "__main__":
    unittest.main()
