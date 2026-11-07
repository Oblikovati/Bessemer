"""Reproduce all independent observations and owned edit goldens (#54)."""
from pathlib import Path
import json,tempfile,unittest
from freeze_alignment import REFERENCE,WORKSPACE,freeze,fixture_family


class AlignmentFixtureTests(unittest.TestCase):
    def test_complete_bodies_and_owned_goldens_regenerate(self) -> None:
        """Compare every primitive observation, map slot and owned fixture byte."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination,fixture_family())
            for key in ('cases','owned','active_owned','reads','packet_sha256'):
                self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/alignment-groups'/fixture.name).read_bytes())
