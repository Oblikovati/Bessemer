"""Keep pre-hook body and end fixture bytes unchanged (#46)."""
from pathlib import Path
import json
import unittest
from legacy_builder_baselines import current_baselines


class LegacyEndBuilderTests(unittest.TestCase):
    def test_all_default_builder_bytes_remain_unchanged(self) -> None:
        """Compare all owned original arguments against their earlier hashes; e.g. 180 cases."""
        expected=json.loads(Path(__file__).with_name('legacy-builder-baselines.json').read_text())
        self.assertEqual(current_baselines(),expected)
        self.assertEqual(len(expected),180)


if __name__=='__main__':
    unittest.main()
