"""Chooser fixtures retain complete independent framing and primitive partitions."""
from pathlib import Path
import tempfile
import unittest
from freeze_body_choosers import WORKSPACE, freeze_choosers


class BodyChooserOracleTests(unittest.TestCase):
    def test_complete_variants_and_independent_edit_match_frozen_owned_bytes(self) -> None:
        expected = WORKSPACE / 'tests/fixtures/resolved-features'
        with tempfile.TemporaryDirectory(prefix='bessemer-chooser-oracle-') as temporary:
            root = Path(temporary)
            report = freeze_choosers(WORKSPACE.parent / 'inventor-interface/solidworks', root)
            self.assertEqual([row['primitive_reads'] for row in report['cases']], [18, 24, 9])
            self.assertEqual([row['bytes'] for row in report['cases']], [70, 100, 60])
            for stem in ['body-chooser', 'named-body-chooser', 'body-bounding-center', 'body-chooser-edited']:
                for suffix in ['.bin', '-primitives.tsv']:
                    self.assertEqual((root / (stem+suffix)).read_bytes(), (expected / (stem+suffix)).read_bytes())


if __name__ == '__main__':
    unittest.main()
