"""Independent literal products, context failures and isolated full fixture regeneration."""
import gzip
import hashlib
import json
import math
import unittest
from pathlib import Path
from tempfile import TemporaryDirectory
from freeze_mapped import ROOT, freeze, source_cases, owned_cases, flowmappedobjects
from swarchive import ArchiveError
from process_reference import edit_rows, field_requests, packet


class MappedFixtures(unittest.TestCase):
    def test_complete_process_oracle_pins_every_editable_field(self) -> None:
        self.assertEqual(len(list(edit_rows())), 3894)
        self.assertEqual(packet()[:8], b"FMO1\xc0\0\0\0")
        sources, _ = source_cases()
        obj = flowmappedobjects.read_payload(sources[2][1], [1])["objects"][0]
        first = next(field_requests(obj))
        self.assertEqual(first, (0, 0, 13, b"\xfa", b"\x05"))

    def test_source_synthetic_products_and_raw_nonfinite_words(self) -> None:
        sources, audit = source_cases()
        self.assertEqual([row["present"] for row in audit], [False, False, True])
        self.assertEqual([len(case[1]) for case in sources], [8, 346, 126])
        obj = flowmappedobjects.read_payload(sources[2][1], [1])["objects"][0]
        self.assertEqual(len(obj["pairs"]), 2)
        self.assertEqual(obj["mapped_value_count"], 2)
        self.assertEqual(obj["mapped_values"]["raw_bits"], (0x8000000000000000, 0x7ff8000000001234))
        self.assertEqual(math.copysign(1, obj["mapped_values"]["values"][0]), -1)
        self.assertTrue(math.isnan(obj["mapped_values"]["values"][1]))
        legacy = flowmappedobjects.read_payload(sources[1][1], [1])["objects"][0]["legacy_version_5"]
        self.assertEqual(legacy["counts"], (2, 1, 1))
        self.assertEqual(legacy["tripled_values"]["raw_bits"], tuple(range(6)))

    def test_pair_count_never_supplies_missing_matched_context(self) -> None:
        sources, _ = source_cases()
        for context in (None, [2], [], [None], [-1], [2147483648]):
            with self.assertRaises(ArchiveError):
                flowmappedobjects.read_payload(sources[2][1], context)
        with self.assertRaises(ArchiveError):
            flowmappedobjects.read_payload(b"\x04\0\0\0\x01\0\0\0\0", [0])

    def test_zero_factor_keeps_unknown_context_and_zero_product_legacy_arrays(self) -> None:
        cases = owned_cases()
        selected = next(case for case in cases if case[0] == "zero-factor" and flowmappedobjects.read_payload(case[1])["objects"][0]["body_version"] == 5)
        obj = flowmappedobjects.read_payload(selected[1])["objects"][0]
        self.assertIsNone(obj["matched_object_count"])
        self.assertEqual(obj["mapped_values"]["raw_bits"], ())
        self.assertEqual(obj["legacy_version_5"]["counts"], (0, 2, 1))
        self.assertEqual(obj["legacy_version_5"]["tripled_values"]["raw_bits"], ())
        self.assertFalse(obj["native_object_context_validated"])
        maximum = next(case for case in cases if case[0] == "maximum-unused-context")
        self.assertEqual(flowmappedobjects.read_payload(maximum[1], maximum[2])["objects"][0]["matched_object_count"], 2147483647)

    def test_regeneration_preserves_goldens_and_all_version_context_cases(self) -> None:
        original = (ROOT / "tests/fixtures/flow-mapped-objects/mapped.bin.gz").read_bytes()
        with TemporaryDirectory(prefix="bessemer-mapped-") as temporary:
            staging = Path(temporary)
            freeze(staging)
            self.assertEqual((staging / "tests/fixtures/flow-mapped-objects/mapped.bin.gz").read_bytes(), original)
            report = json.loads((staging / "docs/features/flow-mapped-objects/fixture-verification.json").read_text())
        self.assertEqual(report["cases"], 192)
        self.assertEqual(report["populated_native_payloads"], 0)
        self.assertEqual(hashlib.sha256(gzip.decompress(original)).hexdigest(), report["sha256"])


if __name__ == "__main__":
    unittest.main()
