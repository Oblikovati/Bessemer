"""Regenerate complete weld-corner records and independent signed-word edits (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_weld_corners import freeze, REFERENCE, WORKSPACE


class WeldCornerFixtureTests(unittest.TestCase):
    def test_complete_graphs_shared_targets_and_edit_goldens_regenerate(self) -> None:
        """Compare every byte, primitive span and slot after source-parsed identity checks; e.g. python -m unittest test_weld_corner_fixtures."""
        expected = json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination = Path(temporary);actual = freeze(REFERENCE, destination)
            for key in ('cases', 'owned', 'reads', 'packet_sha256'):
                self.assertEqual(actual[key], expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(), (WORKSPACE/'tests/fixtures/weld-corners'/fixture.name).read_bytes())

    def test_all_four_classes_independent_arrays_and_late_word_boundary_are_present(self) -> None:
        """Keep empty/fresh/shared bodies and exact 334D/334E versions; e.g. raw signed words and unsigned dwords."""
        facts = json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']), 120)
        self.assertEqual(facts['reads'], 20594)
        self.assertEqual({row['kind'] for row in facts['cases']}, set(range(24)))
        self.assertEqual({row['version'] for row in facts['cases']}, {0x27cc,0x334d,0x334e,19000,0xffffffff})
        self.assertEqual(len(facts['owned']), 25)
        self.assertEqual(facts['owned'][-1]['slots'], 117)


if __name__ == '__main__':
    unittest.main()
