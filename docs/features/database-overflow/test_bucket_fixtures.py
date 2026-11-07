"""Pin complete mixed-bucket references independently of Bessemer's reader."""
import gzip
import hashlib
import json
from pathlib import Path
import struct
import unittest
from bucket_fixtures import bucket_packet, owned_buckets, bucket_controls, native_bucket

FOLDER = Path(__file__).resolve().parent
FIXTURES = FOLDER.parents[2] / 'tests/fixtures/database-overflow'


class MixedBucketFixtureTests(unittest.TestCase):
    def test_complete_packet_regeneration_matches_every_byte_and_record_extent(self) -> None:
        raw = bucket_packet(FIXTURES)
        frozen = (FIXTURES / 'writer-buckets.bin.gz').read_bytes()
        reference = json.loads((FOLDER / 'bucket-fixture-reference.json').read_text(encoding='utf-8'))
        self.assertEqual(raw, gzip.decompress(frozen))
        self.assertEqual(hashlib.sha256(raw).hexdigest(), reference['sha256'])
        self.assertEqual(hashlib.sha256(frozen).hexdigest(), reference['gzip_sha256'])
        self.assertEqual(raw[:8], b'BWB1\x7a\0\0\0')
        cursor = 8
        for _ in range(122):
            length = struct.unpack_from('<I', raw, cursor)[0]
            cursor += 4 + length
        self.assertEqual(cursor, len(raw))

    def test_owned_and_native_storage_kinds_keep_ordered_exact_metadata(self) -> None:
        owned = owned_buckets()
        self.assertEqual((len(owned), sum(c.rejected for c in owned)), (117, 6))
        for case in owned:
            self.assertEqual(case.free, ((12, 0, 65535),))
            self.assertEqual((case.cells[0].offset, case.cells[0].following), (20, 80))
            self.assertEqual(case.cells[1].first, case.pages[0][0])
        self.assertEqual([len(c.cells) for c in bucket_controls()], [1, 0])
        self.assertEqual([len(native_bucket(i, FIXTURES).cells) for i in range(3)], [14, 10, 12])


if __name__ == '__main__':
    unittest.main()
