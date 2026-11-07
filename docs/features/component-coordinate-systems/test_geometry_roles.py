"""Regress concrete coordinate geometry and cached target identity (#46)."""
from pathlib import Path
import json
import tempfile
import unittest
from typing import cast
from geometry_role_fixtures import freeze, WORKSPACE, VERSIONS, geometry_role_object, install_references
from object_fixture_observer import object_cursor


class ComponentGeometryRoleTests(unittest.TestCase):
    def test_regeneration_matches_every_complete_case_and_primitive(self) -> None:
        """Exercise all builders and identity assertions; e.g. python -m unittest test_geometry_roles."""
        expected = json.loads((Path(__file__).parent/'geometry-roles-reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination = Path(temporary)
            observed = freeze(WORKSPACE.parent/'inventor-interface/solidworks', destination)
            self.assertEqual(observed['cases'], expected['cases'])
            self.assertEqual(observed['packet_sha256'], expected['packet_sha256'])
            self.assertEqual((destination/'geometry-roles.bin').read_bytes(),
                (WORKSPACE/'tests/fixtures/component-coordinate-systems/geometry-roles.bin').read_bytes())

    def test_complete_matrix_pins_fresh_and_cached_maps_in_both_component_readers(self) -> None:
        """Keep 24 complete cases and 3,428 observations; e.g. 0x2f86 in VERSIONS."""
        facts = json.loads((Path(__file__).parent/'geometry-roles-reference.json').read_text())
        self.assertEqual(len(facts['cases']), 24)
        self.assertEqual(sum(row['reads'] for row in facts['cases']), 3428)
        self.assertEqual([sum(row['kind'] == kind for row in facts['cases']) for kind in range(4)], [6]*4)
        self.assertTrue({0x27cc, 0x2f85, 0x2f86, 19000} <= set(VERSIONS))

    def test_cached_geometry_is_the_coordinate_object_instead_of_its_nested_name(self) -> None:
        """Catch slot capture after nested object allocation; e.g. geometry_role_object(19000, False, True)."""
        raw, slots = geometry_role_object(19000, False, True)
        cursor = object_cursor(WORKSPACE.parent/'inventor-interface/solidworks', raw, 19000, install_references)
        root = cast(dict[str, object], cursor.object())
        items = cast(list[dict[str, object]], root['items'])
        self.assertIs(items[1]['reference_geometry'], items[0])
        self.assertEqual(items[0]['_class'], 'moCoordSys_c')
        self.assertEqual(len(cursor.map), slots)


if __name__ == '__main__':
    unittest.main()
