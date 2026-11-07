"""Reproduce the corrected spline baseline through the independent reader (#95)."""
from pathlib import Path
import sys,unittest
ROOT=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(ROOT/'docs/features/model-archives'))
import freeze_sketch_splines as fixtures

class EquationFixtureTests(unittest.TestCase):
    """Check every retained graph and both distinct edit goldens."""
    def test_fixed_operator_regeneration_matches_every_owned_byte(self) -> None:
        """Use the real complete reader, including its native fixed-transform guard."""
        rows=[r for v in (0x27cc,19000) for r in fixtures.spline_cases(v)]+fixtures.array_cases()
        self.assertEqual(len(rows),934)
        self.assertEqual(sum(n for _,n in rows),86266)
        folder=ROOT/'tests/fixtures/model-archives'
        self.assertEqual(b''.join(raw for raw,_ in rows),(folder/'sketch-spline-cases.bin').read_bytes())
        for name,raw in fixtures.owned_wires().items():
            self.assertEqual(raw,(folder/name).read_bytes())
