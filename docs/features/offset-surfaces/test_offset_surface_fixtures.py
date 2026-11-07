"""Regenerate composed offsets, complete data/keepers and independent minima (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_offset_surfaces import freeze,REFERENCE,WORKSPACE


class OffsetSurfaceFixtureTests(unittest.TestCase):
    def test_complete_graphs_minima_and_signed_word_edit_regenerate(self) -> None:
        """Compare bytes, spans, direct-array framing and parsed identities; e.g. python -m unittest test_offset_surface_fixtures."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination)
            for key in ('cases','owned','reads','packet_sha256'):
                self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/offset-surfaces'/fixture.name).read_bytes())

    def test_signed_presence_component_role_and_all_minimum_boundaries_are_present(self) -> None:
        """Keep independent native minima and full surface/data/keeper combinations; e.g. 0x2bcc and 0x2bcd."""
        facts=json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']),133);self.assertEqual(facts['reads'],35491)
        self.assertEqual({row['kind'] for row in facts['cases']},set(range(19)))
        self.assertEqual(len({row['version'] for row in facts['cases']}),7)
        self.assertEqual([row['slots'] for row in facts['cases'] if row['kind']==18],[7]*7)
        self.assertEqual(len(facts['owned']),19)


if __name__=='__main__':
    unittest.main()
