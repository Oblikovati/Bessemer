"""Regenerate direct visual maps and complete manager compositions (#54)."""
from pathlib import Path
import json,tempfile,unittest
from freeze_visual_maps import REFERENCE,WORKSPACE,freeze


class VisualMapFixtureTests(unittest.TestCase):
    def test_complete_visual_maps_and_managers_regenerate(self) -> None:
        """Require identical byte/primitive partitions and complete manager goldens."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination)
            for key in ('cases','owned','reads','packet_sha256'):self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/configuration-visual-maps'/fixture.name).read_bytes())
