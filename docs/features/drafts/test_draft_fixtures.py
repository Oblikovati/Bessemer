"""Regenerate complete drafts and preserve independently verified count/stride boundaries (#54)."""
from pathlib import Path
import json
import struct
import tempfile
import unittest
from freeze_drafts import freeze, REFERENCE, WORKSPACE, source_module


class DraftFixtureTests(unittest.TestCase):
    def test_complete_reference_observations_and_word_edit_regenerate(self) -> None:
        """Compare every emitted byte, primitive and object slot; e.g. python -m unittest test_draft_fixtures."""
        expected = json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination = Path(temporary);actual = freeze(REFERENCE, destination)
            for key in ('cases', 'owned', 'reads', 'packet_sha256'):
                self.assertEqual(actual[key], expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(), (WORKSPACE/'tests/fixtures/drafts'/fixture.name).read_bytes())

    def test_all_classes_version_gates_and_distinct_payload_word_projection_are_present(self) -> None:
        """Keep three complete classes, five versions and six independent styles; e.g. stride 24 is not three elements."""
        facts = json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']), 90)
        self.assertEqual(facts['reads'], 18383)
        self.assertEqual(sum(row['source_reads'] for row in facts['cases']), 18343)
        self.assertEqual({row['kind'] for row in facts['cases']}, set(range(18)))
        self.assertEqual({row['version'] for row in facts['cases']}, {0x27cc, 0x3746, 0x3747, 19000, 0xffffffff})
        self.assertEqual([row['slots'] for row in facts['owned']], [101, 115, 115, 115, 117, 115, 115, 103, 103])

    def test_independent_minima_and_empty_large_stride_keep_the_following_sentinel(self) -> None:
        """Check both version gates and the zero-element descriptor exception; e.g. size 0xfffffff8 with count zero."""
        objects = source_module('mfcobjects', REFERENCE);drafts = source_module('drafts', REFERENCE)
        for version, size in ((0x27cc, 311), (0x3746, 311), (0x3747, 325), (19000, 325)):
            cursor = objects.ObjectArchive(bytes(size)+b'NEXT', version)
            cursor.feature_context = 'ordinary_external_xml';cursor.model_root_class = 'moPart_c';drafts.read(cursor)
            self.assertEqual((cursor.p, len(cursor.map), cursor.b[cursor.p:]), (size, 1, b'NEXT'))
        raw = struct.pack('<4I', 0xfffffff8, 3, 255, 0)
        cursor = objects.ObjectArchive(raw+b'NEXT', 19000)
        cursor.feature_context = 'ordinary_external_xml';cursor.model_root_class = 'moPart_c'
        result = source_module('sheetfeatures', REFERENCE).dynamic_array(cursor)
        self.assertEqual((cursor.p, result['raw_data'], cursor.b[cursor.p:]), (16, b'', b'NEXT'))


if __name__ == '__main__':
    unittest.main()
