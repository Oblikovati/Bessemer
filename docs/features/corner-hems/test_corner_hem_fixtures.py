"""Regenerate complete inherited corners and hems and exact scalar-edit oracles (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_corner_hems import REFERENCE,WORKSPACE,freeze,fixture_family
from active_hem_fixtures import freeze_active_hems


class CornerHemFixtureTests(unittest.TestCase):
    def test_active_saved_manager_documents_regenerate(self) -> None:
        """Derive each alternate layout from the actual saved manager; e.g. the omitted sheet pointer."""
        expected=json.loads(Path(__file__).with_name('active-reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary)
            self.assertEqual(freeze_active_hems(REFERENCE,destination),expected)
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/corner-hems'/fixture.name).read_bytes())

    def test_complete_graphs_and_owned_edit_goldens_regenerate(self) -> None:
        """Compare every byte, primitive observation and map slot; e.g. shared dimension identities."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination,fixture_family(REFERENCE))
            for key in ('cases','owned','reads','packet_sha256'):self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/corner-hems'/fixture.name).read_bytes())

    def test_all_scalar_sets_cross_inherited_version_and_pointer_styles(self) -> None:
        """Retain seven versions and all true identity styles; e.g. direct arrays and tagged lists."""
        facts=json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']),378)
        self.assertEqual(facts['reads'],66711)
        self.assertEqual(len(facts['owned']),61)


if __name__=='__main__':unittest.main()
