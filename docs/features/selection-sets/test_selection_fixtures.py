"""Regenerate node-based selection fixtures through the independent reader (#54)."""
from pathlib import Path
import json,tempfile,unittest
from freeze_selection_sets import REFERENCE,WORKSPACE,freeze,fixture_family


class SelectionFixtureTests(unittest.TestCase):
    def test_every_graph_owned_document_and_edit_golden_regenerates(self) -> None:
        """Require identical primitive partitions and complete owned bytes."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination,fixture_family())
            for key in ('cases','owned','reads','packet_sha256'):
                self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/selection-sets'/fixture.name).read_bytes())
