"""Keep the frozen boundary baseline reproducible and independent of the Rust API."""
import gzip
import tempfile
import unittest
from pathlib import Path
import freeze_grid as baseline


class GridBaselineTests(unittest.TestCase):
    def test_regeneration_matches_owned_fixture(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            baseline.freeze(root)
            relative = Path("tests/fixtures/flow-base-grid/grid.bin.gz")
            self.assertEqual(gzip.decompress((root / relative).read_bytes()),
                             gzip.decompress((baseline.ROOT / relative).read_bytes()))

    def test_zero_dimensions_keep_boundary_words_and_empty_bounds(self) -> None:
        for _, raw in baseline.owned_cases():
            parsed = baseline.flowbasegrid.read_decoded(raw)
            self.assertEqual(sum(a["count"] for a in parsed["boundary_arrays"]),
                             sum(n + 1 for n in parsed["dimensions"]))
            self.assertEqual(parsed["end"], len(raw))
            for dimension, bounds in zip(parsed["dimensions"], parsed["default_tree_bounds"]):
                self.assertEqual(bounds, (0, dimension - 1))
            self.assertFalse(parsed["native_refinement_reproduced"])
            self.assertFalse(parsed["native_object_indexing_validated"])


if __name__ == "__main__":
    unittest.main()
