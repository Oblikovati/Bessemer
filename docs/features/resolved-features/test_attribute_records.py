"""Independent owned attribute and parameter graphs have repeatable byte partitions."""
from pathlib import Path
import tempfile
import unittest
from freeze_attributes import WORKSPACE, freeze_attributes


class AttributeRecordOracleTests(unittest.TestCase):
    def test_complete_graphs_parameter_modes_and_edit_match_frozen_bytes(self) -> None:
        expected = WORKSPACE/'tests/fixtures/resolved-features'
        with tempfile.TemporaryDirectory(prefix='bessemer-attribute-oracle-') as temporary:
            root = Path(temporary)
            report = freeze_attributes(WORKSPACE.parent/'inventor-interface/solidworks', root)
            self.assertEqual([row['reads'] for row in report['records']], [149, 52, 149, 20, 22, 23, 26])
            for row in report['records']:
                for suffix in ['.bin', '-primitives.tsv']:
                    name = row['stem']+suffix
                    self.assertEqual((root/name).read_bytes(), (expected/name).read_bytes())


if __name__ == '__main__':
    unittest.main()
