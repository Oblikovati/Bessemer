"""Independent six-model reconstruction and strict kind-two framing boundaries."""
import gzip
import struct
import tempfile
import unittest
from pathlib import Path
import freeze_kind_two as frozen
from test_objects import rebuild_prefix, rebuild_curve
from swarchive import ArchiveError


def read(raw: bytes) -> dict:
    return frozen.schema.read_kind2_body(raw, string_mode=0)


def rebuild_model(value: dict) -> bytes:
    kind = value["kind"]
    raw = bytes((value["version"],))
    if kind in (0, 1, 2):
        raw += b"".join(map(rebuild_curve, value["curves"]))
        return raw + frozen.packed(value["scalar_patterns"], "Q") + value["state_bytes"]
    if kind == 3:
        return raw + frozen.packed(value["scalar_patterns"], "Q")
    if kind == 4:
        return raw + frozen.packed(value["first_scalar_patterns"], "Q") + bytes((value["state_byte"],)) + struct.pack("<Q", value["last_scalar_pattern"])
    return raw + rebuild_table(value)


def rebuild_table(value: dict) -> bytes:
    raw = struct.pack("<ii", value["first_count"], value["second_count"])
    raw += b"".join(frozen.packed(row, "Q") for row in value["first_arrays"])
    raw += frozen.packed(value["second_patterns"], "Q") + frozen.packed(value["second_word_patterns"], "I")
    raw += struct.pack("<i", value["third_count"]) + frozen.packed(value["third_patterns"], "Q")
    return raw + struct.pack("<i", value["member_8c_i32"]) + frozen.packed(value["scalar_patterns"], "Q") + value["state_bytes"]


def rebuild(value: dict) -> bytes:
    raw = rebuild_prefix(value) + struct.pack("<Qi", value["scalar_pattern"], value["model"]["kind"])
    assert len(raw) == value["model"]["start"]
    raw += rebuild_model(value["model"])
    assert len(raw) == value["model"]["end"]
    raw += bytes((value["state_byte"],))
    assert len(raw) == value["body_end"]
    return raw + value["opaque_tail"]


class KindTwoBaselineTests(unittest.TestCase):
    def test_packet_regenerates_without_changing_owned_model_expectations(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            frozen.freeze(root)
            relative = Path("tests/fixtures/flow-medium-kind-two/bodies.bin.gz")
            self.assertEqual(gzip.decompress((root / relative).read_bytes()),
                             gzip.decompress((frozen.ROOT / relative).read_bytes()))

    def test_every_model_rebuilds_all_fields_and_retains_exact_following_bytes(self) -> None:
        for name, raw in frozen.cases():
            value = read(raw)
            self.assertEqual(rebuild(value), raw, name)
            self.assertTrue(value["body_framing_complete"])
            self.assertFalse(value["installed_fixture_validated"])
            self.assertFalse(value["native_runtime_state_reproduced"])
            self.assertFalse(value["base_byte_dispatch"])
            self.assertEqual(value["native_state_after_load"], 1)

    def test_all_proper_prefixes_of_compact_count_and_flag_profiles_are_rejected(self) -> None:
        selected = [(name, raw) for name, raw in frozen.cases() if name.startswith("tag-") and "-count-" in name and len(raw) <= 2048]
        for name, raw in selected:
            for end in range(read(raw)["body_end"]):
                with self.assertRaises(ArchiveError, msg=f"{name}:{end}"):
                    read(raw[:end])

    def test_all_unsupported_body_model_and_curve_version_bytes_are_rejected(self) -> None:
        for kind in range(6):
            raw = frozen.body(kind, 0, 0, 0, 0, b"")
            value = read(raw)
            curves = value["curves"] + value["model"].get("curves", [])
            for position in [0, value["model"]["start"], *[curve["start"] for curve in curves]]:
                for version in range(1, 256):
                    changed = bytearray(raw)
                    changed[position] = version
                    with self.assertRaises(ArchiveError):
                        read(changed)

    def test_unknown_model_tags_are_rejected_before_model_body_dispatch(self) -> None:
        raw = frozen.body(0, 0, 0, 0, 0)
        tag = read(raw)["model"]["start"] - 4
        for kind in (-2147483648, -1, 6, 255, 2147483647):
            changed = raw[:tag] + struct.pack("<i", kind) + raw[tag + 4:]
            with self.assertRaises(ArchiveError):
                read(changed)

    def test_negative_and_physically_impossible_counts_are_rejected_at_every_counted_role(self) -> None:
        for kind in range(6):
            raw = frozen.body(kind, 0, 0, 0, 0, b"")
            value = read(raw)
            curves = value["curves"] + value["model"].get("curves", [])
            positions = [18, *[curve["start"] + 9 for curve in curves], value["curves"][-1]["end"], value["curves"][-1]["end"] + 4]
            if kind == 5:
                positions += [value["model"]["start"] + offset for offset in (1, 5, 9)]
            for position in positions:
                for count in (-1, 2147483647):
                    changed = raw[:position] + struct.pack("<i", count) + raw[position + 4:]
                    with self.assertRaises(ArchiveError):
                        read(changed)

    def test_table_model_counts_are_independent_including_absent_first_and_nonempty_later_arrays(self) -> None:
        for counts in [(0, 1, 0), (1, 0, 3), (0, 0, 5), (1, 7, 11), (0, 1024, 0)]:
            value = read(frozen.body(5, 0, 0, 0, 128, counts=counts))["model"]
            self.assertEqual(tuple(value[key] for key in ("first_count", "second_count", "third_count")), tuple(counts))
            self.assertEqual([len(row) for row in value["first_arrays"]], [counts[0]] * 2)
            self.assertEqual(len(value["second_word_patterns"]), counts[1])
            self.assertEqual(len(value["third_patterns"]), counts[2])

    def test_utf16_mode_terminator_and_native_state_projection_remain_explicit(self) -> None:
        for kind in range(6):
            raw = frozen.body(kind, 0, 0, 0, 165, b"\x00\xd8\0\0")
            value = read(raw)
            self.assertEqual(value["name"]["text"].encode("utf-16le", "surrogatepass"), b"\x00\xd8")
            self.assertEqual((value["state_byte"], value["native_state_after_load"]), (0, 1))
            changed = bytearray(raw)
            changed[value["name"]["end"] - 2] = 1
            with self.assertRaises(ArchiveError):
                read(changed)
            for mode in (False, True, 1, -1, None):
                with self.assertRaises(ArchiveError):
                    frozen.schema.read_kind2_body(raw, string_mode=mode)
