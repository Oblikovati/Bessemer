"""Compare all cosmetic cut-list fixtures with independent reader observations (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_cut_lists import REFERENCE,WORKSPACE,freeze,fixture_family


class CutListFixtureTests(unittest.TestCase):
    def test_complete_graphs_and_edit_goldens_regenerate(self) -> None:
        """Compare bytes, primitive observations and map slots; e.g. shared inherited attributes."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination,fixture_family())
            for key in ('cases','owned','reads','packet_sha256'):self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/cosmetic-weld-cut-lists'/fixture.name).read_bytes())

    def test_wire_modes_and_inherited_gates_remain_covered(self) -> None:
        """Keep both complete kernel modes across eleven versions; e.g. adjacent inherited gates."""
        facts=json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']),132)
        self.assertEqual({row['kind'] for row in facts['cases']},set(range(12)))
        self.assertEqual(len(facts['owned']),14)


if __name__=='__main__':unittest.main()
