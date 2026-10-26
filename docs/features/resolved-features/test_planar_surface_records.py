"""Planar bodies use explicit kernel framing and independent inherited wire builders."""
from pathlib import Path
import tempfile
import unittest
from freeze_planar_surfaces import WORKSPACE, freeze_surfaces


class PlanarSurfaceRecordOracleTests(unittest.TestCase):
    def test_inline_suppressed_present_body_and_keeper_cases_match_frozen_bytes(self) -> None:
        expected = WORKSPACE/'tests/fixtures/resolved-features'
        with tempfile.TemporaryDirectory(prefix='bessemer-planar-oracle-') as temporary:
            root = Path(temporary)
            report = freeze_surfaces(WORKSPACE.parent/'inventor-interface/solidworks', root)
            self.assertEqual([row['reads'] for row in report['records']], [181, 12, 7, 10, 13, 181])
            for row in report['records']:
                for suffix in ['.bin', '-primitives.tsv']:
                    name = row['stem']+suffix
                    self.assertEqual((root/name).read_bytes(), (expected/name).read_bytes())


if __name__ == '__main__':
    unittest.main()
