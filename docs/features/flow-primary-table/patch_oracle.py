"""Rebuild complete expected edits from changed Source fields before byte patching."""
from __future__ import annotations
import struct
from typing import Iterator
import cases
import packet
from patch_fields import PatchField,Path,fields


def at(value: dict | list | tuple, path: Path) -> object:
    result=value
    for key in path:
        result=result[key]
    return result


def replaced(value: object, path: Path, replacement: int) -> object:
    if not path:
        return replacement
    key,*tail=path
    result=list(value) if isinstance(value,(list,tuple)) else dict(value)
    result[key]=replaced(value[key],tuple(tail),replacement)
    return tuple(result) if isinstance(value,tuple) else result


def replacement(value: dict, field: PatchField) -> int | None:
    current=at(value,field.path);role=field.path[-1]
    if role=='kind_byte':
        return 3 if current==2 else 2 if current==3 else 255 if current!=255 else 0
    if role=='element_kind_byte':
        item=at(value,field.path[:-1])
        return None if any(group['typed_elements'] for group in item['groups']) else current^255
    if role=='presence_byte':
        return None if current==0 else 255 if current!=255 else 1
    if role=='stored_flag':
        return 255 if current==1 else 1
    if field.encoding=='i':
        return -current-1
    return current^((1<<(8*field.width))-1)


def changed_tree(value: dict, field: PatchField, next_value: int) -> dict:
    changed=replaced(value,field.path,next_value)
    if field.path[-1]=='stored_flag':
        body=at(changed,field.path[:-1]);body['native_flag']=next_value==1
    return changed


def patch_cases(name: str, raw: bytes) -> Iterator[tuple[str,PatchField,int,int,bytes,bytes]]:
    value=cases.schema.read(raw)
    for field in fields(value):
        next_value=replacement(value,field)
        if next_value is None:
            continue
        changed=changed_tree(value,field,next_value)
        expected=packet.record(changed)[0]
        parsed=cases.schema.read(expected)
        assert parsed==changed
        yield name,field,at(value,field.path),next_value,raw,expected


def requests() -> Iterator[tuple[str,bytes]]:
    for version in range(4):
        for kind in range(1,5):
            spec=cases.GroupSpec(presence=2,first_count=2,second_count=1,rows=(0,1,3),blocks=1,elements=1)
            yield f'v{version}-type{kind}',cases.record(version,(cases.item(2,kind,spec),))
        for presence in (0,1,255):
            spec=cases.GroupSpec(presence=presence,flag=1,mode=255)
            yield f'v{version}-empty-{presence}',cases.record(version,(cases.item(255,255,spec),))


def validate_patch(field: PatchField, before: int, after: int, raw: bytes, expected: bytes) -> None:
    offset=field.offset;end=offset+field.width
    assert raw[offset:end]==struct.pack('<'+field.encoding,before)
    assert expected[offset:end]==struct.pack('<'+field.encoding,after)
    assert raw[:offset]==expected[:offset] and raw[end:]==expected[end:]
    assert before!=after and len(raw)==len(expected)
