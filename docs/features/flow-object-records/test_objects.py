"""Independent version strides, trailer conditions and malformed framing checks."""
import gzip
import struct
import tempfile
import unittest
from pathlib import Path
import freeze_objects as freeze
import object_requests as requests
from swarchive import ArchiveError


class ObjectRecordTests(unittest.TestCase):
    def test_all_cases_have_independent_strides_fields_and_exact_eof(self) -> None:
        strides=(36,36,36,36,40,24,25,81)
        for name,_,raw in requests.cases():
            value=requests.parse(raw);version=value['version'];count=value['count']
            self.assertEqual(len(raw),8+count*strides[version]+(20 if count and version<5 else 0),name)
            for ordinal,item in enumerate(value['items']):
                self.assertEqual((item['ordinal'],item['start'],item['end']),(ordinal,8+ordinal*strides[version],8+(ordinal+1)*strides[version]))
                self.assertEqual(item['discarded_legacy_i32'] is not None,version<5)
                self.assertEqual(item['member_7c_i32'] is not None,version>=4)
                self.assertEqual(item['state_byte'] is not None,version>=6)
                self.assertEqual(item['extension56_raw'] is not None,version==7)
                self.assertEqual(item['native_state_byte'],item['state_byte'] if version>=6 else 0)
            self.assertEqual(value['legacy_trailer'] is not None,bool(count and version<5))
            self.assertEqual(value['end'],len(raw));self.assertTrue(value['complete_payload_framing'])
            self.assertFalse(value['native_runtime_population_reproduced'] or value['field_meanings_assigned'])

    def test_literal_layouts_preserve_all_raw_fields(self) -> None:
        for version in range(8):
            raw=requests.payload(version,[0x80000000,0xffffffff]);value=requests.parse(raw)
            for pattern,item in zip((0x80000000,0xffffffff),value['items']):
                self.assertEqual(item['member_10_i32'],struct.unpack('<i',requests.signed(pattern))[0])
                self.assertEqual(item['block16_raw'],bytes((pattern+index)&255 for index in range(16)))
                if version<5:self.assertEqual(item['discarded_legacy_i32'],tuple(struct.unpack('<4i',b''.join(requests.signed(pattern^mask) for mask in (0,0xffffffff,0x80000000,0x7fffffff)))))
                if version>=4:self.assertEqual(item['member_7c_i32'],struct.unpack('<i',requests.signed(~pattern))[0])
                if version>=6:self.assertEqual(item['state_byte'],pattern&255)
                if version==7:self.assertEqual(item['extension56_raw'],bytes((pattern^index)&255 for index in range(56)))

    def test_legacy_trailer_exists_for_all_positive_counts_but_appends_only_at_one(self) -> None:
        for version in range(5):
            for count in (0,1,2,7):
                trailer=requests.parse(requests.payload(version,[0]*count))['legacy_trailer']
                if count==0:self.assertIsNone(trailer);continue
                self.assertEqual(trailer,dict(member_10_i32=-2147483647,block16_raw=bytes(reversed(range(16))),native_appended_object=count==1))

    def test_all_small_prefixes_and_extensions_are_rejected(self) -> None:
        for version in range(8):
            for count in (0,1,2,7):
                raw=requests.payload(version,[0xffffffff]*count)
                for end in range(len(raw)):
                    with self.assertRaises(ArchiveError):requests.parse(raw[:end])
                for suffix in (b'\0',b'\xff',bytes(20)):
                    with self.assertRaises(ArchiveError):requests.parse(raw+suffix)

    def test_negative_versions_counts_and_impossible_counts_are_rejected(self) -> None:
        for version in (-2147483648,-1,8,2147483647):
            with self.assertRaises(ArchiveError):requests.parse(struct.pack('<ii',version,0))
        for version in range(8):
            for count in (-2147483648,-1,1,2147483647):
                with self.assertRaises(ArchiveError):requests.parse(struct.pack('<ii',version,count))

    def test_wrong_identifier_and_wrapper_extent_are_rejected(self) -> None:
        raw=requests.payload(7,[0])
        for identifier,size in ((303,len(raw)),(304,len(raw)-1),(304,len(raw)+1)):
            with self.assertRaises(ArchiveError):requests.flowrecord304.read_record(dict(identifier=identifier,size=size,payload=raw))

    def test_native_projections_are_complete_and_separately_provenanced(self) -> None:
        rows,proof=requests.native_cases();self.assertEqual(len(rows),2)
        self.assertEqual(rows[0][2],rows[1][2]);self.assertNotEqual(proof[0]['source_sha256'],proof[1]['source_sha256'])
        for _,_,raw in rows:
            value=requests.parse(raw);self.assertEqual((value['version'],value['count'],value['end']),(3,1,64))
            self.assertEqual(value['items'][0]['discarded_legacy_i32'],(1936,624,1424,0))
            self.assertTrue(value['legacy_trailer']['native_appended_object'])

    def test_packets_and_proof_regenerate_exactly_in_temporary_root(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root=Path(directory);freeze.freeze(root)
            relative='tests/fixtures/flow-object-records/record304.bin.gz'
            self.assertEqual(gzip.decompress((root/relative).read_bytes()),gzip.decompress((requests.ROOT/relative).read_bytes()))
            relative='docs/features/flow-object-records/baseline-verification.json'
            self.assertEqual((root/relative).read_bytes(),(requests.ROOT/relative).read_bytes())
