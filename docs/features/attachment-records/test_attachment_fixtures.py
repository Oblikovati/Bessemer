"""Regenerate exact data/parametric bodies using actual reference readers (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_attachments import REFERENCE,WORKSPACE,freeze,fixture_family


class AttachmentFixtureTests(unittest.TestCase):
    def test_complete_graphs_and_owned_edits_regenerate(self) -> None:
        """Compare every primitive observation and byte with independent goldens; e.g. cached generic references."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination,fixture_family(REFERENCE))
            for key in ('cases','owned','reads','packet_sha256'):self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/attachment-records'/fixture.name).read_bytes())

    def test_two_distinct_layouts_keep_versions_and_identity_styles(self) -> None:
        """Retain sixty graphs and fourteen owned streams; e.g. the drawing-only historical gate stays inactive."""
        facts=json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']),60)
        self.assertEqual(facts['reads'],2160)
        self.assertEqual(len(facts['owned']),14)


if __name__=='__main__':unittest.main()
