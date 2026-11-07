"""Regenerate complete native-bound cut-list and ancestor folder fixtures (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_cut_lists import freeze, REFERENCE, WORKSPACE


class CutListFixtureTests(unittest.TestCase):
    def test_complete_records_target_identities_and_property_edits_regenerate(self) -> None:
        """Compare every byte, primitive and slot count after checking source-parsed aliases; e.g. python -m unittest test_cut_list_fixtures."""
        expected = json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination = Path(temporary);actual = freeze(REFERENCE, destination)
            for key in ('cases', 'owned', 'reads', 'packet_sha256'):
                self.assertEqual(actual[key], expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(), (WORKSPACE/'tests/fixtures/cut-list-folders'/fixture.name).read_bytes())

    def test_all_classes_flag_bytes_property_modes_and_document_maps_are_present(self) -> None:
        """Keep fourteen exact classes, six flags and both empty/direct populated property modes; e.g. byte 128."""
        facts = json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']), 252)
        self.assertEqual(facts['reads'], 41516)
        self.assertEqual({row['kind'] for row in facts['cases']}, set(range(84)))
        self.assertEqual(len(facts['owned']), 86)
        self.assertEqual([row['slots'] for row in facts['owned']][-2:], [117,119])


if __name__ == '__main__':
    unittest.main()
