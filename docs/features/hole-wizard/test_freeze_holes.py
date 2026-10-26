"""Reproduce complete Hole Wizard gate/slot fixtures and independently constructed edits (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_holes import freeze, VERSIONS, WORKSPACE
from hole_fixture_bytes import hole_object, hole_text


class HoleFixtureTests(unittest.TestCase):
    def test_complete_regeneration_matches_owned_bytes_oracles_and_case_facts(self) -> None:
        """Exercise every constructor and verifier; e.g. python -m unittest test_freeze_holes."""
        expected = json.loads((Path(__file__).parent/'reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination = Path(temporary)
            observed = freeze(WORKSPACE.parent/'inventor-interface/solidworks', destination)
            for key in ('cases', 'owned', 'packet_sha256'):
                self.assertEqual(observed[key], expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),
                    (WORKSPACE/'tests/fixtures/hole-wizard'/fixture.name).read_bytes(), fixture.name)

    def test_boundaries_include_both_sides_of_every_independent_gate(self) -> None:
        """Keep all 18 model contexts and two tagged-axis contexts; e.g. len(VERSIONS) == 18."""
        self.assertEqual(len(VERSIONS), 18)
        for lower, upper in [(0x2b5d, 0x2b5e), (0x2f45, 0x2f46), (0x2f47, 0x2f48),
            (0x2f67, 0x2f68), (0x332e, 0x332f), (0x3355, 0x3356),
            (0x3f02, 0x3f03), (0x42e9, 0x42ea)]:
            self.assertTrue({lower, upper} <= set(VERSIONS))
        facts = json.loads((Path(__file__).parent/'reference.json').read_text())
        self.assertEqual(len(facts['cases']), 74)
        self.assertEqual(sum(case['kind'] == 1 for case in facts['cases']), 2)

    def test_negative_inherited_slot_fails_and_label_edit_changes_only_its_constructor(self) -> None:
        """Reject impossible map context and retain distinct expected labels; e.g. hole_object(19000, True, True, -1)."""
        with self.assertRaisesRegex(ValueError, 'nonnegative slot'):
            hole_object(19000, True, True, -1)
        self.assertNotEqual(hole_text('828', True, False), hole_text('828', True, True))
        self.assertEqual(hole_text('830', True, False), hole_text('830', True, True))


if __name__ == '__main__':
    unittest.main()
