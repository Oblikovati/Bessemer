"""Final browser serializers are independent of the Rust dispatcher and writer."""
from pathlib import Path
import struct
import sys
import tempfile
import unittest
from freeze_browser_items import WORKSPACE, CLASSES, browser_object, freeze_browser_items, source_module


class BrowserItemOracleTests(unittest.TestCase):
    def test_fixture_rejects_unknown_classes_and_reference_reads_the_edited_section_word(self) -> None:
        with self.assertRaises(ValueError):
            browser_object('moUnknownBrowser_c')
        reference = WORKSPACE.parent / 'inventor-interface/solidworks'
        sys.path.insert(0, str(reference))
        source_module('modelfeatures', reference)
        raw = browser_object('moLiveSectionFolder_c')[:-4]+struct.pack('<i', -2147483648)
        cursor = source_module('mfcobjects', reference).ObjectArchive(raw, 19000)
        cursor.feature_context = 'ordinary_external_xml'
        self.assertEqual(cursor.object()['raw_370'], -2147483648)
        self.assertEqual(cursor.p, len(raw))

    def test_final_runtime_classes_match_the_frozen_primitive_partitions(self) -> None:
        expected = WORKSPACE / 'tests/fixtures/resolved-features'
        with tempfile.TemporaryDirectory(prefix='bessemer-browser-oracle-') as temporary:
            root = Path(temporary)
            report = freeze_browser_items(WORKSPACE.parent / 'inventor-interface/solidworks', root)
            self.assertEqual([row['reads'] for row in report['records']], [10, 10, 11, 11, 51, 51, 50])
            for name in CLASSES:
                for suffix in ['.bin', '-primitives.tsv']:
                    self.assertEqual((root / (name+suffix)).read_bytes(), (expected / (name+suffix)).read_bytes())


if __name__ == '__main__':
    unittest.main()
