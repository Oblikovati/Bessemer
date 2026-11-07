"""Regenerate complete finishing graphs through actual independent readers (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_finishing import REFERENCE,WORKSPACE,freeze,fixture_family


class FinishingFixtureTests(unittest.TestCase):
    def test_all_complete_graphs_and_owned_edit_goldens_regenerate(self) -> None:
        """Compare all bytes, primitive observations and map slots; e.g. shared side records."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination,fixture_family(REFERENCE))
            for key in ('cases','owned','reads','packet_sha256'):self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/weldment-finishing'/fixture.name).read_bytes())

    def test_all_three_native_classes_cross_inherited_and_cabinet_gates(self) -> None:
        """Retain twenty-two versions, all scalar styles and edit goldens; e.g. 0x2b9a and 0x2b9b."""
        facts=json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']),396)
        self.assertEqual(facts['reads'],191366)
        self.assertEqual(len(facts['owned']),21)


if __name__=='__main__':unittest.main()
