"""Independent complete load-state reconstruction, UTF16 and explicit-mode boundaries."""
import gzip
import struct
import tempfile
import unittest
from pathlib import Path
import freeze_load_state as frozen
import load_requests as requests
import load_snapshot as snapshot
from wire_packet import ROOT
from swarchive import ArchiveError


def read(raw: bytes, mode: bool) -> dict:
    return requests.schema.read(raw,load_mode=mode)


def compact(mode: bool) -> bytes:
    spec=requests.ItemSpec(first=(65,0),second=(0xd800,),description=(0,),extra_count=1)
    return requests.record(mode,1,(spec,))


class LoadStateTests(unittest.TestCase):
    def test_packet_regenerates_exactly_in_a_temporary_root(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root=Path(directory);frozen.freeze(root)
            relative='tests/fixtures/flow-load-state/state.bin.gz'
            self.assertEqual(gzip.decompress((root/relative).read_bytes()),gzip.decompress((ROOT/relative).read_bytes()))
            relative='docs/features/flow-load-state/baseline-verification.json'
            self.assertEqual((root/relative).read_bytes(),(ROOT/relative).read_bytes())

    def test_every_complete_payload_rebuilds_all_values_string_prefixes_and_nested_intervals(self) -> None:
        for name,mode,_,raw in requests.cases():
            value=read(raw,mode);expected,texts=snapshot.record(value)
            self.assertEqual(bytes(expected.raw),raw,name)
            offset=0
            for field in expected.fields:
                self.assertEqual(field.offset,offset);offset+=struct.calcsize('<'+field.encoding)*len(field.values)
            self.assertEqual(offset,len(raw));self.assertEqual(value['end'],len(raw))
            for text in texts:
                self.assertEqual(text.end-text.start,4+text.units*2)
                self.assertEqual(raw[text.start+4:text.start+4+text.prefix_units*2],text.prefix_raw)

    def test_utf16_units_keep_all65536_patterns_unpaired_surrogates_and_first_nul_semantics(self) -> None:
        units=set();prefix_lengths=set();stored_lengths=set()
        for _,mode,_,raw in requests.text_cases():
            expected,texts=snapshot.record(read(raw,mode))
            for field in expected.fields:
                if field.encoding=='H':units.update(field.values)
            for text in texts:
                prefix_lengths.add(text.prefix_units);stored_lengths.add(text.units)
                self.assertLessEqual(text.prefix_units,text.units)
        self.assertEqual(units,set(range(65536)))
        self.assertIn(0,prefix_lengths);self.assertIn(65535,prefix_lengths);self.assertIn(65536,stored_lengths)
        self.assertEqual(read(compact(True),True)['items'][0]['names'][1]['text'],'\ud800')

    def test_complete_compact_profiles_reject_every_proper_prefix_and_trailing_byte(self) -> None:
        for mode in (False,True):
            raw=compact(mode)
            for end in range(len(raw)):
                with self.assertRaises(ArchiveError,msg=f'{mode}:{end}'):
                    read(raw[:end],mode)
            with self.assertRaises(ArchiveError):
                read(raw+b'\0',mode)

    def test_every_reached_count_rejects_negative_or_physically_impossible_values(self) -> None:
        for mode in (False,True):
            raw=compact(mode);expected,_=snapshot.record(read(raw,mode))
            for offset in expected.counts:
                for count in (-1,2147483647):
                    changed=raw[:offset]+struct.pack('<i',count)+raw[offset+4:]
                    with self.assertRaises(ArchiveError,msg=f'{mode}:{offset}:{count}'):
                        read(changed,mode)

    def test_load_mode_is_explicit_and_native_true_prefix_is_incomplete_for_the_original_payload(self) -> None:
        rows,proof=requests.native_cases();raw=rows[0][3]
        prefix=requests.schema.read_prefix(raw,load_mode=True)
        self.assertEqual(prefix['end'],11282);self.assertEqual(len(prefix['opaque_tail']),192)
        self.assertFalse(prefix['complete_payload_framing']);self.assertFalse(proof['native_owner_mode_verified'])
        with self.assertRaises(ArchiveError):read(raw,True)
        self.assertTrue(read(rows[1][3],True)['complete_payload_framing'])
        for mode in (None,0,1,'False'):
            with self.assertRaises(ArchiveError):read(raw,mode)

    def test_inactive_empty_records_keep_unused_widths_and_suppress_all_shared_and_mode_tail_arrays(self) -> None:
        for mode in (False,True):
            raw=requests.record(mode,2147483647,());value=read(raw,mode)
            self.assertEqual(len(raw),16);self.assertEqual(value['array_width'],2147483647)
            self.assertEqual(value['items'],[]);self.assertEqual(value['shared_arrays'],[])
            self.assertIsNone(value['final_members_128_12c']);self.assertIsNone(value['mode_tail'])
            self.assertEqual(read(raw,not mode)['end'],16)
        self.assertEqual(len(read(requests.record(False,0,(requests.ItemSpec(),)),False)['shared_arrays']),4)

    def test_member_order_version_and_record_wrapper_contracts_remain_exact(self) -> None:
        raw=compact(False);value=read(raw,False)
        self.assertEqual(tuple(value['items'][0]['member_offsets']),snapshot.MEMBER_ORDER)
        for version in (-2147483648,-1,0,17,19,2147483647):
            with self.assertRaises(ArchiveError):read(struct.pack('<i',version)+raw[4:],False)
        for identifier,size in ((12,len(raw)),(11,len(raw)+1)):
            with self.assertRaises(ArchiveError):
                requests.schema.read_record(dict(identifier=identifier,size=size,payload=raw),load_mode=False)

    def test_raw_numeric_vectors_preserve_every_word_and_wide_bit_complement_and_special(self) -> None:
        words=set();wide=set()
        for _,mode,_,raw in requests.pattern_cases():
            expected,_=snapshot.record(read(raw,mode))
            for field in expected.fields:
                if field.encoding=='I':words.update(field.values)
                if field.encoding=='Q':wide.update(field.values)
        for bit in range(32):self.assertTrue({1<<bit,0xffffffff^(1<<bit)}<=words)
        for bit in range(64):self.assertTrue({1<<bit,0xffffffffffffffff^(1<<bit)}<=wide)
        self.assertTrue({0,0x8000000000000000,0x7ff0000000000000,0xfff0000000000000,0x7ff8000000000042,0x7ff0000000000001}<=wide)
