"""Complete kind-one record composition with strict fresh-state and reference checks."""
import gzip
import struct
import tempfile
import unittest
from pathlib import Path
import freeze_profiles as frozen
from test_objects import rebuild_body
from swarchive import ArchiveError


def read(raw: bytes) -> dict:
    return frozen.objects.schema.read_record_424_kind1_profile(dict(identifier=424, size=len(raw), payload=raw), string_mode=0, fresh_backend=True)


def first_wire(entry: dict) -> bytes:
    body = dict(entry["body"], opaque_tail=b"")
    return struct.pack("<i", entry["kind"]) + entry["key_raw"] + rebuild_body(body)


def second_wire(entry: dict) -> bytes:
    raw = struct.pack("<i", entry["kind"])
    return raw + frozen.references.body(entry["kind"], entry["state_byte"], entry["references_raw"], entry["key_raw"])


def rebuild(raw: bytes, parsed: dict) -> bytes:
    out = raw[:parsed["backend_start"]] + b"\0\0" + struct.pack("<i", len(parsed["first_table"]))
    for entry in parsed["first_table"]:
        assert len(out) == entry["start"]
        out += first_wire(entry)
        assert len(out) == entry["end"]
    out += b"\0" + struct.pack("<i", len(parsed["second_table"]))
    for entry in parsed["second_table"]:
        assert len(out) == entry["start"]
        out += second_wire(entry)
        assert len(out) == entry["end"]
    assert len(out) == parsed["backend_end"]
    out += struct.pack("<i", len(parsed["outer_reference_keys_raw"])) + b"".join(parsed["outer_reference_keys_raw"])
    out += b"".join(struct.pack("<i", value) for value in parsed["outer_attribute_i32"])
    return out + struct.pack("<ii", *parsed["final_collection_counts"])


class MediumProfileBaselineTests(unittest.TestCase):
    def test_complete_profile_packet_regenerates_exactly(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            frozen.freeze(root)
            relative = Path("tests/fixtures/flow-medium-references/profiles.bin.gz")
            self.assertEqual(gzip.decompress((root / relative).read_bytes()),
                             gzip.decompress((frozen.ROOT / relative).read_bytes()))

    def test_every_complete_profile_rebuilds_nested_fields_and_exact_eof(self) -> None:
        for name, raw in frozen.cases():
            parsed = read(raw)
            self.assertEqual(rebuild(raw, parsed), raw, name)
            self.assertEqual(parsed["end"], len(raw))
            self.assertTrue(parsed["complete_record_framing"])
            self.assertTrue(parsed["fresh_backend_assumed"])
            self.assertFalse(parsed["full_engineering_schema"])
            self.assertFalse(parsed["native_runtime_state_reproduced"])

    def test_every_native_and_small_complete_profile_prefix_and_appended_byte_is_rejected(self) -> None:
        selected = frozen.cases()[:1] + [row for row in frozen.cases()[1:] if len(row[1]) <= 256]
        for name, raw in selected:
            for end in range(len(raw)):
                with self.assertRaises(ArchiveError, msg=f"{name}:{end}"):
                    read(raw[:end])
        for name, raw in frozen.cases():
            with self.assertRaises(ArchiveError, msg=name):
                read(raw + b"\0")

    def test_explicit_fresh_backend_string_mode_and_active_outer_branch_are_required(self) -> None:
        raw = frozen.record(3, "one")
        record = dict(identifier=424, size=len(raw), payload=raw)
        for fresh in (False, None, 0, 1):
            with self.assertRaises(ArchiveError):
                frozen.objects.schema.read_record_424_kind1_profile(record, string_mode=0, fresh_backend=fresh)
        for mode in (False, True, 1, None):
            with self.assertRaises(ArchiveError):
                frozen.objects.schema.read_record_424_kind1_profile(record, string_mode=mode, fresh_backend=True)
        for version in (0, 1, 128, 255):
            with self.assertRaises(ArchiveError):
                read(bytes((version,)) + raw[1:])

    def test_reference_membership_is_checked_in_both_fresh_tables(self) -> None:
        raw = frozen.record(3, "one")
        parsed = read(raw)
        first_ref = parsed["second_table"][0]["start"] + 27
        outer_ref = parsed["backend_end"] + 4
        for position in (first_ref, outer_ref):
            changed = raw[:position] + frozen.references.KEYS[3] + raw[position + 16:]
            with self.assertRaises(ArchiveError):
                read(changed)

    def test_duplicate_physical_keys_survive_while_reference_membership_is_set_based(self) -> None:
        raw = frozen.record(3, "duplicate_keys")
        parsed = read(raw)
        self.assertEqual([entry["key_raw"] for entry in parsed["first_table"][:2]], [frozen.references.KEYS[0]] * 2)
        self.assertEqual([entry["key_raw"] for entry in parsed["second_table"]], [frozen.references.KEYS[2]] * 2)
        self.assertEqual(parsed["outer_reference_keys_raw"], [frozen.references.KEYS[2]] * 2)
        self.assertEqual(parsed["outer_attribute_i32"], (-2147483648, 2147483647))
        self.assertEqual(rebuild(raw, parsed), raw)

    def test_populated_final_collections_and_unsupported_table_kinds_remain_outside_profile(self) -> None:
        raw = frozen.record(3, "one")
        parsed = read(raw)
        for position in (len(raw) - 8, len(raw) - 4):
            changed = raw[:position] + struct.pack("<i", 1) + raw[position + 4:]
            with self.assertRaises(ArchiveError):
                read(changed)
        for position in (parsed["first_table"][0]["start"], parsed["second_table"][0]["start"]):
            changed = raw[:position] + struct.pack("<i", 2) + raw[position + 4:]
            with self.assertRaises(ArchiveError):
                read(changed)
