"""Reproduce typed note/text bytes and independent primitive partitions (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_notes import REFERENCE, WORKSPACE, freeze, fixture_family


class NoteFixtureTests(unittest.TestCase):
    def test_complete_graphs_and_owned_edits_regenerate(self) -> None:
        """Compare every fixture byte against actual readers; e.g. a note-to-text cycle."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary)
            actual=freeze(REFERENCE,destination,fixture_family())
            for key in ('cases','owned','reads','packet_sha256'):
                self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/display-notes'/fixture.name).read_bytes())

    def test_all_note_tail_boundaries_and_distinct_classes_are_covered(self) -> None:
        """Retain eight types across fifteen versions and six value styles."""
        facts=json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']),720)
        self.assertEqual(facts['reads'],71832)
        self.assertEqual(len(facts['owned']),55)


if __name__=='__main__':
    unittest.main()
