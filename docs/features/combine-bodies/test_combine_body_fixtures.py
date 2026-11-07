"""Regenerate complete combine/delete features and independent edits (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_combine_bodies import freeze, REFERENCE, WORKSPACE


class CombineBodyFixtureTests(unittest.TestCase):
    def test_complete_graphs_and_signed_word_edit_regenerate(self) -> None:
        """Compare independent bytes, spans and slots after identity checks; e.g. python -m unittest test_combine_body_fixtures."""
        expected = json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination = Path(temporary);actual = freeze(REFERENCE, destination)
            for key in ('cases','owned','reads','packet_sha256'):
                self.assertEqual(actual[key], expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(), (WORKSPACE/'tests/fixtures/combine-bodies'/fixture.name).read_bytes())

    def test_complete_native_classes_and_strict_option_gate_are_covered(self) -> None:
        """Keep three complete classes across both sides of the native option boundary; e.g. 0x3f07 and 0x3f08."""
        facts = json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']), 90)
        self.assertEqual(facts['reads'], 16911)
        self.assertEqual({row['kind'] for row in facts['cases']}, set(range(18)))
        self.assertEqual({row['version'] for row in facts['cases']}, {0x27cc,0x3f07,0x3f08,19000,0xffffffff})
        self.assertEqual(len(facts['owned']), 19)


if __name__ == '__main__':
    unittest.main()
