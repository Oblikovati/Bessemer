"""Regenerate independent fields, identities and owned wire goldens (#54)."""
from pathlib import Path
import json,tempfile,unittest
from freeze_edits import REFERENCE,WORKSPACE,freeze,fixture_family


class FeatureEditFixtureTests(unittest.TestCase):
    def test_complete_graphs_and_owned_edit_goldens_regenerate(self) -> None:
        """Require every independent primitive partition and byte to reproduce."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination,fixture_family())
            for key in ('cases','owned','reads','packet_sha256'):
                self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/feature-edits'/fixture.name).read_bytes())
