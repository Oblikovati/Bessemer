"""Independent version/body boundaries, context-presence gates and numeric limits."""
import gzip
import struct
import tempfile
import unittest
import zlib
from pathlib import Path
import freeze_values as values
import freeze_widths as widths
from swarchive import ArchiveError


class FieldValueBaselineTests(unittest.TestCase):
    def test_both_complete_packets_regenerate_without_changing_owned_baselines(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            values.freeze(root)
            widths.freeze(root)
            for name in ("values", "widths"):
                relative = Path(f"tests/fixtures/flow-field-values/{name}.bin.gz")
                self.assertEqual(gzip.decompress((root / relative).read_bytes()),
                                 gzip.decompress((values.ROOT / relative).read_bytes()))

    def test_raw_version_precedes_the_compressed_body_and_offsets_exclude_it(self) -> None:
        body = bytes.fromhex("02000000 01000000 0000000000000080 341200000000f87f 0700000000000000")
        context = dict(kind="legacy", width_table_count=2, object_widths=[2, 0, 1], expected_width_table=[2, 1])
        payload = bytes.fromhex("05000000") + zlib.compress(body)
        result = values.flowfieldvalues.read_record(dict(identifier=108, size=len(payload), payload=payload),
            body_compression="zlib", context=context, compatibility_restriction=False)
        self.assertEqual((result["stored_version_bytes"], result["end"], result["decoded_payload_bytes"]), (4, 32, 36))
        self.assertEqual([(obj["start"], obj["end"]) for obj in result["objects"]], [(8, 24), (24, 24), (24, 32)])
        self.assertEqual(result["objects"][0]["raw_bits"], (0x8000000000000000, 0x7ff8000000001234))

    def test_empty_legacy_body_accepts_a_zero_decoded_bound_with_either_transport(self) -> None:
        context = dict(kind="legacy", width_table_count=0, object_widths=[0, 0], expected_width_table=[])
        for compression, body in (("none", b""), ("zlib", zlib.compress(b""))):
            payload = struct.pack("<i", 6) + body
            result = values.flowfieldvalues.read_record(dict(identifier=108, size=len(payload), payload=payload),
                body_compression=compression, context=context, compatibility_restriction=True, max_output=0)
            self.assertEqual((result["end"], result["decoded_payload_bytes"]), (0, 4))
            self.assertTrue(result["expected_width_table_checked"])
            self.assertEqual([(obj["start"], obj["end"]) for obj in result["objects"]], [(0, 0), (0, 0)])

    def test_unchecked_wildcard_and_partial_width_tables_preserve_negative_words(self) -> None:
        for name in ("unchecked", "wildcards", "partial"):
            table, context = values.legacy_context(name)
            result = values.flowfieldvalues.read_decoded(5, values.decoded_body(context, table),
                context=context, compatibility_restriction=False)
            self.assertEqual(result["stored_width_table"], (-(1 << 31), -1, 0, (1 << 31) - 1))
            self.assertEqual(result["expected_width_table_checked"], name != "unchecked")
        table, context = values.legacy_context("partial")
        context["expected_width_table"][0] = 0
        with self.assertRaises(ArchiveError):
            values.flowfieldvalues.read_decoded(5, values.decoded_body(context, table), context=context, compatibility_restriction=False)

    def test_inactive_and_zero_width_regions_do_not_allocate_declared_products(self) -> None:
        context = values.region_context("inactive-max")
        body = bytes.fromhex("03000000 ffffff7f 00000000 ffffff7f")
        result = values.flowfieldvalues.read_decoded(7, body, context=context, compatibility_restriction=False)
        self.assertEqual(result["end"], 16)
        self.assertTrue(all(region["raw_bits"] == () for region in result["regions"]))
        self.assertEqual([(region["row_count"], region["width"], region["active"]) for region in result["regions"]],
                         [(values.MAXIMUM, values.MAXIMUM, False), (values.MAXIMUM, 0, True), (0, values.MAXIMUM, True)])
        self.assertTrue(result["layout_context_assumed"])
        self.assertFalse(result["native_storage_reproduced"])

    def test_compatibility_skip_is_checked_before_attempting_body_decompression(self) -> None:
        payload = struct.pack("<i", 8) + b"invalid-body"
        with self.assertRaisesRegex(ArchiveError, "compatibility"):
            values.flowfieldvalues.read_record(dict(identifier=108, size=len(payload), payload=payload),
                body_compression="zlib", context=None, compatibility_restriction=True)
        for version in (5, 6, 7):
            case = next(case for case in values.owned_cases() if case["version"] == version and case["restriction"])
            self.assertTrue(values.flowfieldvalues.read_decoded(version, case["decoded"], context=case["context"], compatibility_restriction=True)["layout_context_assumed"])

    def test_all_signed_type_bytes_use_only_the_selected_width_source(self) -> None:
        for code in range(-128, 128):
            actual = values.flowfieldvalues.region_width(code, cached_width=7, region_index=0, numeric_widths=[14.9])
            self.assertEqual(actual, 7 if code == -1 else 14 if code == 1 else 0)
        self.assertEqual(values.flowfieldvalues.region_width(-2, cached_width=-1), 0)
        self.assertEqual(values.flowfieldvalues.region_width(1, region_index=-1, numeric_widths=[float("nan")]), 0)
        self.assertEqual(values.flowfieldvalues.region_width(1, region_index=1, numeric_widths=[float("nan"), 14]), 14)

    def test_binary64_truncation_precedes_nonnegative_native_count_validation(self) -> None:
        for numeric, expected in ((-0.2, 0), (-0.9999999999999999, 0), (-0.0, 0), (14.9, 14), (2147483647.9999998, values.MAXIMUM)):
            self.assertEqual(values.flowfieldvalues.region_width(1, region_index=0, numeric_widths=[numeric]), expected)
        for numeric in (-1.0, 2147483648.0, float("nan"), float("inf"), float("-inf")):
            with self.assertRaises(ArchiveError):
                values.flowfieldvalues.region_width(1, region_index=0, numeric_widths=[numeric])
        cases = widths.scalar_cases() + widths.numeric_cases()
        self.assertEqual((len(cases), sum(widths.outcome(case) is not None for case in cases)), (315, 289))

    def test_every_small_stored_prefix_and_extra_decoded_byte_is_rejected(self) -> None:
        for case in values.owned_cases():
            if len(case["decoded"]) > 128:
                continue
            record = dict(identifier=108, size=len(case["stored"]), payload=case["stored"])
            for end in range(len(case["stored"])):
                with self.assertRaises(ArchiveError):
                    values.flowfieldvalues.read_record(dict(record, size=end, payload=case["stored"][:end]),
                        body_compression="zlib" if case["compressed"] else "none", context=case["context"], compatibility_restriction=case["restriction"])
            with self.assertRaises(ArchiveError):
                values.flowfieldvalues.read_decoded(case["version"], case["decoded"] + b"x", context=case["context"], compatibility_restriction=case["restriction"])


if __name__ == "__main__":
    unittest.main()
