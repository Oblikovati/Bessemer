"""Preserve the existing valued-dimension fixture defaults (#54)."""
from pathlib import Path
import json
import unittest
from legacy_dimension_baselines import current_baselines


class LegacyDimensionBuilderTests(unittest.TestCase):
    def test_all_pre_correction_bodies_remain_exact_legacy_replay(self) -> None:
        """Retain 442 original hashes after the C-395 correction; e.g. python -m unittest test_legacy_dimensions."""
        expected=json.loads(Path(__file__).with_name('legacy-builder-baselines.json').read_text())
        self.assertEqual(current_baselines(),expected)
        self.assertEqual(len(expected),442)


if __name__=='__main__':
    unittest.main()
