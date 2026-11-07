"""Regenerate base-flange fixtures and check independent branch coverage (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
import freeze_base_flanges as fixtures
from flange_fixture_bytes import FlangeFixtureOptions, flange_object


class FlangeFixtureTests(unittest.TestCase):
    def test_regeneration_matches_every_observation_and_complete_document(self) -> None:
        """Require unchanged source EOF, slots, oracle fields and bytes; e.g. python -m unittest test_flange_fixtures."""
        expected = json.loads((Path(__file__).parent/'reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination = Path(temporary)
            actual = fixtures.freeze(fixtures.REFERENCE, destination)
            for key in ('cases', 'owned', 'packet_sha256'):
                self.assertEqual(actual[key], expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(), (fixtures.WORKSPACE/'tests/fixtures/base-flanges'/fixture.name).read_bytes(), fixture.name)

    def test_matrix_covers_saved_states_boundaries_and_signed_selector_extremes(self) -> None:
        """Keep selector/type/reference-byte variations independent; e.g. mode -2147483648 selects no direction."""
        facts = json.loads((Path(__file__).parent/'reference.json').read_text())
        self.assertEqual(len(facts['cases']), 202)
        options = [row['options'] for row in facts['cases'][48:]]
        self.assertEqual({row['mode'] for row in options}, {-2147483648, -1, 0, 1, 2, 3, 2147483647})
        self.assertEqual({row['first_type'] for row in options}, {-2147483648, -1, 0, 1, 2, 3, 4, 5, 6, 7, 2147483647})
        self.assertEqual({row['reference_byte'] for row in options}, {0, 255})
        self.assertTrue({0x3b0c, 0x3b0d, 0x46b9, 0x46ba} <= set(fixtures.VERSIONS))
        self.assertEqual([row['slots'] for row in facts['owned']], [101, 119, 121, 121])

    def test_invalid_builder_boundaries_reject_before_constructing_a_fixture(self) -> None:
        """Reject values outside persisted byte/slot shapes; e.g. reference_byte 256."""
        for flag, slot in [(-1, 0), (256, 0), (0, -1)]:
            with self.assertRaises(ValueError):
                flange_object(19000, FlangeFixtureOptions(reference_byte=flag), slot)


if __name__ == '__main__':
    unittest.main()
