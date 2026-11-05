"""Reproduce conic goldens and pin the reused default OneBend builder (#218)."""
from pathlib import Path
import hashlib
import json
import struct
import unittest
from fixture_bytes import ROOT, conic_cases, verify_conic_cases
from edge_fixture_bytes import edge_graph


class ConicFixtureTests(unittest.TestCase):
    def test_all_literal_graphs_and_expected_edits_reproduce(self) -> None:
        """Rebuild every new graph without the production encoder; e.g. python -m unittest."""
        verify_conic_cases()
        self.assertEqual(len(conic_cases()), 23)

    def test_optional_conic_callback_preserves_all_684_old_graphs(self) -> None:
        """Keep reviewed default behavior byte-for-byte; e.g. python -m unittest."""
        reference = json.loads((ROOT / 'docs/features/edge-flanges/reference.json').read_text())
        self.assertEqual(len(reference['cases']), 684)
        for case in reference['cases']:
            kind, style = divmod(case['kind'], 6)
            raw, slots = edge_graph(kind, case['version'], style)
            self.assertEqual(hashlib.sha256(raw).hexdigest(), case['sha256'])
            self.assertEqual(slots, case['slots'])

    def test_helper_one_default_keeps_all_six_active_document_streams(self) -> None:
        """Keep the existing saved-helper continuation byte-for-byte; e.g. python -m unittest."""
        for kind in range(3):
            for edited in (False, True):
                raw, slots = edge_graph(kind, 19000, 3, edited, 96, True)
                expected = ROOT / 'tests/fixtures/edge-flanges' / (
                    f'active-{kind}' + ('-edited' if edited else '') + '.bin')
                self.assertEqual(struct.pack('<iH', -123, 1) + raw + bytes(4), expected.read_bytes())
                self.assertEqual(slots, (111, 113, 111)[kind])


if __name__ == '__main__':
    unittest.main()
