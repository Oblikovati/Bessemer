"""Reproduce complete routing records, parameters and geometric dimension callers (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_connections import freeze, WORKSPACE, VERSIONS
from connection_fixture_bytes import NAMES, feature_object, helper_object


class ConnectionFixtureTests(unittest.TestCase):
    def test_regeneration_matches_all_cases_owned_bytes_and_primitive_oracles(self) -> None:
        """Exercise all constructors and compatibility/identity checks; e.g. python -m unittest test_freeze_connections."""
        expected = json.loads((Path(__file__).parent/'reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination = Path(temporary)
            observed = freeze(WORKSPACE.parent/'inventor-interface/solidworks', destination)
            for key in ('cases', 'owned', 'packet_sha256'):
                self.assertEqual(observed[key], expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),
                    (WORKSPACE/'tests/fixtures/connection-points'/fixture.name).read_bytes(), fixture.name)

    def test_matrix_retains_gates_direct_parameters_and_dimension_handle_callers(self) -> None:
        """Keep all 124 complete cases and 19,502 observations; e.g. 0x371e in VERSIONS."""
        self.assertTrue({0x2f85, 0x2f86, 0x371d, 0x371e, 0x42ee, 0x42ef} <= set(VERSIONS))
        facts = json.loads((Path(__file__).parent/'reference.json').read_text())
        self.assertEqual(len(facts['cases']), 124)
        self.assertEqual(sum(row['reads'] for row in facts['cases']), 19502)
        self.assertEqual([sum(row['kind'] == kind for row in facts['cases']) for kind in range(5)], [24, 24, 16, 48, 12])

    def test_invalid_feature_and_helper_boundaries_fail(self) -> None:
        """Reject unknown modes before constructing bytes; e.g. feature_object(NAMES[0], 19000, True, -1)."""
        for name, style, slot in [('unknown', 0, 0), (NAMES[0], -1, 0), (NAMES[1], 2, 0), (NAMES[0], 0, -1)]:
            with self.assertRaisesRegex(ValueError, 'expected known class/style and nonnegative slot'):
                feature_object(name, 19000, True, style, slot)
        for kind, flags in [(-1, 0), (3, 0), (1, 1)]:
            with self.assertRaisesRegex(ValueError, 'expected kind 0..2 and known geometric flags'):
                helper_object(kind, 19000, flags=flags)


if __name__ == '__main__':
    unittest.main()
