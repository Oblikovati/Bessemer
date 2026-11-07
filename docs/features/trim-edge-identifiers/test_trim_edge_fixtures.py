"""Regenerate full left/right trim-edge graphs and both signed-word edits (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_trim_edges import freeze,REFERENCE,WORKSPACE


class TrimEdgeFixtureTests(unittest.TestCase):
    def test_complete_graphs_and_both_edits_regenerate(self) -> None:
        """Compare complete byte partitions and parsed target aliases; e.g. python -m unittest test_trim_edge_fixtures."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination)
            for key in ('cases','owned','reads','packet_sha256'):
                self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/trim-edge-identifiers'/fixture.name).read_bytes())

    def test_both_native_classes_complete_sources_and_declared_owners_are_present(self) -> None:
        """Keep both finite classes, signed extremes and direct versus tagged entity wrappers; e.g. styles 4 and 5."""
        facts=json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']),60);self.assertEqual(facts['reads'],11292)
        self.assertEqual({row['kind'] for row in facts['cases']},set(range(12)))
        self.assertEqual(len({row['version'] for row in facts['cases']}),5)
        self.assertEqual(len(facts['owned']),14)


if __name__=='__main__':
    unittest.main()
