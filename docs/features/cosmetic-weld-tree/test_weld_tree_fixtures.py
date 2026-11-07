"""Regenerate independent direct-body, signed-count and shared-object oracles (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_weld_tree import REFERENCE,WORKSPACE,freeze,fixture_family
from parameter_bytes import endpoint,parameters,wire_map


class WeldTreeFixtureTests(unittest.TestCase):
    def test_all_complete_graphs_and_owned_edits_regenerate(self) -> None:
        """Compare exact bytes, primitive observations and map slots; e.g. direct endpoints."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination,fixture_family(REFERENCE))
            for key in ('cases','owned','reads','packet_sha256'):self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/cosmetic-weld-tree'/fixture.name).read_bytes())

    def test_all_six_tagged_classes_and_fixed_direct_records_are_present(self) -> None:
        """Keep exact fixed widths and negative-empty wire lists; e.g. wire_map(0) is twelve bytes."""
        facts=json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']),324)
        self.assertEqual(len(facts['owned']),42)
        self.assertEqual({row['kind'] for row in facts['cases']},set(range(36)))
        self.assertEqual((len(parameters(False,2)),len(parameters(True,2)),len(endpoint(2)),len(wire_map(0))),(20,24,32,12))


if __name__=='__main__':unittest.main()
