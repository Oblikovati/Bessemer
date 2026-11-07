"""The independent graph oracle retains owned roots and rejects changed topology."""
from pathlib import Path
import tempfile
import unittest
from freeze_linked_atoms import WORKSPACE, freeze_atoms, require_graph


class LinkedAtomOracleTests(unittest.TestCase):
    def test_reference_graph_and_primitive_partition_match_frozen_owned_bytes(self) -> None:
        expected = WORKSPACE / 'tests/fixtures/resolved-features'
        with tempfile.TemporaryDirectory(prefix='bessemer-linked-atom-oracle-') as temporary:
            root = Path(temporary)
            report = freeze_atoms(WORKSPACE.parent / 'inventor-interface/solidworks', root)
            self.assertEqual((report['bytes'], report['primitive_reads']), (136, 37))
            for name in ['linked-atoms.bin', 'linked-atoms-primitives.tsv']:
                self.assertEqual((root / name).read_bytes(), (expected / name).read_bytes())

    def test_graph_oracle_rejects_missing_aliases(self) -> None:
        with self.assertRaises((IndexError, ValueError)):
            require_graph({'entries': []}, 0, 0)


if __name__ == '__main__':
    unittest.main()
