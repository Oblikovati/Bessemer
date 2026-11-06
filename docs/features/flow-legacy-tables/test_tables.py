"""Independent legacy version gates, context admission and bit-preserving wire expectations."""
import gzip
import struct
import tempfile
import unittest
from pathlib import Path
import freeze_tables as baseline


class LegacyTableBaselineTests(unittest.TestCase):
    def test_regeneration_matches_owned_fixture(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            baseline.freeze(root)
            relative = Path("tests/fixtures/flow-legacy-tables/tables.bin.gz")
            self.assertEqual(gzip.decompress((root / relative).read_bytes()),
                             gzip.decompress((baseline.ROOT / relative).read_bytes()))

    def test_version_one_keeps_discarded_count_words_even_for_empty_groups(self) -> None:
        raw = struct.pack("<5i", 1, 0, -17, 0, -18)
        parsed = baseline.flowlegacytables.read_decoded_401(raw, global_pointer_count=0)
        self.assertEqual([g["discarded_count_word"] for g in parsed["groups"]], [-17, -18])
        self.assertEqual([g["end"] - g["start"] for g in parsed["groups"]], [8, 8])
        self.assertTrue(parsed["global_pointer_indices_checked"])
        self.assertFalse(parsed["native_pointer_population_reproduced"])

    def test_zero_arrays_and_duplicate_pairs_are_preserved_in_populated_groups(self) -> None:
        for version in range(4):
            raw = struct.pack("<i", version) + b"".join(baseline.group_wire(version, i, "zero-arrays") for i in range(2))
            parsed = baseline.flowlegacytables.read_decoded_401(raw)
            self.assertTrue(all(g["count"] == 1 and g["array_lengths"] == (0, 0, 0) for g in parsed["groups"]))
            raw = struct.pack("<i", version) + b"".join(baseline.group_wire(version, i, "duplicates") for i in range(2))
            parsed = baseline.flowlegacytables.read_decoded_401(raw, global_pointer_count=8)
            self.assertEqual(parsed["groups"][0]["pairs"], [(0, 7), (0, 7), (7, 0)])

    def test_unbounded_signed_pairs_do_not_become_valid_bounded_pointer_indices(self) -> None:
        raw = struct.pack("<i", 3) + b"".join(baseline.group_wire(3, i, "negative-pairs") for i in range(2))
        parsed = baseline.flowlegacytables.read_decoded_401(raw)
        self.assertEqual(parsed["groups"][0]["pairs"][0], (-1, -2147483648))
        self.assertFalse(parsed["global_pointer_indices_checked"])
        with self.assertRaises(baseline.flowlegacytables.ArchiveError):
            baseline.flowlegacytables.read_decoded_401(raw, global_pointer_count=2147483647)

    def test_word_count_context_and_unsigned_bits_have_independent_fixed_hex_oracle(self) -> None:
        raw = bytes.fromhex("00000000 00000000 ffffffff 00000080")
        parsed = baseline.flowlegacytables.read_decoded_405(raw, legacy_counts=(1, 2))
        self.assertEqual(parsed["raw_words"], (0, 0xffffffff, 0x80000000))
        self.assertEqual(parsed["words_start"], 4)
        self.assertFalse(parsed["count_stored"])
        for context in (None, (1,), (-1, 2), (2147483647, 1)):
            with self.assertRaises(baseline.flowlegacytables.ArchiveError):
                baseline.flowlegacytables.read_decoded_405(raw, legacy_counts=context)
        with self.assertRaises(baseline.flowlegacytables.ArchiveError):
            baseline.flowlegacytables.read_decoded_405(struct.pack("<2i", 1, 0), legacy_counts=(0, 0))


if __name__ == "__main__":
    unittest.main()
