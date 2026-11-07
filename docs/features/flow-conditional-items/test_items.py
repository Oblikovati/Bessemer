"""Independent complete conditional-item serialization and strict profile boundaries."""
import gzip
import struct
import tempfile
import unittest
from dataclasses import replace
from itertools import product
from pathlib import Path
import freeze_items as frozen
import requests
import snapshot
from wire_packet import ROOT
from swarchive import ArchiveError


def compact(identifier: int, version: int, outer: int = 6) -> bytes:
    spec=requests.ItemSpec(version=version,mode=0,wide_count=1,byte_count=1,word_count=1,groups=1)
    group=requests.GroupSpec(items=(spec,))
    return {126:lambda:requests.record126(group),315:lambda:requests.record315(group),128:lambda:requests.record128(group,outer)}[identifier]()


def read_decoded(identifier: int, decoded: bytes, outer: int = 6) -> dict:
    if identifier==126:
        return requests.flowrecord126.read(decoded)
    if identifier==315:
        return requests.flowrecord315.read_decoded(decoded)
    return requests.flowrecord128.read_decoded(decoded,version=outer)


class ConditionalItemTests(unittest.TestCase):
    def test_shared_field_packet_preserves_utf16_units_with_a_distinct_two_byte_tag(self) -> None:
        field=snapshot.StoredField(('units',),12,'H',(0,0xd800,0xdc00,0xffff))
        raw=frozen.field_packet(field)
        path=frozen.path_bytes(field.path)
        self.assertEqual(raw,path+struct.pack('<IBII4H',12,4,4,8,0,0xd800,0xdc00,0xffff))

    def test_packet_regenerates_exactly_in_a_temporary_root(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root=Path(directory);frozen.freeze(root)
            for identifier in (126,315,128):
                relative=f'tests/fixtures/flow-conditional-items/record{identifier}.bin.gz'
                self.assertEqual(gzip.decompress((root/relative).read_bytes()),gzip.decompress((ROOT/relative).read_bytes()))
            relative='docs/features/flow-conditional-items/baseline-verification.json'
            self.assertEqual((root/relative).read_bytes(),(ROOT/relative).read_bytes())

    def test_every_payload_rebuilds_all_source_fields_intervals_and_width_scaled_vectors(self) -> None:
        for name,identifier,stored in requests.cases():
            decoded,value=requests.parse(identifier,stored);expected=snapshot.record(identifier,value)
            self.assertEqual(bytes(expected.raw),decoded,name)
            offset=0
            for field in expected.fields:
                self.assertEqual(field.offset,offset);offset+=struct.calcsize('<'+field.encoding)*len(field.values)
            self.assertEqual(offset,len(decoded))
            self.assertEqual(expected.scopes[0],((),0,len(decoded)))

    def test_compact_profiles_reject_every_proper_decoded_prefix_and_trailing_bytes(self) -> None:
        for identifier,version,outer in product((126,315,128),range(6),(6,7)):
            decoded,_=requests.parse(identifier,compact(identifier,version,outer))
            for end in range(len(decoded)):
                with self.assertRaises(ArchiveError,msg=f'{identifier}:{version}:{outer}:{end}'):
                    read_decoded(identifier,decoded[:end],outer)
            with self.assertRaises(ArchiveError):
                read_decoded(identifier,decoded+b'\0',outer)

    def test_all_reached_count_fields_reject_negative_and_impossible_values(self) -> None:
        for identifier,version,outer in product((126,315,128),range(6),(6,7)):
            decoded,value=requests.parse(identifier,compact(identifier,version,outer))
            expected=snapshot.record(identifier,value)
            for offset,count in product(expected.counts,(-1,2147483647)):
                raw=decoded[:offset]+struct.pack('<i',count)+decoded[offset+4:]
                with self.assertRaises(ArchiveError,msg=f'{identifier}:{version}:{offset}:{count}'):
                    read_decoded(identifier,raw,outer)

    def test_compressed_profiles_reject_short_trailing_concatenated_and_corrupt_streams(self) -> None:
        for identifier,outer in product((315,128),(6,7)):
            stored=compact(identifier,5,outer);prefix=4 if identifier==128 else 0
            stream=stored[prefix:]
            candidates=[stored[:end] for end in range(len(stored))]
            candidates.extend((stored+b'\0',stored+stream,stored[:-1]+bytes((stored[-1]^255,))))
            for raw in candidates:
                with self.assertRaises(ArchiveError):
                    requests.parse(identifier,raw)

    def test_native_samples_keep_complete_populated_items_without_runtime_or_engineering_credit(self) -> None:
        rows,proof=requests.native_cases()
        self.assertEqual([row[1] for row in rows],[126,315,128]);self.assertEqual(len(proof),3)
        for _,identifier,stored in rows:
            decoded,value=requests.parse(identifier,stored)
            self.assertTrue(value['complete_payload_framing']);self.assertEqual(value['end'],len(decoded))
            self.assertEqual(bytes(snapshot.record(identifier,value).raw),decoded)

    def test_version_dimension_mode_protocol_and_wrapper_gates_reject_outside_profiles(self) -> None:
        for identifier,version,outer in product((126,315,128),range(6),(6,7)):
            stored=compact(identifier,version,outer);decoded,value=requests.parse(identifier,stored)
            expected=snapshot.record(identifier,value)
            for field in expected.fields:
                invalid=invalid_value(field,expected)
                if invalid is None:
                    continue
                changed=struct.pack('<'+field.encoding,invalid)
                raw=decoded[:field.offset]+changed+decoded[field.offset+len(changed):]
                with self.assertRaises(ArchiveError,msg=f'{identifier}:{field.path}'):
                    read_decoded(identifier,raw,outer)
            for record in (dict(identifier=999,size=len(stored),payload=stored),dict(identifier=identifier,size=len(stored)+1,payload=stored)):
                schema={126:requests.flowrecord126,315:requests.flowrecord315,128:requests.flowrecord128}[identifier]
                with self.assertRaises(ArchiveError):
                    schema.read_record(record)

    def test_presence_activity_and_mode_bytes_keep_stored_bits_and_nonzero_projection(self) -> None:
        seen=set();modes=set();presences=set();versions=set()
        for _,identifier,stored in requests.active_cases():
            _,value=requests.parse(identifier,stored);expected=snapshot.record(identifier,value)
            for field in expected.fields:
                if field.path[-1]=='active_byte':seen.update(field.values)
                if field.path[-1]=='mode_byte':modes.update(field.values)
                if field.path[-1]=='present_byte':presences.update(field.values)
                if field.path[-1]=='version' and 'item' in field.path:versions.update(field.values)
            for path,flag in expected.projections:
                if path[-1]=='native_mode_boolean':
                    mode=next(field.values[0] for field in expected.fields if field.path==path[:-1]+('mode_byte',))
                    self.assertEqual(flag,mode!=0)
        self.assertEqual(seen,set(range(256)));self.assertEqual(presences,set(range(256)))
        self.assertEqual(modes,set(range(256)));self.assertEqual(versions,set(range(6)))


def invalid_value(field: snapshot.StoredField, expected: snapshot.Snapshot) -> int | None:
    role=field.path[-1]
    if role=='version':
        return 6 if 'item' in field.path else 99
    if role=='dimension':
        return 99
    if role in ('mode_i32','protocol_i32','inner_word_i32'):
        return {'mode_i32':3,'protocol_i32':2,'inner_word_i32':1}[role]
    if role=='mode_byte':
        version=next(value.values[0] for value in expected.fields if value.path==field.path[:-1]+('version',))
        return 1 if version==1 else None
    return None
