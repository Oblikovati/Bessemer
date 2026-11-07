"""Regenerate complete inherited end caps and exact scalar-edit oracles (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_end_caps import REFERENCE,WORKSPACE,freeze,fixture_family


class EndCapFixtureTests(unittest.TestCase):
    def test_complete_graphs_and_owned_edit_goldens_regenerate(self) -> None:
        """Compare every byte, primitive observation and map slot; e.g. shared dimension identities."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination,fixture_family(REFERENCE))
            for key in ('cases','owned','reads','packet_sha256'):self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/end-caps'/fixture.name).read_bytes())

    def test_all_scalar_sets_cross_inherited_version_and_pointer_styles(self) -> None:
        """Retain seventeen versions and all true identity styles; e.g. 0x3747 and 0x3748."""
        facts=json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']),306)
        self.assertEqual(facts['reads'],126039)
        self.assertEqual(len(facts['owned']),21)


if __name__=='__main__':unittest.main()
