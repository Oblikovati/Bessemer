"""Sensor fixture generation observes the independent reader and injected output roots."""
from pathlib import Path
import tempfile
import unittest
from freeze_sensors import WORKSPACE, freeze_sensors


class SensorRecordOracleTests(unittest.TestCase):
    def test_all_precision_cases_full_sensor_and_edit_match_frozen_bytes(self) -> None:
        expected = WORKSPACE/'tests/fixtures/resolved-features'
        with tempfile.TemporaryDirectory(prefix='bessemer-sensor-oracle-') as temporary:
            root = Path(temporary)
            report = freeze_sensors(WORKSPACE.parent/'inventor-interface/solidworks', root)
            self.assertEqual(report['data_cases'], 32)
            self.assertEqual([row['reads'] for row in report['objects']], [149, 149, 103])
            self.assertEqual((root/'sensor-data-cases.bin').read_bytes(), (expected/'sensor-data-cases.bin').read_bytes())
            for row in report['objects']:
                for suffix in ['.bin', '-primitives.tsv']:
                    name = row['stem']+suffix
                    self.assertEqual((root/name).read_bytes(), (expected/name).read_bytes())


if __name__ == '__main__':
    unittest.main()
