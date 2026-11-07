"""Regenerate complete mates, sections and independent edit/removal goldens (#54)."""
from pathlib import Path
import json,tempfile,unittest
from freeze_library import REFERENCE,WORKSPACE,freeze,fixture_family


class LibraryFixtureTests(unittest.TestCase):
    def test_complete_mate_plane_fixtures_and_owned_goldens_regenerate(self) -> None:
        """Retain every primitive observation, slot count and owned byte."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination,fixture_family())
            for key in ('cases','owned','reads','packet_sha256'):
                self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/library-features'/fixture.name).read_bytes())
