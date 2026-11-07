"""Owned complete raw state-table requests with explicit record138 count context."""
from __future__ import annotations
from itertools import product
import hashlib
import json
import struct
import sys
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parent.parent/'flow-conditional-items'))
import requests as item_requests
from wire_packet import ROOT,SOURCE,packed
import flowresults
import flowrecord129
import flowrecord138
import flowrecord139
import flowrecord417
import flowrecord432

SCHEMAS={129:flowrecord129,138:flowrecord138,139:flowrecord139,417:flowrecord417,432:flowrecord432}
WORDS=item_requests.words
WIDE=item_requests.wide


def signed(pattern: int) -> int:
    return pattern if pattern<0x80000000 else pattern-0x100000000


def parse(identifier: int, raw: bytes, context: int | None = None) -> dict:
    kwargs=dict(context_count=context) if identifier==138 else {}
    return SCHEMAS[identifier].read_record(dict(identifier=identifier,size=len(raw),payload=raw),**kwargs)


def record129(tags: tuple[int,...], widths: tuple[int,...], seed: int = 0) -> bytes:
    assert len(tags)==len(widths)
    raw=struct.pack('<ii',9,len(tags))+bytes(tags)
    for index,(tag,width) in enumerate(zip(tags,widths)):
        if tag:
            raw+=struct.pack('<i',width)+packed(WIDE(width,seed+index),'Q')
    return raw+(struct.pack('<i',signed(WORDS(1,seed)[0])) if tags else b'')


def record138(version: int, objects: int, context: int | None, seed: int = 0) -> bytes:
    raw=struct.pack('<ii',version,objects)
    if objects and version<5:
        assert context is not None;raw+=packed(WORDS(context,seed),'I')
    return raw+(struct.pack('<i',signed(WORDS(1,seed)[0])) if version<4 else b'')


def record139(version: int, count: int, seed: int = 0) -> bytes:
    raw=struct.pack('<i',version)
    if version==0:
        assert count==0;return raw
    raw+=struct.pack('<i',count)
    for index in range(count):
        raw+=packed(WIDE(2,seed+index),'Q')
        if version>=2:raw+=packed(WIDE(320,seed+index+2),'Q')+packed(WORDS(100,seed+index),'I')
        if version>=3:raw+=struct.pack('<i',signed(WORDS(1,seed+index+1)[0]))
    return raw


def record417(first: tuple[int,int], second: tuple[int,int], seed: int = 0) -> bytes:
    raw=struct.pack('<i',1)
    for index,(count,width) in enumerate((first,second)):
        raw+=struct.pack('<i',count)+packed(WORDS(count,seed+index),'I')
        if count:raw+=struct.pack('<i',width)+packed(WIDE(width,seed+index),'Q')
    return raw


def record432(version: int, seed: int = 0) -> bytes:
    raw=struct.pack('<i',version)+packed(tuple(map(signed,WORDS(3,seed))),'i')
    return raw+(struct.pack('<i',signed(WORDS(1,seed+3)[0])) if version in (1,2) else b'')


def native_context(identifier: int, raw: bytes) -> int | None:
    if identifier!=138:
        return None
    version,objects=struct.unpack_from('<ii',raw)
    if not objects or version>=5:
        return None
    tail=4 if version<4 else 0;extent=len(raw)-8-tail;assert extent%4==0
    return extent//4


def native_cases() -> tuple[list[tuple[str,int,int | None,bytes]],list[dict]]:
    requests=[];proof=[]
    for entry in json.loads((SOURCE/'research/flow-metadata-prefix-corpus.json').read_text()):
        raw=Path(entry['path']).read_bytes();assert hashlib.sha256(raw).hexdigest()==entry['source_sha256']
        carrier=flowresults.read(raw)
        for identifier in SCHEMAS:
            record=flowresults.latest(carrier,identifier)
            if record is None:continue
            stored=bytes(record['payload']);context=native_context(identifier,stored);value=parse(identifier,stored,context)
            name=f'native-{identifier}-{Path(entry["path"]).suffix[1:]}'
            requests.append((name,identifier,context,stored))
            proof.append(dict(identifier=identifier,source_sha256=entry['source_sha256'],payload_sha256=hashlib.sha256(stored).hexdigest(),bytes=len(stored),version=value['version'],context=context,context_provenance='extent-derived explicit assumption' if context is not None else 'not applied',native_context_verified=False))
    return requests,proof


def tag_cases() -> list[tuple[str,int,int | None,bytes]]:
    rows=[]
    for tag,width in product(range(256),(0,1,3)):
        rows.append((f'tag-{tag}-w{width}',129,None,record129((0,tag,tag,0),(0,width,width+1,0),tag)))
    for count in (0,1,32,1024):
        tags=tuple(0 if index%3==0 else index%255+1 for index in range(count))
        rows.append((f'tags-n{count}',129,None,record129(tags,tuple(index%4 for index in range(count)))))
    return rows


def context_cases() -> list[tuple[str,int,int | None,bytes]]:
    rows=[]
    for version,objects,context in product(range(2,6),(0,1,17,2147483647),(0,1,3,32,1024)):
        rows.append((f'context-v{version}-n{objects}-c{context}',138,context,record138(version,objects,context,context)))
    for version,objects in product(range(2,6),(0,1)):
        if not objects or version==5:
            rows.append((f'context-v{version}-n{objects}-none',138,None,record138(version,objects,None)))
    return rows


def fixed_cases() -> list[tuple[str,int,int | None,bytes]]:
    rows=[('fixed-v0',139,None,record139(0,0))]
    rows.extend((f'fixed-v{version}-n{count}',139,None,record139(version,count,version)) for version,count in product(range(1,5),(0,1,2,8)))
    for version,seed in product((1,2,3,4),range(len(item_requests.patterns.PATTERNS))):
        rows.append((f'fixed-v{version}-seed{seed}',139,None,record139(version,1,seed)))
    return rows


def paired_cases() -> list[tuple[str,int,int | None,bytes]]:
    layouts=((0,0),(1,0),(1,1),(1,3),(2,1),(2,3),(3,2),(3,16),(32,1),(1,1024))
    rows=[(f'paired-{a}-{b}',417,None,record417(a,b,index)) for index,(a,b) in enumerate(product(layouts,layouts))]
    rows.extend((f'paired-seed{seed}',417,None,record417((3,3),(2,3),seed)) for seed in range(len(item_requests.patterns.PATTERNS)))
    return rows


def scalar_cases() -> list[tuple[str,int,int | None,bytes]]:
    versions=tuple(sorted({0,1,2,3,2147483647,*[1<<bit for bit in range(31)],*[2147483647^(1<<bit) for bit in range(31)]}))
    rows=[(f'scalar-v{version}',432,None,record432(version,version)) for version in versions]
    rows.extend((f'scalar-v{version}-seed{seed}',432,None,record432(version,seed)) for version,seed in product((0,1,2,3,2147483647),range(len(item_requests.patterns.PATTERNS))))
    return rows


def cases() -> list[tuple[str,int,int | None,bytes]]:
    native,_=native_cases()
    return native+tag_cases()+context_cases()+fixed_cases()+paired_cases()+scalar_cases()
