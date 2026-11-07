"""Check complete dome regeneration and construction boundaries (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_domes import freeze, WORKSPACE, VERSIONS
from dome_fixture_bytes import dome_object


class DomeFixtureTests(unittest.TestCase):
    def test_regeneration_matches_every_case_and_owned_stream(self) -> None:
        """Exercise inherited constructors and pointer checks; e.g. python -m unittest test_freeze_domes."""
        expected = json.loads((Path(__file__).parent/'reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination = Path(temporary)
            observed = freeze(WORKSPACE.parent/'inventor-interface/solidworks', destination)
            for key in ('cases', 'owned', 'packet_sha256'):
                self.assertEqual(observed[key], expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),
                    (WORKSPACE/'tests/fixtures/domes'/fixture.name).read_bytes(), fixture.name)

    def test_matrix_covers_all_nullable_combinations_and_shared_base_versions(self) -> None:
        """Retain all 128 complete cases; e.g. 0x3b01 in VERSIONS."""
        self.assertTrue({0x27cc, 0x3b00, 0x3b01, 19000} <= set(VERSIONS))
        facts = json.loads((Path(__file__).parent/'reference.json').read_text())
        self.assertEqual(len(facts['cases']), 128)
        self.assertEqual(sum(row['reads'] for row in facts['cases']), 42006)
        self.assertEqual({row['kind'] for row in facts['cases']}, set(range(16)))

    def test_invalid_nullable_masks_and_negative_slots_fail(self) -> None:
        """Reject fixture values outside the schema matrix; e.g. dome_object(19000, False, 16)."""
        for mask, slot in [(-1, 0), (16, 0), (0, -1)]:
            with self.assertRaisesRegex(ValueError, 'expected mask 0..15 and nonnegative slot'):
                dome_object(19000, False, mask, slot)


if __name__ == '__main__':
    unittest.main()
