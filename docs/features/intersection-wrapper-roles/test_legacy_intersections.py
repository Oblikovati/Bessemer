"""Keep preexisting sketch-tail fixture bytes and identities unchanged (#46)."""
from pathlib import Path
import json
import unittest
from legacy_intersection_baselines import current_baselines


class LegacyIntersectionBuilderTests(unittest.TestCase):
    def test_original_builders_remain_byte_and_slot_identical(self) -> None:
        """Compare 448 owned defaults and signed-count branches; e.g. python -m unittest test_legacy_intersections."""
        expected=json.loads(Path(__file__).with_name('legacy-builder-baselines.json').read_text())
        self.assertEqual(current_baselines(),expected)
        self.assertEqual(len(expected),448)


if __name__=='__main__':
    unittest.main()
