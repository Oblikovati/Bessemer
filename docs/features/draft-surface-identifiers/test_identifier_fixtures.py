"""Regenerate complete draft surface-identifier graphs and isolated edit goldens (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_draft_identifiers import freeze, REFERENCE, WORKSPACE


class DraftIdentifierFixtureTests(unittest.TestCase):
    def test_complete_reference_graphs_and_single_identifier_edits_regenerate(self) -> None:
        """Compare every emitted byte, primitive and map slot; e.g. python -m unittest test_identifier_fixtures."""
        expected = json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination = Path(temporary);actual = freeze(REFERENCE, destination)
            for key in ('cases', 'owned', 'reads', 'packet_sha256'):
                self.assertEqual(actual[key], expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(), (WORKSPACE/'tests/fixtures/draft-surface-identifiers'/fixture.name).read_bytes())

    def test_both_classes_pointer_modes_and_full_signed_word_range_are_present(self) -> None:
        """Keep null/fresh/shared references and distinct equal records; e.g. i32 minimum, zero and maximum."""
        facts = json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']), 54)
        self.assertEqual(facts['reads'], 5016)
        self.assertEqual({row['kind'] for row in facts['cases']}, set(range(6)))
        self.assertEqual({row['label'].split('/')[-1] for row in facts['cases']}, {'-2147483648', '0', '2147483647'})
        self.assertEqual([row['slots'] for row in facts['owned']], [99, 105, 113, 99, 105, 113, 113, 113])


if __name__ == '__main__':
    unittest.main()
