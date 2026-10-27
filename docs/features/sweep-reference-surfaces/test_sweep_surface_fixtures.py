"""Regenerate complete native-bound edge graphs without Bessemer-generated expectations (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_sweep_surfaces import FAMILY, REFERENCE, WORKSPACE, freeze


class SweepSurfaceFixtureTests(unittest.TestCase):
    def test_all_graphs_and_owned_edits_regenerate_exactly(self) -> None:
        """Compare primitive partitions, source aliases and all frozen bytes; e.g. python -m unittest test_sweep_surface_fixtures."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination,FAMILY)
            for key in ('cases','owned','reads','packet_sha256'):
                self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/sweep-reference-surfaces'/fixture.name).read_bytes())

    def test_three_classes_have_complete_versions_styles_and_signed_edits(self) -> None:
        """Keep all native shapes and distinct equal roots; e.g. styles 2 and 5."""
        facts=json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']),162)
        self.assertEqual(facts['reads'],44798)
        self.assertEqual({row['kind'] for row in facts['cases']},set(range(18)))
        self.assertEqual(len(facts['owned']),19)


if __name__=='__main__':
    unittest.main()
