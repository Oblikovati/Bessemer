"""Independent presence, count origin, inactive layout and raw-pattern gates."""
import gzip
import struct
import tempfile
import unittest
from pathlib import Path
import freeze_tables as baseline


class SimpleTableBaselineTests(unittest.TestCase):
    def test_regeneration_matches_complete_wire_baseline(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            baseline.freeze(root)
            relative = Path("tests/fixtures/flow-simple-tables/tables.bin.gz")
            self.assertEqual(gzip.decompress((root / relative).read_bytes()),
                             gzip.decompress((baseline.ROOT / relative).read_bytes()))

    def test_parallel_declared_counts_do_not_create_absent_arrays(self) -> None:
        raw = bytes.fromhex("00000000 ffffff7f 07000000 00000000 00000000")
        parsed = baseline.flowsimpletables.read_decoded(raw, identifier=407)
        self.assertEqual([group["count"] for group in parsed["groups"]], [2147483647, 7])
        self.assertEqual([group["arrays"] for group in parsed["groups"]], [[], []])
        raw = bytes.fromhex("00000000 00000000 00000000 ffffffff 02000000")
        parsed = baseline.flowsimpletables.read_decoded(raw, identifier=407)
        self.assertEqual([len(group["arrays"]) for group in parsed["groups"]], [4, 4])
        self.assertTrue(all(array["raw_bits"] == () for group in parsed["groups"] for array in group["arrays"]))

    def test_leaf_pair_fixed_hex_preserves_nonfinite_patterns_and_requires_context(self) -> None:
        raw = bytes.fromhex("00000000 0000000000000080 341200000000f87f")
        parsed = baseline.flowsimpletables.read_decoded(raw, identifier=437, leaf_count=1)
        self.assertEqual(parsed["rows"][0]["raw_bits"], (0x8000000000000000, 0x7ff8000000001234))
        self.assertEqual((parsed["rows"][0]["start"], parsed["rows"][0]["end"]), (4, 20))
        self.assertFalse(parsed["block_to_tree_order_assigned"])
        for count in (None, False, -1, 0, 2, 2147483648):
            with self.assertRaises(baseline.flowsimpletables.ArchiveError):
                baseline.flowsimpletables.read_decoded(raw, identifier=437, leaf_count=count)

    def test_inactive_five_word_versions_keep_optional_external_count_without_rows(self) -> None:
        raw = bytes.fromhex("01000000")
        for context in (None, 0, 1, 2147483647):
            parsed = baseline.flowsimpletables.read_decoded(raw, identifier=411, legacy_count=context)
            self.assertEqual(parsed["count"], context)
            self.assertFalse(parsed["count_stored"])
            self.assertEqual((parsed["rows"], parsed["words_start"], parsed["end"]), ([], 4, 4))
            self.assertFalse(parsed["wire_rows_present"])

    def test_five_raw_words_and_byte_values_have_fixed_hex_oracles(self) -> None:
        raw = bytes.fromhex("04000000 01000000 00000000 ffffffff 00000080 3412c07f 09000000")
        parsed = baseline.flowsimpletables.read_decoded(raw, identifier=411)
        self.assertEqual(parsed["rows"], [(0, 0xffffffff, 0x80000000, 0x7fc01234, 9)])
        for version in (0, 1):
            raw = struct.pack("<i", version) + (struct.pack("<i", 3) if version else b"") + b"\0\xff\x80"
            parsed = baseline.flowsimpletables.read_decoded(raw, identifier=413, legacy_count=3 if version == 0 else None)
            self.assertEqual(parsed["raw_bytes"], b"\0\xff\x80")
            self.assertEqual(parsed["count_stored"], version == 1)

    def test_unused_contexts_and_missing_required_counts_reject(self) -> None:
        cases = [(407, struct.pack("<3i", 1, 0, 0), dict(leaf_count=0)),
                 (407, struct.pack("<3i", 1, 0, 0), dict(legacy_count=0)),
                 (437, struct.pack("<i", 0), dict(leaf_count=0, legacy_count=0)),
                 (411, struct.pack("<i", 3), {}), (411, struct.pack("<2i", 4, 0), dict(legacy_count=0)),
                 (411, struct.pack("<i", 0), dict(leaf_count=0)), (413, struct.pack("<i", 0), {}),
                 (413, struct.pack("<2i", 1, 0), dict(legacy_count=0))]
        for identifier, raw, context in cases:
            with self.assertRaises(baseline.flowsimpletables.ArchiveError):
                baseline.flowsimpletables.read_decoded(raw, identifier=identifier, **context)

    def test_later_empty_layouts_are_complete_but_populated_restores_remain_unsupported(self) -> None:
        for identifier, version in ((407, 1), (407, 2147483647), (411, 5), (411, 2147483647), (413, 2), (413, 2147483647)):
            raw = struct.pack("<3i", version, 0, 0) if identifier == 407 else struct.pack("<2i", version, 0)
            parsed = baseline.flowsimpletables.read_decoded(raw, identifier=identifier)
            self.assertEqual(parsed["end"], len(raw))
            self.assertFalse(parsed["native_runtime_state_reproduced"])
            malformed = raw[:4] + struct.pack("<i", 1) + raw[8:]
            with self.assertRaises(baseline.flowsimpletables.ArchiveError):
                baseline.flowsimpletables.read_decoded(malformed, identifier=identifier)


if __name__ == "__main__":
    unittest.main()
