"""Regenerate complete surface-cut/trim graphs and both edit goldens (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_surface_operations import freeze, REFERENCE, WORKSPACE


class SurfaceOperationFixtureTests(unittest.TestCase):
    def test_complete_graphs_and_both_signed_word_edits_regenerate(self) -> None:
        """Compare bytes, primitive spans, direct ownership and aliases; e.g. python -m unittest test_surface_operation_fixtures."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination)
            for key in ('cases','owned','reads','packet_sha256'):
                self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/surface-operations'/fixture.name).read_bytes())

    def test_direct_members_and_complete_nested_version_boundaries_are_present(self) -> None:
        """Keep both complete classes across inherited profile/face gates; e.g. 0x332c and 0x332d."""
        facts=json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']),84)
        self.assertEqual(facts['reads'],33927)
        self.assertEqual({row['kind'] for row in facts['cases']},set(range(12)))
        self.assertEqual(len({row['version'] for row in facts['cases']}),7)
        self.assertEqual(len(facts['owned']),14)


if __name__=='__main__':
    unittest.main()
