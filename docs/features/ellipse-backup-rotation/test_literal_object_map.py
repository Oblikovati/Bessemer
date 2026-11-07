"""Owned literal oracles for the reader-free fixture map extraction (#192)."""
from pathlib import Path
import struct
import sys
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]/'model-archives'))
from fixture_object_map import WireMap, object_tag, shared_object


class LiteralObjectMapTests(unittest.TestCase):
    def test_class_headers_retain_default_and_explicit_schema(self) -> None:
        """Pin old schema7 and explicit schema1 without reference imports; e.g. object_tag('X')."""
        self.assertEqual(object_tag('X'), b'\xff\xff\x07\x00\x01\x00X')
        self.assertEqual(WireMap(1).new('X', lambda: b'\x80'), b'\xff\xff\x01\x00\x01\x00X\x80')
        with self.assertRaises(UnicodeEncodeError):
            object_tag('Ω')
        with self.assertRaises(struct.error):
            object_tag('X', 65536)

    def test_handle_and_both_count_encodings_match_existing_literals(self) -> None:
        """Keep escaped handle indices and both null-array count prefixes; e.g. WireMap().array()."""
        mapping = WireMap()
        handle = b'\xff\xff\x07\x00\x0d\x00sgPointHandle'
        self.assertEqual(mapping.handle(), handle + b'\x7f\x77\xff\xff\xff\xff\xed\xff\xff\xff\xff\xff\xff\x7f')
        array = b'\xff\xff\x07\x00\x09\x00suObArray'
        self.assertEqual(mapping.array(), array + b'\x02\x00' + bytes(4))
        self.assertEqual(mapping.array(extended=True), array + b'\xff\xff\x02\x00\x00\x00' + bytes(4))
        self.assertEqual(mapping.slot, 6)

    def test_nested_identity_is_reserved_before_body_and_alias_adds_no_slots(self) -> None:
        """Pin the same root/child cycle and alias as the existing builders; e.g. shared_object(w, ids, 'root', 'Root', body)."""
        mapping = WireMap()
        identities: dict[str, int] = {}
        def root_body() -> bytes:
            return shared_object(mapping, identities, 'child', 'Child', lambda: struct.pack('<H', identities['root']))
        raw = shared_object(mapping, identities, 'root', 'Root', root_body)
        expected = b'\xff\xff\x07\x00\x04\x00Root\xff\xff\x07\x00\x05\x00Child\x02\x00'
        self.assertEqual(raw, expected)
        self.assertEqual(shared_object(mapping, identities, 'root', 'Root', lambda: b'ignored'), b'\x02\x00')
        self.assertEqual(identities, {'root': 2, 'child': 4})
        self.assertEqual(mapping.slot, 4)


if __name__ == '__main__':
    unittest.main()
