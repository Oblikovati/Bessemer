"""Regenerate complete C-392 feature/helper bytes and independent edits (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_frontier_helpers import FAMILY, REFERENCE, WORKSPACE, freeze


class FrontierHelperFixtureTests(unittest.TestCase):
    def test_all_complete_graphs_owned_streams_and_edits_regenerate(self) -> None:
        """Keep actual source identities and every primitive partition; e.g. python -m unittest test_frontier_helper_fixtures."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination,FAMILY)
            for key in ('cases','owned','reads','packet_sha256'):
                self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/frontier-feature-helpers'/fixture.name).read_bytes())

    def test_all_five_classes_version_boundaries_and_primitive_widths_are_present(self) -> None:
        """Keep thirteen versions and all six IEEE/raw-byte combinations; e.g. styles 0 through 5."""
        facts=json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']),390)
        self.assertEqual(facts['reads'],77028)
        self.assertEqual({row['kind'] for row in facts['cases']},set(range(30)))
        self.assertEqual(len(facts['owned']),34)


if __name__=='__main__':
    unittest.main()
