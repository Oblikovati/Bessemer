"""Regenerate complete inherited flatten-bends and exact scalar-edit oracles (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_flatten_bends import REFERENCE,WORKSPACE,freeze,fixture_family,freeze_active_flatten


class FrozenFeatureFixtureTests(unittest.TestCase):
    def test_alternate_saved_manager_documents_regenerate(self) -> None:
        """Keep saved state independent of ordinary archive mode; e.g. a helper-one raw body."""
        expected=json.loads(Path(__file__).with_name('active-reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary)
            self.assertEqual(freeze_active_flatten(REFERENCE,destination),expected)
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/flatten-bends'/fixture.name).read_bytes())

    def test_complete_graphs_and_owned_edit_goldens_regenerate(self) -> None:
        """Compare every byte, primitive observation and map slot; e.g. shared dimension identities."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination,fixture_family(REFERENCE))
            for key in ('cases','owned','reads','packet_sha256'):self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/flatten-bends'/fixture.name).read_bytes())

    def test_all_scalar_sets_cross_inherited_version_and_pointer_styles(self) -> None:
        """Retain seven versions and all true identity styles; e.g. all signed-word styles."""
        facts=json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']),84)
        self.assertEqual(facts['reads'],21188)
        self.assertEqual(len(facts['owned']),14)


if __name__=='__main__':unittest.main()
