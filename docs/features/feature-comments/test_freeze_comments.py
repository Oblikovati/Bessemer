"""Regenerate complete comments and pin direct storage/version boundaries (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_comments import freeze, WORKSPACE, VERSIONS
from comment_fixture_bytes import comment_object, extra, text_item


class CommentFixtureTests(unittest.TestCase):
    def test_regeneration_matches_all_owned_bytes_and_primitive_oracles(self) -> None:
        """Exercise every constructor and identity check; e.g. python -m unittest test_freeze_comments."""
        expected = json.loads((Path(__file__).parent/'reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination = Path(temporary)
            observed = freeze(WORKSPACE.parent/'inventor-interface/solidworks', destination)
            for key in ('cases', 'owned', 'packet_sha256'):
                self.assertEqual(observed[key], expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),
                    (WORKSPACE/'tests/fixtures/feature-comments'/fixture.name).read_bytes(), fixture.name)

    def test_matrix_retains_raw_kind_time_width_direct_extra_and_all_string_occurrences(self) -> None:
        """Pin native migrations without applying them; e.g. extra(0x3b0f, True) == b''."""
        self.assertTrue({0x332b, 0x332c, 0x3b0f, 0x3b10} <= set(VERSIONS))
        facts = json.loads((Path(__file__).parent/'reference.json').read_text())
        self.assertEqual(len(facts['cases']), 48)
        self.assertEqual(sum(row['reads'] for row in facts['cases']), 910)
        self.assertEqual(extra(0x3b0f, True), b'')
        self.assertEqual(extra(0x3b10, True).hex(), '00000080ffffff7fffffffff')
        self.assertEqual(len(text_item(0x3b0f, False)), 11)
        self.assertEqual(len(text_item(0x3b10, False)), 12)

    def test_invalid_kind_and_imported_slot_boundaries_fail(self) -> None:
        """Reject fixture inputs before constructing objects; e.g. comment_object(-1, 19000, True)."""
        for kind, slot in [(-1, 0), (3, 0), (0, -1)]:
            with self.assertRaisesRegex(ValueError, 'expected kind 0..2 and nonnegative slot'):
                comment_object(kind, 19000, True, first_slot=slot)


if __name__ == '__main__':
    unittest.main()
