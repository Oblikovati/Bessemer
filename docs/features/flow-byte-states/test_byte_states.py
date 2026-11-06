"""Strict outer and inner framing, header-bit, raw-value and native oracles."""
import gzip
from collections import Counter
import struct
import tempfile
import unittest
import zlib
from pathlib import Path
import byte_state_requests as requests
import freeze_byte_states as frozen
from swarchive import ArchiveError


class ByteStateTests(unittest.TestCase):
    def test_all_complete_source_results_have_independent_literal_decoded_images(self) -> None:
        for name,_,stored in requests.cases():
            value=requests.parse(stored)
            raw=struct.pack('<ii',value['header_i32'],value['count'])+value['raw_bytes']
            self.assertEqual(raw,zlib.decompress(stored),name)
            self.assertEqual(value['end'],len(raw));self.assertEqual(value['count'],len(raw)-8)
            self.assertFalse(value['native_header_dispatch'])

    def test_all_header_bits_complements_and_raw_bytes_are_owned_without_dispatch(self) -> None:
        parsed=[requests.parse(stored) for _,_,stored in requests.cases()]
        headers={value['header_i32']&0xffffffff for value in parsed}
        self.assertEqual(headers.issuperset(requests.HEADERS),True)
        self.assertEqual(set(b''.join(value['raw_bytes'] for value in parsed)),set(range(256)))
        for bit in range(32):self.assertIn(1<<bit,headers);self.assertIn(0xffffffff^(1<<bit),headers)

    def test_every_compact_outer_and_inner_prefix_is_rejected(self) -> None:
        raw=requests.decoded(-1,bytes(range(16)));stored=zlib.compress(raw)
        for end in range(len(stored)):
            with self.assertRaises(ArchiveError):requests.parse(stored[:end])
        for end in range(len(raw)):
            with self.assertRaises(ArchiveError):requests.parse(zlib.compress(raw[:end]))

    def test_outer_extensions_concatenation_and_decoded_extensions_are_rejected(self) -> None:
        raw=requests.decoded(0,b'abc');stored=zlib.compress(raw)
        for suffix in (b'\0',b'\xff'*16,zlib.compress(b'')):
            with self.assertRaises(ArchiveError):requests.parse(stored+suffix)
        for suffix in (b'\0',b'\xff'*16):
            with self.assertRaises(ArchiveError):requests.parse(zlib.compress(raw+suffix))

    def test_header_checksum_raw_deflate_and_gzip_fail(self) -> None:
        raw=requests.decoded(0,b'abc');stored=zlib.compress(raw)
        for offset in (0,1,*range(len(stored)-4,len(stored))):
            broken=bytearray(stored);broken[offset]^=1
            with self.assertRaises(ArchiveError):requests.parse(broken)
        for window in (-15,31):
            compressor=zlib.compressobj(wbits=window)
            with self.assertRaises(ArchiveError):requests.parse(compressor.compress(raw)+compressor.flush())

    def test_negative_impossible_and_mismatched_counts_and_carrier_guards_fail(self) -> None:
        for count in (-2147483648,-1,0,1,4,2147483647):
            with self.assertRaises(ArchiveError):requests.parse(zlib.compress(struct.pack('<ii',-1,count)+b'abc'))
        stored=zlib.compress(requests.decoded(0,b'abc'))
        for identifier,size in ((999,len(stored)),(316,len(stored)-1),(316,len(stored)+1)):
            with self.assertRaises(ArchiveError):requests.flowauxrecords.read_record(dict(identifier=identifier,size=size,payload=stored))

    def test_native_manifest_preserves_exact_stream_and_high_bit_values(self) -> None:
        rows,proof=requests.native_cases();self.assertEqual(len(rows),1)
        value=requests.parse(rows[0][2])
        self.assertEqual((value['header_i32'],value['count'],value['end']),(0,5532,5540))
        self.assertEqual(proof['stored_bytes'],265)
        self.assertEqual(dict(Counter(value['raw_bytes'])),{3:90,4:90,236:1476,253:3876})
        self.assertTrue(any(byte<128 for byte in value['raw_bytes']))
        self.assertTrue(any(byte>=128 for byte in value['raw_bytes']))

    def test_packet_and_hash_report_regenerate_exactly_in_a_temporary_root(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root=Path(directory);frozen.freeze(root)
            relative='tests/fixtures/flow-byte-states/record316.bin.gz'
            self.assertEqual(gzip.decompress((root/relative).read_bytes()),gzip.decompress((requests.ROOT/relative).read_bytes()))
            relative='docs/features/flow-byte-states/baseline-verification.json'
            self.assertEqual((root/relative).read_bytes(),(requests.ROOT/relative).read_bytes())
