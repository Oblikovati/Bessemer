"""Independent prefix length/metadata equations and opaque-suffix framing contracts."""
import gzip
import struct
import tempfile
import unittest
from pathlib import Path
import freeze_viewer as freeze
import viewer_requests as requests
from swarchive import ArchiveError


class ViewerPrefixTests(unittest.TestCase):
    def test_every_prefix_array_triplet_scalar_and_opaque_extent_rebuilds_decoded_body(self) -> None:
        for name,_,payload in requests.cases():
            value=requests.parse(payload);raw=requests.zlib.decompress(payload[4:]);lengths=value['lengths']
            self.assertEqual(len(lengths),3);self.assertTrue(all(length>0 for length in lengths))
            self.assertEqual(value['prefix_end'],48+sum(lengths)*8,name)
            arrays=[list(array['patterns']) for array in value['arrays']]
            rebuilt=requests.body(value['ignored_initial_i32'],value['stored_second_i32'],arrays,value['discarded_wide_triplet_patterns'],value['final_output_i32'],value['opaque_tail'])
            self.assertEqual(rebuilt,raw);self.assertEqual(raw[value['prefix_end']:],value['opaque_tail'])
            self.assertEqual(value['unknown_decoded_bytes'],len(raw)-value['prefix_end'])
            start=20
            for length,array in zip(lengths,value['arrays']):
                self.assertEqual((array['start'],array['end'],array['count']),(start,start+length*8,length));start+=length*8
            self.assertFalse(value['complete_payload_framing'] or value['engineering_fields_assigned'] or value['native_runtime_state_reproduced'])

    def test_second_word_one_normalizes_to_two_and_other_signed_values_remain_unchanged(self) -> None:
        for initial,second in ((99,1),(-2147483648,0),(2147483647,2),(1,-7),(0,-2147483648),(0,2147483647)):
            payload=requests.stored(3,requests.body(initial,second,[[0],[1],[2]],(3,4,5),-3))
            value=requests.parse(payload);expected=2 if second==1 else second
            self.assertEqual(value['native_second_output_i32'],expected)
            self.assertEqual(value['native_metadata_outputs'],dict(first_i32=-3,second_i32=expected))
            self.assertEqual(value['stored_second_i32'],second);self.assertEqual(value['ignored_initial_i32'],initial)

    def test_all_raw_wide_bits_scalar_bits_and_positive_length_boundaries_are_owned(self) -> None:
        rows=[requests.parse(payload) for _,_,payload in requests.cases()]
        for field in ('ignored_initial_i32','stored_second_i32','final_output_i32'):
            values={row[field]&0xffffffff for row in rows}
            for bit in range(32):self.assertTrue({1<<bit,0xffffffff^(1<<bit)}<=values)
        patterns={pattern for row in rows for array in row['arrays'] for pattern in array['patterns']}
        triplets={pattern for row in rows for pattern in row['discarded_wide_triplet_patterns']}
        for bit in range(64):self.assertTrue({1<<bit,0xffffffffffffffff^(1<<bit)}<=patterns & triplets)
        self.assertTrue({0,0xffffffffffffffff,0x7ff0000000000000,0xfff0000000000000,0x7ff8000000000042,0x7ff0000000000001}<=patterns)
        lengths={length for row in rows for length in row['lengths']}
        self.assertTrue({1,2,255,256,257,4095,4096,65535,65536}<=lengths)

    def test_arbitrary_decoded_suffixes_and_truncated_unknown_native_tails_stay_opaque(self) -> None:
        for name,_,payload in requests.tail_cases():
            value=requests.parse(payload);self.assertFalse(value['complete_payload_framing'],name)
            self.assertEqual(value['known_decoded_prefix_bytes'],value['prefix_end'])
            extended=requests.stored(value['outer_version'],requests.zlib.decompress(payload[4:])+b'\xffopaque\0')
            after=requests.parse(extended);self.assertEqual(after['prefix_end'],value['prefix_end'])
            self.assertEqual(after['opaque_tail'],value['opaque_tail']+b'\xffopaque\0')
        empty=requests.parse(requests.stored(2,requests.body(0,1,[[0],[1],[2]],(3,4,5),6)))
        self.assertEqual(empty['unknown_decoded_bytes'],0);self.assertFalse(empty['complete_payload_framing'])

    def test_short_known_prefixes_zero_negative_and_impossible_lengths_fail(self) -> None:
        raw=requests.body(0,1,[[0],[1,2],[3]],(4,5,6),-7)
        for end in range(len(raw)):self.assertRaises(ArchiveError,requests.parse,requests.stored(3,raw[:end]))
        for ordinal,length in ((0,0),(1,-1),(2,-2147483648),(0,2147483647)):
            malformed=bytearray(raw);malformed[8+ordinal*4:12+ordinal*4]=struct.pack('<i',length)
            self.assertRaises(ArchiveError,requests.parse,requests.stored(3,malformed))

    def test_outer_version_and_exact_compressed_stream_and_record_wrapper_are_required(self) -> None:
        raw=requests.body(0,1,[[0],[1],[2]],(3,4,5),6);payload=requests.stored(3,raw)
        for version in (-2147483648,-1,0,1):self.assertRaises(ArchiveError,requests.parse,requests.stored(version,raw))
        for end in range(len(payload)):self.assertRaises(ArchiveError,requests.parse,payload[:end])
        for suffix in (b'\0',b'junk',payload[4:]):self.assertRaises(ArchiveError,requests.parse,payload+suffix)
        corrupt=bytearray(payload);corrupt[-1]^=1;self.assertRaises(ArchiveError,requests.parse,corrupt)
        for identifier,size in ((122,len(payload)),(121,len(payload)-1),(121,len(payload)+1)):
            self.assertRaises(ArchiveError,requests.flowrecord121.read_record_prefix,dict(identifier=identifier,size=size,payload=payload))

    def test_native_manifest_known_prefix_and_full_unknown_suffix_are_preserved(self) -> None:
        rows,proof=requests.native_cases();self.assertEqual(len(rows),1);value=requests.parse(rows[0][2])
        self.assertEqual((value['outer_version'],value['prefix_end'],value['unknown_decoded_bytes'],value['decoded_body_bytes']),(3,616,141227,141843))
        self.assertEqual(value['lengths'],(45,13,13));self.assertEqual(proof['stored_bytes'],33601)
        self.assertEqual(value['native_metadata_outputs'],dict(first_i32=1,second_i32=2))
        self.assertFalse(value['complete_payload_framing'])

    def test_compression_variants_share_complete_decoded_images_and_regenerate_owned_packet(self) -> None:
        images=[requests.zlib.decompress(payload[4:]) for _,_,payload in requests.compression_cases()]
        self.assertEqual(len(images),20);self.assertTrue(all(raw==images[0] for raw in images))
        with tempfile.TemporaryDirectory() as directory:
            root=Path(directory);freeze.freeze(root)
            relative='tests/fixtures/flow-viewer-prefix/record121.bin.gz'
            self.assertEqual(gzip.decompress((root/relative).read_bytes()),gzip.decompress((requests.ROOT/relative).read_bytes()))
            relative='docs/features/flow-viewer-prefix/baseline-verification.json'
            self.assertEqual((root/relative).read_bytes(),(requests.ROOT/relative).read_bytes())
