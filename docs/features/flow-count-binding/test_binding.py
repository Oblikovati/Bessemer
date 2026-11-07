"""Keep prefix scope and conditional runtime-count provenance separate."""
import gzip
import struct
import tempfile
import unittest
import zlib
from pathlib import Path
import freeze_binding as baseline


class CountBindingBaselineTests(unittest.TestCase):
    def test_regeneration_matches_owned_fixture(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            baseline.freeze(root)
            relative = Path("tests/fixtures/flow-count-binding/binding.bin.gz")
            self.assertEqual(gzip.decompress((root / relative).read_bytes()),
                             gzip.decompress((baseline.ROOT / relative).read_bytes()))

    def test_positive_delegated_version_retains_opaque_tail_without_claiming_body_validation(self) -> None:
        raw = struct.pack("<3i", 18, 3, 5) + b"arbitrary\0body\xff"
        parsed = baseline.flowcontextcounts.read_decoded(raw)
        self.assertEqual(parsed["native_body_branch"], "restored_18_plus")
        self.assertEqual(parsed["known_prefix_bytes"], 12)
        self.assertEqual(parsed["opaque_tail"], raw[12:])
        self.assertEqual(parsed["unknown_bytes"], len(raw) - 12)
        self.assertFalse(parsed["native_433_context_proven"])

    def test_initialization_assumption_is_required_and_wrapped_totals_do_not_allocate(self) -> None:
        raw = struct.pack("<3i", 14, 3, 5)
        for assumed in (False, None, 1, "true"):
            with self.assertRaises(baseline.flowcountbinding.ArchiveError):
                baseline.flowcountbinding.derive_decoded(raw, initialize_absent_map=assumed)
        for counts in ((2147483647, 1), (2147483647, 2147483647)):
            raw = struct.pack("<3i", 14, *counts)
            parsed = baseline.flowcontextcounts.read_decoded(raw)
            self.assertGreater(parsed["g_total_mathematical"], 2147483647)
            self.assertLess(parsed["g_total_native_i32"], 0)
            with self.assertRaises(baseline.flowcountbinding.ArchiveError):
                baseline.flowcountbinding.derive_decoded(raw, initialize_absent_map=True)

    def test_closed_form_pointer_partition_requires_no_native_pointer_population(self) -> None:
        for fluid, solid in ((0, 0), (0, 5), (3, 0), (3, 5), (2147483647, 0)):
            raw = struct.pack("<3i", 14, fluid, solid)
            binding = baseline.flowcountbinding.derive_decoded(raw, initialize_absent_map=True)
            self.assertEqual(binding["pointer_array_bytes"], (fluid + solid) * 8)
            self.assertEqual(binding["solid_pointer_byte_offset"], fluid * 8)
            self.assertEqual(binding["solid_start_index"], fluid)
            self.assertFalse(binding["native_context_executed"])
            self.assertFalse(binding["pointer_population_reproduced"])
            self.assertFalse(binding["existing_map_context_supported"])

    def test_independent_signed_array_wire_uses_conditional_count_only(self) -> None:
        decoded = bytes.fromhex("00000000 03000000 07000000 ffffffff 05000000 04000000 03000000 02000000")
        stored = zlib.compress(decoded, 1)
        record = dict(identifier=433, size=len(stored), payload=stored)
        bound = baseline.flowcountbinding.read_record_433(record, counts_decoded=struct.pack("<3i", 14, 1, 2), initialize_absent_map=True)
        self.assertEqual(bound["first"], [7, -1, 5])
        self.assertEqual(bound["second"], [4, 3, 2])
        self.assertTrue(bound["native_context_checked"])
        self.assertFalse(bound["count_binding"]["native_context_executed"])


if __name__ == "__main__":
    unittest.main()
