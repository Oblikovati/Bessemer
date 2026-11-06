"""Independent truth-set, high-bit, dense-map, wide-pattern and native profile checks."""
from dataclasses import replace
import gzip
import json
import struct
import tempfile
import unittest
from pathlib import Path
import selected_requests as requests
import freeze_selected as frozen
from swarchive import ArchiveError


def converted(code: int, version: int) -> int:
    for first in (30,40,50,60):
        if version==0 and first<=code<first+6:return first+(code-first)//2
    return code


def check_selection(test: unittest.TestCase, spec: requests.SelectionSpec) -> None:
    links,states,context,value=requests.parse(spec)
    mapping=[];selected=[]
    for index,entry in enumerate(links['entries']):
        code=converted(entry['code_byte'],links['version'])
        test.assertEqual(entry['native_code_byte'],code)
        accepts=bool(states['raw_bytes']) and code in requests.ACCEPTED and states['raw_bytes'][index]>=128
        mapping.append(len(selected) if accepts else -1)
        if accepts:selected.append(index)
    test.assertEqual(context['dense_indices'],tuple(mapping));test.assertEqual(context['selected_ordinals'],tuple(selected))
    test.assertEqual(value['count'],len(selected));test.assertEqual(value['end'],8*len(selected))
    test.assertEqual(spec.values,requests.links_fixture.numbers(value['patterns'],'Q'))
    expected=tuple(None if index==-1 else value['patterns'][index] for index in mapping)
    test.assertEqual(value['entry_patterns'],expected)
    test.assertTrue(context['fresh_map_assumed'] and value['complete_payload_framing'])
    test.assertFalse(context['native_owner_binding_verified'] or value['native_owner_binding_verified'])
    test.assertFalse(value['native_runtime_scatter_reproduced'] or value['engineering_fields_assigned'])


class SelectedValueTests(unittest.TestCase):
    def test_all_codes_match_the_independent_reviewed_truth_set(self) -> None:
        self.assertEqual(tuple(code for code in range(256) if requests.schema.accepted_code(code)),requests.ACCEPTED)
        for code in (-1,256,False,None,1.0,'4'):
            with self.assertRaises(ArchiveError):requests.schema.accepted_code(code)

    def test_all_generated_and_native_contexts_match_independent_selection_and_raw_patterns(self) -> None:
        rows=requests.cases();self.assertEqual(len(rows),2729)
        for spec in rows:check_selection(self,spec)

    def test_version_zero_and_nine_cover_every_code_and_state_pair(self) -> None:
        seen={version:set() for version in range(10)}
        for spec in requests.grid_cases():
            links,states,_,_=requests.parse(spec)
            seen[links['version']].update((entry['code_byte'],flag) for entry,flag in zip(links['entries'],states['raw_bytes']))
        expected={(code,flag) for code in range(256) for flag in range(256)}
        self.assertEqual(seen[0],expected);self.assertEqual(seen[9],expected)
        boundary={(code,flag) for code in range(256) for flag in (0,1,127,128,129,255)}
        for version in range(1,9):self.assertEqual(seen[version],boundary)

    def test_missing_selection_is_distinct_from_selected_zero_and_empty_state_arrays(self) -> None:
        rows=requests.shape_cases()
        zero=next(row for row in rows if row.name=='selected-zero-distinct-from-absent')
        self.assertEqual(requests.parse(zero)[3]['entry_patterns'],(0,None,0,None))
        for spec in rows:
            if spec.name.startswith('absent-states-'):
                self.assertEqual(requests.parse(spec)[3]['entry_patterns'],(None,)*4)
                self.assertEqual(spec.values,b'')

    def test_every_wide_bit_complement_and_special_raw_pattern_is_owned(self) -> None:
        patterns=set()
        for spec in requests.shape_cases():patterns.update(requests.parse(spec)[3]['patterns'])
        for bit in range(64):self.assertIn(1<<bit,patterns);self.assertIn(0xffffffffffffffff^(1<<bit),patterns)
        for pattern in (0,0xffffffffffffffff,0x8000000000000000,0x7ff0000000000001,0x7ff8000000000042):self.assertIn(pattern,patterns)

    def test_fresh_map_state_lengths_link_ordinals_and_dense_map_guards_are_enforced(self) -> None:
        spec=requests.from_inputs('guards',9,b'\0',b'\x80',0)
        links,states,context,_=requests.parse(spec)
        for fresh in (False,None,1):
            with self.assertRaises(ArchiveError):requests.schema.derive_context(links,states,fresh_map=fresh)
        with self.assertRaises(ArchiveError):requests.schema.derive_context(links,dict(states,count=2),fresh_map=True)
        with self.assertRaises(ArchiveError):requests.schema.derive_context(links,dict(states,count=2,raw_bytes=b'\x80\xff'),fresh_map=True)
        with self.assertRaises(ArchiveError):requests.schema.derive_context(dict(links,entries=[dict(links['entries'][0],ordinal=1)]),states,fresh_map=True)
        for mapping,count in (((1,),1),((-2,),0),((0,0),2),((-1,),2)):
            with self.assertRaises(ArchiveError):requests.schema.read(b'',context=dict(context,dense_indices=mapping,selected_count=count))
        with self.assertRaises(ArchiveError):requests.schema.read(spec.values,context=dict(context,fresh_map_assumed=False))

    def test_compact_raw_value_prefixes_extensions_and_carrier_guards_fail(self) -> None:
        spec=requests.from_inputs('two-values',9,b'\0\4',b'\x80\xff',0)
        for end in range(len(spec.values)):
            with self.assertRaises(ArchiveError):requests.parse(replace(spec,values=spec.values[:end]))
        for suffix in (b'\0',b'\xff'*8):
            with self.assertRaises(ArchiveError):requests.parse(replace(spec,values=spec.values+suffix))
        context=requests.parse(spec)[2]
        for identifier,size in ((113,len(spec.values)),(112,len(spec.values)-1)):
            with self.assertRaises(ArchiveError):requests.schema.read_record(dict(identifier=identifier,size=size,payload=spec.values),context=context)

    def test_all_three_native_inputs_and_complete_original_context_and_projection_match(self) -> None:
        rows,proof=requests.native_cases();self.assertEqual(len(rows),1)
        _,_,context,value=requests.parse(rows[0])
        self.assertEqual((context['link_count'],context['selected_count'],value['end']),(5532,3876,31008))
        self.assertFalse(proof['automatic_pairing_proven']);self.assertTrue(proof['tag_equality_is_observation_only'])
        self.assertEqual([row['identifier'] for row in proof['inputs']],[402,316,112])

    def test_all_packet_parts_and_hash_report_regenerate_in_a_temporary_root(self) -> None:
        original=json.loads((requests.ROOT/'docs/features/flow-selected-values/baseline-verification.json').read_text())
        with tempfile.TemporaryDirectory() as directory:
            root=Path(directory);frozen.freeze(root)
            for packet in original['packets']:
                relative=packet['path']
                self.assertEqual(gzip.decompress((root/relative).read_bytes()),gzip.decompress((requests.ROOT/relative).read_bytes()))
            relative='docs/features/flow-selected-values/baseline-verification.json'
            self.assertEqual((root/relative).read_bytes(),(requests.ROOT/relative).read_bytes())
