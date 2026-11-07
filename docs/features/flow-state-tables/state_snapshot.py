"""Independent full state-table serialization, scope intervals and derived projections."""
from __future__ import annotations
import state_requests
from snapshot import Snapshot

Path=tuple[str | int,...]


def tagged(out: Snapshot, value: dict) -> None:
    out.count(('count',),value['count']);out.put(('tags_raw',),'B',value['tags_raw'])
    for index,row in enumerate(value['rows']):
        assert row['ordinal']==index and row['tag_byte']==value['tags_raw'][index]
        with out.scope(('rows',index),row):
            if row['width'] is not None:
                out.count(('rows',index,'width'),row['width'])
            out.put(('rows',index,'value_patterns'),'Q',row['value_patterns'])
    if value['tail_i32'] is not None:
        out.scalar(('tail_i32',),value['tail_i32'])


def context(out: Snapshot, value: dict) -> None:
    out.count(('object_count',),value['object_count'])
    out.put(('index_patterns',),'I',value['index_patterns'])
    if value['tail_i32'] is not None:
        out.scalar(('tail_i32',),value['tail_i32'])
    assert not value['opaque_tail'] and not value['native_context_provenance_verified']


def fixed_row(out: Snapshot, path: Path, value: dict) -> None:
    with out.scope(path,value):
        out.scalar(path+('member_600_pattern',),value['member_600_pattern'],'Q')
        out.scalar(path+('discarded_wide_pattern',),value['discarded_wide_pattern'],'Q')
        if value['settings_wide_patterns'] is not None:
            out.put(path+('settings_wide_patterns',),'Q',value['settings_wide_patterns'])
            out.put(path+('settings_i32_patterns',),'I',value['settings_i32_patterns'])
        if value['final_member_e0_i32'] is not None:
            out.scalar(path+('final_member_e0_i32',),value['final_member_e0_i32'])


def fixed(out: Snapshot, value: dict) -> None:
    if value['count'] is not None:
        out.count(('count',),value['count'])
    for index,row in enumerate(value['items']):
        assert row['ordinal']==index;fixed_row(out,('items',index),row)


def paired(out: Snapshot, value: dict, quotients: list[tuple[Path,int | None]]) -> None:
    for index,group in enumerate(value['groups']):
        path=('groups',index);assert group['ordinal']==index
        with out.scope(path,group):
            out.count(path+('count',),group['count']);out.put(path+('index_patterns',),'I',group['index_patterns'])
            if group['wide_count'] is not None:
                out.count(path+('wide_count',),group['wide_count']);out.put(path+('wide_patterns',),'Q',group['wide_patterns'])
        expected=group['wide_count']//group['count'] if group['count'] else None
        assert group['native_quotient']==expected;quotients.append((path,expected))


def scalar(out: Snapshot, value: dict) -> None:
    for key in ('stored_member_5e0','stored_member_5d8','stored_member_5dc'):
        out.scalar((key,),value[key])
    if value['discarded_i32'] is not None:
        out.scalar(('discarded_i32',),value['discarded_i32'])
    assert value['native_recomputes_members']==(value['version']<2)
    out.projections.append((('native_recomputes_members',),value['native_recomputes_members']))


def record(identifier: int, value: dict) -> tuple[Snapshot,list[tuple[Path,int | None]]]:
    out=Snapshot();quotients=[]
    with out.scope((),dict(start=0,end=value['end'])):
        out.scalar(('version',),value['version'])
        if identifier==129:tagged(out,value)
        elif identifier==138:context(out,value)
        elif identifier==139:fixed(out,value)
        elif identifier==417:paired(out,value,quotients)
        else:
            assert identifier==432;scalar(out,value)
    return out,quotients
