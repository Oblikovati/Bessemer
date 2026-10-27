"""Reproduce every selector, direct-array and coordinate edit fixture (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_coordinates import freeze, WORKSPACE, REFERENCE, VERSIONS, SELECTORS
from coordinate_fixture_bytes import coordinate_object


class CoordinateFixtureTests(unittest.TestCase):
    def test_regeneration_matches_all_complete_bytes_slots_and_primitive_oracles(self) -> None:
        """Exercise every builder and source identity check; e.g. python -m unittest test_freeze_coordinates."""
        expected = json.loads((Path(__file__).parent/'reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination = Path(temporary)
            observed = freeze(REFERENCE, destination)
            for key in ('cases', 'owned', 'packet_sha256'):
                self.assertEqual(observed[key], expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),
                    (WORKSPACE/'tests/fixtures/coordinate-systems'/fixture.name).read_bytes(), fixture.name)

    def test_matrix_covers_equality_order_derived_arrays_and_version_boundaries(self) -> None:
        """Retain duplicate/out-of-range load framing without native writer claims; e.g. (1, 1) in SELECTORS."""
        self.assertTrue({0x371a, 0x371b, 0x2f85, 0x2f86} <= set(VERSIONS))
        self.assertTrue({(1, 1), (0, 65535), (3, 1), (2, 1), (2, 3)} <= set(SELECTORS))
        facts = json.loads((Path(__file__).parent/'reference.json').read_text())
        self.assertEqual(len(facts['cases']), 304)
        self.assertEqual(sum(row['reads'] for row in facts['cases']), 61976)
        self.assertEqual([sum(row['kind'] == kind for row in facts['cases']) for kind in range(4)], [256, 16, 16, 16])
        self.assertEqual([row['slots'] for row in facts['owned']], [101, 109, 109, 111, 109, 107, 111])

    def test_invalid_selectors_styles_and_imported_slots_fail(self) -> None:
        """Reject constructor inputs outside the stored shape; e.g. coordinate_object(19000, True, (-1, 2))."""
        for selectors, style, slot in [((-1, 2), 0, 0), ((1, 65536), 0, 0), ((1,), 0, 0),
                                       ((1, 2), -1, 0), ((1, 2), 3, 0), ((1, 2), 0, -1)]:
            with self.assertRaisesRegex(ValueError, 'expected style 0..2, nonnegative slot and two u16 selectors'):
                coordinate_object(19000, True, selectors, style, slot)


if __name__ == '__main__':
    unittest.main()
