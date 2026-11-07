"""Independent sentinel, code conversion and conditional-tail gates."""
import gzip
import struct
import tempfile
import unittest
from pathlib import Path
import freeze_links as baseline


class LinkBaselineTests(unittest.TestCase):
    def test_regeneration_matches_complete_owned_packet(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            baseline.freeze(root)
            relative = Path("tests/fixtures/flow-link-records/links.bin.gz")
            self.assertEqual(gzip.decompress((root / relative).read_bytes()),
                             gzip.decompress((baseline.ROOT / relative).read_bytes()))

    def test_all_byte_values_use_an_independent_conversion_table(self) -> None:
        expected = list(range(256))
        for start in (30, 40, 50, 60):
            expected[start:start + 6] = [start, start, start + 1, start + 1, start + 2, start + 2]
        for code in range(256):
            raw = struct.pack("<2i", 0, 1) + baseline.entry_wire(0, "full", code) + baseline.tail_wire(0, "full")
            entry = baseline.flowlinkrecords.read_decoded(raw)["entries"][0]
            self.assertEqual((entry["code_byte"], entry["native_code_byte"]), (code, expected[code]))

    def test_negative_one_sentinel_remains_legal_with_zero_bound(self) -> None:
        raw = struct.pack("<2i", 4, 1) + baseline.entry_wire(0, "sentinels", 255) + baseline.tail_wire(4, "full")
        parsed = baseline.flowlinkrecords.read_decoded(raw, global_pointer_count=0)
        self.assertEqual(parsed["entries"][0]["reference_indices"], (-1, -1))
        self.assertTrue(parsed["global_pointer_indices_checked"])
        invalid = raw[:8] + struct.pack("<i", -2) + raw[12:]
        with self.assertRaises(baseline.flowlinkrecords.ArchiveError):
            baseline.flowlinkrecords.read_decoded(invalid, global_pointer_count=0)
        self.assertFalse(baseline.flowlinkrecords.read_decoded(invalid)["global_pointer_indices_checked"])

    def test_control_is_strictly_positive_and_member_length_is_signed(self) -> None:
        for profile, expected in (("full", (-2147483648, 0, 2147483647)), ("control-zero", ()),
                                  ("control-negative", ()), ("negative-length", ())):
            raw = struct.pack("<2i", 5, 0) + baseline.tail_wire(5, profile)
            parsed = baseline.flowlinkrecords.read_decoded(raw)
            self.assertEqual(parsed["conditional_i32"], expected)
            self.assertFalse(parsed["native_runtime_state_reproduced"])

    def test_empty_fixed_hex_and_versioned_numeric_indices(self) -> None:
        raw = bytes.fromhex("00000000 00000000") + bytes(44)
        parsed = baseline.flowlinkrecords.read_decoded(raw)
        self.assertEqual((parsed["count"], parsed["tail_start"], parsed["end"]), (0, 8, 52))
        for version in range(6, 10):
            raw = struct.pack("<2i", version, 0) + baseline.tail_wire(version, "full")
            parsed = baseline.flowlinkrecords.read_decoded(raw)
            tail = parsed["numeric_tail"]
            self.assertEqual(tail["runtime_default_index"], -1 if version < 8 else None)
            self.assertEqual(tail["stored_indices"], None if version < 8 else (-1, -2147483648, 2147483647))
            if version >= 7:
                self.assertEqual(parsed["final_tail"]["element_bytes"], 8 if version == 9 else 4)


if __name__ == "__main__":
    unittest.main()
