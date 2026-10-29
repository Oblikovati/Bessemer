"""Regenerate bitmap fields and byte-plane boundaries through independent readers (#54)."""
from pathlib import Path
import json,tempfile,unittest
from freeze_bitmaps import REFERENCE,WORKSPACE,freeze,fixture_family


class BitmapFixtureTests(unittest.TestCase):
    def test_all_bitmap_graphs_planes_and_owned_goldens_regenerate(self) -> None:
        """Keep complete field observations and every owned fixture byte unchanged."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination,fixture_family())
            for key in ('cases','owned','reads','packet_sha256'):
                self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/sketch-bitmaps'/fixture.name).read_bytes())
