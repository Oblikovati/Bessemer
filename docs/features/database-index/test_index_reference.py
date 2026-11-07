"""Owned index continuation, role ownership and rejection boundaries (#230)."""
import struct
import unittest
from index_reference import inspect_default_file, map_record_page, page_bytes, read_index


def selected_file() -> bytearray:
    """Construct all five explicit roles without native fixture assumptions."""
    raw = bytearray(4096 * 6)
    raw[:11] = b"unqlite\xdb\x7c\x27\x12"
    struct.pack_into('>I', raw, 19, 4096)
    struct.pack_into('>IIQQQQI', raw, 4096, 0xfa782dcb, 0x03f523f5, 4, 0, 2, 3, 1)
    struct.pack_into('>QQ', raw, 4096 + 44, 0, 2)
    struct.pack_into('>QIQQ', raw, 3 * 4096, 0, 1, 1, 5)
    return raw


class NativeIndexReferenceTests(unittest.TestCase):
    def test_continuation_free_page_and_empty_buckets_have_complete_unique_roles(self) -> None:
        raw = selected_file()
        index = read_index(raw, 4096)
        self.assertEqual(index.map_pages, (1, 3))
        self.assertEqual(index.entries, ((0, 2), (1, 5)))
        self.assertEqual(inspect_default_file(raw), {'pages': 6, 'index_pages': 2,
                         'buckets': 2, 'cells': 0, 'free_pages': 1, 'overflow_pages': 0})

    def test_map_count_requires_entire_pairs_including_final_sixteen_bytes(self) -> None:
        for identity, count_at, start in ((1, 40, 44), (3, 8, 12)):
            raw = selected_file()
            count = (4096 - start) // 16
            struct.pack_into('>I', raw, identity * 4096 + count_at, count)
            self.assertEqual(len(map_record_page(raw, identity, 4096)[1]), count)
            struct.pack_into('>I', raw, identity * 4096 + count_at, count + 1)
            with self.assertRaises(AssertionError):
                map_record_page(raw, identity, 4096)

    def test_saved_duplicate_entries_are_preserved_before_partition_admission(self) -> None:
        raw = selected_file()
        struct.pack_into('>Q', raw, 3 * 4096 + 12, 0)
        self.assertEqual(read_index(raw, 4096).entries, ((0, 2), (0, 5)))
        with self.assertRaises(AssertionError):
            inspect_default_file(raw)

    def test_cycles_cross_role_aliases_unknown_pages_and_invalid_partition_are_rejected(self) -> None:
        changes = ((3 * 4096, '>Q', 3), (4 * 4096, '>Q', 4),
                   (4096 + 8, '>Q', 2), (4096 + 8, '>Q', 0),
                   (4096 + 24, '>Q', 0), (4096 + 24, '>Q', 3),
                   (4096 + 16, '>Q', 2), (4096 + 32, '>Q', 6))
        for offset, encoding, value in changes:
            with self.subTest(offset=offset, value=value):
                raw = selected_file()
                struct.pack_into(encoding, raw, offset, value)
                with self.assertRaises(AssertionError):
                    inspect_default_file(raw)

    def test_partial_pages_negative_identity_and_wrong_hash_profile_are_rejected(self) -> None:
        for raw, identity in ((selected_file()[:-1], 1), (selected_file(), -1),
                              (selected_file(), 6)):
            with self.assertRaises(AssertionError):
                page_bytes(raw, identity, 4096)
        raw = selected_file()
        raw[4100] ^= 1
        with self.assertRaises(AssertionError):
            read_index(raw, 4096)


if __name__ == '__main__':
    unittest.main()
