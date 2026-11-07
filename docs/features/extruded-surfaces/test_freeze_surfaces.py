"""Reproduce complete surface/helper/keeper fixtures with actual inherited context (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_surfaces import freeze, VERSIONS, WORKSPACE
from surface_fixture_bytes import surface_object


class SurfaceFixtureTests(unittest.TestCase):
    def test_regeneration_matches_owned_bytes_oracles_and_all_case_facts(self) -> None:
        """Exercise every constructor and verifier; e.g. python -m unittest test_freeze_surfaces."""
        expected = json.loads((Path(__file__).parent/'reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination = Path(temporary)
            observed = freeze(WORKSPACE.parent/'inventor-interface/solidworks', destination)
            for key in ('cases', 'owned', 'packet_sha256'):
                self.assertEqual(observed[key], expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),
                    (WORKSPACE/'tests/fixtures/extruded-surfaces'/fixture.name).read_bytes(), fixture.name)

    def test_case_matrix_retains_helpers_keepers_and_both_end_spec_gate_sides(self) -> None:
        """Keep all 62 independent cases; e.g. 0x3b00 in VERSIONS."""
        self.assertTrue({0x3b00, 0x3b01} <= set(VERSIONS))
        facts = json.loads((Path(__file__).parent/'reference.json').read_text())
        self.assertEqual(len(facts['cases']), 62)
        self.assertEqual(sum(case['kind'] == 1 for case in facts['cases']), 8)
        self.assertEqual(sum(case['kind'] == 2 for case in facts['cases']), 2)
        self.assertEqual(sum('header-keeper' in case['label'] for case in facts['cases']), 4)

    def test_invalid_surface_mode_and_negative_inherited_slot_are_rejected(self) -> None:
        """Reject malformed fixture construction; e.g. surface_object(19000, False, 'null', False, -1)."""
        for mode, slot in [('invalid', 0), ('populated', -1)]:
            with self.assertRaisesRegex(ValueError, 'known mode and nonnegative slot'):
                surface_object(19000, False, mode, False, slot)


if __name__ == '__main__':
    unittest.main()
