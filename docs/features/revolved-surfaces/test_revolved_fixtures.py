"""Regenerate complete revolved-surface records and independent signed-word edits (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_revolved import freeze, REFERENCE, WORKSPACE


class RevolvedFixtureTests(unittest.TestCase):
    def test_complete_reference_graphs_and_signed_word_edits_regenerate(self) -> None:
        """Compare every byte, primitive and map slot; e.g. python -m unittest test_revolved_fixtures."""
        expected = json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination = Path(temporary);actual = freeze(REFERENCE, destination)
            for key in ('cases', 'owned', 'reads', 'packet_sha256'):
                self.assertEqual(actual[key], expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(), (WORKSPACE/'tests/fixtures/revolved-surfaces'/fixture.name).read_bytes())

    def test_all_three_classes_version_boundaries_and_inline_kernel_are_present(self) -> None:
        """Keep empty/fresh/shared/rich graphs and a typed kernel; e.g. a nonzero backend marker."""
        facts = json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']), 253)
        self.assertEqual(facts['reads'], 61722)
        self.assertEqual({row['kind'] for row in facts['cases']}, set(range(13)))
        self.assertEqual(len({row['version'] for row in facts['cases']}), 7)
        self.assertEqual([row['slots'] for row in facts['owned']], [101,109,115,119,99,103,107,107,99,103,103,103,115,107,111])


if __name__ == '__main__':
    unittest.main()
