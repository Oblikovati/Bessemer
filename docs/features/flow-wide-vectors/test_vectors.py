"""Independent length-table order, empty-vector presence and exact EOF checks."""
import gzip
import struct
import tempfile
import unittest
from pathlib import Path
import freeze_vectors as freeze
import vector_requests as requests
from swarchive import ArchiveError


class WideVectorTests(unittest.TestCase):
    def test_every_case_has_all_lengths_before_any_array_body(self) -> None:
        for name,_,raw in requests.cases():
            value=requests.parse(raw);count=value['count'];lengths=value['lengths']
            self.assertEqual(value['version'],0);self.assertEqual(len(lengths),count,name)
            self.assertEqual(len(value['arrays']),count)
            self.assertEqual(len(raw),8+count*4+sum(lengths)*8)
            self.assertEqual(struct.unpack('<'+'i'*count,raw[8:8+count*4]),lengths)
            start=8+count*4
            for length,values in zip(lengths,value['arrays']):
                self.assertEqual(len(values),length)
                self.assertEqual(raw[start:start+8*length],struct.pack('<'+'Q'*length,*values));start+=length*8
            self.assertEqual(start,len(raw));self.assertEqual(value['end'],len(raw))

    def test_literal_ragged_layout_and_present_empty_vectors_are_distinct(self) -> None:
        raw=struct.pack('<5i3Q',0,3,1,0,2,11,12,13)
        self.assertEqual(requests.payload([[11],[],[12,13]]),raw)
        self.assertEqual(requests.parse(raw)['arrays'],[(11,),(),(12,13)])
        self.assertEqual(requests.parse(requests.payload([]))['count'],0)
        for count in (1,2,257,4096):
            value=requests.parse(requests.payload([[] for _ in range(count)]))
            self.assertEqual(value['count'],count);self.assertEqual(value['lengths'],(0,)*count)
            self.assertEqual(value['arrays'],[() for _ in range(count)])

    def test_owned_patterns_counts_and_lengths_cover_independent_boundary_sets(self) -> None:
        values=[requests.parse(raw) for _,_,raw in requests.cases()]
        patterns={pattern for value in values for array in value['arrays'] for pattern in array}
        for bit in range(64):self.assertTrue({1<<bit,0xffffffffffffffff^(1<<bit)}<=patterns)
        self.assertTrue({0,0xffffffffffffffff,0x8000000000000000,0x7ff0000000000000,0xfff0000000000000,0x7ff8000000000042,0x7ff0000000000001}<=patterns)
        counts={value['count'] for value in values};lengths={length for value in values for length in value['lengths']}
        self.assertTrue({0,1,2,255,256,257,1024,4096}<=counts)
        self.assertTrue({0,1,2,255,256,257,4095,4096,65535,65536}<=lengths)

    def test_every_small_prefix_and_trailing_extension_is_rejected(self) -> None:
        for arrays in ([],[[]],[[0]],[[11],[],[12,13]],[[],[],[]],[[0]*7,[],[1]*3]):
            raw=requests.payload(arrays)
            for end in range(len(raw)):
                with self.assertRaises(ArchiveError):requests.parse(raw[:end])
            for suffix in (b'\0',b'\xff',bytes(8)):
                with self.assertRaises(ArchiveError):requests.parse(raw+suffix)

    def test_negative_absurd_and_inconsistent_counts_or_lengths_are_rejected(self) -> None:
        for count in (-2147483648,-1,1,2147483647):
            with self.assertRaises(ArchiveError):requests.parse(struct.pack('<ii',0,count))
        for length in (-2147483648,-1,1,2147483647):
            with self.assertRaises(ArchiveError):requests.parse(struct.pack('<iii',0,1,length))
        for lengths in ((0,1),(1,0),(2147483647,2147483647),(-1,0)):
            with self.assertRaises(ArchiveError):requests.parse(struct.pack('<4i',0,2,*lengths))

    def test_unsupported_versions_and_wrong_record_wrapper_are_rejected(self) -> None:
        for version in (-2147483648,-1,1,2147483647):
            with self.assertRaises(ArchiveError):requests.parse(struct.pack('<ii',version,0))
        raw=requests.payload([[11],[]])
        for identifier,size in ((147,len(raw)),(148,len(raw)-1),(148,len(raw)+1)):
            with self.assertRaises(ArchiveError):requests.wides.flowauxrecords.read_record(dict(identifier=identifier,size=size,payload=raw))

    def test_native_source_payload_and_complete_projection_match_the_manifest(self) -> None:
        rows,proof=requests.native_cases();self.assertEqual(len(rows),1)
        value=requests.parse(rows[0][2]);self.assertEqual(value,dict(version=0,count=1,lengths=(1,),arrays=[(4698062975489146916,)],end=20))
        self.assertEqual(proof['payload_bytes'],20);self.assertEqual(proof['record_end']-proof['record_start'],32)

    def test_packet_and_proof_regenerate_exactly_under_a_temporary_root(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root=Path(directory);freeze.freeze(root)
            relative='tests/fixtures/flow-wide-vectors/record148.bin.gz'
            self.assertEqual(gzip.decompress((root/relative).read_bytes()),gzip.decompress((requests.ROOT/relative).read_bytes()))
            relative='docs/features/flow-wide-vectors/baseline-verification.json'
            self.assertEqual((root/relative).read_bytes(),(requests.ROOT/relative).read_bytes())
