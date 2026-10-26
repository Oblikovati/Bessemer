"""Independent ordinary scene framing reuses the established owned scalar builders."""
from pathlib import Path
import tempfile
import unittest
from freeze_configuration_scenes import WORKSPACE, freeze_scenes


class ConfigurationSceneOracleTests(unittest.TestCase):
    def test_complete_scenes_maps_and_edit_match_frozen_independent_partitions(self) -> None:
        expected = WORKSPACE / 'tests/fixtures/resolved-features'
        with tempfile.TemporaryDirectory(prefix='bessemer-configuration-scene-') as temporary:
            root = Path(temporary)
            report = freeze_scenes(WORKSPACE.parent / 'inventor-interface/solidworks', root)
            self.assertEqual([row['reads'] for row in report['cases']], [291, 291, 195, 291, 302])
            self.assertEqual([row['reference_preflight_reads_excluded'] for row in report['cases']], [23]*5)
            for row in report['cases']:
                for suffix in ['.bin', '-primitives.tsv']:
                    name = row['stem']+suffix
                    self.assertEqual((root / name).read_bytes(), (expected / name).read_bytes())


if __name__ == '__main__':
    unittest.main()
