"""Independent code masks, ambiguous context, tagged extents and opaque payload gates."""
import gzip
import struct
import tempfile
import unittest
import zlib
from pathlib import Path
import freeze_bodies as baseline


class PointerBodyBaselineTests(unittest.TestCase):
    def test_regeneration_matches_complete_frozen_packet(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            baseline.freeze(root)
            relative = Path("tests/fixtures/flow-pointer-bodies/bodies.bin.gz")
            self.assertEqual(gzip.decompress((root / relative).read_bytes()),
                             gzip.decompress((baseline.ROOT / relative).read_bytes()))

    def test_all_256_current_codes_match_independent_native_masks(self) -> None:
        for code in range(256):
            offset = (code - 30) & 255
            expected = (offset < 55 and ((0x700001c0701c07 >> (offset & 63)) & 1))
            expected |= ((code + 171) & 255) < 3
            expected |= code < 63 and ((0x7000000000000088 >> code) & 1)
            expected |= code in (73, 77)
            self.assertEqual(code in baseline.flowpointerbodies.WIDE_CODES, bool(expected))

    def test_same_empty_wire_has_three_different_context_selected_modes(self) -> None:
        raw = bytes.fromhex("00000000 00")
        contexts = [((1, 0, 0), None, "single", 0), ((0, 0, 1), [3], "wide", 2), ((0, 0, 1), [0], "multiple", 2)]
        for counts, codes, mode, group in contexts:
            parsed = baseline.flowpointerbodies.read_decoded(raw, leaf_count=0, group_counts=counts, entry_codes=codes)
            entry = parsed["groups"][group]["entries"][0]
            self.assertEqual((entry["mode"], entry["start"], entry["end"]), (mode, 4, 5))
            self.assertFalse(parsed["native_runtime_state_reproduced"])

    def test_version_three_tag_two_zero_count_still_stores_one_wide_word(self) -> None:
        raw = bytes.fromhex("03000000 02 00000000 0000000000000080")
        parsed = baseline.flowpointerbodies.read_decoded(raw, leaf_count=0, group_counts=(1, 0, 0))
        entry = parsed["groups"][0]["entries"][0]
        self.assertEqual((entry["stored_count"], entry["wide_bits"], entry["end"]), (0, (0x8000000000000000,), 17))
        self.assertFalse(entry["packed_payload_decoded"])
        raw = bytes.fromhex("04000000 02 00000000")
        entry = baseline.flowpointerbodies.read_decoded(raw, leaf_count=0, group_counts=(1, 0, 0))["groups"][0]["entries"][0]
        self.assertEqual((entry["raw_payload"], entry["end"]), (b"", 9))
        self.assertNotIn("wide_bits", entry)

    def test_unknown_tags_consume_only_the_tag_and_do_not_invent_a_decode_flag(self) -> None:
        for version in (3, 4, 5):
            for tag in (0, 9, 255):
                raw = struct.pack("<iB", version, tag)
                entry = baseline.flowpointerbodies.read_decoded(raw, leaf_count=0, group_counts=(1, 0, 0))["groups"][0]["entries"][0]
                self.assertEqual((entry["tag_byte"], entry["start"], entry["end"]), (tag, 4, 5))
                self.assertNotIn("stored_count", entry)
                self.assertNotIn("packed_payload_decoded", entry)

    def test_packed_looking_raw_payload_remains_opaque(self) -> None:
        payload = zlib.compress(b"opaque fixture payload")
        raw = struct.pack("<iBi", 5, 1, len(payload)) + payload
        entry = baseline.flowpointerbodies.read_decoded(raw, leaf_count=0, group_counts=(1, 0, 0))["groups"][0]["entries"][0]
        self.assertEqual(entry["raw_payload"], payload)
        self.assertFalse(entry["packed_payload_decoded"])

    def test_context_presence_domains_and_unused_legacy_codes_are_explicit(self) -> None:
        raw = bytes.fromhex("00000000")
        for codes in (None, [], [0, 255]):
            parsed = baseline.flowpointerbodies.read_decoded(raw, leaf_count=0, group_counts=(0, 0, 0), entry_codes=codes)
            self.assertEqual(parsed["end"], 4)
        for codes in ([], [0]):
            with self.assertRaises(baseline.flowpointerbodies.ArchiveError):
                baseline.flowpointerbodies.read_decoded(struct.pack("<i", 3), leaf_count=0, group_counts=(0, 0, 0), entry_codes=codes)
        for codes in (None, [], [3, 0], [256], [True]):
            with self.assertRaises(baseline.flowpointerbodies.ArchiveError):
                baseline.flowpointerbodies.read_decoded(raw + b"\0", leaf_count=0, group_counts=(0, 0, 1), entry_codes=codes)

    def test_maximum_byte_counts_do_not_truncate_pool_or_vector_shapes(self) -> None:
        raw, leaves, counts, codes = baseline.profile(2, "maximum")
        parsed = baseline.flowpointerbodies.read_decoded(raw, leaf_count=leaves, group_counts=counts, entry_codes=codes)
        self.assertEqual((parsed["pools"][0]["count"], len(parsed["pools"][0]["wide_bits"])), (255, 765))
        third = parsed["groups"][2]["entries"]
        self.assertEqual((third[0]["count"], len(third[0]["wide_bits"])), (255, 765))
        self.assertEqual(len(third[1]["byte_vectors"]), 255)
        self.assertEqual([len(vector) for vector in third[1]["byte_vectors"][:2]], [255, 0])


if __name__ == "__main__":
    unittest.main()
