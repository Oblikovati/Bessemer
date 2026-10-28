"""Regenerate complete native-bound edge graphs without Bessemer-generated expectations (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_thin_features import FAMILY, REFERENCE, WORKSPACE, freeze, freeze_bend_boundary


class ThinFeatureFixtureTests(unittest.TestCase):
    def test_all_graphs_and_owned_edits_regenerate_exactly(self) -> None:
        """Compare primitive partitions, source aliases and all frozen bytes; e.g. python -m unittest test_thin_fixtures."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination,FAMILY)
            for key in ('cases','owned','reads','packet_sha256'):
                self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/thin-extrusions'/fixture.name).read_bytes())

    def test_five_families_have_complete_versions_styles_and_signed_edits(self) -> None:
        """Keep all native shapes and distinct equal roots; e.g. styles 2 and 5."""
        facts=json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']),360)
        self.assertEqual(facts['reads'],256702)
        self.assertEqual({row['kind'] for row in facts['cases']},set(range(30)))
        self.assertEqual(len(facts['owned']),35)

    def test_full_u16_boundary_matches_the_real_payload_reader(self) -> None:
        """Verify every full-width triple and cached specification identity; e.g. 65,535 records."""
        expected=json.loads(Path(__file__).with_name('bend-boundary-reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze_bend_boundary(REFERENCE,destination)
            self.assertEqual(actual,expected)
            self.assertEqual((destination/'max-bends.bin').read_bytes(),(WORKSPACE/'tests/fixtures/thin-extrusions/max-bends.bin').read_bytes())


if __name__=='__main__':
    unittest.main()
