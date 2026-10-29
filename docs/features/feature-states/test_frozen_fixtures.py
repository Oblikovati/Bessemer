"""Regenerate complete inherited feature-states and exact scalar-edit oracles (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_feature_states import REFERENCE,WORKSPACE,freeze,fixture_family


class FrozenFeatureFixtureTests(unittest.TestCase):
    def test_complete_graphs_and_owned_edit_goldens_regenerate(self) -> None:
        """Compare every byte, primitive observation and map slot; e.g. shared dimension identities."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination,fixture_family(REFERENCE))
            for key in ('cases','owned','reads','packet_sha256'):self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/feature-states'/fixture.name).read_bytes())

    def test_all_scalar_sets_cross_inherited_version_and_pointer_styles(self) -> None:
        """Retain seven versions and all true identity styles; e.g. all signed-word styles."""
        facts=json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']),42)
        self.assertEqual(facts['reads'],420)
        self.assertEqual(len(facts['owned']),7)


if __name__=='__main__':unittest.main()
