"""Keep the independent reference observation strict and scoped."""
from pathlib import Path
import struct
import sys
import tempfile
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "docs/features/model-archives"))
from primitive_inventory import (PrimitiveRead, append_primitive, observe_primitives,
                                 reference_callsite, replaced_methods, verify_primitive_coverage)
from freeze_blank_part import WORKSPACE, oracle_payload, reference_snapshot, require_complete_model, source_module
from owned_name_edit import OWNED_NAME, OWNED_ROOT_PREFIX, owned_name_edit, short_wide_name
from map_inventory import map_slot_inventory, reference_slot


class FakeReferenceCursor:
    def __init__(self, content: bytes) -> None:
        self.b = content
        self.p = 0

    def _take(self, kind: str, size: int) -> object:
        value = struct.unpack_from("<" + kind, self.b, self.p)[0]
        self.p += size
        return value

    def raw(self, size: int) -> bytes:
        value = self.b[self.p:self.p + size]
        self.p += size
        return value


class PrimitiveInventoryTests(unittest.TestCase):
    def test_typed_and_raw_reads_partition_exact_bytes(self) -> None:
        cursor = FakeReferenceCursor(b"\x07\x00ab")
        with observe_primitives(FakeReferenceCursor, cursor.b) as reads:
            self.assertEqual(cursor._take("H", 2), 7)
            self.assertEqual(cursor.raw(2), b"ab")
        verify_primitive_coverage(cursor.b, reads)
        self.assertEqual([(r.start, r.end, r.kind) for r in reads], [(0, 2, "H"), (2, 4, "raw")])
        self.assertTrue(all(r.source_function == "test_typed_and_raw_reads_partition_exact_bytes" for r in reads))

    def test_hooks_restore_after_reference_failure(self) -> None:
        original = FakeReferenceCursor._take, FakeReferenceCursor.raw
        with self.assertRaisesRegex(RuntimeError, "reference failed"):
            with observe_primitives(FakeReferenceCursor, b"a"):
                raise RuntimeError("reference failed")
        self.assertEqual(original, (FakeReferenceCursor._take, FakeReferenceCursor.raw))

    def test_other_stream_and_invalid_cursor_are_distinct(self) -> None:
        reads: list[PrimitiveRead] = []
        cursor = FakeReferenceCursor(b"other")
        append_primitive(reads, b"a", cursor, 0, "raw")
        self.assertEqual(reads, [])
        cursor.b, cursor.p = b"a", 2
        with self.assertRaisesRegex(ValueError, "range 0..2; expected within 0..1"):
            append_primitive(reads, b"a", cursor, 0, "raw")

    def test_missing_overlapping_and_changed_bytes_fail(self) -> None:
        cases = [([], "end 0"), ([PrimitiveRead(1, 2, "raw", "62", "f", "g", 1)], "expected start 0"),
                 ([PrimitiveRead(0, 2, "raw", "6163", "f", "g", 1)], "expected 6162"),
                 ([PrimitiveRead(0, 1, "raw", "61", "f", "g", 1),
                   PrimitiveRead(0, 2, "raw", "6162", "f", "g", 1)], "expected start 1"),
                 ([PrimitiveRead(0, 3, "raw", "6162", "f", "g", 1)], "end <= 2")]
        for reads, message in cases:
            with self.subTest(message=message), self.assertRaisesRegex(ValueError, message):
                verify_primitive_coverage(b"ab", reads)

    def test_callsite_and_nested_hook_scope(self) -> None:
        self.assertEqual(reference_callsite()[1], "test_callsite_and_nested_hook_scope")
        original = FakeReferenceCursor.raw
        with replaced_methods(FakeReferenceCursor, FakeReferenceCursor._take, original):
            with observe_primitives(FakeReferenceCursor, b"") as reads:
                self.assertEqual(FakeReferenceCursor(b"").raw(0), b"")
            verify_primitive_coverage(b"", reads)
        self.assertIs(FakeReferenceCursor.raw, original)

    def test_complete_model_rejects_partial_and_trailing_sections(self) -> None:
        require_complete_model(24028, 24028)
        for position in (0, 24027, 24029):
            with self.assertRaisesRegex(ValueError, f"model end {position}; expected stream end 24028"):
                require_complete_model(position, 24028)

    def test_source_snapshot_and_oracle_bind_owned_bytes(self) -> None:
        with tempfile.TemporaryDirectory(dir=WORKSPACE / "target") as temporary:
            root = Path(temporary)
            fixture = root / "owned.bin"
            fixture.write_bytes(b"ab")
            (root / "reader.py").write_text("pass\n")
            sources = reference_snapshot(root)
            self.assertEqual(set(sources), {"reader.py"})
            reads = [PrimitiveRead(0, 2, "raw", "6162", "reader.py", "read", 1)]
            oracle = oracle_payload(fixture, b"ab", reads, sources, 19000)
            self.assertEqual(oracle["stream_bytes"], 2)
            self.assertEqual(oracle["fixture_sha256"], oracle["stream_sha256"])
            (root / "reader.py").write_text("raise ValueError()\n")
            self.assertNotEqual(reference_snapshot(root), sources)

    def test_imported_source_must_come_from_declared_root(self) -> None:
        root = Path(unittest.__file__).resolve().parent
        self.assertIs(source_module("unittest", root), unittest)
        with self.assertRaisesRegex(ValueError, "expected source directory"):
            source_module("unittest", root.parent)

    def test_independent_wide_name_counts_utf16_units(self) -> None:
        self.assertEqual(short_wide_name(""), b"\xff\xfe\xff\x00")
        self.assertEqual(short_wide_name("🚂"), bytes.fromhex("fffeff023dd882de"))
        self.assertEqual(short_wide_name("a" * 254)[3], 254)
        for name in ("a" * 255, "🚂" * 128):
            with self.assertRaisesRegex(ValueError, "expected fewer than 255 UTF-16 code units"):
                short_wide_name(name)
        with self.assertRaisesRegex(ValueError, "expected valid UTF-16 text"):
            short_wide_name("\ud800")

    def test_owned_edit_changes_name_and_only_lightweight_size(self) -> None:
        original = bytearray(24028)
        original[:14] = OWNED_ROOT_PREFIX
        original[14:18] = struct.pack("<i", 2903)
        original[44:58] = OWNED_NAME
        changed = owned_name_edit(bytes(original), "Bessemer Ω")
        self.assertEqual(len(changed), 24038)
        self.assertEqual(struct.unpack_from("<i", changed, 14)[0], 2913)
        self.assertEqual(changed[18:44], original[18:44])
        self.assertEqual(changed[44:68], short_wide_name("Bessemer Ω"))
        self.assertEqual(changed[68:], original[58:])
        self.assertEqual(original[44:58], OWNED_NAME)
        with self.assertRaisesRegex(ValueError, "expected owned 24028-byte"):
            owned_name_edit(b"", "x")
        original[44] = 0
        with self.assertRaisesRegex(ValueError, "expected Part1 and 2903"):
            owned_name_edit(bytes(original), "x")

    def test_mfc_slots_keep_classes_nulls_and_imported_identity(self) -> None:
        external = {"_class": "moExtObject_c"}
        equal_but_distinct = {"_class": "moExtObject_c"}
        slots = map_slot_inventory([None, ("class", "moPart_c", 1), external, None, external, equal_but_distinct])
        self.assertEqual(slots[1], {"index": 1, "kind": "class", "class_name": "moPart_c", "schema": 1})
        self.assertEqual([slots[i]["identity_slot"] for i in (2, 4, 5)], [2, 2, 5])
        self.assertEqual(slots[3], {"index": 3, "kind": "null"})
        for invalid in ([], [external]):
            with self.assertRaisesRegex(ValueError, "expected null slot at index zero"):
                map_slot_inventory(invalid)

    def test_mfc_slot_shapes_reject_silent_coercion(self) -> None:
        for invalid in (("class", "", 1), ("class", "moPart_c", True), ("class", "moPart_c", 65536),
                        ("class", "moPart_c", -1), ("class", "moPart_c"), {"_class": 1}, 4):
            with self.subTest(invalid=invalid), self.assertRaisesRegex(ValueError, "MFC slot 2.*expected null"):
                reference_slot(2, invalid)


if __name__ == "__main__":
    unittest.main()
