"""Check deterministic sweep fixtures and constructor boundaries (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_sweeps import freeze, VERSIONS, WORKSPACE
from sweep_fixture_bytes import SweepOptions, sweep_object, helper_object


class SweepFixtureTests(unittest.TestCase):
    def test_regeneration_matches_all_facts_and_owned_oracles(self) -> None:
        """Exercise constructors against independent readers; e.g. python -m unittest test_freeze_sweeps."""
        expected = json.loads((Path(__file__).parent/'reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination = Path(temporary)
            observed = freeze(WORKSPACE.parent/'inventor-interface/solidworks', destination)
            for key in ('cases', 'owned', 'packet_sha256'):
                self.assertEqual(observed[key], expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),
                    (WORKSPACE/'tests/fixtures/sweeps'/fixture.name).read_bytes(), fixture.name)

    def test_matrix_keeps_sweep_and_wrapper_version_boundaries(self) -> None:
        """Require both sides of the sweep tail gate; e.g. 0x27da in VERSIONS."""
        self.assertTrue({0x27d9, 0x27da} <= set(VERSIONS))
        facts = json.loads((Path(__file__).parent/'reference.json').read_text())
        self.assertEqual(len(facts['cases']), 160)
        self.assertEqual([sum(row['kind'] == kind for row in facts['cases']) for kind in range(6)], [64, 8, 8, 8, 8, 64])

    def test_invalid_slots_enablement_and_helper_classes_fail(self) -> None:
        """Reject inputs outside fixture contracts; e.g. sweep_object(19000, False, SweepOptions(), -1)."""
        for slot, enabled in [(-1, 0), (0, 256)]:
            with self.assertRaisesRegex(ValueError, 'nonnegative slot and u8 enablement'):
                sweep_object(19000, False, SweepOptions(enabled), slot)
        with self.assertRaisesRegex(ValueError, 'expected one of'):
            helper_object('unknown', 19000, False)
        with self.assertRaisesRegex(ValueError, 'expected moSweep_c or moSweepCut_c'):
            sweep_object(19000, False, SweepOptions(), name='unknown')


if __name__ == '__main__':
    unittest.main()
