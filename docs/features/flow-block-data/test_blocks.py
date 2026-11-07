"""Keep legacy block layout oracles reproducible before the typed implementation."""
import gzip
import struct
import tempfile
import unittest
import zlib
from pathlib import Path
import freeze_blocks as baseline
import process_reference


class BlockBaselineTests(unittest.TestCase):
    def test_process_oracle_requires_complete_expected_raw_carrier(self) -> None:
        raw = baseline.owned_case(14, "populated", False)["decoded"]
        wire = process_reference.carrier(raw)
        process_reference.check_wire(wire, raw, False)
        with self.assertRaises(AssertionError):
            process_reference.check_wire(wire[:-1] + b"x", raw, False)

    def test_process_oracle_checks_noncanonical_zlib_end_and_checksum(self) -> None:
        raw = baseline.owned_case(14, "populated", False)["decoded"]
        stored = zlib.compress(raw, 1)
        process_reference.check_wire(process_reference.carrier(stored), raw, True)
        for malformed in (stored[:-1], stored + b"x", stored + stored):
            with self.assertRaises(AssertionError):
                process_reference.check_wire(process_reference.carrier(malformed), raw, True)

    def test_process_edit_scope_is_explicit_for_native_and_owned_views(self) -> None:
        native = baseline.native_cases()[0]
        self.assertEqual(len(process_reference.requests(native)), 53)
        self.assertEqual(sum(len(process_reference.requests(case)) for case in baseline.owned_cases()), 4654)

    def test_regeneration_matches_owned_fixture(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            baseline.freeze(root)
            relative = Path("tests/fixtures/flow-block-data/blocks.bin.gz")
            self.assertEqual(gzip.decompress((root / relative).read_bytes()),
                             gzip.decompress((baseline.ROOT / relative).read_bytes()))

    def test_independent_v14_bytes_have_exact_unpadded_entry_order(self) -> None:
        raw = bytes.fromhex("0e000000 00000000 00000000 ab34120100 f9ffffff f3ffffff faf0 0100020003000400050006000100 efbeadde 01000000 efcdab89")
        parsed = baseline.flowblockdata.read_decoded(raw, leaf_count=1)
        entry = parsed["blocks"][0]["entries"][0]
        self.assertEqual((entry["start"], entry["end"], entry["member_28_i32"]), (17, 53, -13))
        self.assertEqual(entry["first_words"], (0xdeadbeef,))
        self.assertEqual(entry["second_words"], (0x89abcdef,))
        self.assertEqual(entry["descriptors"], (1, 2, 3, 4, 5, 6, 1))

    def test_every_version_has_explicit_signed_and_bitmap_boundaries(self) -> None:
        for version in range(1, 18):
            wrapped = baseline.owned_case(version, "wrap", False)
            parsed = baseline.flowblockdata.read_decoded(wrapped["decoded"], leaf_count=0)
            self.assertEqual(parsed["g_total_mathematical"], 2147483648)
            self.assertEqual(parsed["g_total_native_i32"], -2147483648)
            self.assertEqual(parsed["bitmap_bytes"], b"")
            negative = baseline.owned_case(version, "negative-local", False)
            blocks = baseline.flowblockdata.read_decoded(negative["decoded"], leaf_count=3)["blocks"]
            self.assertEqual([b["count_12_i16"] for b in blocks], [-32768, -1, 0])
            self.assertTrue(all(not b["entries"] for b in blocks))

    def test_exact_payload_end_does_not_infer_leaf_count_from_global_totals(self) -> None:
        for version in range(1, 18):
            case = baseline.owned_case(version, "populated", False)
            parsed = baseline.flowblockdata.read_decoded(case["decoded"], leaf_count=2)
            self.assertEqual(parsed["entry_count"], 3)
            self.assertFalse(parsed["entry_total_equals_g_total_observed"])
            self.assertFalse(parsed["block_to_tree_order_assigned"])
            self.assertFalse(parsed["native_433_context_proven"])
            self.assertEqual(parsed["end"], len(case["decoded"]))


if __name__ == "__main__":
    unittest.main()
