"""Independent strict writer framing, footer equations and malformed-boundary oracles."""
import struct
import tempfile
import unittest
from pathlib import Path
import control_requests as requests
import freeze_controls as freeze
from swarchive import ArchiveError


def read(identifier: int, width: int, stored: bytes, limit: int) -> dict:
    if identifier == requests.SAVED:
        return requests.reference.read_saved_spans(stored, max_decoded_bytes=limit)
    return requests.reference.read_index(stored, length_width=width, max_decoded_bytes=limit)


def sample(width: int) -> bytes:
    return requests.index_stored(requests.index_body([(12, 123, 25), (12, 9, 0)]), width)


def assert_bad_index(test: unittest.TestCase, stored: bytes, width: int) -> None:
    with test.assertRaises(ArchiveError):
        requests.reference.read_index(stored, length_width=width)


class ControlFixtureTests(unittest.TestCase):
    def test_exact_body_sizes_full_footer_equations_and_independent_decode_limits(self):
        for _, _, identifier, width, stored in requests.cases():
            value = requests.parse(identifier, width, stored)
            stride, prefix = (16, 16) if identifier == requests.SAVED else (20, 4)
            self.assertEqual(value['decoded_end'], prefix + stride * value['count'])
            self.assertEqual(value['count'], len(value['entries']))
            self.assertTrue(value['complete_payload_framing'])
            self.assertEqual(read(identifier, width, stored, value['decoded_end']), value)
            with self.assertRaises(ArchiveError):
                read(identifier, width, stored, value['decoded_end'] - 1)
            if identifier == requests.INDEX:
                self.assertEqual(value['footer_start'], 8 + value['compressed_bytes'])
                self.assertEqual(value['backward_distance'], value['footer_start'] + width + 4)
                self.assertEqual(value['backward_distance'], len(stored))
                self.assertEqual(value['footer_identifier'], requests.INDEX)

    def test_original_writer_examples_are_authenticated_and_have_no_native_payload_fixture(self):
        rows, proof = requests.reference_cases()
        self.assertEqual(len(rows), 3)
        self.assertTrue(all(row[1] for row in rows))
        self.assertTrue(all(row['control_record_count'] == 0 for row in proof['installed_fixture_audit']))
        self.assertIn('no installed control-record fixture', proof['limitations'][0])

    def test_raw_bits_identifiers_and_unordered_duplicates_are_retained(self):
        saved = [set(), set()]
        indexed = [set(), set()]
        identifiers = set()
        for _, _, identifier, width, stored in requests.cases():
            entries = requests.parse(identifier, width, stored)['entries']
            accumulate_fields(identifier, entries, saved, indexed, identifiers)
        for values in saved + indexed:
            self.assertTrue(set(requests.wides.RAW_PATTERNS) <= values)
        self.assertTrue(set(requests.definitions.PATTERNS) <= identifiers)
        raw = requests.saved_body([(7, 3), (7, 0), (0, 0xffffffffffffffff)])
        entries = requests.reference.read_decoded_saved_spans(raw)['entries']
        self.assertEqual([(entry['key_bits'], entry['value_bits']) for entry in entries], [(7, 3), (7, 0), (0, 0xffffffffffffffff)])

    def test_every_small_index_prefix_and_wrong_footer_width_is_rejected(self):
        for width in (4, 8):
            stored = sample(width)
            for length in range(len(stored)):
                assert_bad_index(self, stored[:length], width)
            assert_bad_index(self, stored, 12 - width)
        for width in (0, 1, 2, 16, True, None):
            assert_bad_index(self, sample(8), width)

    def test_zero_marker_and_footer_distance_and_identifier_bits_are_strict(self):
        for width in (4, 8):
            stored = sample(width)
            offsets = list(range(8)) + list(range(len(stored) - width - 4, len(stored)))
            flip_index_bits(self, stored, width, offsets)
            assert_bad_index(self, stored + b'x', width)

    def test_saved_markers_counts_and_body_suffixes_are_strict(self):
        raw = requests.saved_body([(7, 9)])
        for length in range(len(raw)):
            with self.assertRaises(ArchiveError):
                requests.reference.read_decoded_saved_spans(raw[:length])
        for changed in (struct.pack('<QQ', 1, 0), struct.pack('<QQ', 0, 0xffffffffffffffff), raw + b'x'):
            with self.assertRaises(ArchiveError):
                requests.reference.read_decoded_saved_spans(changed)
        for marker in (1 << bit for bit in range(64)):
            with self.assertRaises(ArchiveError):
                requests.reference.read_decoded_saved_spans(struct.pack('<QQ', marker, 0))

    def test_index_signed_counts_rows_and_decoded_suffixes_are_strict(self):
        raw = requests.index_body([(12, 123, 25)])
        for length in range(len(raw)):
            with self.assertRaises(ArchiveError):
                requests.reference.read_decoded_index(raw[:length])
        for changed in (struct.pack('<i', -1), struct.pack('<i', 0x7fffffff), raw + b'x'):
            with self.assertRaises(ArchiveError):
                requests.reference.read_decoded_index(changed)

    def test_compressed_stream_checksums_members_wrappers_and_saved_limits(self):
        raw = requests.saved_body([])
        stored = requests.zlib.compress(raw)
        for length in range(len(stored)):
            assert_bad_saved(self, stored[:length])
        corrupt = bytearray(stored)
        corrupt[-1] ^= 1
        for changed in (stored + b'x', stored + stored, bytes(corrupt),
                        requests.zlib.compress(raw, wbits=-15), requests.zlib.compress(raw, wbits=31)):
            assert_bad_saved(self, changed)
        for limit in (0, -1, True, 15):
            with self.assertRaises(ArchiveError):
                requests.reference.read_saved_spans(stored, max_decoded_bytes=limit)

    def test_index_compressed_corruption_concatenation_and_alternative_wrappers(self):
        for width in (4, 8):
            raw = requests.index_body([])
            compressed = requests.zlib.compress(raw)
            corrupt = bytearray(compressed)
            corrupt[-1] ^= 1
            for changed in (compressed[:-1], compressed + compressed, bytes(corrupt),
                            requests.zlib.compress(raw, wbits=-15), requests.zlib.compress(raw, wbits=31)):
                assert_bad_index(self, requests.wrap_index(changed, width), width)

    def test_wrong_control_identifiers_and_stored_sizes_are_rejected(self):
        raw = sample(8)
        for identifier, size in ((0xffffff9d, len(raw)), (requests.INDEX, len(raw) - 1)):
            with self.assertRaises(ArchiveError):
                requests.reference.read_record(dict(identifier=identifier, size=size, payload=raw))

    def test_regeneration_matches_both_owned_packets_and_verification(self):
        with tempfile.TemporaryDirectory() as folder:
            root = Path(folder)
            freeze.freeze(root)
            for relative in ('tests/fixtures/flow-control-records/saved-spans.bin.gz',
                             'tests/fixtures/flow-control-records/terminal-index.bin.gz',
                             'docs/features/flow-control-records/baseline-verification.json'):
                self.assertEqual((root / relative).read_bytes(), (requests.ROOT / relative).read_bytes())


def flip_index_bits(test: unittest.TestCase, stored: bytes, width: int, offsets: list[int]) -> None:
    for offset in offsets:
        for bit in range(8):
            changed = bytearray(stored)
            changed[offset] ^= 1 << bit
            assert_bad_index(test, bytes(changed), width)


def assert_bad_saved(test: unittest.TestCase, stored: bytes) -> None:
    with test.assertRaises(ArchiveError):
        requests.reference.read_saved_spans(stored)


def accumulate_fields(identifier: int, entries: list[dict], saved: list[set[int]], indexed: list[set[int]], identifiers: set[int]) -> None:
    for entry in entries:
        if identifier == requests.SAVED:
            saved[0].add(entry['key_bits'])
            saved[1].add(entry['value_bits'])
        else:
            identifiers.add(entry['identifier'])
            indexed[0].add(entry['record_start'])
            indexed[1].add(entry['record_span'])


if __name__ == '__main__':
    unittest.main()
