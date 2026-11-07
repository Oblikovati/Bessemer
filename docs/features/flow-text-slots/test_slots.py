"""Independent UTF-16 prefix bytes, exact fifty-slot framing and state retention."""
import gzip
import struct
import tempfile
import unittest
from pathlib import Path
import freeze_slots as freeze
import slot_requests as requests
from swarchive import ArchiveError


def prefix(raw: bytes) -> bytes:
    units=struct.unpack('<'+'H'*(len(raw)//2),raw)
    end=units.index(0) if 0 in units else len(units)
    return raw[:end*2]


class TextSlotTests(unittest.TestCase):
    def test_every_slot_span_units_and_native_prefix_rebuild_complete_wire(self) -> None:
        for name,_,raw,previous in requests.cases():
            value=requests.parse(raw);rebuilt=struct.pack('<i',value['header_i32']);position=4
            self.assertEqual(len(value['slots']),50,name);self.assertFalse(value['native_header_dispatch'])
            for slot in value['slots']:
                self.assertEqual(slot['start'],position);self.assertEqual(slot['text_start'],position+4)
                stored=slot['raw_bytes'];self.assertEqual(len(stored),slot['stored_units']*2)
                self.assertEqual(requests.encoded(slot['text']),prefix(stored))
                if stored:self.assertEqual(stored[-2:],b'\0\0')
                rebuilt+=struct.pack('<i',slot['stored_units'])+stored;position+=4+len(stored)
                self.assertEqual(slot['end'],position)
            self.assertEqual(rebuilt,raw);self.assertEqual(value['end'],len(raw));self.assertEqual(position,len(raw))

    def test_existing_state_is_retained_only_for_empty_native_prefixes(self) -> None:
        for _,_,raw,previous in requests.cases():
            value=requests.parse(raw);snapshot=previous.copy();applied=requests.flowauxrecords.apply_311(value,existing=previous)
            self.assertEqual(len(applied),50)
            for slot,old,result in zip(value['slots'],previous,applied):
                native=prefix(slot['raw_bytes'])
                self.assertEqual(requests.encoded(result),native if native else requests.encoded(old))
            self.assertEqual(previous,snapshot)

    def test_all_code_units_header_bits_positions_and_boundary_lengths_are_owned(self) -> None:
        rows=[requests.parse(raw) for _,_,raw,_ in requests.cases()]
        headers={value['header_i32']&0xffffffff for value in rows}
        for bit in range(32):self.assertTrue({1<<bit,0xffffffff^(1<<bit)}<=headers)
        units={unit for value in rows for slot in value['slots'] for unit in struct.unpack('<'+'H'*slot['stored_units'],slot['raw_bytes'])}
        self.assertEqual(units,set(range(65536)))
        lengths={slot['stored_units'] for value in rows for slot in value['slots']}
        self.assertTrue({0,1,2,255,256,257,4096,65536,65537}<=lengths)
        for ordinal in range(50):
            variants={value['slots'][ordinal]['raw_bytes'] for value in rows}
            self.assertTrue({b'',b'\0\0',b'\0\xd8\0\0',b'\0\xdc\0\0'}<=variants)

    def test_every_small_prefix_extension_negative_count_and_missing_terminal_nul_fails(self) -> None:
        for units in ((),(0,),(65,0),(0,65,0),(0xd800,0)):
            raw=requests.single_slot(-7,0,units)
            for end in range(len(raw)):self.assertRaises(ArchiveError,requests.parse,raw[:end])
            for suffix in (b'\0',b'\xff',bytes(4)):self.assertRaises(ArchiveError,requests.parse,raw+suffix)
        for count in (-2147483648,-1,2147483647):
            raw=struct.pack('<ii',0,count)+bytes(49*4)
            self.assertRaises(ArchiveError,requests.parse,raw)
        for units in ((65,),(0,65),(0xd800,)):
            raw=struct.pack('<ii',0,len(units))+struct.pack('<'+'H'*len(units),*units)+bytes(49*4)
            self.assertRaises(ArchiveError,requests.parse,raw)

    def test_exact_integer_mode_zero_and_record_wrapper_are_required(self) -> None:
        raw=requests.single_slot(0,0,())
        for mode in (None,False,True,0.0,'0',-1,1,2147483647):
            self.assertRaises(ArchiveError,requests.flowauxrecords.read_311,raw,string_mode=mode)
        for identifier,size in ((310,len(raw)),(311,len(raw)-1),(311,len(raw)+1)):
            self.assertRaises(ArchiveError,requests.flowauxrecords.read_record,dict(identifier=identifier,size=size,payload=raw),string_mode=0)

    def test_apply_requires_fifty_actual_text_slots_and_complete_parsed_slot_count(self) -> None:
        value=requests.parse(requests.single_slot(0,0,(65,0)))
        for previous in ([],['old']*49,['old']*51,[0]*50,['old']*49+[None]):
            self.assertRaises(ArchiveError,requests.flowauxrecords.apply_311,value,existing=previous)
        for count in (0,49,51):
            invalid=dict(value);invalid['slots']=value['slots'][:count] if count<50 else value['slots']+[value['slots'][0]]
            self.assertRaises(ArchiveError,requests.flowauxrecords.apply_311,invalid,existing=['old']*50)

    def test_native_manifest_and_all_fifty_existing_slot_projections_are_complete(self) -> None:
        rows,proof=requests.native_cases();self.assertEqual(len(rows),1)
        value=requests.parse(rows[0][2]);self.assertEqual((value['header_i32'],len(value['slots']),value['end']),(0,50,644))
        self.assertEqual(sum(slot['stored_units'] for slot in value['slots']),220)
        self.assertEqual(proof['payload_bytes'],644);self.assertFalse(proof['native_owner_mode_verified'])
        applied=requests.flowauxrecords.apply_311(value,existing=rows[0][3]);self.assertEqual(len(applied),50)

    def test_packet_and_proof_regenerate_exactly_under_a_temporary_root(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root=Path(directory);freeze.freeze(root)
            relative='tests/fixtures/flow-text-slots/record311.bin.gz'
            self.assertEqual(gzip.decompress((root/relative).read_bytes()),gzip.decompress((requests.ROOT/relative).read_bytes()))
            relative='docs/features/flow-text-slots/baseline-verification.json'
            self.assertEqual((root/relative).read_bytes(),(requests.ROOT/relative).read_bytes())
