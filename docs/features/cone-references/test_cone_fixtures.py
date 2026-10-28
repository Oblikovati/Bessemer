"""Regenerate complete cone records, parent identities and isolated double edits (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_cones import REFERENCE, WORKSPACE, freeze, fixture_family


class ConeFixtureTests(unittest.TestCase):
    def test_complete_graphs_and_owned_edits_regenerate(self) -> None:
        """Compare every independent byte and primitive partition; e.g. python -m unittest test_cone_fixtures."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination,fixture_family(REFERENCE))
            for key in ('cases','owned','reads','packet_sha256'):self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/cone-references'/fixture.name).read_bytes())

    def test_extreme_raw_fields_and_all_parent_identity_styles_remain_present(self) -> None:
        """Keep three raw-value sets, nine supported versions and six caller styles; e.g. signaling NaN and cached parents."""
        facts=json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']),162)
        self.assertEqual(facts['reads'],9171)
        self.assertEqual(len(facts['owned']),21)

    def test_all_generic_wrapper_callers_regenerate(self) -> None:
        """Preserve the previous family vectors and add every cone caller; e.g. end and intersection fields."""
        from freeze_cone_wrappers import freeze,REFERENCE,WORKSPACE,fixture_family
        expected=json.loads(Path(__file__).with_name('wrapper-reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination,fixture_family(REFERENCE))
            for key in ('cases','owned','reads','packet_sha256'):self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/cone-wrappers'/fixture.name).read_bytes())


if __name__=='__main__':
    unittest.main()
