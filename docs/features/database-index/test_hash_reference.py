"""Pin unsigned, empty, unrolled-loop and2048-byte default-hash boundaries."""
import unittest
import gzip
from pathlib import Path
import tempfile
from freeze_hash_vectors import hash_packet, freeze_hashes
from hash_reference import default_hash, closed_form_hash, requested_bucket


class NativeHashReferenceTests(unittest.TestCase):
    def test_frozen_polynomial_hash_vectors_match_regeneration(self) -> None:
        root = Path(__file__).resolve().parents[3] / 'tests/fixtures/database-index'
        stored = (root / 'hash-vectors.bin.gz').read_bytes()
        self.assertEqual(gzip.decompress(stored), hash_packet())
        with tempfile.TemporaryDirectory() as temporary:
            freeze_hashes(Path(temporary))
            self.assertEqual((Path(temporary) / 'hash-vectors.bin.gz').read_bytes(), stored)

    def test_empty_test_word_unsigned_bytes_and_all_unrolled_length_boundaries(self) -> None:
        self.assertEqual(default_hash(b''), 5381)
        self.assertEqual(default_hash(b'chm@symisc'), 0x03f523f5)
        self.assertEqual(default_hash(b'abc'), 193485963)
        for count in (0, 1, 2, 3, 4, 7, 8, 15, 31, 255, 256, 1023, 1024, 2047, 2048, 2049, 4096):
            key = bytes((index * 73 + index // 251) % 256 for index in range(count))
            for candidate in (key, bytes([255]) * count):
                self.assertEqual(default_hash(candidate), closed_form_hash(candidate))

    def test_native_prefix_limit_preserves_full_key_collision_distinction(self) -> None:
        prefix = bytes(range(256)) * 8
        self.assertNotEqual(prefix + b'\0', prefix + b'\xff')
        self.assertEqual(default_hash(prefix), default_hash(prefix + b'\0'))
        self.assertEqual(default_hash(prefix + b'\0'), default_hash(prefix + b'\xff'))

    def test_normal_linear_hash_requests_match_independent_split_partition(self) -> None:
        for maximum in (1, 2, 4, 8, 16, 32, 256):
            for split in range(maximum):
                for key_hash in (0, 1, 2, 3, 7, 31, 0x80000000, 0xffffffff):
                    base = key_hash % maximum
                    expected = key_hash % (2 * maximum) if base < split else base
                    self.assertEqual(requested_bucket(key_hash, split, maximum), expected)

    def test_wrapping_requests_do_not_impose_unproved_saved_field_domains(self) -> None:
        self.assertEqual(requested_bucket(0xffffffff, 0, 0), 0xffffffff)
        self.assertEqual(requested_bucket(7, 0, 1 << 63), 7)
        self.assertEqual(requested_bucket(7, 1, 0xffffffffffffffff), 6)


if __name__ == '__main__':
    unittest.main()
