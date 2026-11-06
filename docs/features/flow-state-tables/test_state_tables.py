"""Independent complete framing and contextual boundary checks for owned state tables."""
import gzip
import struct
import tempfile
import unittest
from itertools import product
from pathlib import Path
import freeze_state_tables as frozen
import state_requests as requests
import state_snapshot as snapshot
from swarchive import ArchiveError


def compact() -> list[tuple[int,int | None,bytes]]:
    return [(129,None,requests.record129((0,1,255),(0,0,2))),
            (138,2,requests.record138(3,1,2)),
            (139,None,requests.record139(3,1)),
            (417,None,requests.record417((2,3),(1,0))),
            (432,None,requests.record432(1))]


class StateTableTests(unittest.TestCase):
    def test_regeneration_matches_owned_packets_and_source_hashes(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root=Path(directory);frozen.freeze(root)
            for identifier in requests.SCHEMAS:
                relative=f'tests/fixtures/flow-state-tables/record{identifier}.bin.gz'
                self.assertEqual(gzip.decompress((root/relative).read_bytes()),gzip.decompress((requests.ROOT/relative).read_bytes()))
            relative='docs/features/flow-state-tables/baseline-verification.json'
            self.assertEqual((root/relative).read_bytes(),(requests.ROOT/relative).read_bytes())

    def test_every_payload_reconstructs_all_fields_scopes_and_counts(self) -> None:
        for name,identifier,context,raw in requests.cases():
            value=requests.parse(identifier,raw,context);expected,_=snapshot.record(identifier,value)
            self.assertEqual(bytes(expected.raw),raw,name);offset=0
            for field in expected.fields:
                self.assertEqual(field.offset,offset,name)
                offset+=struct.calcsize('<'+field.encoding)*len(field.values)
            self.assertEqual(offset,len(raw));self.assertEqual(expected.scopes[0],((),0,len(raw)))
            for _,start,end in expected.scopes:
                self.assertTrue(0<=start<=end<=len(raw))
            for start in expected.counts:
                self.assertGreaterEqual(struct.unpack_from('<i',raw,start)[0],0)

    def test_all_tag_bytes_preserve_zero_absence_and_nonzero_empty_width(self) -> None:
        seen=set()
        for _,identifier,context,raw in requests.tag_cases():
            value=requests.parse(identifier,raw,context);seen.update(value['tags_raw'])
            for row in value['rows']:
                self.assertEqual(row['width'] is None,row['tag_byte']==0)
                self.assertEqual(row['end']-row['start'],0 if row['width'] is None else 4+8*row['width'])
        self.assertEqual(seen,set(range(256)))

    def test_context_is_required_only_for_active_old_versions_and_is_not_object_count(self) -> None:
        for version in range(2,6):
            for count in (0,1,2147483647):
                raw=requests.record138(version,count,0)
                for invalid in (None,True,-1,2147483648,'0'):
                    if count and version<5:
                        with self.assertRaises(ArchiveError):requests.parse(138,raw,invalid)
                    else:self.assertEqual(requests.parse(138,raw,invalid)['index_patterns'],())
                value=requests.parse(138,raw,0)
                self.assertEqual(value['object_count'],count);self.assertFalse(value['native_context_provenance_verified'])

    def test_fixed_settings_versions_have_independent_exact_row_widths(self) -> None:
        for version,width in ((0,0),(1,16),(2,2976),(3,2980),(4,2980)):
            count=0 if version==0 else 2;raw=requests.record139(version,count)
            self.assertEqual(len(raw),4 if version==0 else 8+count*width)
            value=requests.parse(139,raw)
            self.assertEqual(value['count'],None if version==0 else count)
            for row in value['items']:
                self.assertEqual(row['end']-row['start'],width)
                if version>=2:
                    self.assertEqual(len(row['settings_wide_patterns']),320)
                    self.assertEqual(len(row['settings_i32_patterns']),100)

    def test_paired_groups_keep_absent_empty_and_nondivisible_quotients_distinct(self) -> None:
        for first,second in product(((0,0),(1,0),(2,3),(3,2)),repeat=2):
            value=requests.parse(417,requests.record417(first,second))
            for group,(count,width) in zip(value['groups'],(first,second)):
                self.assertEqual(group['wide_count'],width if count else None)
                self.assertEqual(group['native_quotient'],width//count if count else None)
                self.assertEqual(group['wide_patterns'] is None,count==0)

    def test_scalar_versions_are_open_nonnegative_with_exact_discard_and_recompute_rules(self) -> None:
        for _,identifier,context,raw in requests.scalar_cases():
            value=requests.parse(identifier,raw,context);version=value['version']
            self.assertEqual(len(raw),20 if version in (1,2) else 16)
            self.assertEqual(value['discarded_i32'] is not None,version in (1,2))
            self.assertEqual(value['native_recomputes_members'],version<2)

    def test_all_compact_prefixes_extensions_and_reached_counts_reject(self) -> None:
        for identifier,context,raw in compact():
            for end in range(len(raw)):
                with self.assertRaises(ArchiveError,msg=f'{identifier}:{end}'):requests.parse(identifier,raw[:end],context)
            with self.assertRaises(ArchiveError):requests.parse(identifier,raw+b'\0',context)
            expected,_=snapshot.record(identifier,requests.parse(identifier,raw,context))
            for offset,invalid in product(expected.counts,(-1,2147483647)):
                if identifier==138 and invalid>=0:continue
                changed=raw[:offset]+struct.pack('<i',invalid)+raw[offset+4:]
                with self.assertRaises(ArchiveError):requests.parse(identifier,changed,context)

    def test_versions_and_record_wrappers_reject_invalid_profiles(self) -> None:
        for identifier,context,raw in compact():
            for version in (-1,):
                with self.assertRaises(ArchiveError):requests.parse(identifier,struct.pack('<i',version)+raw[4:],context)
            for record in (dict(identifier=999,size=len(raw),payload=raw),dict(identifier=identifier,size=len(raw)+1,payload=raw)):
                kwargs=dict(context_count=context) if identifier==138 else {}
                with self.assertRaises(ArchiveError):requests.SCHEMAS[identifier].read_record(record,**kwargs)


if __name__=='__main__':
    unittest.main()
