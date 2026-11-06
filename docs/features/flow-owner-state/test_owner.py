"""Independent native-member update equations and no-fallback selection checks."""
import gzip
import struct
import tempfile
import unittest
from pathlib import Path
import freeze_owner as freeze
import owner_requests as requests
from swarchive import ArchiveError


def expected_updates(identifier: int, raw: bytes) -> dict:
    if identifier==434:
        values=struct.unpack('<6i',raw[4:28])+(raw[28],)
    else:
        if raw[:4]==b'\xff'*4:return {}
        words=struct.unpack('<50i',raw)
        values=(words[5],words[7],words[1],words[8],words[2],-1,0)
    return {hex(offset):value for offset,value in zip((8,12,16,20,24,28,32),values)}


def malformed_record(identifier: int, raw: bytes) -> bool:
    if identifier==122:return len(raw)!=200
    return len(raw)!=29 or struct.unpack_from('<i',raw)[0] not in (0,1)


class OwnerStateTests(unittest.TestCase):
    def test_every_stored_field_and_native_update_matches_independent_raw_equations(self) -> None:
        for name,identifier,_,raw in requests.cases():
            value=requests.parse(identifier,raw)
            self.assertEqual(value['updates'],expected_updates(identifier,raw),name)
            if identifier==122:
                self.assertEqual(len(raw),200);self.assertEqual(len(value['words']),50)
                self.assertEqual(requests.words(value['words']),raw)
                self.assertEqual(value['native_update_gate'],raw[:4]!=b'\xff'*4)
            else:
                self.assertEqual(len(raw),29);self.assertIn(value['version'],(0,1))
                self.assertEqual(requests.modern(value['version'],value['values'],value['state_byte']),raw)
            self.assertEqual(value['end'],len(raw))

    def test_each_legacy_word_and_modern_member_has_every_signed_bit_and_complement(self) -> None:
        rows=requests.cases();patterns=set(requests.PATTERNS)
        for identifier,count,start in ((122,50,0),(434,6,4)):
            for position in range(count):
                values={struct.unpack_from('<I',raw,start+position*4)[0] for _,kind,_,raw in rows if kind==identifier}
                self.assertTrue(patterns<=values)
        for version in (0,1):
            states={raw[-1] for _,identifier,_,raw in rows if identifier==434 and struct.unpack_from('<i',raw)[0]==version}
            self.assertEqual(states,set(range(256)))

    def test_gate_is_exact_all_ones_and_no_legacy_header_dispatch_occurs(self) -> None:
        for gate in (0,1,0x7fffffff,0x80000000,0xfffffffe,0xffffffff):
            value=requests.parse(122,requests.legacy(0,gate))
            self.assertEqual(value['native_update_gate'],gate!=0xffffffff)
            self.assertEqual(bool(value['updates']),gate!=0xffffffff)
        self.assertEqual(requests.parse(122,requests.legacy(0,0xffffffff))['words'][0],0xffffffff)

    def test_latest_preferred_identifier_is_selected_before_any_payload_parse(self) -> None:
        for _,raw in requests.selection_cases():
            document=requests.flowresults.read(raw);records=document['records']
            preferred=[record for record in records if record['identifier']==434]
            if not preferred:preferred=[record for record in records if record['identifier']==122]
            if not preferred:
                self.assertIsNone(requests.flowauxrecords.select_122_or_434(document));continue
            record=preferred[-1]
            if malformed_record(record['identifier'],record['payload']):
                self.assertRaises(ArchiveError,requests.flowauxrecords.select_122_or_434,document)
                continue
            selected=requests.flowauxrecords.select_122_or_434(document)
            self.assertEqual(selected['identifier'],record['identifier'])
            self.assertEqual(selected['projection']['updates'],expected_updates(record['identifier'],record['payload']))

    def test_all_small_prefixes_extensions_and_unsupported_modern_versions_fail(self) -> None:
        for identifier,raw in ((122,requests.legacy(5,0x80000000)),(434,requests.modern(0,list(range(6)),255)),(434,requests.modern(1,list(range(6)),0))):
            for end in range(len(raw)):
                self.assertRaises(ArchiveError,requests.parse,identifier,raw[:end])
            for suffix in (b'\0',b'\xff',bytes(8)):
                with self.assertRaises(ArchiveError):requests.parse(identifier,raw+suffix)
        for version in (-2147483648,-1,2,2147483647):
            with self.assertRaises(ArchiveError):requests.parse(434,requests.modern(version,list(range(6)),0))

    def test_wrong_auxiliary_identifier_or_wrapper_extent_fails(self) -> None:
        raw=requests.legacy(0,3)
        for identifier,size in ((123,200),(122,199),(122,201)):
            with self.assertRaises(ArchiveError):requests.flowauxrecords.read_record(dict(identifier=identifier,size=size,payload=raw))

    def test_native_sources_retain_fields_and_projected_carrier_relative_order(self) -> None:
        rows,proof=requests.native_cases();self.assertEqual([row[1] for row in rows],[122,434,122])
        self.assertEqual(rows[0][3],rows[2][3]);self.assertNotEqual(proof[0]['source_sha256'],proof[2]['source_sha256'])
        self.assertEqual(requests.parse(434,rows[1][3])['values'],(3368,2584,5952,0,1936,0))
        projections=dict(requests.selection_cases())
        for name,identifier in (('native-cpt-projection',434),('native-fld-projection',122)):
            document=requests.flowresults.read(projections[name])
            self.assertEqual(requests.flowauxrecords.select_122_or_434(document)['identifier'],identifier)
        self.assertLess(proof[1]['record_start'],proof[0]['record_start'])

    def test_packet_and_proof_regenerate_exactly_in_a_temporary_root(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root=Path(directory);freeze.freeze(root)
            relative='tests/fixtures/flow-owner-state/records.bin.gz'
            self.assertEqual(gzip.decompress((root/relative).read_bytes()),gzip.decompress((requests.ROOT/relative).read_bytes()))
            relative='docs/features/flow-owner-state/baseline-verification.json'
            self.assertEqual((root/relative).read_bytes(),(requests.ROOT/relative).read_bytes())
