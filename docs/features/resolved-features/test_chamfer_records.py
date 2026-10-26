"""Independent chamfer suffix framing and inherited bodies are reproducible."""
from pathlib import Path
import tempfile
import unittest
from freeze_chamfers import WORKSPACE, freeze_chamfers


class ChamferRecordOracleTests(unittest.TestCase):
    def test_chamfer_objects_direct_keeper_and_edit_match_frozen_owned_bytes(self) -> None:
        expected = WORKSPACE/'tests/fixtures/resolved-features'
        with tempfile.TemporaryDirectory(prefix='bessemer-chamfer-oracle-') as temporary:
            root = Path(temporary)
            report = freeze_chamfers(WORKSPACE.parent/'inventor-interface/solidworks', root)
            self.assertEqual([row['reads'] for row in report['records']], [35, 35, 124])
            for row in report['records']:
                for suffix in ['.bin', '-primitives.tsv']:
                    name = row['stem']+suffix
                    self.assertEqual((root/name).read_bytes(), (expected/name).read_bytes())


if __name__ == '__main__':
    unittest.main()
