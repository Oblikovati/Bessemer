"""Independent complete Source-field serialization and nested physical accounting."""
from __future__ import annotations
from dataclasses import dataclass,field
from contextlib import contextmanager
from typing import Iterator
import struct

Path = tuple[str | int,...]


@dataclass(frozen=True)
class StoredField:
    path: Path
    offset: int
    encoding: str
    values: tuple[int,...]


@dataclass
class Snapshot:
    wide_scale: int=1
    raw: bytearray=field(default_factory=bytearray)
    fields: list[StoredField]=field(default_factory=list)
    scopes: list[tuple[Path,int,int]]=field(default_factory=list)
    counts: list[int]=field(default_factory=list)
    projections: list[tuple[Path,bool]]=field(default_factory=list)

    def put(self, path: Path, encoding: str, values: tuple | list | bytes) -> None:
        self.fields.append(StoredField(path,len(self.raw),encoding,tuple(values)))
        self.raw.extend(struct.pack('<'+encoding*len(values),*values))

    def scalar(self, path: Path, value: int, encoding: str = 'i') -> None:
        self.put(path,encoding,(value,))

    def count(self, path: Path, value: int) -> None:
        self.counts.append(len(self.raw));self.scalar(path,value)

    def vector(self, path: Path, value: dict, encoding: str) -> None:
        self.count(path+('count',),value['count'])
        values=value['raw_bytes'] if encoding=='B' else value['patterns']
        scale=self.wide_scale if encoding=='Q' else 1
        assert len(values)==value['count']*scale
        self.put(path+('values',),encoding,values)

    @contextmanager
    def scope(self, path: Path, value: dict | None = None) -> Iterator[None]:
        start=len(self.raw);index=len(self.scopes);self.scopes.append((path,start,start))
        if value is not None:
            assert start==value['start']
        yield
        end=len(self.raw);self.scopes[index]=(path,start,end)
        if value is not None:
            assert end==value['end']


def subrecord(out: Snapshot, path: Path, value: dict) -> None:
    with out.scope(path,value):
        for key in ('kind_i32','member_c_i32','member_10_i32'):
            out.scalar(path+(key,),value[key])
        out.vector(path+('first',),value['first'],'I')
        if value['extra'] is not None:
            out.vector(path+('extra',),value['extra'],'I')
        out.projections.append((path+('native_inverse_map_reproduced',),value['native_inverse_map_reproduced']))


def item_group(out: Snapshot, path: Path, value: dict) -> None:
    with out.scope(path):
        out.vector(path+('wide',),value['wide'],'Q')
        out.vector(path+('bytes',),value['bytes'],'B')
        if 'subrecord' in value:
            subrecord(out,path+('subrecord',),value['subrecord'])


def legacy_item(out: Snapshot, path: Path, value: dict) -> None:
    if value['version']<4:
        for index,group in enumerate(value['groups']):
            item_group(out,path+('groups',index),group)
    out.scalar(path+('dimension',),value['dimension'])
    if value['legacy_word_i32'] is not None:
        out.scalar(path+('legacy_word_i32',),value['legacy_word_i32'])
    out.scalar(path+('member_84_i32',),value['member_84_i32'])


def mode_item(out: Snapshot, path: Path, value: dict) -> None:
    if value['mode_byte'] is None:
        return
    out.scalar(path+('mode_byte',),value['mode_byte'],'B')
    out.projections.append((path+('native_mode_boolean',),value['native_mode_boolean']))
    for index,vector in enumerate(value.get('legacy_vectors',())):
        out.vector(path+('legacy_vectors',index),vector,'I')
    for index,sub in enumerate(value.get('subrecords',())):
        subrecord(out,path+('subrecords',index),sub)


def item(out: Snapshot, path: Path, value: dict) -> None:
    with out.scope(path,value):
        out.scalar(path+('version',),value['version'])
        out.scalar(path+('active_byte',),value['active_byte'],'B')
        if not value['active_byte']:
            return
        legacy_item(out,path,value);mode_item(out,path,value)
        if value['version']>=4:
            out.count(path+('group_count',),len(value['groups']))
            for index,group in enumerate(value['groups']):
                item_group(out,path+('groups',index),group)


def items(out: Snapshot, path: Path, value: dict) -> None:
    if value['count'] is None:
        return
    out.count(path+('count',),value['count'])
    assert len(value['items'])==value['count']
    for index,entry in enumerate(value['items']):
        assert entry['ordinal']==index
        entry_path=path+('items',index)
        with out.scope(entry_path):
            out.scalar(entry_path+('present_byte',),entry['present_byte'],'B')
            assert (entry['item'] is None)==(entry['present_byte']==0)
            if entry['item'] is not None:
                item(out,entry_path+('item',),entry['item'])


def group(out: Snapshot, path: Path, value: dict) -> None:
    with out.scope(path,value):
        out.scalar(path+('version',),value['version'])
        out.scalar(path+('member_a0_pattern',),value['member_a0_pattern'],'I')
        out.scalar(path+('mode_i32',),value['mode_i32'])
        out.scalar(path+('protocol_i32',),value['protocol_i32'])
        if value['gate_pattern'] is not None:
            out.scalar(path+('gate_pattern',),value['gate_pattern'],'I')
        items(out,path,value)


def record126(out: Snapshot, value: dict) -> None:
    out.scalar(('version',),value['version'])
    if value['version']==1:
        out.count(('count',),value['count'])
        for index,array in enumerate(value['arrays']):
            assert len(array)==value['count'];out.put(('arrays',index),'Q',array)
        return
    for key in ('inner_word_i32','member_a0_i32','mode_i32','dimension'):
        out.scalar((key,),value[key])
    if value['gate_i32'] is not None:
        out.scalar(('gate_i32',),value['gate_i32'])
    items(out,(),value)


def record315(out: Snapshot, value: dict) -> None:
    out.scalar(('version',),value['version'])
    for index,vector in enumerate(value['legacy_arrays']):
        out.vector(('legacy_arrays',index),vector,'Q')
    out.vector(('common_vector',),value['common_vector'],'Q')
    out.count(('extra_count',),value['extra_count'])
    if value['extra_mode_i32'] is not None:
        out.scalar(('extra_mode_i32',),value['extra_mode_i32'])
    out.put(('extra_patterns',),'Q',value['extra_patterns'])
    for index,entry in enumerate(value['groups']):
        group(out,('groups',index),entry)


def record(identifier: int, value: dict) -> Snapshot:
    out=Snapshot(wide_scale=value.get('element_bytes',8)//8)
    with out.scope((),dict(start=0,end=value['end'])):
        if identifier==126:
            record126(out,value)
        elif identifier==315:
            record315(out,value)
        else:
            assert identifier==128;group(out,('group',),value['group'])
    assert value['complete_payload_framing']
    return out
