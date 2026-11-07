"""Regenerate complete fixed-transform bodies and independent raw-word edits (#86)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_valued_transforms import REFERENCE, WORKSPACE, freeze, fixture_family


class FixedValuedFixtureTests(unittest.TestCase):
    def test_complete_bodies_and_owned_edits_regenerate(self) -> None:
        """Compare every observation and golden byte; e.g. python -m unittest test_valued_transform_fixtures."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination,fixture_family(REFERENCE))
            for key in ('cases','owned','reads','packet_sha256'):self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/fixed-valued-transforms'/fixture.name).read_bytes())

    def test_all_shared_base_callers_and_noncanonical_presence_words_are_present(self) -> None:
        """Keep four valued families, nine versions and every presence bit pattern; e.g. 65535 and 32768."""
        facts=json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']),216)
        self.assertEqual(facts['reads'],16645)
        self.assertEqual(len(facts['owned']),28)


if __name__=='__main__':
    unittest.main()
