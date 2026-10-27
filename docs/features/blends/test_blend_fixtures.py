"""Regenerate complete blends and verify independent counts and condition tables (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_blends import freeze, WORKSPACE, REFERENCE, DIRECTIONS, VERSIONS
from blend_fixture_bytes import BlendFixtureOptions, blend_object


class BlendFixtureTests(unittest.TestCase):
    def test_regeneration_matches_every_field_byte_slot_and_complete_document_edit(self) -> None:
        """Compare independently decoded full bodies and owned streams; e.g. python -m unittest test_blend_fixtures."""
        expected = json.loads((Path(__file__).parent/'reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination = Path(temporary)
            actual = freeze(REFERENCE, destination)
            for key in ('cases', 'owned', 'packet_sha256'):
                self.assertEqual(actual[key], expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(), (WORKSPACE/'tests/fixtures/blends'/fixture.name).read_bytes(), fixture.name)

    def test_matrix_covers_low_byte_controls_full_word_equality_and_independent_extents(self) -> None:
        """Keep 0x102/-254 direction cases distinct from 0x103 dimension equality; e.g. DIRECTIONS includes both low-byte aliases."""
        facts = json.loads((Path(__file__).parent/'reference.json').read_text())
        self.assertEqual(len(facts['cases']), 68)
        self.assertEqual(sum(row['reads'] for row in facts['cases']), 37370)
        self.assertEqual([sum(row['kind'] == kind for row in facts['cases']) for kind in range(2)], [34, 34])
        self.assertEqual(set(VERSIONS), {0x27cc, 19000, 0xffffffff})
        self.assertIn((0x102, -254), DIRECTIONS)
        options = [row['options'] for row in facts['cases']]
        self.assertTrue(any(row['declared'] == 2147483647 and row['count'] == 0 for row in options))
        self.assertTrue(any(row['declared'] == 1 and row['count'] == 2 for row in options))
        self.assertTrue(any(row['dimensions'] == 255 and row['selector'] == 0x103 for row in options))
        self.assertEqual([row['slots'] for row in facts['owned']], [101, 109, 109, 109])

    def test_invalid_class_map_count_and_byte_shapes_reject_before_emitting_bytes(self) -> None:
        """Reject malformed constructor inputs without weakening retained raw domains; e.g. dimensions 256."""
        for kind, slot, options in [(2, 0, BlendFixtureOptions()), (0, -1, BlendFixtureOptions()),
            (0, 0, BlendFixtureOptions(count=4)), (0, 0, BlendFixtureOptions(declared=-1)),
            (0, 0, BlendFixtureOptions(dimensions=256))]:
            with self.assertRaisesRegex(ValueError, 'expected kind 0..1, nonnegative slot/count'):
                blend_object(kind, 19000, options, slot)


if __name__ == '__main__':
    unittest.main()
