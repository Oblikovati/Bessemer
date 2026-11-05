"""Check complete native corpus integrity and owned independent span expectations (#230)."""
import gzip
import hashlib
import json
from pathlib import Path
import unittest
from corpus_reference import corpus_file, value_spans
from file_fixtures import complete_file
from freeze_graph_corpus import complete_bucket_ids
from bucket_fixtures import native_bucket_bytes


class GraphCorpusReferenceTests(unittest.TestCase):
    def test_owned_corpus_includes_all_linked_buckets_and_literal_value_spans(self) -> None:
        raw, expected = complete_file()
        physical = complete_bucket_ids(raw, (2, 5))
        self.assertEqual(physical, (2, 5, 6))
        record, checked = corpus_file(raw, 88, physical)
        self.assertEqual(checked['cells'], 4)
        self.assertEqual(record[12:12 + len(raw)], raw)
        for case in expected:
            bucket = native_bucket_bytes(raw, case.bucket)
            cell = next(cell for cell in bucket.cells if cell.offset == case.cell)
            self.assertEqual(value_spans(raw, case.bucket, cell), case.spans)

    def test_complete_native_packet_matches_persisted_raw_and_compressed_hashes(self) -> None:
        root = Path(__file__).resolve().parents[3] / 'tests/fixtures/database-index'
        proof = json.loads((root / 'native-files-integrity.json').read_text())
        compressed = (root / 'native-files.bin.gz').read_bytes()
        raw = gzip.decompress(compressed)
        self.assertEqual(len(compressed), proof['compressed_bytes'])
        self.assertEqual(hashlib.sha256(compressed).hexdigest(), proof['compressed_sha256'])
        self.assertEqual(len(raw), proof['raw_bytes'])
        self.assertEqual(hashlib.sha256(raw).hexdigest(), proof['raw_sha256'])
        self.assertEqual(raw[:8], b'BFG1\xb6\x09\x00\x00')


if __name__ == '__main__':
    unittest.main()
