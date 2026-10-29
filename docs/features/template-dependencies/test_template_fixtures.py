"""Regenerate full saved-state template graphs and independent edit goldens (#54)."""
from pathlib import Path
import json,tempfile,unittest
from freeze_templates import REFERENCE,WORKSPACE,freeze


class TemplateFixtureTests(unittest.TestCase):
    def test_complete_alias_and_id_fixtures_regenerate(self) -> None:
        """Compare complete primitive partitions, saved helper controls and every owned byte."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination)
            for key in ('cases','owned','reads','packet_sha256'):
                self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/template-dependencies'/fixture.name).read_bytes())
