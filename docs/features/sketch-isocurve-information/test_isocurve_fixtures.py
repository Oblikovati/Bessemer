"""Regenerate complete isocurve records, parent identities and isolated ID edits (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_isocurves import REFERENCE, WORKSPACE, freeze, fixture_family


class IsocurveFixtureTests(unittest.TestCase):
    def test_complete_graphs_and_owned_edits_regenerate(self) -> None:
        """Compare every independent byte and primitive partition; e.g. python -m unittest test_isocurve_fixtures."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination,fixture_family(REFERENCE))
            for key in ('cases','owned','reads','packet_sha256'):self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/sketch-isocurve-information'/fixture.name).read_bytes())

    def test_extreme_raw_fields_and_all_parent_identity_styles_remain_present(self) -> None:
        """Keep nine raw-value sets, three supported versions and six caller styles; e.g. signaling NaN and cached parents."""
        facts=json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']),162)
        self.assertEqual(facts['reads'],3942)
        self.assertEqual(len(facts['owned']),63)


if __name__=='__main__':
    unittest.main()
