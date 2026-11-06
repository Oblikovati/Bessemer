"""Owned complete record301 cases, including legacy byte transfers and explicit modern mode."""
from __future__ import annotations
from dataclasses import dataclass
from itertools import product
import hashlib
import json
import struct
import sys
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parent.parent/'flow-load-state'))
import load_requests
from wire_packet import ROOT,SOURCE,packed
import flowrecord301 as schema

MODERN=tuple(units if not units or units[-1]==0 else units+(0,) for units in load_requests.TEXTS)
LEGACY=(b'',b'\0',b'A',b'A\0',b'A\0B',b'\0\xd8\0',b'\xff\xfe',b'\xfe\xff',bytes(range(256)))
Text=tuple[int,...] | bytes


@dataclass(frozen=True)
class MessageSpec:
    member: int=0
    primary: Text=()
    first: tuple[Text,...]=()
    second: tuple[Text,...] | None=None


def text(version: int, value: Text) -> bytes:
    if version==1:
        assert isinstance(value,bytes);return struct.pack('<i',len(value))+value
    assert isinstance(value,tuple) and (not value or value[-1]==0)
    return struct.pack('<i',len(value))+packed(value,'H')


def string_list(version: int, values: tuple[Text,...]) -> bytes:
    return struct.pack('<i',len(values))+b''.join(text(version,value) for value in values)


def item(version: int, spec: MessageSpec) -> bytes:
    raw=struct.pack('<I',spec.member)+text(version,spec.primary)+string_list(version,spec.first)
    if version<3:
        assert spec.second is None;return raw
    assert spec.second is not None
    return raw+string_list(version,spec.second)


def record(version: int, items: tuple[MessageSpec,...]) -> bytes:
    return struct.pack('<ii',version,len(items))+b''.join(item(version,spec) for spec in items)


def parse(raw: bytes, mode: int | None) -> dict:
    return schema.read_record(dict(identifier=301,size=len(raw),payload=raw),string_mode=mode)


def native_cases() -> tuple[list[tuple[str,int | None,bool,bytes]],dict]:
    manifest=SOURCE/'research/flow-state-records-corpus.json'
    entry=next(row for row in json.loads(manifest.read_text()) if row['identifier']==301)
    source=Path(entry['path']).read_bytes();assert hashlib.sha256(source).hexdigest()==entry['source_sha256']
    raw=source[entry['record_start']+12:entry['record_end']]
    assert struct.unpack_from('<IQ',source,entry['record_start'])==(301,len(raw))
    assert hashlib.sha256(raw).hexdigest()==entry['payload_sha256']
    value=parse(raw,0);assert value['end']==328 and value['count']==1
    proof=dict(source_sha256=entry['source_sha256'],payload_sha256=entry['payload_sha256'],record_start=entry['record_start'],record_end=entry['record_end'],payload_bytes=len(raw),version=value['version'],items=value['count'],mode_assumption=0,native_mode_provenance_verified=False,manifest_sha256=hashlib.sha256(manifest.read_bytes()).hexdigest())
    return [('native-message-list',0,True,raw)],proof


def shape_spec(version: int, ordinal: int, first: int, second: int) -> MessageSpec:
    texts=LEGACY if version==1 else MODERN
    a=tuple(texts[(ordinal+index)%len(texts)] for index in range(first))
    b=tuple(texts[(ordinal-index-1)%len(texts)] for index in range(second)) if version==3 else None
    return MessageSpec(load_requests.WORDS(1,ordinal)[0],texts[ordinal%len(texts)],a,b)


def shape_cases() -> list[tuple[str,int | None,bool,bytes]]:
    rows=[]
    for version,count,first in product(range(4),(0,1,2,8),(0,1,3,32)):
        for second in ((0,1,3,32) if version==3 else (0,)):
            specs=tuple(shape_spec(version,index,first,second) for index in range(count))
            rows.append((f'shape-v{version}-n{count}-a{first}-b{second}',None if version==1 else 0,False,record(version,specs)))
    return rows


def modern_cases() -> list[tuple[str,int | None,bool,bytes]]:
    rows=[]
    for version,index in product((0,2,3),range(len(MODERN))):
        spec=MessageSpec(index,MODERN[index],(MODERN[-index-1],(),(0,)),(MODERN[(index+1)%len(MODERN)],) if version==3 else None)
        rows.append((f'modern-v{version}-text{index}',0,False,record(version,(spec,))))
    for version in (0,2,3):
        for name,units in (('all-units',tuple(range(1,65536))+(0,)),('all-units-hidden',(0,)+tuple(range(65535,0,-1))+(0,)),('nfd',(101,0x301,0)),('nfc',(0xe9,0))):
            spec=MessageSpec(0xffffffff,units,((0xd800,0),(0xdc00,0)),() if version==3 else None)
            rows.append((f'{name}-v{version}',0,False,record(version,(spec,))))
    return rows


def legacy_cases() -> list[tuple[str,int | None,bool,bytes]]:
    rows=[]
    for byte in range(256):
        spec=MessageSpec(byte,bytes((byte,)),(bytes((byte,0,255-byte)),b''))
        rows.append((f'legacy-byte{byte}',None,False,record(1,(spec,))))
    for count in (0,1,2,3,7,32,1024):
        raw=bytes(index%256 for index in range(count));spec=MessageSpec(count,raw,(raw[::-1],b'\xff'))
        rows.append((f'legacy-length{count}',None,False,record(1,(spec,))))
    for mode in (None,0,1,-1,2147483647):
        rows.append((f'legacy-unused-mode{mode}',mode,False,record(1,(MessageSpec(0,b'\0\xd8\x80'),))))
    return rows


def pattern_cases() -> list[tuple[str,int | None,bool,bytes]]:
    rows=[]
    for version,seed in product(range(4),range(len(load_requests.conditional_requests.patterns.PATTERNS))):
        texts=LEGACY if version==1 else MODERN
        spec=MessageSpec(load_requests.WORDS(1,seed)[0],texts[seed%len(texts)],(texts[-seed%len(texts)],),() if version==3 else None)
        rows.append((f'pattern-v{version}-s{seed}',None if version==1 else 0,False,record(version,(spec,))))
    for version,count in product(range(4),(0,1,32,1024)):
        spec=shape_spec(version,1,count,count)
        rows.append((f'large-list-v{version}-n{count}',None if version==1 else 0,False,record(version,(spec,))))
    return rows


def cases() -> list[tuple[str,int | None,bool,bytes]]:
    native,_=native_cases()
    return native+shape_cases()+modern_cases()+legacy_cases()+pattern_cases()
