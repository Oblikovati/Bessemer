"""Reproduce raw-body goldens and retain the reused source fixtures (#220)."""
import hashlib
import json
import unittest
import zlib
from fixture_bytes import ROOT, positive_cases, positive_tail, verify_positive_cases
from edge_fixture_bytes import edge_graph
from kernel_geometry_fixture import frame, transmit


class OneBendRawFixtureTests(unittest.TestCase):
    def test_all_original_and_edited_document_streams_reproduce(self) -> None:
        """Pin all eighteen streams and map counts; e.g. python -m unittest."""
        verify_positive_cases()
        self.assertEqual(len(positive_cases()), 18)
        self.assertEqual(positive_tail(0).count(frame(-7)[1:]), 5)
        self.assertEqual(positive_tail(1).count(frame(9)[1:]), 5)

    def test_base_kernel_frame_and_transmit_match_independent_literals(self) -> None:
        """Keep the portable kernel oracle separate from Bessemer; e.g. python -m unittest."""
        folder = ROOT / 'tests/fixtures/model-archives'
        retained = (folder / 'kernel-frame-one.bin').read_bytes()
        compressed_length = int.from_bytes(retained[30:34], 'little')
        self.assertEqual(zlib.decompress(retained[34:34 + compressed_length]), transmit())
        self.assertEqual(transmit(), (folder / 'kernel-transmit-one.bin').read_bytes())
        self.assertEqual(transmit(9), (folder / 'kernel-transmit-edited.bin').read_bytes())

    def test_all_684_prior_edge_graphs_keep_their_original_hashes(self) -> None:
        """Preserve reviewed defaults without changing the shared assembler; e.g. python -m unittest."""
        reference = json.loads((ROOT / 'docs/features/edge-flanges/reference.json').read_text())
        self.assertEqual(len(reference['cases']), 684)
        for case in reference['cases']:
            kind, style = divmod(case['kind'], 6)
            raw, slots = edge_graph(kind, case['version'], style)
            self.assertEqual(hashlib.sha256(raw).hexdigest(), case['sha256'])
            self.assertEqual(slots, case['slots'])


if __name__ == '__main__':
    unittest.main()
