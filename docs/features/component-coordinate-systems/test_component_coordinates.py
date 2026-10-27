"""Regenerate component-coordinate bytes, containers and kind boundaries (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_component_coordinates import freeze, WORKSPACE, REFERENCE, VERSIONS, install_references
from component_coordinate_bytes import component_coordinate_object
from object_fixture_observer import object_cursor
from freeze_blank_part import source_module


class ComponentCoordinateFixtureTests(unittest.TestCase):
    def test_regeneration_matches_all_owned_bytes_and_primitive_observations(self) -> None:
        """Exercise every builder and complete identity check; e.g. python -m unittest test_component_coordinates."""
        expected = json.loads((Path(__file__).parent/'reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination = Path(temporary)
            observed = freeze(REFERENCE, destination)
            for key in ('cases', 'owned', 'packet_sha256'):
                self.assertEqual(observed[key], expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),
                    (WORKSPACE/'tests/fixtures/component-coordinate-systems'/fixture.name).read_bytes(), fixture.name)

    def test_matrix_pins_all_inherited_fixed_boundaries_and_direct_container_slots(self) -> None:
        """Keep eighteen versions and all sixteen supported low nibbles; e.g. 0x3b28 in VERSIONS."""
        facts = json.loads((Path(__file__).parent/'reference.json').read_text())
        self.assertEqual(len(VERSIONS), 18)
        self.assertTrue({0x3b27, 0x3b28, 0x3ef5, 0x3ef6, 0x371a, 0x371b} <= set(VERSIONS))
        self.assertEqual(len(facts['cases']), 160)
        self.assertEqual(sum(row['reads'] for row in facts['cases']), 29844)
        self.assertEqual([sum(row['kind'] == kind for row in facts['cases']) for kind in range(5)], [36, 36, 36, 36, 16])
        self.assertEqual([row['slots'] for row in facts['owned']], [99, 99, 113, 115, 115, 113])

    def test_invalid_construction_and_source_special_kinds_fail(self) -> None:
        """Retain native fixed-kind stops and explicit input shapes; e.g. component_coordinate_object(19000, True, 0, kind=16)."""
        for mode, slot, kind in [(-1, 0, 0), (4, 0, 0), (0, -1, 0), (0, 0, -1), (0, 0, 256)]:
            with self.assertRaisesRegex(ValueError, 'expected mode 0..3, nonnegative slot and u8 kind'):
                component_coordinate_object(19000, True, mode, slot, kind)
        error = source_module('swarchive', REFERENCE).ArchiveError
        for kind in (16, 32, 255):
            raw, _ = component_coordinate_object(19000, True, 0, kind=kind)
            with self.assertRaisesRegex(error, 'fixed transform special kind'):
                object_cursor(REFERENCE, raw, 19000, install_references).object()


if __name__ == '__main__':
    unittest.main()
