"""Cross-check complete native bucket expectations against the independent chain census."""
import gzip
import hashlib
import json
from pathlib import Path
import unittest
from freeze_bucket_census import checked_bucket, grouped_rows

FOLDER = Path(__file__).resolve().parent
FIXTURES = FOLDER.parents[2] / 'tests/fixtures/database-overflow'


class NativeBucketCensusTests(unittest.TestCase):
    def test_three_frozen_sources_match_earlier_overflow_hashes_and_all_bucket_cells(self) -> None:
        references = json.loads((FOLDER / 'native-sample-reference.json').read_text(encoding='utf-8'))
        groups = grouped_rows()
        for index, row in enumerate(references['samples']):
            raw = (FIXTURES / f'native-{index}.db').read_bytes()
            case = checked_bucket(groups[(row['ordinal'], 2)], raw)
            self.assertEqual(len(case.cells), (14, 10, 12)[index])
            self.assertEqual(sum(c.first != 0 for c in case.cells), (4, 1, 4)[index])

    def test_source_and_expected_value_mutations_cannot_replace_native_baselines(self) -> None:
        rows = grouped_rows()[(65, 2)]
        raw = (FIXTURES / 'native-0.db').read_bytes()
        with self.assertRaises(AssertionError):
            checked_bucket(rows, raw[:-1] + bytes([raw[-1] ^ 1]))
        changed = [row.copy() for row in rows]
        changed[0]['value_sha256'] = '0' * 64
        with self.assertRaises(AssertionError):
            checked_bucket(changed, raw)

    def test_frozen_complete_census_packet_has_exact_independent_identity(self) -> None:
        compressed = (FIXTURES / 'native-buckets.bin.gz').read_bytes()
        raw = gzip.decompress(compressed)
        reference = json.loads((FOLDER / 'native-buckets-reference.json').read_text(encoding='utf-8'))
        self.assertEqual(hashlib.sha256(raw).hexdigest(), reference['sha256'])
        self.assertEqual(hashlib.sha256(compressed).hexdigest(), reference['gzip_sha256'])
        self.assertEqual(raw[:8], b'BCW1\xff\x03\0\0')
        self.assertEqual(reference['cells'], 13842)


if __name__ == '__main__':
    unittest.main()
