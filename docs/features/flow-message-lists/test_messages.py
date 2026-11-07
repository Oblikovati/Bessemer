"""Complete message framing, strict modern transfer and distinct legacy raw-byte oracles."""
import gzip
import struct
import tempfile
import unittest
from pathlib import Path
import freeze_messages as frozen
import message_requests as requests
import message_snapshot as snapshot
from swarchive import ArchiveError


def compact(version: int) -> bytes:
    primary=b'A\0B' if version==1 else (65,0,66,0)
    first=(b'',b'\xff',b'\x80\0\1') if version==1 else ((),(0xd800,0),(65,0,66,0))
    second=((0xdc00,0),()) if version==3 else None
    return requests.record(version,(requests.MessageSpec(0x80000000,primary,first,second),))


def reject_counts(test: unittest.TestCase, raw: bytes, mode: int | None, offsets: list[int]) -> None:
    for offset in offsets:
        for count in (-1,2147483647):
            changed=raw[:offset]+struct.pack('<i',count)+raw[offset+4:]
            with test.assertRaises(ArchiveError):requests.parse(changed,mode)


class MessageListTests(unittest.TestCase):
    def test_packet_and_source_hash_report_regenerate_exactly_in_a_temporary_root(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root=Path(directory);frozen.freeze(root)
            relative='tests/fixtures/flow-message-lists/messages.bin.gz'
            self.assertEqual(gzip.decompress((root/relative).read_bytes()),gzip.decompress((requests.ROOT/relative).read_bytes()))
            relative='docs/features/flow-message-lists/baseline-verification.json'
            self.assertEqual((root/relative).read_bytes(),(requests.ROOT/relative).read_bytes())

    def test_every_source_payload_reconstructs_every_field_scope_and_count(self) -> None:
        for name,mode,_,raw in requests.cases():
            value=requests.parse(raw,mode);expected,texts=snapshot.record(value)
            self.assertEqual(bytes(expected.raw),raw,name);offset=0
            for field in expected.fields:
                self.assertEqual(field.offset,offset);offset+=struct.calcsize('<'+field.encoding)*len(field.values)
            self.assertEqual(offset,len(raw));self.assertEqual(expected.scopes[0],((),0,len(raw)))
            for _,start,end in expected.scopes:self.assertTrue(0<=start<=end<=len(raw))
            for text in texts:self.assertEqual(text.end-text.start,4+text.count*(1 if text.legacy else 2))
            for offset in expected.counts:self.assertGreaterEqual(struct.unpack_from('<i',raw,offset)[0],0)

    def test_legacy_count_is_exact_bytes_not_doubled_utf16_and_text_stays_unset(self) -> None:
        raw=requests.record(1,(requests.MessageSpec(7,b'\0\xd8\xff'),))
        self.assertEqual(len(raw),23)
        value=requests.parse(raw,None);text=value['items'][0]['text']
        self.assertEqual((text['start'],text['end'],text['stored_count']),(12,19,3))
        self.assertEqual(text['raw_bytes'],b'\0\xd8\xff');self.assertIsNone(text['text'])
        for mode in (None,0,1,-1,True,'unused',object()):
            self.assertEqual(requests.schema.read(raw,string_mode=mode),value)

    def test_modern_context_requires_exact_integer_zero_even_for_empty_records(self) -> None:
        for version in (0,2,3):
            raw=requests.record(version,())
            self.assertEqual(requests.parse(raw,0)['count'],0)
            for mode in (None,True,False,1,-1,'0',0.0):
                with self.assertRaises(ArchiveError):requests.schema.read(raw,string_mode=mode)

    def test_modern_empty_surrogate_and_hidden_suffix_prefixes_keep_terminal_nul_rules(self) -> None:
        for units in ((),(0,),(0xd800,0),(0xdc00,0),(65,0,66,0),(0,0xd800,0),(0xfffe,0xfeff,0)):
            raw=requests.record(3,(requests.MessageSpec(0,units,(),()),))
            value=requests.parse(raw,0);expected,texts=snapshot.record(value)
            self.assertEqual(bytes(expected.raw),raw)
            prefix=next((index for index,unit in enumerate(units) if unit==0),len(units))
            self.assertEqual(texts[0].prefix,struct.pack('<'+'H'*prefix,*units[:prefix]))
            if units:
                field=next(field for field in expected.fields if field.path==('items',0,'text','raw_units'))
                changed=raw[:field.offset+len(units)*2-2]+b'\1\0'+raw[field.offset+len(units)*2:]
                with self.assertRaises(ArchiveError):requests.parse(changed,0)

    def test_unicode_corpus_contains_every_unit_legacy_byte_and_separate_nfc_nfd(self) -> None:
        units=set();legacy=set();encodings={}
        for name,mode,_,raw in requests.cases():
            expected,_=snapshot.record(requests.parse(raw,mode))
            for field in expected.fields:
                if field.encoding=='H':units.update(field.values)
                if field.encoding=='B':legacy.update(field.values)
            if name in ('nfc-v0','nfd-v0'):encodings[name]=raw
        self.assertEqual(units,set(range(65536)));self.assertEqual(legacy,set(range(256)))
        self.assertNotEqual(encodings['nfc-v0'],encodings['nfd-v0'])

    def test_every_compact_prefix_and_all_payload_extensions_are_rejected(self) -> None:
        for version in range(4):
            raw=compact(version);mode=None if version==1 else 0
            for end in range(len(raw)):
                with self.assertRaises(ArchiveError):requests.parse(raw[:end],mode)
        for _,mode,_,raw in requests.cases():
            with self.assertRaises(ArchiveError):requests.parse(raw+b'\0',mode)

    def test_every_reached_compact_count_invalid_version_and_wrapper_is_rejected(self) -> None:
        for version in range(4):
            raw=compact(version);mode=None if version==1 else 0;expected,_=snapshot.record(requests.parse(raw,mode))
            reject_counts(self,raw,mode,expected.counts)
            for invalid in (-2147483648,-1,4,2147483647):
                with self.assertRaises(ArchiveError):requests.parse(struct.pack('<i',invalid)+raw[4:],mode)
            for record in (dict(identifier=999,size=len(raw),payload=raw),dict(identifier=301,size=len(raw)+1,payload=raw)):
                with self.assertRaises(ArchiveError):requests.schema.read_record(record,string_mode=mode)

    def test_native_message_and_independent_lists_reach_complete_eof_without_runtime_credit(self) -> None:
        rows,proof=requests.native_cases();self.assertEqual(len(rows),1)
        value=requests.parse(rows[0][3],0);item=value['items'][0]
        self.assertEqual((value['version'],value['count'],value['end']),(3,1,328))
        self.assertEqual(item['member_8_pattern'],4)
        self.assertEqual(item['text']['text'],'A vortex crosses the pressure opening')
        self.assertEqual((len(item['first_strings']),len(item['second_strings'])),(1,1))
        self.assertFalse(proof['native_mode_provenance_verified']);self.assertFalse(value['legacy_string_interpretation_reproduced'])


if __name__=='__main__':
    unittest.main()
