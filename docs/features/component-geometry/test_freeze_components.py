"""Verify component geometry constructors, inherited gates and complete oracles (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_components import freeze, WORKSPACE, VERSIONS
from component_geometry_bytes import NAMES, component_object


class ComponentGeometryFixtureTests(unittest.TestCase):
    def test_regeneration_matches_all_owned_bytes_oracles_and_case_facts(self) -> None:
        """Exercise complete composition and local registrations; e.g. python -m unittest test_freeze_components."""
        expected = json.loads((Path(__file__).parent/'reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination = Path(temporary)
            observed = freeze(WORKSPACE.parent/'inventor-interface/solidworks', destination)
            for key in ('cases', 'owned', 'packet_sha256'):
                self.assertEqual(observed[key], expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),
                    (WORKSPACE/'tests/fixtures/component-geometry'/fixture.name).read_bytes(), fixture.name)

    def test_matrix_keeps_every_inherited_gate_and_wrapper_class(self) -> None:
        """Retain 152 complete objects across twelve versions; e.g. 0x2b61 in VERSIONS."""
        self.assertTrue({0x2b60, 0x2b61, 0x2bb8, 0x2bb9, 0x2f55, 0x2f56,
            0x2f85, 0x2f86, 0x3ef5, 0x3ef6} <= set(VERSIONS))
        facts = json.loads((Path(__file__).parent/'reference.json').read_text())
        self.assertEqual(len(facts['cases']), 152)
        self.assertEqual(sum(row['reads'] for row in facts['cases']), 21264)
        self.assertEqual([sum(row['kind'] == kind for row in facts['cases']) for kind in range(4)], [72, 72, 4, 4])

    def test_invalid_classes_geometry_kinds_and_slots_fail(self) -> None:
        """Reject fixture options outside the documented composition; e.g. component_object(NAMES[0], 19000, False, -1)."""
        for name, kind, slot in [('unknown', 0, 0), (NAMES[0], -1, 0), (NAMES[1], 3, 0), (NAMES[0], 0, -1)]:
            with self.assertRaisesRegex(ValueError, 'expected known axis/point class, geometry 0..2 and nonnegative slot'):
                component_object(name, 19000, False, kind, slot)


if __name__ == '__main__':
    unittest.main()
