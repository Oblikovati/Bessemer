"""Independent closed-form stride and local path checks for the frozen projection."""
import gzip
import tempfile
import unittest
from pathlib import Path
import freeze_indices as baseline


class RootIndexBaselineTests(unittest.TestCase):
    def test_regeneration_matches_owned_fixture(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            baseline.freeze(root)
            relative = Path("tests/fixtures/flow-root-indices/indices.bin.gz")
            self.assertEqual(gzip.decompress((root / relative).read_bytes()),
                             gzip.decompress((baseline.ROOT / relative).read_bytes()))

    def test_every_owned_root_index_is_first_coordinate_fast_with_56_byte_stride(self) -> None:
        for case in baseline.owned_cases():
            projected = baseline.project(case)
            roots = projected["roots"]
            self.assertEqual([r["native_root_index"] for r in roots], list(range(len(roots))))
            self.assertEqual([r["native_root_byte_offset"] for r in roots], [i * 56 for i in range(len(roots))])
            self.assertEqual(sum(len(r["leaves"]) for r in roots), projected["leaf_count"])
            self.assertEqual([projected[key] for key in ("fresh_context_assumed", "native_context_executed", "global_block_ordinals_assigned", "parallel_scan_offsets_validated")], [True, False, False, False])

    def test_depth_256_retains_true_child_paths_and_local_ordinals(self) -> None:
        for version in range(4):
            projected = baseline.project(baseline.owned_case((1, 1, 1), version, False, 256))
            leaves = projected["roots"][0]["leaves"]
            self.assertEqual(len(leaves), 1793)
            self.assertEqual(leaves[0]["child_path"], (0,) * 256)
            self.assertEqual(leaves[-1]["child_path"], (7,))
            self.assertEqual([leaf["local_leaf_ordinal"] for leaf in leaves], list(range(1793)))
            self.assertTrue(all(leaf["wire_end"] - leaf["wire_start"] == (5 if version == 2 else 1) for leaf in leaves))

    def test_native_reference_has_only_root_storage_and_local_leaf_order(self) -> None:
        projected = baseline.project(baseline.native_cases()[0])
        self.assertEqual(len(projected["roots"]), 792)
        self.assertEqual(projected["leaf_count"], 3984)
        self.assertEqual(projected["roots"][-1]["indices"], (21, 5, 5))
        self.assertEqual(projected["roots"][-1]["native_root_byte_offset"], 791 * 56)
        self.assertFalse(projected["global_block_ordinals_assigned"])
        self.assertFalse(projected["parallel_scan_offsets_validated"])


if __name__ == "__main__":
    unittest.main()
