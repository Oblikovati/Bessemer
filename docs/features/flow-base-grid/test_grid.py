"""Keep the frozen boundary baseline reproducible and independent of the Rust API."""
import gzip
import tempfile
import unittest
import zlib
from pathlib import Path
import freeze_grid as baseline
import process_reference


class GridBaselineTests(unittest.TestCase):
    def test_process_oracle_checks_complete_noncanonical_zlib_stream(self) -> None:
        raw = baseline.owned_cases()[0][1]
        stored = zlib.compress(raw, 1)
        process_reference.check_wire(process_reference.carrier(stored), raw, True)
        for malformed in (stored[:-1], stored + b"x", stored + stored):
            with self.assertRaises(AssertionError):
                process_reference.check_wire(process_reference.carrier(malformed), raw, True)

    def test_process_oracle_requires_whole_expected_raw_carrier(self) -> None:
        raw = baseline.owned_cases()[0][1]
        wire = process_reference.carrier(raw)
        process_reference.check_wire(wire, raw, False)
        with self.assertRaises(AssertionError):
            process_reference.check_wire(wire[:-1] + b"x", raw, False)

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
