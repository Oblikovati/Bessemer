"""Independent body reconstruction and strict count/version/string boundaries."""
import gzip
import struct
import tempfile
import unittest
from pathlib import Path
import freeze_objects as frozen
from swarchive import ArchiveError


def rebuild_curve(curve: dict) -> bytes:
    raw = bytes((curve["version"],)) + struct.pack("<Qi", curve["scalar_pattern"], curve["count"])
    raw += frozen.packed(curve["first_patterns"], "Q") + frozen.packed(curve["second_patterns"], "Q")
    raw += bytes((curve["extra_flag_byte"],))
    raw += frozen.packed(curve["extra_first_patterns"], "Q") + frozen.packed(curve["extra_second_patterns"], "Q")
    return raw


def rebuild_body(value: dict) -> bytes:
    raw = bytes((value["version"], value["base_byte"])) + value["base_key_raw"]
    raw += struct.pack("<i", value["name"]["stored_units"]) + value["name"]["raw_bytes"]
    for curve in value["curves"]:
        assert len(raw) == curve["start"]
        raw += rebuild_curve(curve)
        assert len(raw) == curve["end"]
    for count, arrays in (("first_count", "first_arrays"), ("second_count", "second_arrays")):
        raw += struct.pack("<i", value[count]) + b"".join(frozen.packed(row, "Q") for row in value[arrays])
    raw += frozen.packed(value["scalar_patterns"], "Q") + bytes((value["state_byte"],))
    assert len(raw) == value["body_end"]
    return raw + value["opaque_tail"]


class MediumObjectBaselineTests(unittest.TestCase):
    def test_packet_regenerates_without_changing_owned_expectations(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            frozen.freeze(root)
            relative = Path("tests/fixtures/flow-medium-objects/kind1.bin.gz")
            self.assertEqual(gzip.decompress((root / relative).read_bytes()),
                             gzip.decompress((frozen.ROOT / relative).read_bytes()))

    def test_every_complete_body_reconstructs_without_normalizing_numeric_or_string_bits(self) -> None:
        for name, raw in frozen.cases():
            value = frozen.schema.read_kind1_body(raw, string_mode=0)
            self.assertEqual(rebuild_body(value), raw, name)
            self.assertTrue(value["body_framing_complete"])
            self.assertFalse(value["native_runtime_state_reproduced"])
            self.assertFalse(value["engineering_columns_assigned"])
            self.assertFalse(value["base_byte_dispatch"])
            self.assertEqual(value["native_state_after_load"], 1)

    def test_all_proper_body_prefixes_of_small_cases_and_native_body_are_rejected(self) -> None:
        selected = frozen.cases()[:1] + [row for row in frozen.cases()[2:] if len(row[1]) <= 1024]
        for name, raw in selected:
            value = frozen.schema.read_kind1_body(raw, string_mode=0)
            for end in range(value["body_end"]):
                with self.assertRaises(ArchiveError, msg=f"{name}:{end}"):
                    frozen.schema.read_kind1_body(raw[:end], string_mode=0)

    def test_all_nonzero_body_and_curve_versions_are_rejected(self) -> None:
        raw = frozen.body(255, 128, 0, 0, b"", 0)
        value = frozen.schema.read_kind1_body(raw, string_mode=0)
        for position in [0, *[curve["start"] for curve in value["curves"]]]:
            for version in range(1, 256):
                changed = bytearray(raw)
                changed[position] = version
                with self.assertRaises(ArchiveError):
                    frozen.schema.read_kind1_body(changed, string_mode=0)

    def test_negative_and_impossible_curve_name_and_group_counts_are_rejected(self) -> None:
        raw = frozen.body(0, 0, 0, 0, b"", 0)
        value = frozen.schema.read_kind1_body(raw, string_mode=0)
        positions = [18, *[curve["start"] + 9 for curve in value["curves"]], value["curves"][-1]["end"], value["curves"][-1]["end"] + 4]
        for position in positions:
            for count in (-1, 2147483647):
                changed = bytearray(raw)
                changed[position:position + 4] = struct.pack("<i", count)
                with self.assertRaises(ArchiveError):
                    frozen.schema.read_kind1_body(changed, string_mode=0)

    def test_string_mode_and_terminal_nul_are_explicit_but_surrogates_are_lossless(self) -> None:
        raw = frozen.body(0, 0, 0, 0, b"\x00\xd8\0\0", 0)
        for mode in (False, True, 1, -1, None):
            with self.assertRaises(ArchiveError):
                frozen.schema.read_kind1_body(raw, string_mode=mode)
        value = frozen.schema.read_kind1_body(raw, string_mode=0)
        self.assertEqual(value["name"]["text"].encode("utf-16le", "surrogatepass"), b"\x00\xd8")
        changed = bytearray(raw)
        changed[value["name"]["end"] - 2] = 1
        with self.assertRaises(ArchiveError):
            frozen.schema.read_kind1_body(changed, string_mode=0)
