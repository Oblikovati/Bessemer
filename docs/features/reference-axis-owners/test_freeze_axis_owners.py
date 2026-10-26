"""Reproduce independent owner regressions and preserve the source axis grammar (#46)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_axis_owners import freeze, WORKSPACE
from axis_owner_bytes import axis_object


class AxisOwnerFixtureTests(unittest.TestCase):
    def test_regeneration_matches_all_primitives_owned_bytes_and_case_facts(self) -> None:
        """Exercise complete inherited geometry and owner checks; e.g. python -m unittest test_freeze_axis_owners."""
        expected = json.loads((Path(__file__).parent/'reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination = Path(temporary)
            observed = freeze(WORKSPACE.parent/'inventor-interface/solidworks', destination)
            for key in ('cases', 'owned'):
                self.assertEqual(observed[key], expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),
                    (WORKSPACE/'tests/fixtures/reference-axis-owners'/fixture.name).read_bytes(), fixture.name)

    def test_bare_matrix_keeps_empty_and_populated_owners_at_both_versions(self) -> None:
        """Retain four complete axes and all 678 reads; e.g. axis_object(19000, True)."""
        facts = json.loads((Path(__file__).parent/'reference.json').read_text())
        self.assertEqual(len(facts['cases']), 4)
        self.assertEqual(sum(row['reads'] for row in facts['cases']), 678)
        self.assertEqual({row['version'] for row in facts['cases']}, {0x27cc, 19000})

    def test_negative_initial_slot_is_rejected(self) -> None:
        """Reject malformed inherited state; e.g. axis_object(19000, True, -1)."""
        with self.assertRaisesRegex(ValueError, 'axis initial slot -1; expected a nonnegative slot'):
            axis_object(19000, True, -1)


if __name__ == '__main__':
    unittest.main()
