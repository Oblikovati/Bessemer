"""Regenerate exact shared-handle aliases and independently selected stored index encodings (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_shared_handles import freeze, REFERENCE, WORKSPACE
from shared_handle_bytes import INDICES


class SharedHandleFixtureTests(unittest.TestCase):
    def test_complete_observations_and_reference_word_edits_regenerate(self) -> None:
        """Compare every byte, primitive and map slot; e.g. python -m unittest test_shared_handle_fixtures."""
        expected = json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination = Path(temporary);actual = freeze(REFERENCE, destination)
            for key in ('cases', 'owned', 'reads', 'packet_sha256'):
                self.assertEqual(actual[key], expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(), (WORKSPACE/'tests/fixtures/shared-sketch-handles'/fixture.name).read_bytes())

    def test_both_native_classes_pointer_modes_and_full_width_indices_are_present(self) -> None:
        """Keep every sentinel boundary, signed extreme and noncanonical escape; e.g. (7, True)."""
        facts = json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']), 144)
        self.assertEqual(facts['reads'], 2696)
        self.assertEqual({row['kind'] for row in facts['cases']}, set(range(6)))
        self.assertEqual(set(INDICES), {(0, False), (0x777e, False), (0x777f, True), (0x7780, False), (65535, False), (-2147483648, True), (2147483647, True), (7, True)})
        self.assertEqual([row['slots'] for row in facts['owned']], [99, 101, 101, 99, 101, 101, 101, 101])


if __name__ == '__main__':
    unittest.main()
