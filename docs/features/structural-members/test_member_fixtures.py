"""Regenerate complete member graphs and check independent signed-count/mode domains (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_structural_members import freeze, WORKSPACE, REFERENCE, MODES, VERSIONS
from member_reference_bytes import MemberFixtureOptions
from member_fixture_bytes import structural_object


class MemberFixtureTests(unittest.TestCase):
    def test_regeneration_preserves_complete_graphs_primitive_oracles_and_owned_metadata_edits(self) -> None:
        """Compare every observation, map slot and emitted byte; e.g. python -m unittest test_member_fixtures."""
        expected = json.loads((Path(__file__).parent/'reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination = Path(temporary)
            actual = freeze(REFERENCE, destination)
            for key in ('cases', 'owned', 'packet_sha256'):
                self.assertEqual(actual[key], expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(), (WORKSPACE/'tests/fixtures/structural-members'/fixture.name).read_bytes(), fixture.name)

    def test_matrix_covers_all_nine_classes_full_word_modes_and_point_and_curve_subtypes(self) -> None:
        """Keep mode -1 separate from positive modes and retain all three point classes; e.g. MODES includes signed extrema."""
        facts = json.loads((Path(__file__).parent/'reference.json').read_text())
        self.assertEqual(len(facts['cases']), 90)
        self.assertEqual(sum(row['reads'] for row in facts['cases']), 25171)
        self.assertEqual({row['kind'] for row in facts['cases']}, set(range(9)))
        self.assertEqual(set(VERSIONS), {0x27cc, 19000, 0xffffffff})
        self.assertEqual(set(MODES), {-2147483648, -1, 0, 1, 2, 2147483647})
        variations = [row['options'] for row in facts['cases'][54:]]
        self.assertEqual({row['point_style'] for row in variations}, {0, 1, 2})
        self.assertEqual({row['curve_style'] for row in variations}, {0, 1})
        self.assertEqual([row['slots'] for row in facts['owned']], [101, 125, 133, 133, 125, 133])

    def test_unknown_constructor_class_slot_and_reference_styles_reject(self) -> None:
        """Validate typed fixture boundaries before constructing a graph; e.g. structural_object(9, 19000, options)."""
        for kind, slot, point, curve in [(9, 0, 0, 0), (-1, 0, 0, 0), (0, -1, 0, 0), (0, 0, 3, 0), (0, 0, 0, 2)]:
            with self.assertRaisesRegex(ValueError, 'expected kind 0..8, nonnegative slot, point 0..2 and curve 0..1'):
                structural_object(kind, 19000, MemberFixtureOptions(point_style=point, curve_style=curve), slot)


if __name__ == '__main__':
    unittest.main()
