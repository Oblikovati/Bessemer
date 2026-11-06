"""Immutable requests for the complete primary-class record439 grammar."""
from __future__ import annotations
from dataclasses import dataclass,replace
from itertools import product
import hashlib
import json
import struct
import sys
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parent.parent/'flow-medium-objects'))
import freeze_objects as patterns
from wire_packet import ROOT,SOURCE
import flowrecord439 as schema
import flowresults

WORDS=(0,1,0xffffffff,0x80000000,*[1<<bit for bit in range(32)],*[0xffffffff^(1<<bit) for bit in range(32)])

@dataclass(frozen=True)
class GroupSpec:
    presence: int=0
    mode: int=0
    flag: int=0
    first_count: int=0
    second_count: int=0
    rows: tuple[int,...]=()
    blocks: int=0
    elements: int=0
    seed: int=0

@dataclass(frozen=True)
class ItemSpec:
    kind: int
    element_kind: int
    groups: tuple[GroupSpec,GroupSpec]


def words(count: int, seed: int) -> tuple[int,...]:
    return tuple(WORDS[(seed+index)%len(WORDS)] for index in range(count))


def wide(count: int, seed: int) -> tuple[int,...]:
    return tuple(patterns.PATTERNS[(seed+index)%len(patterns.PATTERNS)] for index in range(count))


def signed(value: int) -> int:
    return value if value<0x80000000 else value-0x100000000


def row(kind: int, count: int, seed: int) -> bytes:
    result=struct.pack('<i',count)
    for index in range(count):
        first,second=words(2,seed+index*2);value=wide(1,seed+index)[0]
        entry={1:struct.pack('<I',first),2:struct.pack('<II',first,second),
               3:struct.pack('<IQ',first,value),4:struct.pack('<IQI',first,value,second)}[kind]
        result+=entry
    return result


def typed(kind: int, spec: GroupSpec, seed: int) -> bytes:
    raw=struct.pack('<i',len(spec.rows))+b''.join(row(kind,count,seed+index) for index,count in enumerate(spec.rows))
    raw+=struct.pack('<i',spec.blocks)
    return raw+b''.join(patterns.packed(wide(16,seed+index*16),'Q') for index in range(spec.blocks))


def body(version: int, spec: GroupSpec) -> bytes:
    first=words(spec.first_count,spec.seed);second=words(spec.second_count,spec.seed+1)
    raw=struct.pack('<i',len(first))+patterns.packed(first,'I')+struct.pack('<i',len(second))+patterns.packed(second,'I')
    raw+=patterns.packed(tuple(map(signed,words(3,spec.seed+2))),'i')
    raw+=patterns.packed(wide(6,spec.seed),'Q')+patterns.packed(wide(3,spec.seed+6),'Q')
    if version==0:
        raw+=patterns.packed(wide(9,spec.seed+9),'Q')
    return raw+(bytes((spec.flag,)) if version>=2 else b'')


def group(version: int, item: ItemSpec, spec: GroupSpec) -> bytes:
    raw=bytes((spec.mode,)) if version==3 else b''
    raw+=patterns.packed(tuple(map(signed,words(2,spec.seed))),'i')+bytes((spec.presence,))
    if spec.presence:
        raw+=body(version,spec)
    if item.kind in (2,3):
        raw+=struct.pack('<i',spec.elements)
        raw+=b''.join(typed(item.element_kind,spec,spec.seed+index) for index in range(spec.elements))
    return raw


def record(version: int, items: tuple[ItemSpec,...]) -> bytes:
    raw=struct.pack('<ii',version,len(items))
    for item in items:
        raw+=bytes((item.kind,item.element_kind))+b''.join(group(version,item,spec) for spec in item.groups)
    return raw


def item(kind: int, element_kind: int, spec: GroupSpec) -> ItemSpec:
    other=replace(spec,seed=spec.seed+17,mode=spec.mode^255,rows=spec.rows[::-1],elements=max(spec.elements-1,0))
    return ItemSpec(kind,element_kind,(spec,other))


def native_rows() -> tuple[list[tuple[str,bytes]],list[dict]]:
    rows=[];proof=[]
    for entry in json.loads((SOURCE/'research/flow-metadata-prefix-corpus.json').read_text()):
        raw=Path(entry['path']).read_bytes();assert hashlib.sha256(raw).hexdigest()==entry['source_sha256']
        selected=flowresults.latest(flowresults.read(raw),439)
        if selected is None:
            continue
        parsed=schema.read_record(selected);name=Path(entry['path']).suffix[1:]
        rows.append((f'native-{name}',bytes(selected['payload'])))
        proof.append(dict(name=name,source_sha256=entry['source_sha256'],payload_sha256=hashlib.sha256(selected['payload']).hexdigest(),version=parsed['version'],items=parsed['count']))
    return rows,proof


def kind_cases() -> list[tuple[str,bytes]]:
    spec=GroupSpec(elements=1,rows=(0,1,3),blocks=1)
    return [(f'kind-v{version}-{kind}',record(version,(item(kind,1,spec),))) for version in range(4) for kind in range(256)]


def unused_element_cases() -> list[tuple[str,bytes]]:
    return [(f'unused-element-v{version}-k{kind}-{element}',record(version,(item(kind,element,GroupSpec()),)))
            for version in range(4) for kind in (2,3) for element in range(256)]


def presence_cases() -> list[tuple[str,bytes]]:
    rows=[]
    for version in range(4):
        for presence in range(256):
            left=GroupSpec(presence=presence,mode=presence,flag=presence^128,first_count=presence%3,second_count=presence%4,seed=presence,rows=(0,1),elements=1)
            right=replace(left,presence=presence^255,mode=presence^255,flag=(presence+17)%256,elements=0)
            spec=ItemSpec(2 if version%2==0 else 3,presence%4+1,(left,right))
            rows.append((f'presence-v{version}-{presence}',record(version,(spec,))))
    return rows


def flag_cases() -> list[tuple[str,bytes]]:
    return [(f'flag-v{version}-{flag}',record(version,(item(0,255,GroupSpec(presence=1,flag=flag,seed=flag)),)))
            for version in (2,3) for flag in range(256)]


def layout_cases() -> list[tuple[str,bytes]]:
    profiles=[((),0),((0,),0),((1,),1),((0,1,3),2),((32,),3),((1024,),0),((0,)*128,0),((),128)]
    rows=[]
    for version,element,(profile,(lengths,blocks)),kind,presence in product(range(4),range(1,5),enumerate(profiles),(2,3),(0,255)):
        spec=GroupSpec(presence=presence,flag=2,first_count=3,second_count=1,rows=lengths,blocks=blocks,elements=2,seed=profile)
        rows.append((f'layout-v{version}-t{element}-p{profile}-k{kind}-b{presence}',record(version,(item(kind,element,spec),))))
    return rows


def pattern_cases() -> list[tuple[str,bytes]]:
    rows=[]
    for version in range(4):
        for seed in range(max(len(WORDS),len(patterns.PATTERNS))):
            spec=GroupSpec(presence=1,flag=seed%256,first_count=3,second_count=2,rows=(3,1),blocks=1,elements=1,seed=seed)
            rows.append((f'patterns-v{version}-{seed}',record(version,(item(2,seed%4+1,spec),))))
    return rows


def consecutive_cases() -> list[tuple[str,bytes]]:
    rows=[]
    for version in range(4):
        specs=tuple(item(kind,element,GroupSpec(presence=index%2,flag=1,rows=(0,1,3),blocks=1,elements=2,seed=index))
                    for index,(kind,element) in enumerate(((0,255),(2,1),(3,2),(2,3),(3,4),(255,255))))
        rows.extend(((f'consecutive-v{version}',record(version,specs)),(f'reversed-v{version}',record(version,specs[::-1]))))
        for kind in (2,3):
            rows.append((f'many-elements-v{version}-k{kind}',record(version,(item(kind,4,GroupSpec(elements=128)),))))
    return rows


def cases() -> list[tuple[str,bytes]]:
    native,_=native_rows()
    empty=[(f'empty-v{version}',record(version,())) for version in range(4)]
    return native+empty+kind_cases()+unused_element_cases()+presence_cases()+flag_cases()+layout_cases()+pattern_cases()+consecutive_cases()
