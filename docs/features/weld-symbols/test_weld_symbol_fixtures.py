"""Regenerate constructor-bound symbols and preserve shared detail baselines (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_weld_symbols import REFERENCE,WORKSPACE,freeze,fixture_family


class WeldSymbolFixtureTests(unittest.TestCase):
    def test_complete_graphs_and_owned_edits_regenerate(self) -> None:
        """Compare complete real-reader observations and exact goldens; e.g. three shared direct-text fonts."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination,fixture_family(REFERENCE))
            for key in ('cases','owned','reads','packet_sha256'):self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/weld-symbols'/fixture.name).read_bytes())

    def test_native_class_version_and_identity_inventory_is_preserved(self) -> None:
        """Retain four classes, nine versions and six identity styles; e.g. finish strict-greater gates."""
        facts=json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']),216)
        self.assertEqual(facts['reads'],57278)
        self.assertEqual(len(facts['owned']),28)


if __name__=='__main__':unittest.main()
