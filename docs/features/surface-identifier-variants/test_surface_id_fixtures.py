"""Reproduce exact surface-identifier primitive and identity observations (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_surface_ids import REFERENCE,WORKSPACE,freeze,fixture_family


class SurfaceIdentifierFixtureTests(unittest.TestCase):
    def test_complete_graphs_and_owned_edits_regenerate(self) -> None:
        """Compare every byte and primitive observation with its independent source; e.g. signed extrema."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination,fixture_family(REFERENCE))
            for key in ('cases','owned','reads','packet_sha256'):self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/surface-identifier-variants'/fixture.name).read_bytes())

    def test_exact_class_and_caller_inventory_is_preserved(self) -> None:
        """Retain two variants, nine versions, six styles and two edit goldens; e.g. cached entity references."""
        facts=json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']),108)
        self.assertEqual(facts['reads'],17637)
        self.assertEqual(len(facts['owned']),14)


if __name__=='__main__':unittest.main()
