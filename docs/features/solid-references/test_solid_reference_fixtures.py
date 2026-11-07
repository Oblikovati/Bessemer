"""Regenerate complete solid-reference branches and independent raw-word edits (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_solid_references import freeze, REFERENCE, WORKSPACE


class SolidReferenceFixtureTests(unittest.TestCase):
    def test_complete_graphs_shared_targets_and_unsigned_edits_regenerate(self) -> None:
        """Compare every byte, primitive span and slot after parsed target checks; e.g. python -m unittest test_solid_reference_fixtures."""
        expected = json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination = Path(temporary);actual = freeze(REFERENCE, destination)
            for key in ('cases', 'owned', 'reads', 'packet_sha256'):
                self.assertEqual(actual[key], expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(), (WORKSPACE/'tests/fixtures/solid-references'/fixture.name).read_bytes())

    def test_all_wrapper_boundaries_presence_values_and_body_modes_are_present(self) -> None:
        """Keep full signed presence, independent suppression and explicit serialized bodies; e.g. presence i32::MIN."""
        facts = json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']), 243)
        self.assertEqual(facts['reads'], 25938)
        self.assertEqual({row['kind'] for row in facts['cases']}, set(range(9)))
        self.assertEqual(len({row['version'] for row in facts['cases']}), 9)
        self.assertEqual({row['label'].split('/')[1] for row in facts['cases']}, {'0','-1','1','-2147483648','2147483647'})
        self.assertEqual([row['slots'] for row in facts['owned']], [99,101,101,101,103,103,105,107,107,107])


if __name__ == '__main__':
    unittest.main()
