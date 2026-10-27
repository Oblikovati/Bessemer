"""Regenerate weldments and prove existing custom-property fixtures remain unchanged (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_weldments import freeze, WORKSPACE, REFERENCE, VERSIONS
from weldment_fixture_bytes import weldment_object
from freeze_custom_properties import property_record, property_bytes


class WeldmentFixtureTests(unittest.TestCase):
    def test_regeneration_matches_complete_records_aliases_and_owned_property_edits(self) -> None:
        """Compare all independent oracle bytes and imported slots; e.g. python -m unittest test_weldment_fixtures."""
        expected = json.loads((Path(__file__).parent/'reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination = Path(temporary)
            actual = freeze(REFERENCE, destination)
            for key in ('cases', 'owned', 'packet_sha256'):
                self.assertEqual(actual[key], expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(), (WORKSPACE/'tests/fixtures/weldments'/fixture.name).read_bytes(), fixture.name)

    def test_matrix_preserves_direct_map_identity_minimum_records_and_existing_shared_builder_bytes(self) -> None:
        """Pin each class and prove extraction retained the earlier property list; e.g. property_bytes('original')."""
        facts = json.loads((Path(__file__).parent/'reference.json').read_text())
        self.assertEqual(len(facts['cases']), 42)
        self.assertEqual(sum(row['reads'] for row in facts['cases']), 2367)
        self.assertEqual([sum(row['kind'] == kind for row in facts['cases']) for kind in range(4)], [12, 12, 6, 12])
        self.assertEqual(set(VERSIONS), {0x27cc, 19000, 0xffffffff})
        self.assertEqual([row['slots'] for row in facts['owned']], [101]*4)
        self.assertEqual(property_bytes('original'), (WORKSPACE/'tests/fixtures/resolved-features/custom-properties.bin').read_bytes())
        self.assertEqual(property_record('original'), property_bytes('original')[4:4+len(property_record('original'))])

    def test_invalid_constructor_shapes_reject_without_selecting_an_unrelated_serializer(self) -> None:
        """Fixture constructors require explicit classes and map origins; e.g. weldment_object(4, 19000, 0)."""
        for kind, style, slot in [(4, 0, 0), (-1, 0, 0), (0, 4, 0), (0, -1, 0), (0, 0, -1)]:
            with self.assertRaisesRegex(ValueError, 'expected kind/style 0..3 and nonnegative slot'):
                weldment_object(kind, 19000, style, slot)


if __name__ == '__main__':
    unittest.main()
