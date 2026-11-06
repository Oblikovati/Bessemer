"""Owned record11 version18 load-mode requests and audited native boundaries."""
from __future__ import annotations
from dataclasses import dataclass
from itertools import product
import hashlib
import json
import struct
import sys
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parent.parent/'flow-conditional-items'))
import requests as conditional_requests
from wire_packet import ROOT,SOURCE,packed
import flowrecord11 as schema

WORDS=conditional_requests.words
WIDE=conditional_requests.wide
TEXTS=((),(0,),(65,),(65,0),(65,0,66),(0,0xd800),(0xd800,),(0xdc00,),
       (0xd800,0,0xdc00),(0xd83d,0xde80),(101,0x301),(0xe9,),(0xfffe,0xfeff))


@dataclass(frozen=True)
class ItemSpec:
    first: tuple[int,...]=(65,0)
    second: tuple[int,...]=(0xd800,0)
    description: tuple[int,...]=()
    extra_count: int=1
    seed: int=0


def signed(pattern: int) -> int:
    return pattern if pattern<0x80000000 else pattern-0x100000000


def text(units: tuple[int,...]) -> bytes:
    return struct.pack('<i',len(units))+packed(units,'H')


def item(spec: ItemSpec, width: int) -> bytes:
    raw=text(spec.first)+text(spec.second)
    raw+=packed(tuple(map(signed,WORDS(9,spec.seed))),'i')
    raw+=packed(tuple(map(signed,WORDS(2,spec.seed+9))),'i')
    raw+=packed(WIDE(1,spec.seed),'Q')+struct.pack('<i',signed(WORDS(1,spec.seed+11)[0]))
    raw+=packed(WIDE(2,spec.seed+1),'Q')
    raw+=b''.join(packed(WIDE(width,spec.seed+index),'Q') for index in range(4))
    raw+=struct.pack('<i',spec.extra_count)+packed(WORDS(spec.extra_count,spec.seed),'I')
    return raw+text(spec.description)


def record(load_mode: bool, width: int, items: tuple[ItemSpec,...], seed: int = 0) -> bytes:
    raw=struct.pack('<iiii',18,len(items),signed(WORDS(1,seed)[0]),width)
    raw+=b''.join(item(spec,width) for spec in items)
    if not items:
        return raw
    raw+=packed(WORDS(width,seed),'I')+packed(WORDS(width,seed+1),'I')
    raw+=packed(WIDE(width,seed),'Q')+packed(WIDE(width,seed+1),'Q')
    raw+=packed(tuple(map(signed,WORDS(2,seed+2))),'i')
    if not load_mode:
        raw+=packed(WORDS(16,seed),'I')+packed(WIDE(16,seed),'Q')
    return raw


def native_cases() -> tuple[list[tuple[str,bool,int,bytes]],dict]:
    entry=json.loads((SOURCE/'research/flow-record11-corpus.json').read_text())[0]
    raw=Path(entry['path']).read_bytes();assert hashlib.sha256(raw).hexdigest()==entry['source_sha256']
    stored=raw[entry['start']+12:entry['end']]
    assert struct.unpack_from('<IQ',raw,entry['start'])==(11,len(stored))
    assert hashlib.sha256(stored).hexdigest()==entry['payload_sha256']
    complete=schema.read(stored,load_mode=False);prefix=schema.read_prefix(stored,load_mode=True)
    assert prefix['end']==entry['selector_true_mode_prefix_end']
    tail=prefix['opaque_tail'];assert len(tail)==192 and hashlib.sha256(tail).hexdigest()==entry['selector_opaque_tail']['sha256']
    derived=stored[:prefix['end']];schema.read(derived,load_mode=True)
    proof=dict(source_sha256=entry['source_sha256'],payload_sha256=entry['payload_sha256'],original_mode_assumption=False,
               original_payload_bytes=len(stored),items=complete['count'],width=complete['array_width'],
               true_mode_prefix_end=prefix['end'],true_mode_opaque_bytes=len(tail),true_mode_opaque_sha256=hashlib.sha256(tail).hexdigest(),
               derived_true_mode_prefix_sha256=hashlib.sha256(derived).hexdigest(),native_owner_mode_verified=False)
    return [('native-false',False,1,stored),('derived-native-true-prefix',True,2,derived)],proof


def shape_cases() -> list[tuple[str,bool,int,bytes]]:
    rows=[]
    for mode,count,width in product((False,True),(0,1,2,8),(0,1,3,32,128)):
        items=tuple(ItemSpec(extra_count=index%4,seed=index,first=TEXTS[index%len(TEXTS)]) for index in range(count))
        rows.append((f'shape-m{int(mode)}-n{count}-w{width}',mode,0,record(mode,width,items)))
    for mode,width in product((False,True),(0,1,2147483647)):
        rows.append((f'empty-m{int(mode)}-w{width}',mode,0,record(mode,width,())))
    return rows


def text_cases() -> list[tuple[str,bool,int,bytes]]:
    rows=[]
    for mode,index in product((False,True),range(len(TEXTS))):
        spec=ItemSpec(first=TEXTS[index],second=TEXTS[-index-1],description=TEXTS[(index+1)%len(TEXTS)],seed=index)
        rows.append((f'text-m{int(mode)}-{index}',mode,0,record(mode,1,(spec,))))
    units=tuple(range(65536))
    for mode in (False,True):
        spec=ItemSpec(first=units,second=units[::-1],description=(0xd800,0,0xdc00))
        rows.append((f'all-code-units-m{int(mode)}',mode,0,record(mode,0,(spec,))))
    return rows


def pattern_cases() -> list[tuple[str,bool,int,bytes]]:
    rows=[]
    for mode,seed in product((False,True),range(len(conditional_requests.patterns.PATTERNS))):
        spec=ItemSpec(seed=seed,extra_count=3)
        rows.append((f'patterns-m{int(mode)}-{seed}',mode,0,record(mode,3,(spec,),seed)))
    for mode,count in product((False,True),(0,1,32,1024)):
        rows.append((f'extra-m{int(mode)}-n{count}',mode,0,record(mode,1,(ItemSpec(extra_count=count),))))
    return rows


def cases() -> list[tuple[str,bool,int,bytes]]:
    native,_=native_cases()
    return native+shape_cases()+text_cases()+pattern_cases()
