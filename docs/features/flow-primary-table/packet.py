"""Rebuild Source fields and freeze complete nested intervals independently of Bessemer."""
from __future__ import annotations
import struct
import cases
from wire_packet import counted,optional


def body(value: dict, start: int) -> tuple[bytes,bytes]:
    first=value['first_patterns'];second=value['second_patterns'];triplet=value['member_triplet']
    raw=struct.pack('<i',len(first))+cases.patterns.packed(first,'I')+struct.pack('<i',len(second))+cases.patterns.packed(second,'I')
    raw+=cases.patterns.packed(triplet,'i')+cases.patterns.packed(value['wide6'],'Q')+cases.patterns.packed(value['wide3'],'Q')
    if value['legacy_wide9'] is not None:
        raw+=cases.patterns.packed(value['legacy_wide9'],'Q')
    if value['stored_flag'] is not None:
        raw+=bytes((value['stored_flag'],))
    packet=struct.pack('<II',start,start+len(raw))+counted(first,'I')+counted(second,'I')+cases.patterns.packed(triplet,'i')
    packet+=counted(value['wide6'],'Q')+counted(value['wide3'],'Q')
    packet+=b'\0' if value['legacy_wide9'] is None else b'\1'+counted(value['legacy_wide9'],'Q')
    packet+=optional(value['stored_flag'],'B')+bytes((value['native_flag'],))
    return raw,packet


def row(kind: int, values: list | tuple, start: int) -> tuple[bytes,bytes]:
    shape={1:'I',2:'II',3:'IQ',4:'IQI'}[kind]
    entries=[(value,) if kind==1 else tuple(value) for value in values]
    fields=b''.join(struct.pack('<'+shape,*entry) for entry in entries)
    raw=struct.pack('<i',len(values))+fields
    return raw,struct.pack('<III',start,start+len(raw),len(values))+fields


def blocks(values: list | tuple, start: int) -> tuple[bytes,bytes]:
    raw=struct.pack('<i',len(values));packet=struct.pack('<I',len(values))
    for block in values:
        assert len(block)==16
        begin=start+len(raw);raw+=cases.patterns.packed(block,'Q')
        packet+=struct.pack('<II',begin,start+len(raw))+counted(block,'Q')
    return raw,packet


def element(value: dict, start: int) -> tuple[bytes,bytes]:
    assert value['start']==start
    raw=struct.pack('<i',len(value['rows']));packet=struct.pack('<IIBI',value['start'],value['end'],value['kind'],len(value['rows']))
    for values in value['rows']:
        wire,expected=row(value['kind'],values,start+len(raw));raw+=wire;packet+=expected
    wire,expected=blocks(value['wide_blocks'],start+len(raw));raw+=wire;packet+=expected
    assert value['end']==start+len(raw)
    return raw,packet


def typed(values: list | None, start: int) -> tuple[bytes,bytes]:
    if values is None:
        return b'',b'\0'
    raw=struct.pack('<i',len(values));packet=b'\1'+struct.pack('<I',len(values))
    for value in values:
        wire,expected=element(value,start+len(raw));raw+=wire;packet+=expected
    return raw,packet


def group(value: dict, start: int) -> tuple[bytes,bytes]:
    mode=value['stored_mode_byte'];raw=bytes((mode,)) if mode is not None else b''
    raw+=cases.patterns.packed(value['member_i32'],'i')+bytes((value['presence_byte'],))
    packet=optional(mode,'B')+cases.patterns.packed(value['member_i32'],'i')+bytes((value['presence_byte'],))
    if value['body'] is None:
        packet+=b'\0'
    else:
        wire,expected=body(value['body'],start+len(raw));raw+=wire;packet+=b'\1'+expected
    wire,expected=typed(value['typed_elements'],start+len(raw));raw+=wire;packet+=expected
    return raw,struct.pack('<III',value['ordinal'],start,start+len(raw))+packet


def item(value: dict) -> tuple[bytes,bytes]:
    raw=bytes((value['kind_byte'],value['element_kind_byte']))
    packet=struct.pack('<IIIBB',value['ordinal'],value['start'],value['end'],value['kind_byte'],value['element_kind_byte'])+struct.pack('<I',len(value['groups']))
    for value_group in value['groups']:
        wire,expected=group(value_group,value['start']+len(raw));raw+=wire;packet+=expected
    assert value['end']==value['start']+len(raw)
    return raw,packet


def record(value: dict) -> tuple[bytes,bytes]:
    raw=struct.pack('<ii',value['version'],value['count']);packet=struct.pack('<iI',value['version'],len(value['items']))
    assert len(value['items'])==value['count']
    for value_item in value['items']:
        assert value_item['start']==len(raw)
        wire,expected=item(value_item);raw+=wire;packet+=expected
    assert len(raw)==value['end']
    assert value['primary_class_vtable']=='0x1809ec4b0'
    return raw,packet+struct.pack('<I',value['end'])+bytes((value['complete_payload_framing'],value['engineering_fields_assigned']))
