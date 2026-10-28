"""Regenerate complete three-point planes and unrelated role witnesses (#46)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_three_point_roles import REFERENCE, WORKSPACE, freeze, fixture_family, negative_roles


class ThreePointFixtureTests(unittest.TestCase):
    def test_all_graphs_and_owned_edits_regenerate_exactly(self) -> None:
        """Compare primitive partitions, source aliases and all frozen bytes; e.g. python -m unittest test_three_point_fixtures."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination,fixture_family(REFERENCE))
            for key in ('cases','owned','reads','packet_sha256'):
                self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/three-point-plane-roles'/fixture.name).read_bytes())

    def test_six_styles_have_complete_versions_and_one_coordinate_edit(self) -> None:
        """Keep all native shapes and distinct equal roots; e.g. styles 2 and 5."""
        facts=json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']),78)
        self.assertEqual(facts['reads'],10084)
        self.assertEqual({row['kind'] for row in facts['cases']},set(range(6)))
        self.assertEqual(len(facts['owned']),7)

    def test_unrelated_wrappers_are_complete_byte_valid_negative_witnesses(self) -> None:
        """Keep fresh/cached rejection separate from source expected-class permissiveness; e.g. wrapper kind 3."""
        expected=json.loads(Path(__file__).with_name('negative-reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=negative_roles(REFERENCE,destination)
            self.assertEqual(actual,expected)
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/three-point-plane-roles'/fixture.name).read_bytes())


if __name__=='__main__':
    unittest.main()
