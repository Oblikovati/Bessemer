"""Complete file edits preserve every unrelated role and reverse exactly (#230)."""
import gzip
from pathlib import Path
import tempfile
import unittest
from freeze_file_fixtures import file_packet, freeze_packet
from file_fixtures import complete_file, expected_edit
from index_reference import inspect_default_file, read_index
from bucket_fixtures import native_bucket_bytes


class CompleteFileFixtureTests(unittest.TestCase):
    def test_frozen_cross_language_packet_matches_generator_and_two_independent_writes(self) -> None:
        root = Path(__file__).resolve().parents[3] / 'tests/fixtures/database-index'
        self.assertEqual(gzip.decompress((root / 'complete-file.bin.gz').read_bytes()), file_packet())
        with tempfile.TemporaryDirectory() as temporary:
            first, second = Path(temporary) / 'first', Path(temporary) / 'second'
            freeze_packet(first)
            freeze_packet(second)
            for name in ('complete-file.bin.gz', 'complete-file-integrity.json'):
                self.assertEqual((first / name).read_bytes(), (second / name).read_bytes())
                self.assertEqual((first / name).read_bytes(), (root / name).read_bytes())

    def test_complete_owned_file_includes_continuation_buckets_free_chain_and_overflow(self) -> None:
        raw, cells = complete_file()
        self.assertEqual(len(cells), 4)
        self.assertEqual(inspect_default_file(raw), {'pages': 10, 'index_pages': 2,
                         'buckets': 3, 'cells': 4, 'free_pages': 2, 'overflow_pages': 2})
        self.assertEqual(read_index(raw, 4096).entries, ((0, 2), (1, 5)))

    def test_each_independent_whole_file_edit_reparses_and_reverses_byte_for_byte(self) -> None:
        original, cells = complete_file()
        for cell in cells:
            with self.subTest(bucket=cell.bucket, cell=cell.cell):
                replacement = bytes(byte ^ 0xa5 for byte in cell.value)
                result = expected_edit(original, cell, replacement)
                self.assertEqual(inspect_default_file(result), inspect_default_file(original))
                bucket = native_bucket_bytes(result, cell.bucket)
                selected = next(candidate for candidate in bucket.cells if candidate.offset == cell.cell)
                self.assertEqual((selected.key, selected.value), (cell.key, replacement))
                self.assertEqual(expected_edit(result, cell, cell.value), original)
                self.assertEqual(expected_edit(original, cell, cell.value), original)

    def test_declared_spans_are_the_only_changed_bytes_including_empty_value_noop(self) -> None:
        original, cells = complete_file()
        for cell in cells:
            result = expected_edit(original, cell, bytes(byte ^ 0xff for byte in cell.value))
            expected = {offset for start, end in cell.spans for offset in range(start, end)}
            actual = {offset for offset, (before, after) in enumerate(zip(original, result)) if before != after}
            self.assertEqual(actual, expected)
            self.assertEqual(len(result), len(original))
            with self.assertRaises(AssertionError):
                expected_edit(original, cell, cell.value + b'x')

    def test_saved_hash_and_bucket_request_mismatches_are_rejected(self) -> None:
        original, _ = complete_file()
        for offset in (2 * 4096 + 12, 2 * 4096 + 38):
            result = bytearray(original)
            result[offset] ^= 1
            with self.assertRaises(AssertionError):
                inspect_default_file(result)


if __name__ == '__main__':
    unittest.main()
