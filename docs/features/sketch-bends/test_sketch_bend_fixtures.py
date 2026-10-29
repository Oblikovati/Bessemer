"""Regenerate complete sketch-bend fields and handle identities (#54)."""
from pathlib import Path
import json,tempfile,unittest
from freeze_sketch_bends import REFERENCE,WORKSPACE,freeze,fixture_family


class SketchBendFixtureTests(unittest.TestCase):
    def test_all_inherited_graphs_and_owned_word_edit_regenerate(self) -> None:
        """Keep the independent One Bend base and every tagged suffix byte."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination,fixture_family())
            for key in ('cases','owned','reads','packet_sha256'):
                self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/sketch-bends'/fixture.name).read_bytes())
