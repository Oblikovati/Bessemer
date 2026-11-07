"""Owned complete conditional-item requests for records126/315/128."""
from __future__ import annotations
from dataclasses import dataclass,replace
from itertools import product
import hashlib
import json
import struct
import sys
import zlib
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parent.parent/'flow-medium-objects'))
import freeze_objects as patterns
from wire_packet import ROOT,SOURCE,packed
import flowrecord126
import flowrecord315
import flowrecord128


@dataclass(frozen=True)
class ItemSpec:
    version: int=5
    active: int=1
    mode: int=0
    wide_count: int=1
    byte_count: int=3
    word_count: int=2
    groups: int=1
    subkind: int=2
    seed: int=0


@dataclass(frozen=True)
class GroupSpec:
    mode: int=1
    gate: int=1
    presence: int=1
    items: tuple[ItemSpec | None,...]=()


def words(count: int, seed: int) -> tuple[int,...]:
    return tuple((patterns.PATTERNS[(seed+index)%len(patterns.PATTERNS)]&0xffffffff) for index in range(count))


def wide(count: int, seed: int) -> tuple[int,...]:
    return tuple(patterns.PATTERNS[(seed+index)%len(patterns.PATTERNS)] for index in range(count))


def vector(values: tuple[int,...], shape: str, count: int | None = None) -> bytes:
    return struct.pack('<i',len(values) if count is None else count)+packed(values,shape)


def subrecord(spec: ItemSpec) -> bytes:
    raw=struct.pack('<iii',spec.subkind,-2147483648,2147483647)
    raw+=vector(words(spec.word_count,spec.seed),'I')
    return raw+(vector(words(spec.word_count+1,spec.seed+1),'I') if spec.subkind==2 else b'')


def item_group(spec: ItemSpec, scale: int, modern: bool) -> bytes:
    raw=vector(wide(spec.wide_count*scale,spec.seed),'Q',spec.wide_count)
    raw+=vector(tuple((spec.seed+index)%256 for index in range(spec.byte_count)),'B')
    return raw+(subrecord(spec) if modern else b'')


def active_prefix(spec: ItemSpec, dimension: int, scale: int) -> bytes:
    raw=b''
    if spec.version<4:
        raw=b''.join(item_group(replace(spec,seed=spec.seed+index),scale,False) for index in range(2))
    raw+=struct.pack('<i',dimension)
    if spec.version<3:
        raw+=struct.pack('<i',-2147483648)
    return raw+struct.pack('<i',2147483647)


def item(spec: ItemSpec, dimension: int, scale: int) -> bytes:
    raw=struct.pack('<iB',spec.version,spec.active)
    if not spec.active:
        return raw
    raw+=active_prefix(spec,dimension,scale)
    if spec.version:
        raw+=bytes((spec.mode,))
    if spec.version==1:
        assert spec.mode==0
        raw+=vector(words(spec.word_count,spec.seed),'I')+vector(words(spec.word_count+1,spec.seed+1),'I')
    if spec.version in (2,3):
        raw+=subrecord(spec)+subrecord(replace(spec,subkind=-1,seed=spec.seed+1))
    if spec.version>=4:
        raw+=struct.pack('<i',spec.groups)+b''.join(item_group(replace(spec,seed=spec.seed+index),scale,True) for index in range(spec.groups))
    return raw


def presence_items(spec: GroupSpec, dimension: int, scale: int) -> bytes:
    raw=struct.pack('<i',len(spec.items))
    for value in spec.items:
        raw+=b'\0' if value is None else bytes((spec.presence,))+item(value,dimension,scale)
    return raw


def group(spec: GroupSpec, scale: int) -> bytes:
    raw=struct.pack('<iIii',0,0x80000000,spec.mode,1)
    if spec.mode==0:
        raw+=struct.pack('<I',spec.gate)
    return raw+(presence_items(spec,1,scale) if spec.mode!=0 or spec.gate else b'')


def record126(spec: GroupSpec, inner: int = 0) -> bytes:
    raw=struct.pack('<iiiii',2,inner,-2147483648,spec.mode,3)
    if spec.mode==0:
        raw+=struct.pack('<I',spec.gate)
    return raw+(presence_items(spec,3,1) if spec.mode!=0 or spec.gate else b'')


def record315(spec: GroupSpec, extra: int = 1) -> bytes:
    vectors=b''.join(vector(wide(count,seed),'Q') for seed,count in enumerate((1,2,3)))
    tail=struct.pack('<ii',extra,-2147483648) if extra else struct.pack('<i',0)
    decoded=struct.pack('<i',4)+vectors+tail+packed(wide(extra,17),'Q')
    return zlib.compress(decoded+group(spec,1)+group(replace(spec,items=spec.items[::-1]),1))


def record128(spec: GroupSpec, version: int) -> bytes:
    return struct.pack('<i',version)+zlib.compress(group(spec,9 if version==6 else 6))


def parse(identifier: int, stored: bytes) -> tuple[bytes,dict]:
    record=dict(identifier=identifier,size=len(stored),payload=stored)
    schema={126:flowrecord126,315:flowrecord315,128:flowrecord128}[identifier]
    parsed=schema.read_record(record)
    decoded=stored if identifier==126 else zlib.decompress(stored if identifier==315 else stored[4:])
    return decoded,parsed


def native_cases() -> tuple[list[tuple[str,int,bytes]],list[dict]]:
    rows=[];proof=[]
    for identifier in (126,315,128):
        manifest=SOURCE/f'research/flow-record{identifier}-corpus.json'
        for entry in json.loads(manifest.read_text()):
            raw=Path(entry['path']).read_bytes();assert hashlib.sha256(raw).hexdigest()==entry['source_sha256']
            stored=raw[entry['record_start']+12:entry['record_end']]
            assert struct.unpack_from('<IQ',raw,entry['record_start'])==(identifier,len(stored))
            assert hashlib.sha256(stored).hexdigest()==entry['payload_sha256']
            rows.append((f'native-{identifier}',identifier,stored));decoded,value=parse(identifier,stored)
            proof.append(dict(identifier=identifier,source_sha256=entry['source_sha256'],payload_sha256=entry['payload_sha256'],stored_bytes=len(stored),decoded_bytes=len(decoded),version=value['version']))
    return rows,proof


def active_cases() -> list[tuple[str,int,bytes]]:
    result=[]
    for version,active in product(range(6),range(256)):
        spec=ItemSpec(version=version,active=active,mode=0 if version==1 else active^255,seed=active)
        result.extend(family(f'item-v{version}-active{active}',GroupSpec(presence=active or 1,items=(None,spec))))
    for version in range(2,6):
        result.extend(family(f'mode255-v{version}',GroupSpec(items=(ItemSpec(version=version,mode=255),))))
    return result


def family(name: str, header: GroupSpec) -> list[tuple[str,int,bytes]]:
    return [(name+'-126',126,record126(header)),(name+'-315',315,record315(header)),
            *[(name+f'-128v{outer}',128,record128(header,outer)) for outer in (6,7)]]


def branch_cases() -> list[tuple[str,int,bytes]]:
    result=[]
    for version,(mode,gate),count in product(range(6),((0,0),(0,1),(0,0xffffffff),(1,0),(2,0)),(0,1,3)):
        spec=ItemSpec(version=version,mode=0,groups=count,wide_count=count,byte_count=count,word_count=count)
        header=GroupSpec(mode=mode,gate=gate,items=(spec,None,replace(spec,active=0)))
        name=f'branches-v{version}-m{mode}-g{gate}-n{count}'
        result.extend(((name+'-126',126,record126(header,-1)),(name+'-315',315,record315(header,count))))
        result.extend((name+f'-128v{outer}',128,record128(header,outer)) for outer in (6,7))
    return result


def boundary_cases() -> list[tuple[str,int,bytes]]:
    rows=[]
    for version,kind in product(range(2,6),(-2147483648,-1,0,1,2,3,2147483647)):
        spec=ItemSpec(version=version,subkind=kind)
        rows.extend(family(f'subkind-v{version}-k{kind}',GroupSpec(items=(spec,))))
    for count in (0,32,128,1024):
        spec=ItemSpec(groups=1,wide_count=count,byte_count=count,word_count=count)
        rows.extend(family(f'large-n{count}',GroupSpec(items=(spec,))))
    for inner in (-2147483648,-1,0):
        rows.append((f'inner-{inner}',126,record126(GroupSpec(items=()),inner)))
    return rows


def cases() -> list[tuple[str,int,bytes]]:
    native,_=native_cases()
    legacy=[(f'legacy-v1-n{count}',126,struct.pack('<ii',1,count)+b''.join(packed(wide(count,seed),'Q') for seed in range(3))) for count in (0,1,2,32,1024)]
    return native+legacy+active_cases()+branch_cases()+boundary_cases()
