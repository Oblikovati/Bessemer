"""Independent typed reference framing, context validation and strict boundaries."""
import gzip
import struct
import tempfile
import unittest
from pathlib import Path
import freeze_references as frozen
from swarchive import ArchiveError


def read(case: dict) -> dict:
    return frozen.schema.read_reference_body(case["raw"], kind=case["kind"], first_keys=case["keys"])


class MediumReferenceBaselineTests(unittest.TestCase):
    def test_packet_regenerates_exactly(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            frozen.freeze(root)
            relative = Path("tests/fixtures/flow-medium-references/bodies.bin.gz")
            self.assertEqual(gzip.decompress((root / relative).read_bytes()),
                             gzip.decompress((frozen.ROOT / relative).read_bytes()))

    def test_every_complete_body_reconstructs_keys_and_ordered_references(self) -> None:
        for case in frozen.cases():
            value = read(case)
            rebuilt = frozen.body(case["kind"], value.get("state_byte", 0), value["references_raw"], value["key_raw"])
            self.assertEqual(len(rebuilt), value["body_end"])
            self.assertEqual(rebuilt + value["opaque_tail"], case["raw"])
            self.assertFalse(value["native_runtime_state_reproduced"])
            self.assertTrue(all(key in case["keys"] for key in value["references_raw"]))
            self.assertEqual(value.get("native_state_after_load"), 1 if case["kind"] == 1 else None)

    def test_all_small_and_native_reference_body_prefixes_are_rejected(self) -> None:
        for case in frozen.cases():
            value = read(case)
            if value["body_end"] > 256:
                continue
            for end in range(value["body_end"]):
                with self.assertRaises(ArchiveError):
                    frozen.schema.read_reference_body(case["raw"][:end], kind=case["kind"], first_keys=case["keys"])

    def test_external_kind_and_key_shape_are_strict_before_wire_decoding(self) -> None:
        for kind in (0, 3, -1, True, 1.0, None):
            with self.assertRaises(ArchiveError):
                frozen.schema.read_reference_body(b"", kind=kind, first_keys=[])
        for key in (b"", bytes(15), bytes(17), "sixteen", bytearray(16)):
            with self.assertRaises(ArchiveError):
                frozen.schema.read_reference_body(b"", kind=1, first_keys=[key])

    def test_context_duplicates_and_order_do_not_replace_wire_order_or_mutate_input(self) -> None:
        refs = [frozen.KEYS[1], frozen.KEYS[1], frozen.KEYS[0]]
        for kind in (1, 2):
            raw = frozen.body(kind, 255, refs, frozen.KEYS[3])
            existing = frozen.KEYS[::-1] + frozen.KEYS
            before = list(existing)
            value = frozen.schema.read_reference_body(raw, kind=kind, first_keys=existing)
            self.assertEqual(value["references_raw"], refs)
            self.assertEqual(existing, before)
            with self.assertRaises(ArchiveError):
                frozen.schema.read_reference_body(raw, kind=kind, first_keys=[frozen.KEYS[1]])

    def test_all_nonzero_versions_and_negative_or_physically_impossible_counts_are_rejected(self) -> None:
        for kind in (1, 2):
            raw = frozen.body(kind, 0, [], frozen.KEYS[0])
            for position in (0, 1):
                for version in range(1, 256):
                    changed = bytearray(raw)
                    changed[position] = version
                    with self.assertRaises(ArchiveError):
                        frozen.schema.read_reference_body(changed, kind=kind, first_keys=[])
            for count in (-1, 2147483647):
                changed = raw[:-4] + struct.pack("<i", count)
                with self.assertRaises(ArchiveError):
                    frozen.schema.read_reference_body(changed, kind=kind, first_keys=[])

    def test_every_stored_state_byte_loads_one_and_same_wire_kind_boundaries_remain_distinct(self) -> None:
        for state in range(256):
            raw = frozen.body(1, state, [], frozen.KEYS[0])
            value = frozen.schema.read_reference_body(raw, kind=1, first_keys=[])
            self.assertEqual((value["state_byte"], value["native_state_after_load"]), (state, 1))
        raw = frozen.body(1, 0, [], frozen.KEYS[1])
        first = frozen.schema.read_reference_body(raw, kind=1, first_keys=[])
        second = frozen.schema.read_reference_body(raw, kind=2, first_keys=[])
        self.assertEqual((first["body_end"], second["body_end"]), (23, 22))
        self.assertEqual(second["opaque_tail"], b"\0")
