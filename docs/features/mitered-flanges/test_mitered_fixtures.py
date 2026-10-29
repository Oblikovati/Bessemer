"""Regenerate complete mitered-flange observations from immutable readers (#54)."""
from pathlib import Path
import json,tempfile,unittest
from freeze_mitered_flanges import REFERENCE,WORKSPACE,freeze,fixture_family


class MiteredFixtureTests(unittest.TestCase):
    def test_all_inherited_collections_handles_and_owned_edits_regenerate(self) -> None:
        """Require all independent primitive partitions, map sizes and owned bytes."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination,fixture_family())
            for key in ('cases','owned','reads','packet_sha256'):
                self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/mitered-flanges'/fixture.name).read_bytes())
