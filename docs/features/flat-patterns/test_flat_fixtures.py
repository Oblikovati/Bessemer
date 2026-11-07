"""Check complete flat-pattern fixtures, kernel presence and duplicate pairs (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_flat_patterns import freeze, WORKSPACE, REFERENCE, VERSIONS
from flat_fixture_bytes import FlatFixtureOptions, flat_object, box_wire


class FlatFixtureTests(unittest.TestCase):
    def test_regeneration_matches_all_complete_records_and_owned_documents(self) -> None:
        """Compare every independent oracle and emitted byte; e.g. python -m unittest test_flat_fixtures."""
        expected = json.loads((Path(__file__).parent/'reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination = Path(temporary)
            actual = freeze(REFERENCE, destination)
            for key in ('cases', 'owned', 'packet_sha256'):
                self.assertEqual(actual[key], expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(), (WORKSPACE/'tests/fixtures/flat-patterns'/fixture.name).read_bytes(), fixture.name)

    def test_matrix_covers_independent_gates_both_states_and_both_kernel_forms(self) -> None:
        """Pin direct boxes and present/absent raw kernels; e.g. 0x372b excludes raw_9b4."""
        facts = json.loads((Path(__file__).parent/'reference.json').read_text())
        self.assertEqual(len(facts['cases']), 144)
        self.assertEqual({kind: sum(row['kind'] == kind for row in facts['cases']) for kind in (0, 1, 2, 3, 4, 8)},
                         {0: 24, 1: 24, 2: 24, 3: 24, 4: 24, 8: 24})
        self.assertTrue({0x3364, 0x3365, 0x372b, 0x372c, 0x3761, 0x3762} <= set(VERSIONS))
        self.assertEqual(len(box_wire(True)), 108)
        self.assertEqual(len(box_wire(False)), 108)

    def test_invalid_fixture_class_or_imported_slot_rejects(self) -> None:
        """Constructor validation precedes archive emission; e.g. flat_object(19000, options, -1)."""
        for slot, name in [(-1, 'moFlatPattern_c'), (0, 'unknown-flat-class')]:
            with self.assertRaisesRegex(ValueError, 'expected nonnegative slot and flat-pattern/box class'):
                flat_object(19000, FlatFixtureOptions(), slot, name)


if __name__ == '__main__':
    unittest.main()
