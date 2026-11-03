"""Complete Cartesian root acceptance with original primitive partitions (#184)."""
from pathlib import Path
import sys
import unittest
from dimension_state_owned import owned_dimension_context
from owned_intersection_corpus import build_intersection_matrix, persist_intersection_matrix


class OwnedIntersectionCorpusTests(unittest.TestCase):
    def test_all_complete_roots_and_independent_primitive_oracles(self) -> None:
        feature = Path(__file__).resolve().parent
        fixture = feature.parents[2] / 'tests/fixtures/intersection-reference-match'
        document, _ = owned_dimension_context()
        payloads = build_intersection_matrix(document, (fixture / 'controls.bin').read_bytes())
        result = persist_intersection_matrix(fixture, feature, payloads, '--write-matrix' in sys.argv)
        self.assertEqual(result['cases'], 1344)
        self.assertEqual(result['groups'], 10752)
        self.assertEqual(result['primitive_reads'], 606144)
