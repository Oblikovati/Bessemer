"""Independent boundary examples for the record-416 fixture baseline."""
import gzip
import struct
import tempfile
import unittest
from pathlib import Path
import freeze_groups as baseline


def parsed(version: int, name: str, mapping: bool = False, raw_count: int | None = None) -> dict:
    raw = baseline.profile(version, name, mapping, raw_count)
    return baseline.flowreferencegroups.read_decoded(raw, first_group_mapping_enabled=mapping)


class ReferenceGroupBaselineTests(unittest.TestCase):
    def test_complete_regeneration_matches_frozen_packet(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            baseline.freeze(root)
            relative = Path("tests/fixtures/flow-reference-groups/groups.bin.gz")
            self.assertEqual(gzip.decompress((root / relative).read_bytes()),
                             gzip.decompress((baseline.ROOT / relative).read_bytes()))

    def test_minimum_version_one_entry_has_seventeen_wire_bytes(self) -> None:
        raw = bytes.fromhex("01000000 00000080 01000000 00000000 00000080 00 00000000000000 00 00000000")
        result = baseline.flowreferencegroups.read_decoded(raw, first_group_mapping_enabled=False)
        entry = result["groups"][0]["entries"][0]
        self.assertEqual((entry["start"], entry["end"], result["end"]), (12, 29, 33))
        self.assertEqual((entry["reference_words"], entry["first_words"], entry["second_words"]), ((), (), ()))
        self.assertEqual(entry["descriptors"], (0,) * 7)
        self.assertEqual(result["groups"][1]["count"], 0)

    def test_all_256_version_one_raw_secondary_counts_preserve_signed_value(self) -> None:
        for byte in range(256):
            entry = parsed(1, "minimum", raw_count=byte)["groups"][0]["entries"][0]
            expected = byte if byte < 128 else byte - 256
            self.assertEqual(entry["second_count_raw_small"], byte)
            self.assertEqual(entry["second_count_i32"], expected)
            self.assertEqual(len(entry["second_words"]), max(expected, 0))

    def test_all_eighteen_version_gates_preserve_discarded_and_selector_fields(self) -> None:
        for version in range(18):
            result = parsed(version, "flags-noncanonical")
            entry = result["groups"][0]["entries"][0]
            self.assertEqual(result["discarded_header_word"], -(1 << 31) if version < 6 else None)
            self.assertEqual(result["stored_header_word"], 2 if version >= 3 else None)
            self.assertEqual(entry["member_20_i32"], -(1 << 31) if version < 12 else None)
            self.assertEqual(entry["discarded_prefix_byte"], 255 if 11 <= version <= 13 else None)
            self.assertEqual(entry["state_byte"], 255)
            self.assertEqual(entry["flag_2_byte"], 128 if version >= 5 else None)
            self.assertEqual(entry["flag_4_byte"], 255 if version == 17 else None)
            self.assertEqual(entry["discarded_suffix_word"], 0xf0aa if version >= 16 else None)

    def test_stored_header_never_replaces_the_fixed_two_group_loop(self) -> None:
        for header in (-(1 << 31), -999, 0, 1, 2, (1 << 31) - 1):
            raw = struct.pack("<4i", 14, header, 0, 0)
            result = baseline.flowreferencegroups.read_decoded(raw, first_group_mapping_enabled=True)
            self.assertEqual(result["stored_header_word"], header)
            self.assertEqual([group["count"] for group in result["groups"]], [0, 0])
            self.assertEqual(result["end"], 16)

    def test_mapping_presence_and_appended_wide_arrays_follow_independent_gates(self) -> None:
        for version in range(18):
            for mapping in (False, True):
                result = parsed(version, "rich", mapping)
                first, second = result["groups"]
                self.assertEqual(first["stored_refinement_level"], -17 if mapping else None)
                self.assertEqual(first["mapping_words"], (-1, (1 << 31) - 1) if mapping else ())
                self.assertIsNone(second["stored_refinement_level"])
                self.assertEqual(len(first["wide_bits"]), 3 if version >= 15 else 0)
                self.assertEqual(len(second["wide_bits"]), 3)
                self.assertFalse(result["reference_fixups_reproduced"])
                self.assertFalse(result["refinement_postprocessing_reproduced"])
                empty = parsed(version, "empty", mapping)
                self.assertTrue(all(group["stored_refinement_level"] is None for group in empty["groups"]))

    def test_negative_secondary_counts_consume_no_patterns_without_normalization(self) -> None:
        for version in (1, 10, 11, 12, 13, 14, 15, 16, 17):
            entry = parsed(version, "negative-secondary")["groups"][0]["entries"][0]
            self.assertEqual(entry["second_count_i32"], -128 if version == 1 else -(1 << 31))
            self.assertEqual(entry["second_words"], ())
            self.assertEqual(entry["reference_words"], (-1, 0, -(1 << 31), (1 << 31) - 1))

    def test_maximum_descriptor_and_unsigned_secondary_widths_are_not_truncated(self) -> None:
        for version in (0, 1, 9, 10, 17):
            entry = parsed(version, "maximum", True)["groups"][0]["entries"][0]
            self.assertEqual(entry["descriptors"][6], 255 if version == 1 else 65535)
            self.assertEqual(len(entry["first_words"]), 255 if version == 1 else 65535)
            self.assertEqual(len(entry["second_words"]), 127 if version == 1 else 65535)
            self.assertEqual(entry["first_words"][:4], baseline.WORDS[:4])


if __name__ == "__main__":
    unittest.main()
