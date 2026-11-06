"""Independent fixed frames, update semantics and explicit metadata-prefix limits."""
import gzip
import struct
import tempfile
import unittest
from pathlib import Path
import freeze_states as states
import freeze_prefix as prefix
from wire_packet import flowmetadataprefix as schema
from swarchive import ArchiveError


class MetadataBaselineTests(unittest.TestCase):
    def test_both_packets_regenerate_without_changing_frozen_values(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            states.freeze(root)
            prefix.freeze(root)
            for name in ("states", "prefix"):
                relative = Path(f"tests/fixtures/flow-metadata/{name}.bin.gz")
                self.assertEqual(gzip.decompress((root / relative).read_bytes()),
                                 gzip.decompress((states.ROOT / relative).read_bytes()))

    def test_fixed_raw_states_preserve_duplicates_all_bits_and_existing_keys(self) -> None:
        raw = bytes.fromhex("ffffffff 03000000 07000000 02 ffffffff 80 07000000 ff")
        parsed = schema.read_id_states(raw)
        existing = {7: 0, 42: 9}
        result = schema.apply_id_states(parsed, existing=existing)
        self.assertEqual((parsed["version_word_u32"], parsed["end"]), (0xffffffff, 23))
        self.assertFalse(parsed["native_version_dispatch"])
        self.assertEqual([(entry["identifier_i32"], entry["state_byte"]) for entry in parsed["entries"]], [(7, 2), (-1, 128), (7, 255)])
        self.assertEqual(result["states"], {7: 255, 42: 9, -1: 128})
        self.assertEqual(existing, {7: 0, 42: 9})
        self.assertTrue(result["unmentioned_existing_entries_preserved"])
        self.assertFalse(result["native_hash_table_reproduced"])

    def test_all_header_patterns_remain_raw_without_dispatch(self) -> None:
        self.assertEqual(len(states.headers()), 67)
        for header in states.headers():
            parsed = schema.read_id_states(states.wire(header, [(states.MINIMUM, 255)]))
            self.assertEqual(parsed["version_word_u32"], header)
            self.assertEqual(parsed["entries"][0]["identifier_i32"], states.MINIMUM)
            self.assertFalse(parsed["native_version_dispatch"])

    def test_every_state_byte_is_an_assignment_value_rather_than_a_boolean(self) -> None:
        profiles = states.profiles()
        duplicate = schema.read_id_states(states.wire(0, profiles["all_states_duplicate"]))
        self.assertEqual(schema.apply_id_states(duplicate, existing={7: 31})["states"], {7: 255})
        distinct = schema.read_id_states(states.wire(0, profiles["all_states_distinct"]))
        applied = schema.apply_id_states(distinct, existing={states.MAXIMUM: 77})["states"]
        self.assertEqual([applied[index - 128] for index in range(256)], list(range(256)))
        self.assertEqual(applied[states.MAXIMUM], 77)
        empty = schema.read_id_states(states.wire(0, []))
        self.assertEqual(schema.apply_id_states(empty, existing={states.MINIMUM: 255})["states"], {states.MINIMUM: 255})

    def test_state_boundaries_and_existing_map_domains_are_strict(self) -> None:
        raw = states.wire(0, [(7, 255)])
        for end in range(len(raw)):
            with self.assertRaises(ArchiveError):
                schema.read_id_states(raw[:end])
        for malformed in (raw + b"x", struct.pack("<Ii", 0, -1), struct.pack("<Ii", 0, (1 << 31) - 1)):
            with self.assertRaises(ArchiveError):
                schema.read_id_states(malformed)
        for existing in (None, {True: 1}, {1 << 31: 0}, {1: True}, {1: 256}, {1: -1}):
            with self.assertRaises(ArchiveError):
                schema.apply_id_states(schema.read_id_states(raw), existing=existing)

    def test_all_version_bytes_use_the_signed_gate_without_decoding_inactive_tails(self) -> None:
        for version in range(256):
            raw = prefix.prefix(version, "rich") + prefix.TAIL
            result = schema.read_metadata_prefix(raw, string_mode=0)
            signed = version if version < 128 else version - 256
            self.assertEqual(result["version_i8"], signed)
            self.assertEqual(result["body_active"], 2 <= version <= 127)
            self.assertEqual(result["opaque_tail"], prefix.TAIL)
            if not result["body_active"]:
                self.assertEqual((result["prefix_end"], result["vectors"], result["objects"]), (1, [], []))
            self.assertFalse(result["full_payload_schema"])

    def test_raw_utf16_and_first_nul_text_preserve_surrogates_and_noncharacters(self) -> None:
        for name in ("embedded-nul", "surrogates", "noncharacters", "long-string"):
            result = schema.read_metadata_prefix(prefix.prefix(3, name), string_mode=0)
            item = result["objects"][0]
            expected = prefix.object_strings(name)
            for key, units in zip(("member_30_string", "member_00_string", "member_60_string", "member_90_stored_string"), expected):
                self.assertEqual(item[key]["raw_bytes"], struct.pack("<" + "H" * len(units), *units))
                text_units = units[:units.index(0)] if 0 in units else units
                self.assertEqual(item[key]["text"].encode("utf-16le", "surrogatepass"), struct.pack("<" + "H" * len(text_units), *text_units))
            self.assertFalse(result["full_payload_schema"])

    def test_zero_units_and_stored_nul_are_distinct_empty_text_encodings(self) -> None:
        for name, count in (("zero-strings", 0), ("one-nul", 1)):
            result = schema.read_metadata_prefix(prefix.prefix(3, name), string_mode=0)
            first = result["objects"][0]["member_30_string"]
            self.assertEqual((first["stored_units"], first["text"], len(first["raw_bytes"])), (count, "", count * 2))
            self.assertFalse(result["full_payload_schema"])

    def test_version_two_derives_fourth_text_while_version_three_can_store_empty(self) -> None:
        two = schema.read_metadata_prefix(prefix.prefix(2, "embedded-nul"), string_mode=0)["objects"][0]
        three = schema.read_metadata_prefix(prefix.prefix(3, "embedded-nul"), string_mode=0)["objects"][0]
        self.assertEqual(two["member_90_native_text"], "A")
        self.assertIsNone(two["member_90_stored_string"])
        self.assertTrue(two["member_90_derived_from_30"])
        self.assertEqual(three["member_90_native_text"], "")
        self.assertEqual(three["member_90_stored_string"]["stored_units"], 0)
        self.assertFalse(three["member_90_derived_from_30"])
        for state in range(256):
            item = schema.read_metadata_prefix(prefix.prefix(3, "zero-strings", state), string_mode=0)["objects"][0]
            self.assertEqual((item["state_byte"], item["native_state_boolean"]), (state, state != 0))

    def test_prefix_truncation_and_tail_accounting_never_turn_into_full_schema_credit(self) -> None:
        for version in (0, 1, 2, 3, 127, 128, 255):
            raw = prefix.prefix(version, "surrogates")
            for end in range(len(raw)):
                with self.assertRaises(ArchiveError):
                    schema.read_metadata_prefix(raw[:end], string_mode=0)
            for tail in (b"", b"\0" * 7, prefix.TAIL):
                result = schema.read_metadata_prefix(raw + tail, string_mode=0)
                self.assertEqual((result["prefix_end"], result["unknown_bytes"], result["opaque_tail"]), (len(raw), len(tail), tail))
                self.assertFalse(result["full_payload_schema"])
        for mode in (None, False, 1, -1):
            with self.assertRaises(ArchiveError):
                schema.read_metadata_prefix(prefix.prefix(2, "empty"), string_mode=mode)


if __name__ == "__main__":
    unittest.main()
