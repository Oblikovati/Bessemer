"""Independent raw framing, signed bit coverage and exact compressed-stream checks."""
import gzip
import struct
import tempfile
import unittest
from pathlib import Path
import freeze_wides as freeze
import wide_requests as requests
from swarchive import ArchiveError


class AuxWideTests(unittest.TestCase):
    def test_every_case_rebuilds_the_complete_decoded_body(self) -> None:
        for name,_,stored in requests.cases():
            value=requests.parse(stored);version=value['version'];count=value['count']
            self.assertEqual(len(requests.zlib.decompress(stored)),(12 if version==0 else 8)+count*8,name)
            self.assertEqual(value['discarded_legacy_i32'] is not None,version==0)
            self.assertEqual(len(value['patterns']),count)
            raw=requests.decoded(version,value['discarded_legacy_i32'],value['patterns'])
            self.assertEqual(requests.zlib.decompress(stored),raw)
            self.assertEqual(value['end'],len(raw));self.assertTrue(value['native_values_discarded'])

    def test_every_signed_legacy_bit_and_complement_and_raw_wide_pattern_is_owned(self) -> None:
        values=[requests.parse(stored) for _,_,stored in requests.cases()]
        legacy={value['discarded_legacy_i32']&0xffffffff for value in values if value['version']==0}
        for bit in range(32):self.assertTrue({1<<bit,0xffffffff^(1<<bit)}<=legacy)
        for version in (0,1):
            patterns={pattern for value in values if value['version']==version for pattern in value['patterns']}
            expected={0,0x8000000000000000,0x7ff0000000000000,0xfff0000000000000,0x7ff8000000000042,0x7ff0000000000001,0xffffffffffffffff}
            self.assertTrue(expected<=patterns)
            for bit in range(64):self.assertTrue({1<<bit,0xffffffffffffffff^(1<<bit)}<=patterns)
            counts={value['count'] for value in values if value['version']==version}
            self.assertTrue({0,1,2,255,256,257,4095,4096,65535,65536}<=counts)

    def test_all_small_decoded_prefixes_extensions_and_negative_counts_fail(self) -> None:
        for version in (0,1):
            for count in (0,1,2,7):
                raw=requests.decoded(version,-2147483648,[0xffffffffffffffff]*count)
                for end in range(len(raw)):
                    with self.assertRaises(ArchiveError):requests.parse(requests.zlib.compress(raw[:end]))
                with self.assertRaises(ArchiveError):requests.parse(requests.zlib.compress(raw+b'\0'))
            for count in (-2147483648,-1,1,2147483647):
                raw=struct.pack('<ii',version,count) if version==1 else struct.pack('<iii',version,0,count)
                with self.assertRaises(ArchiveError):requests.parse(requests.zlib.compress(raw))

    def test_unsupported_versions_and_wrong_wrapper_identity_or_size_fail(self) -> None:
        for version in (-2147483648,-1,2,2147483647):
            with self.assertRaises(ArchiveError):requests.parse(requests.zlib.compress(struct.pack('<ii',version,0)))
        stored=requests.zlib.compress(requests.decoded(1,0,[0]))
        for identifier,size in ((443,len(stored)),(444,len(stored)-1),(444,len(stored)+1)):
            with self.assertRaises(ArchiveError):requests.flowauxrecords.read_record(dict(identifier=identifier,size=size,payload=stored))

    def test_compressed_prefixes_trailing_bytes_concatenation_and_checksum_fail(self) -> None:
        stored=requests.zlib.compress(requests.decoded(0,-7,[0,0xffffffffffffffff]))
        for end in range(len(stored)):
            with self.assertRaises(ArchiveError):requests.parse(stored[:end])
        for suffix in (b'\0',b'junk',stored):
            with self.assertRaises(ArchiveError):requests.parse(stored+suffix)
        corrupt=bytearray(stored);corrupt[-1]^=1
        with self.assertRaises(ArchiveError):requests.parse(corrupt)

    def test_compression_variants_have_identical_decoded_projections(self) -> None:
        groups={version:[] for version in (0,1)}
        for _,_,stored in requests.compression_cases():
            value=requests.parse(stored);groups[value['version']].append((stored,value))
        for rows in groups.values():
            self.assertEqual(len(rows),20);self.assertGreater(len({stored for stored,_ in rows}),1)
            for _,value in rows:self.assertEqual(value,rows[0][1])

    def test_native_manifest_preserves_discarded_legacy_and_all_180_stored_zeros(self) -> None:
        rows,proof=requests.native_cases();self.assertEqual(len(rows),1)
        value=requests.parse(rows[0][2]);self.assertEqual((value['version'],value['discarded_legacy_i32'],value['count'],value['end']),(0,6,180,1452))
        self.assertEqual(value['patterns'],(0,)*180);self.assertEqual(proof['stored_bytes'],32)

    def test_packet_and_proof_regenerate_exactly_in_temporary_root(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root=Path(directory);freeze.freeze(root)
            relative='tests/fixtures/flow-aux-wide-values/record444.bin.gz'
            self.assertEqual(gzip.decompress((root/relative).read_bytes()),gzip.decompress((requests.ROOT/relative).read_bytes()))
            relative='docs/features/flow-aux-wide-values/baseline-verification.json'
            self.assertEqual((root/relative).read_bytes(),(requests.ROOT/relative).read_bytes())
