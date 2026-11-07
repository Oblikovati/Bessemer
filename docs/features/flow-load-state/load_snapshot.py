"""Independently serialize reference fields and UTF16 prefix projections through EOF."""
from __future__ import annotations
from dataclasses import dataclass
import struct
import load_requests
from snapshot import Snapshot

MEMBER_ORDER=(8,24,12,16,20,28,40,36,44)
Path=tuple[str | int,...]


@dataclass(frozen=True)
class TextSpan:
    path: Path
    start: int
    end: int
    units: int
    prefix_units: int
    prefix_raw: bytes


def text(out: Snapshot, texts: list[TextSpan], path: Path, value: dict) -> None:
    raw=bytes.fromhex(value['raw_hex']);units=struct.unpack('<'+'H'*value['code_units'],raw)
    start=len(out.raw)
    with out.scope(path):
        out.count(path+('code_units',),value['code_units']);out.put(path+('raw_units',),'H',units)
    prefix=next((index for index,unit in enumerate(units) if unit==0),len(units))
    expected=value['text'].encode('utf-16-le',errors='surrogatepass')
    assert expected==raw[:prefix*2]
    texts.append(TextSpan(path,start,len(out.raw),len(units),prefix,expected))


def item_prefix(out: Snapshot, texts: list[TextSpan], path: Path, value: dict) -> None:
    assert tuple(value['member_offsets'])==MEMBER_ORDER
    for index,name in enumerate(value['names']):
        text(out,texts,path+('names',index),name)
    out.put(path+('member_i32',),'i',value['member_i32'])
    out.put(path+('state_f0_f4',),'i',value['state_f0_f4'])
    out.scalar(path+('pattern_f8',),value['pattern_f8'],'Q')
    out.scalar(path+('member_1f8',),value['member_1f8'])
    out.put(path+('patterns_a0_b0',),'Q',value['patterns_a0_b0'])


def item(out: Snapshot, texts: list[TextSpan], path: Path, value: dict, width: int) -> None:
    with out.scope(path,value):
        item_prefix(out,texts,path,value)
        for index,array in enumerate(value['wide_arrays']):
            assert len(array)==width;out.put(path+('wide_arrays',index),'Q',array)
        out.count(path+('extra_count',),len(value['extra_patterns']))
        out.put(path+('extra_patterns',),'I',value['extra_patterns'])
        text(out,texts,path+('description',),value['description'])


def tail(out: Snapshot, value: dict) -> None:
    assert len(value['shared_arrays'])==(4 if value['count'] else 0)
    for index,array in enumerate(value['shared_arrays']):
        assert len(array)==value['array_width'];out.put(('shared_arrays',index),'I' if index<2 else 'Q',array)
    if value['final_members_128_12c'] is not None:
        out.put(('final_members_128_12c',),'i',value['final_members_128_12c'])
    if value['mode_tail'] is not None:
        assert not value['load_mode'] and value['count']
        with out.scope(('mode_tail',)):
            out.put(('mode_tail','four_patterns'),'I',value['mode_tail']['four_patterns'])
            out.put(('mode_tail','wide_patterns'),'Q',value['mode_tail']['wide_patterns'])


def record(value: dict) -> tuple[Snapshot,list[TextSpan]]:
    out=Snapshot();texts=[]
    with out.scope((),dict(start=0,end=value['end'])):
        out.scalar(('version',),value['version']);out.count(('count',),value['count'])
        out.scalar(('member_130',),value['member_130']);out.count(('array_width',),value['array_width'])
        for index,entry in enumerate(value['items']):
            assert entry['ordinal']==index;item(out,texts,('items',index),entry,value['array_width'])
        tail(out,value)
    assert value['complete_payload_framing'] and not value['opaque_tail']
    assert not value['engineering_fields_assigned'] and not value['runtime_transformations_reproduced']
    return out,texts
