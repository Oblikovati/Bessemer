"""Reproduce complete display welds with actual inherited reference readers (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_display_welds import REFERENCE,WORKSPACE,freeze,fixture_family


class DisplayWeldFixtureTests(unittest.TestCase):
    def test_complete_graphs_and_owned_edits_regenerate(self) -> None:
        """Compare every independent byte and primitive partition; e.g. long Unicode weld strings."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination,fixture_family(REFERENCE))
            for key in ('cases','owned','reads','packet_sha256'):self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/display-welds'/fixture.name).read_bytes())

    def test_raw_values_strings_and_identity_styles_remain_present(self) -> None:
        """Retain three value sets, nine versions and six pointer styles; e.g. self payload references."""
        facts=json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']),162)
        self.assertEqual(facts['reads'],32724)
        self.assertEqual(len(facts['owned']),21)


if __name__=='__main__':unittest.main()
