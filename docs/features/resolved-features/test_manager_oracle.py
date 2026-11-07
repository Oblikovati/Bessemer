"""Owned independent CMgr evidence uses injected output roots."""
from pathlib import Path
import tempfile
import unittest
from freeze_manager_context import WORKSPACE, write_manager_oracle


class ManagerOracleTests(unittest.TestCase):
    def test_complete_observation_matches_owned_bytes_and_frozen_primitive_rows(self) -> None:
        reference = WORKSPACE.parent / 'inventor-interface/solidworks'
        fixture = WORKSPACE / 'tests/fixtures/blank-part.SLDPRT'
        expected = WORKSPACE / 'tests/fixtures/resolved-features'
        with tempfile.TemporaryDirectory(prefix='bessemer-manager-oracle-') as temporary:
            root = Path(temporary)
            report = write_manager_oracle(reference, fixture, root)
            self.assertEqual(report['stream_bytes'], 1968)
            self.assertEqual(report['primitive_reads'], 504)
            for name in ['owned-manager.bin', 'owned-manager-primitives.tsv']:
                self.assertEqual((root / name).read_bytes(), (expected / name).read_bytes())


if __name__ == '__main__':
    unittest.main()
