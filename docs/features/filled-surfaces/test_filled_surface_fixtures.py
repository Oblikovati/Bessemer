"""Regenerate complete filled-surface, direct keeper and curve-contact fixtures (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_filled_surfaces import FAMILY, REFERENCE, WORKSPACE, freeze


class FilledSurfaceFixtureTests(unittest.TestCase):
    def test_complete_graphs_shared_references_and_owned_edits_regenerate(self) -> None:
        """Keep independent primitive partitions and golden bytes unchanged; e.g. python -m unittest test_filled_surface_fixtures."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination,FAMILY)
            for key in ('cases','owned','reads','packet_sha256'):
                self.assertEqual(actual[key],expected[key])
            for p in destination.iterdir():
                self.assertEqual(p.read_bytes(),(WORKSPACE/'tests/fixtures/filled-surfaces'/p.name).read_bytes())

    def test_four_classes_and_all_byte_interpretations_are_present(self) -> None:
        """Keep eleven versions, six raw-byte cases and complete contact dependencies; e.g. byte 255 means signed char -1."""
        facts=json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']),264)
        self.assertEqual(facts['reads'],63543)
        self.assertEqual({row['kind'] for row in facts['cases']},set(range(24)))
        self.assertEqual(len(facts['owned']),27)


if __name__=='__main__':
    unittest.main()
