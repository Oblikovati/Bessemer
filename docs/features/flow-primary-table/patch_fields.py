"""Physical existing-field locations, derived from the independent Source tree."""
from __future__ import annotations
from dataclasses import dataclass
from typing import Iterator
import struct
import packet

Path = tuple[str | int, ...]


@dataclass(frozen=True)
class PatchField:
    path: Path
    offset: int
    encoding: str

    @property
    def width(self) -> int:
        return struct.calcsize('<'+self.encoding)


def array_fields(path: Path, offset: int, values: list | tuple, encoding: str) -> Iterator[PatchField]:
    width=struct.calcsize('<'+encoding)
    for index in range(len(values)):
        yield PatchField(path+(index,),offset+index*width,encoding)


def body_fields(value: dict, path: Path, offset: int) -> Iterator[PatchField]:
    for key in ('first_patterns','second_patterns'):
        offset+=4
        yield from array_fields(path+(key,),offset,value[key],'I')
        offset+=4*len(value[key])
    yield from array_fields(path+('member_triplet',),offset,value['member_triplet'],'i')
    offset+=12
    for key in ('wide6','wide3','legacy_wide9'):
        if value[key] is not None:
            yield from array_fields(path+(key,),offset,value[key],'Q')
            offset+=8*len(value[key])
    if value['stored_flag'] is not None:
        yield PatchField(path+('stored_flag',),offset,'B')


def row_fields(value: list | tuple, kind: int, path: Path, offset: int) -> Iterator[PatchField]:
    shape={1:('I',),2:('I','I'),3:('I','Q'),4:('I','Q','I')}[kind]
    for index,entry in enumerate(value):
        for component,encoding in enumerate(shape):
            suffix=(index,) if kind==1 else (index,component)
            yield PatchField(path+suffix,offset,encoding)
            offset+=struct.calcsize('<'+encoding)


def element_fields(value: dict, path: Path) -> Iterator[PatchField]:
    offset=value['start']+4;width={1:4,2:8,3:12,4:16}[value['kind']]
    for index,row in enumerate(value['rows']):
        yield from row_fields(row,value['kind'],path+('rows',index),offset+4)
        offset+=4+width*len(row)
    offset+=4
    for index,block in enumerate(value['wide_blocks']):
        yield from array_fields(path+('wide_blocks',index),offset,block,'Q')
        offset+=128
    assert offset==value['end']


def group_fields(value: dict, path: Path, offset: int) -> Iterator[PatchField]:
    if value['stored_mode_byte'] is not None:
        yield PatchField(path+('stored_mode_byte',),offset,'B');offset+=1
    yield from array_fields(path+('member_i32',),offset,value['member_i32'],'i')
    offset+=8
    yield PatchField(path+('presence_byte',),offset,'B');offset+=1
    if value['body'] is not None:
        yield from body_fields(value['body'],path+('body',),offset)
        offset+=len(packet.body(value['body'],offset)[0])
    if value['typed_elements'] is not None:
        for index,element in enumerate(value['typed_elements']):
            yield from element_fields(element,path+('typed_elements',index))


def fields(value: dict) -> Iterator[PatchField]:
    for index,item in enumerate(value['items']):
        path=('items',index)
        yield PatchField(path+('kind_byte',),item['start'],'B')
        yield PatchField(path+('element_kind_byte',),item['start']+1,'B')
        offset=item['start']+2
        for group_index,group in enumerate(item['groups']):
            yield from group_fields(group,path+('groups',group_index),offset)
            offset+=len(packet.group(group,offset)[0])
        assert offset==item['end']
