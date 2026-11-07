"""Regenerate complete punch revision fixtures from immutable readers (#54)."""
from pathlib import Path
import json,tempfile,unittest
from freeze_punches import REFERENCE,WORKSPACE,freeze,fixture_family


class PunchFixtureTests(unittest.TestCase):
    def test_all_revision_fields_extensions_and_owned_goldens_regenerate(self) -> None:
        """Keep every independent partition, map slot and owned fixture byte."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination,fixture_family())
            for key in ('cases','owned','reads','packet_sha256'):
                self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/punches'/fixture.name).read_bytes())
