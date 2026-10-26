"""Verify complete point-data variants, callers and constructor boundaries (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_points import freeze, WORKSPACE, VERSIONS
from point_fixture_bytes import point_object


class ReferencePointFixtureTests(unittest.TestCase):
    def test_regeneration_matches_all_case_facts_owned_bytes_and_oracles(self) -> None:
        """Exercise every constructor and semantic alias check; e.g. python -m unittest test_freeze_points."""
        expected = json.loads((Path(__file__).parent/'reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination = Path(temporary)
            observed = freeze(WORKSPACE.parent/'inventor-interface/solidworks', destination)
            for key in ('cases', 'owned', 'packet_sha256'):
                self.assertEqual(observed[key], expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),
                    (WORKSPACE/'tests/fixtures/reference-points'/fixture.name).read_bytes(), fixture.name)

    def test_matrix_retains_zero_body_data_every_variant_and_component_callers(self) -> None:
        """Keep all 132 objects and geometry gate sides; e.g. 0x2f86 in VERSIONS."""
        self.assertTrue({0x2f85, 0x2f86} <= set(VERSIONS))
        facts = json.loads((Path(__file__).parent/'reference.json').read_text())
        self.assertEqual(len(facts['cases']), 132)
        self.assertEqual(sum(row['reads'] for row in facts['cases']), 21968)
        self.assertTrue(set(range(14)) <= {row['kind'] for row in facts['cases']})
        self.assertEqual(sum(row['kind'] >= 14 for row in facts['cases']), 4)

    def test_invalid_variants_and_conflicting_root_modes_fail(self) -> None:
        """Reject malformed fixture boundaries; e.g. point_object(19000, 7, True)."""
        for options in [dict(kind=-1), dict(kind=7), dict(kind=0, style=4),
                dict(kind=0, point_style=3), dict(kind=0, first_slot=-1),
                dict(kind=0, standalone=True, component=True)]:
            with self.assertRaisesRegex(ValueError, 'expected known variants, nonnegative slots and distinct root modes'):
                point_object(19000, rich=True, **options)


if __name__ == '__main__':
    unittest.main()
