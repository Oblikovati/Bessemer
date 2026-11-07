"""Saved-index preservation is distinct from complete graph validity (#230)."""
from pathlib import Path
import gzip
import tempfile
import unittest
from index_reference import inspect_default_file, read_index
from saved_index_fixtures import mapping_locations, saved_cases, saved_variants
from freeze_saved_indexes import freeze_saved, saved_packet


class SavedIndexFixtureTests(unittest.TestCase):
    def test_frozen_requests_and_complete_results_match_two_independent_writes(self) -> None:
        root = Path(__file__).resolve().parents[3] / 'tests/fixtures'
        raw, count = saved_packet(root / 'database-overflow')
        self.assertEqual(count, 34)
        self.assertEqual(gzip.decompress((root / 'database-index/saved-indexes.bin.gz').read_bytes()), raw)
        with tempfile.TemporaryDirectory() as temporary:
            output = Path(temporary)
            freeze_saved(root / 'database-overflow', output)
            for name in ('saved-indexes.bin.gz', 'saved-indexes-integrity.json'):
                self.assertEqual((output / name).read_bytes(), (root / 'database-index' / name).read_bytes())

    def test_duplicates_extreme_words_and_empty_maps_remain_lossless_without_graph_acceptance(self) -> None:
        variants = saved_variants()
        duplicate = read_index(variants[1], 4096)
        self.assertEqual(duplicate.entries, ((0, 2), (0, 2)))
        extreme = read_index(variants[2], 4096)
        self.assertEqual((extreme.free_head, extreme.split, extreme.maximum),
                         ((1 << 64) - 1, (1 << 64) - 1, 0))
        self.assertEqual(read_index(variants[3], 4096).entries, ())
        for raw in variants[1:]:
            with self.assertRaises(AssertionError):
                inspect_default_file(raw)

    def test_every_saved_mutation_changes_only_its_literal_word_span_and_reverses(self) -> None:
        root = Path(__file__).resolve().parents[3] / 'tests/fixtures/database-overflow'
        for raw, changes in saved_cases(root):
            self.assertEqual(len(mapping_locations(raw)), len(read_index(raw, 4096).entries))
            for change in changes:
                start, end = change.span
                self.assertEqual(change.expected[:start], raw[:start])
                self.assertEqual(change.expected[end:], raw[end:])
                result = bytearray(change.expected)
                result[start:end] = raw[start:end]
                self.assertEqual(bytes(result), raw)
                self.assertEqual(len(read_index(change.expected, 4096).entries),
                                 len(read_index(raw, 4096).entries))


if __name__ == '__main__':
    unittest.main()
