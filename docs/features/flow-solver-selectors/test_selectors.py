"""Independent selector corpus boundaries and deterministic regeneration."""
from pathlib import Path
import tempfile
import unittest
import freeze_selectors as fixtures


class SolverFixtureTests(unittest.TestCase):
    def test_frozen_decisions_use_last_physical_occurrence(self):
        raw, identifiers = fixtures.build_carrier()
        rows = fixtures.select_decisions(raw, identifiers)
        self.assertEqual(len(rows), len(identifiers) + 1)
        for line in rows[:-1]:
            identifier, decision = line.split('\t')
            value = int(identifier)
            if value in (0xffffff9b, 0xffffff9c, 0xffffff9d):
                self.assertEqual(decision, 'reserved')
                continue
            self.assertEqual(int(decision), 2 * len(identifiers) - 1 - identifiers.index(value))
        self.assertEqual(rows[-1], '305419896\t-1')

    def test_owned_boundary_and_bit_patterns(self):
        _, identifiers = fixtures.build_carrier()
        values = set(identifiers)
        self.assertTrue(set(range(256)) <= values)
        self.assertTrue(set(range(0xffffff00, 0x100000000)) <= values)
        for bit in range(32):
            self.assertIn(1 << bit, values)
            self.assertIn(0xffffffff ^ (1 << bit), values)

    def test_regeneration_matches_owned_bytes(self):
        with tempfile.TemporaryDirectory() as folder:
            root = Path(folder)
            (root / 'docs/features/flow-solver-selectors').mkdir(parents=True)
            fixtures.freeze(root)
            for relative in ('tests/fixtures/flow-solver-selectors/carrier.bin',
                             'tests/fixtures/flow-solver-selectors/decisions.tsv',
                             'docs/features/flow-solver-selectors/baseline-verification.json'):
                self.assertEqual((root / relative).read_bytes(), (fixtures.ROOT / relative).read_bytes())


if __name__ == '__main__':
    unittest.main()
