"""Check complete mixed records against independent reconstruction and typed domains."""
import gzip
import struct
import tempfile
import unittest
from pathlib import Path
import freeze_mixed as frozen
from test_objects import rebuild_body
from test_kind_two import rebuild as rebuild_two
from swarchive import ArchiveError


def rebuild(raw: bytes, value: dict) -> bytes:
    out = raw[:value['backend_start']] + b'\0\0' + struct.pack('<i', len(value['first_table']))
    for entry in value['first_table']:
        assert len(out) == entry['start']
        encode = rebuild_body if entry['kind'] == 1 else rebuild_two
        out += struct.pack('<i', entry['kind']) + entry['key_raw'] + encode(dict(entry['body'], opaque_tail=b''))
        assert len(out) == entry['end']
    out += b'\0' + struct.pack('<i', len(value['second_table']))
    for entry in value['second_table']:
        assert len(out) == entry['start']
        body = entry['body']
        out += struct.pack('<i', entry['kind']) + frozen.profiles.references.body(entry['kind'], body.get('state_byte', 0), body['references_raw'], body['key_raw'])
        assert len(out) == entry['end']
    assert len(out) == value['backend_end']
    out += struct.pack('<i', len(value['outer_reference_keys_raw'])) + b''.join(value['outer_reference_keys_raw'])
    return out + b''.join(struct.pack('<i', item) for item in value['outer_attribute_i32']) + struct.pack('<ii', *value['final_collection_counts'])


def patched(raw: bytes, position: int, value: bytes) -> bytes:
    return raw[:position] + value + raw[position + len(value):]


class MixedProfileBaselineTests(unittest.TestCase):
    def test_frozen_packet_regenerates_exactly(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            frozen.freeze(root)
            relative = Path('tests/fixtures/flow-medium-mixed/profiles.bin.gz')
            self.assertEqual(gzip.decompress((root / relative).read_bytes()), gzip.decompress((frozen.ROOT / relative).read_bytes()))

    def test_every_nested_field_rebuilds_complete_records_and_exact_eof(self) -> None:
        for name, raw in frozen.cases():
            value = frozen.read(raw)
            self.assertEqual(rebuild(raw, value), raw, name)
            self.assertEqual(value['end'], len(raw))
            self.assertTrue(value['complete_record_framing'])
            self.assertTrue(value['fresh_backend_assumed'])
            for key in ('full_engineering_schema', 'native_runtime_state_reproduced', 'installed_kind2_fixture_validated'):
                self.assertFalse(value[key])

    def test_every_proper_prefix_of_six_models_in_both_orders_is_rejected(self) -> None:
        for model in range(6):
            for order in ('one-first', 'two-first'):
                raw = frozen.record(3, model, order)
                for end in range(len(raw)):
                    with self.assertRaises(ArchiveError, msg=f'{model}:{order}:{end}'):
                        frozen.read(raw[:end])
                with self.assertRaises(ArchiveError):
                    frozen.read(raw + b'\0')

    def test_reference_maps_are_separate_even_with_equal_keys_in_both_kinds(self) -> None:
        for order in ('one-first', 'two-first', 'duplicates'):
            raw = frozen.record(3, 5, order)
            value = frozen.read(raw)
            for entry in value['second_table']:
                wrong = 2 if entry['kind'] == 1 else 1
                position = entry['start'] + (27 if entry['kind'] == 1 else 26)
                key = frozen.profiles.references.KEYS[wrong]
                with self.assertRaises(ArchiveError):
                    frozen.read(patched(raw, position, key))
            self.assertEqual(rebuild(raw, value), raw)

    def test_outer_references_use_all_second_entries_but_require_membership(self) -> None:
        raw = frozen.record(3, 0, 'duplicates')
        value = frozen.read(raw)
        self.assertEqual(value['outer_reference_keys_raw'], [frozen.profiles.references.KEYS[1]] * 3)
        self.assertEqual(value['outer_attribute_i32'], (-2147483648, 0, 2147483647))
        with self.assertRaises(ArchiveError):
            frozen.read(patched(raw, value['backend_end'] + 4, frozen.profiles.references.KEYS[3]))

    def test_duplicate_entries_and_opposite_kinds_remain_physically_ordered(self) -> None:
        value = frozen.read(frozen.record(3, 4, 'duplicates'))
        self.assertEqual([entry['kind'] for entry in value['first_table']], [1, 2, 1, 2, 1, 2])
        self.assertEqual([entry['kind'] for entry in value['second_table']], [1, 2, 1])
        for entries in (value['first_table'], value['second_table']):
            self.assertEqual([entry['ordinal'] for entry in entries], list(range(len(entries))))
            self.assertTrue(all(left['end'] == right['start'] for left, right in zip(entries, entries[1:])))
        second = value['second_table']
        self.assertIn('state_byte', second[0]['body'])
        self.assertNotIn('state_byte', second[1]['body'])

    def test_table_counts_tags_versions_and_final_collections_reject_invalid_framing(self) -> None:
        raw = frozen.record(3, 5, 'two-first')
        value = frozen.read(raw)
        counts = [value['backend_start'] + 2, value['first_table'][-1]['end'] + 1, value['backend_end'], len(raw) - 8, len(raw) - 4]
        for position in counts:
            for count in (-1, 2147483647):
                with self.assertRaises(ArchiveError):
                    frozen.read(patched(raw, position, struct.pack('<i', count)))
        for entry in value['first_table'] + value['second_table']:
            for kind in (-2147483648, -1, 0, 3, 2147483647):
                with self.assertRaises(ArchiveError):
                    frozen.read(patched(raw, entry['start'], struct.pack('<i', kind)))
        for position in (value['backend_start'], value['backend_start'] + 1, value['first_table'][-1]['end']):
            for version in range(1, 256):
                with self.assertRaises(ArchiveError):
                    frozen.read(patched(raw, position, bytes((version,))))

    def test_fresh_backend_active_outer_and_string_mode_are_explicit(self) -> None:
        raw = frozen.record(3, 0, 'two-first')
        record = dict(identifier=424, size=len(raw), payload=raw)
        read = frozen.profiles.references.schema.read_record_424_liquid_profile
        for fresh in (False, None, 0, 1):
            with self.assertRaises(ArchiveError):
                read(record, string_mode=0, fresh_backend=fresh)
        for mode in (False, True, 1, None):
            with self.assertRaises(ArchiveError):
                read(record, string_mode=mode, fresh_backend=True)
        for version in (0, 1, 128, 255):
            with self.assertRaises(ArchiveError):
                frozen.read(bytes((version,)) + raw[1:])
