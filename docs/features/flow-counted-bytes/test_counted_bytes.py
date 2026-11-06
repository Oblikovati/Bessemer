"""Independent length, value, framing and native baseline checks."""
import gzip
import struct
import tempfile
import unittest
from pathlib import Path
import freeze
import counted_requests as requests
from swarchive import ArchiveError


class CountedByteTests(unittest.TestCase):
    def test_every_case_rebuilds_exact_count_values_and_eof(self) -> None:
        for name,native,raw in requests.cases():
            value=requests.parse(raw)
            self.assertEqual(raw,struct.pack('<i',value['count'])+value['values'],name)
            self.assertEqual(value['end'],4+value['count'])
            self.assertTrue(value['complete_payload_framing'])
            self.assertFalse(value['engineering_fields_assigned'] or value['runtime_index_mapping_reproduced'])

    def test_every_byte_single_bit_complement_and_boundary_length_is_owned(self) -> None:
        values=[requests.parse(raw)['values'] for _,_,raw in requests.cases()]
        self.assertEqual(set(b''.join(values)),set(range(256)))
        for byte in range(256):self.assertIn(bytes((byte,)),values)
        lengths={len(raw) for raw in values}
        for length in (0,1,2,255,256,257,4095,4096,65535,65536):self.assertIn(length,lengths)

    def test_every_small_prefix_and_extensions_fail(self) -> None:
        for length in (0,1,2,7,16,257):
            raw=requests.payload(bytes(index&255 for index in range(length)))
            for end in range(len(raw)):
                with self.assertRaises(ArchiveError):requests.parse(raw[:end])
            for suffix in (b'\0',b'\xff',b'\0'*16):
                with self.assertRaises(ArchiveError):requests.parse(raw+suffix)

    def test_negative_impossible_and_inconsistent_counts_fail(self) -> None:
        for count in (-2147483648,-1,1,2147483647):
            with self.assertRaises(ArchiveError):requests.parse(struct.pack('<i',count))
        for count in (0,1,2,4,2147483647):
            with self.assertRaises(ArchiveError):requests.parse(struct.pack('<i',count)+b'abc')

    def test_wrong_identifier_and_wrapper_size_fail(self) -> None:
        raw=requests.payload(b'\xff')
        for identifier,size in ((14,5),(12,4),(12,6)):
            with self.assertRaises(ArchiveError):requests.flowrecord12.read_record(dict(identifier=identifier,size=size,payload=raw))

    def test_native_manifest_count_raw_projection_and_extent_are_exact(self) -> None:
        rows,proof=requests.native_cases();self.assertEqual(len(rows),1)
        value=requests.parse(rows[0][2])
        self.assertEqual((value['count'],value['end']),(3984,3988))
        self.assertEqual(value['values'],bytes(3984))
        self.assertEqual(proof['record_end']-proof['record_start'],4000)

    def test_owned_packet_and_proof_regenerate_exactly_in_a_temporary_root(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root=Path(directory);freeze.freeze(root)
            relative='tests/fixtures/flow-counted-bytes/record12.bin.gz'
            self.assertEqual(gzip.decompress((root/relative).read_bytes()),gzip.decompress((requests.ROOT/relative).read_bytes()))
            relative='docs/features/flow-counted-bytes/baseline-verification.json'
            self.assertEqual((root/relative).read_bytes(),(requests.ROOT/relative).read_bytes())
