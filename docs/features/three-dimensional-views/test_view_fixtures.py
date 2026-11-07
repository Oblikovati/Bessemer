"""Reproduce direct view graphs and complete configuration-manager goldens (#54)."""
from pathlib import Path
import json,tempfile,unittest
from freeze_three_dimensional_views import REFERENCE,WORKSPACE,freeze,fixture_family


class ThreeDimensionalViewFixtureTests(unittest.TestCase):
    def test_complete_graphs_and_managers_regenerate(self) -> None:
        """Compare all primitive observations, identities and owned manager bytes."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination,fixture_family())
            for key in ('cases','owned','reads','packet_sha256'):self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/three-dimensional-views'/fixture.name).read_bytes())
