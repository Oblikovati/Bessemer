"""Regenerate complete helix branches and wire-edge identifier edit goldens (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_helices import freeze, REFERENCE, WORKSPACE


class HelixFixtureTests(unittest.TestCase):
    def test_complete_reference_graphs_and_isolated_edits_regenerate(self) -> None:
        """Compare every byte, observed primitive and map slot; e.g. python -m unittest test_helix_fixtures."""
        expected = json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination = Path(temporary);actual = freeze(REFERENCE, destination)
            for key in ('cases', 'owned', 'reads', 'packet_sha256'):
                self.assertEqual(actual[key], expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(), (WORKSPACE/'tests/fixtures/helices'/fixture.name).read_bytes())

    def test_presence_threshold_versions_and_full_signed_identifier_range_are_present(self) -> None:
        """Keep adjacent doubles, NaN payloads, infinities and independent presence predicates; e.g. flags 1 and 255."""
        facts = json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']), 243)
        self.assertEqual(facts['reads'], 110808)
        self.assertEqual({row['kind'] for row in facts['cases']}, set(range(6)))
        threshold = [row for row in facts['cases'] if row['kind'] == 0]
        self.assertEqual(len({row['label'].split('/')[1] for row in threshold}), 14)
        self.assertEqual({row['label'].split('/')[2] for row in threshold}, {'0','1','2','255'})
        self.assertEqual([row['slots'] for row in facts['owned']], [101,101,101,101,105,105,107,103,101,111,99,105,113,111,113])


if __name__ == '__main__':
    unittest.main()
