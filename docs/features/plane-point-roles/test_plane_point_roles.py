"""Verify finite plane point-role regressions and independent cached identities (#46)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_plane_point_roles import freeze, WORKSPACE, REFERENCE, VERSIONS
from plane_point_fixture_bytes import role_object


class PlanePointRoleTests(unittest.TestCase):
    def test_regeneration_preserves_complete_bodies_primitive_oracles_and_map_slots(self) -> None:
        """Require identical new/cached positives and negative wire shapes; e.g. python -m unittest test_plane_point_roles."""
        expected = json.loads((Path(__file__).parent/'reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination = Path(temporary)
            actual = freeze(REFERENCE, destination)
            for key in ('cases', 'owned', 'packet_sha256'):
                self.assertEqual(actual[key], expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(), (WORKSPACE/'tests/fixtures/plane-point-roles'/fixture.name).read_bytes(), fixture.name)

    def test_matrix_retains_three_plane_kinds_finite_family_and_unrelated_wrappers(self) -> None:
        """Pin all 144 cases and the independent expected membership table; e.g. point 3 remains unassignable."""
        facts = json.loads((Path(__file__).parent/'reference.json').read_text())
        self.assertEqual(len(facts['cases']), 144)
        self.assertEqual(sum(row['reads'] for row in facts['cases']), 16836)
        self.assertEqual(sum(row['assignable'] for row in facts['cases']), 108)
        self.assertEqual(sum(row['kind'] & 16 != 0 for row in facts['cases']), 72)
        self.assertEqual([sum(row['kind'] & 3 == plane for row in facts['cases']) for plane in range(3)], [48, 48, 48])
        self.assertTrue({0x2b83, 0x2b84, 0x2ba5, 0x2ba6} <= set(VERSIONS))
        self.assertEqual(len(facts['owned']), 11)
        self.assertEqual(sum(row['assignable'] for row in facts['owned']), 10)

    def test_unknown_fixture_classes_reject_without_a_name_based_fallback(self) -> None:
        """Fixtures choose explicit known constructors only; e.g. role_object(3, 0, 19000, False)."""
        for plane, point in [(3, 0), (-1, 0), (0, 4), (0, -1)]:
            with self.assertRaisesRegex(ValueError, 'expected plane 0..2 and point 0..3'):
                role_object(plane, point, 19000, False)


if __name__ == '__main__':
    unittest.main()
