"""Reproduce all independent annotation observations and owned goldens (#54)."""
from pathlib import Path
import json,tempfile,unittest
from freeze_annotations import REFERENCE,WORKSPACE,freeze,fixture_family


class AnnotationFixtureTests(unittest.TestCase):
    def test_complete_graphs_and_owned_goldens_regenerate(self) -> None:
        """Compare every primitive observation, map slot and owned byte."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination,fixture_family())
            for key in ('cases','owned','reads','packet_sha256'):self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/annotation-extras'/fixture.name).read_bytes())
