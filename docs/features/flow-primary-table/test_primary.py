"""Independent complete reconstruction, all conditional byte values and strict count framing."""
import gzip
import struct
import tempfile
import unittest
from itertools import product
from pathlib import Path
import freeze_primary as frozen
import bounds
from swarchive import ArchiveError


def read(raw: bytes) -> dict:
    return frozen.cases.schema.read(raw)


def compact(version: int, kind: int = 4) -> bytes:
    spec=frozen.cases.GroupSpec(presence=2,first_count=2,second_count=1,rows=(0,1,3),blocks=1,elements=1)
    return frozen.cases.record(version,(frozen.cases.item(2,kind,spec),))


class PrimaryTableBaselineTests(unittest.TestCase):
    def test_packet_regenerates_exactly_in_a_temporary_root(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root=Path(directory);frozen.freeze(root)
            relative=Path('tests/fixtures/flow-primary-table/primary.bin.gz')
            self.assertEqual(gzip.decompress((root/relative).read_bytes()),gzip.decompress((frozen.ROOT/relative).read_bytes()))

    def test_every_complete_primary_record_rebuilds_all_source_fields_and_nested_intervals(self) -> None:
        for name,raw in frozen.cases.cases():
            value=read(raw);rebuilt,expected=frozen.packet.record(value)
            self.assertEqual(rebuilt,raw,name);self.assertTrue(expected)
            self.assertEqual(value['end'],len(raw));self.assertTrue(value['complete_payload_framing'])
            self.assertFalse(value['engineering_fields_assigned'])

    def test_every_raw_kind_mode_presence_and_stored_flag_byte_has_explicit_coverage(self) -> None:
        kinds=set();modes=set();presence=set();flags=set()
        for _,raw in frozen.cases.cases():
            for item in read(raw)['items']:
                kinds.add(item['kind_byte']);self.capture_bytes(item,modes,presence,flags)
        for values in (kinds,modes-{None},presence,flags-{None}):
            self.assertEqual(values,set(range(256)))
        self.assertIn(None,modes);self.assertIn(None,flags)

    def capture_bytes(self, item: dict, modes: set[int | None], presence: set[int], flags: set[int | None]) -> None:
        for group in item['groups']:
            presence.add(group['presence_byte']);modes.add(group['stored_mode_byte'])
            if group['body'] is not None:
                flag=group['body']['stored_flag'];flags.add(flag)
                self.assertEqual(group['body']['native_flag'],flag==1)

    def test_version_branches_keep_two_groups_optional_bodies_and_typed_lists_distinct(self) -> None:
        for _,raw in frozen.cases.cases():
            value=read(raw)
            for item in value['items']:
                self.assertEqual(len(item['groups']),2)
                self.check_groups(value['version'],item)

    def check_groups(self, version: int, item: dict) -> None:
        for ordinal,group in enumerate(item['groups']):
            self.assertEqual(group['ordinal'],ordinal)
            self.assertEqual(group['stored_mode_byte'] is None,version!=3)
            self.assertEqual(group['body'] is None,group['presence_byte']==0)
            self.assertEqual(group['typed_elements'] is None,item['kind_byte'] not in (2,3))
            if group['body'] is not None:
                self.assertEqual(group['body']['legacy_wide9'] is not None,version==0)
                self.assertEqual(group['body']['stored_flag'] is not None,version>=2)

    def test_every_compact_typed_layout_rejects_all_proper_prefixes_and_appended_bytes(self) -> None:
        for version,kind in product(range(4),range(1,5)):
            self.check_prefixes(compact(version,kind))

    def check_prefixes(self, raw: bytes) -> None:
        for end in range(len(raw)):
            with self.assertRaises(ArchiveError,msg=f'prefix{end}'):
                read(raw[:end])
        with self.assertRaises(ArchiveError):
            read(raw+b'\0')

    def test_every_nested_count_rejects_negative_or_physically_impossible_values(self) -> None:
        for version,kind in product(range(4),range(1,5)):
            raw=compact(version,kind)
            for position in bounds.positions(read(raw)):
                self.check_bad_counts(raw,position)

    def check_bad_counts(self, raw: bytes, position: int) -> None:
        for count in (-1,2147483647):
            changed=raw[:position]+struct.pack('<i',count)+raw[position+4:]
            with self.assertRaises(ArchiveError,msg=f'count{position}:{count}'):
                read(changed)

    def test_unknown_element_factory_is_only_accepted_when_all_typed_lists_are_empty(self) -> None:
        for kind,element in product((2,3),(0,5,127,255)):
            raw=frozen.cases.record(2,(frozen.cases.item(kind,element,frozen.cases.GroupSpec()),))
            self.assertEqual(read(raw)['items'][0]['element_kind_byte'],element)
            populated=bytearray(compact(2));populated[9]=element
            with self.assertRaises(ArchiveError):
                read(populated)

    def test_prefix_and_record_wrapper_contracts_remain_distinct_from_complete_payloads(self) -> None:
        for version in range(4):
            for raw in (frozen.cases.record(version,()),compact(version)):
                value=frozen.cases.schema.read_prefix(raw)
                self.assertEqual(value['prefix_end'],8);self.assertEqual(value['opaque_tail'],raw[8:])
                self.assertEqual(value['complete_payload_framing'],len(raw)==8)
                self.assertEqual(frozen.cases.schema.read_record(dict(identifier=439,size=len(raw),payload=raw))['end'],len(raw))
        for version in (-2147483648,-1,4,2147483647):
            with self.assertRaises(ArchiveError):
                read(struct.pack('<ii',version,0))
        for identifier,size in ((432,8),(439,9)):
            with self.assertRaises(ArchiveError):
                frozen.cases.schema.read_record(dict(identifier=identifier,size=size,payload=bytes(8)))
