"""Reproduce independent smart-component observations and all owned goldens (#54)."""
from pathlib import Path
import json,tempfile,unittest
from freeze_smart import REFERENCE,WORKSPACE,freeze,fixture_family


class SmartFixtureTests(unittest.TestCase):
    def test_complete_graphs_and_owned_goldens_regenerate(self) -> None:
        """Compare every primitive observation, slot count and owned byte."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination,fixture_family())
            for key in ('cases','owned','reads','packet_sha256'):self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/smart-components'/fixture.name).read_bytes())
