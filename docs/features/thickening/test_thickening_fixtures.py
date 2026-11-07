"""Regenerate complete thickening and reference-layer fixtures (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_thickening import freeze, REFERENCE, WORKSPACE


class ThickeningFixtureTests(unittest.TestCase):
    def test_complete_graphs_and_signed_word_edit_regenerate(self) -> None:
        """Compare independent bytes, primitive spans and identities; e.g. python -m unittest test_thickening_fixtures."""
        expected = json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination = Path(temporary);actual = freeze(REFERENCE, destination)
            for key in ('cases','owned','reads','packet_sha256'):
                self.assertEqual(actual[key], expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(), (WORKSPACE/'tests/fixtures/thickening'/fixture.name).read_bytes())

    def test_inclusive_gate_and_all_three_native_reference_layers_are_present(self) -> None:
        """Keep raw presence extremes and selected strict kernel bodies; e.g. 0x2f73 and 0x2f74."""
        facts = json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']), 126)
        self.assertEqual(facts['reads'], 40578)
        self.assertEqual({row['kind'] for row in facts['cases']}, set(range(18)))
        self.assertEqual(len({row['version'] for row in facts['cases']}), 7)
        self.assertEqual(len(facts['owned']), 19)


if __name__ == '__main__':
    unittest.main()
