"""Generated finite/ordered numeric requests and deliberately unvalidated unused fields."""
from __future__ import annotations
import math
import random
import struct
import sys
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parent.parent/'flow-medium-objects'))
import freeze_objects as objects
from oracle import bits,number

FINITE=tuple(pattern for pattern in objects.PATTERNS if math.isfinite(number(pattern)))
MAX=number(0x7fefffffffffffff)
MIN=number(1)


def curve(scalar: int, x: tuple[int,...], y: tuple[int,...], flag: int) -> bytes:
    assert len(x)==len(y)
    raw=b'\0'+struct.pack('<Qi',scalar,len(x))+objects.packed(x,'Q')+objects.packed(y,'Q')+bytes((flag,))
    return raw+objects.packed([0x7ff8000000000042]*(2*len(x)),'Q') if flag else raw


def body(ordinal: int, scalar: int, x: tuple[int,...], y: tuple[int,...], flag: int = 255) -> bytes:
    raw=bytes((0,254))+bytes(range(16))+bytes(4)
    poison=curve(0x7ff0000000000001,(0x7ff8000000000042,),(0x7ff0000000000000,),1)
    raw+=b''.join(curve(scalar,x,y,flag) if index==ordinal else poison for index in range(7))
    raw+=struct.pack('<i',1)+objects.packed([0xfff0000000000000]*2,'Q')
    raw+=struct.pack('<i',1)+objects.packed([0x7ff0000000000001]*4,'Q')
    return raw+objects.packed([0x7ff8000000000042]*4,'Q')+bytes((99,))


def row(name: str, ordinal: int, x: tuple[float,...], y: tuple[float,...], query: float, scalar: int = 0x7ff8000000000042, flag: int = 255) -> tuple[str,bytes,int,int]:
    raw=body(ordinal,scalar,tuple(map(bits,x)),tuple(map(bits,y)),flag)
    return name,raw,ordinal,bits(query)


def native_cases() -> list[tuple[str,bytes,int,int]]:
    rows=[]
    for name,raw in objects.native_cases()[:1]:
        parsed=objects.schema.read_kind1_body(raw,string_mode=0)
        for ordinal in (1,2,4):
            x=tuple(map(number,parsed['curves'][ordinal]['first_patterns']))
            queries=[x[0]-1,x[-1]+1,*x,*[(a+b)/2 for a,b in zip(x,x[1:])]]
            rows.extend((f'{name}-curve{ordinal}-q{index}',raw,ordinal,bits(query)) for index,query in enumerate(queries))
    return rows


def constants() -> list[tuple[str,bytes,int,int]]:
    return [(f'constant-c{ordinal}-p{pattern:016x}-q{index}',body(ordinal,pattern,(),()),ordinal,bits(query))
            for ordinal in (1,2,4) for pattern in FINITE for index,query in enumerate((-MAX,-0.,0.,MIN,MAX))]


def tables() -> list[tuple[str,bytes,int,int]]:
    rows=[]
    for ordinal in (1,2,4):
        for count in (1,2,3,7,39,64,257,1024):
            x=tuple(float(index*index+1) for index in range(count));y=tuple(3*value-5 for value in x)
            queries=[x[0]-1,x[-1]+1,*x,*[(a+b)/2 for a,b in zip(x,x[1:])]]
            if count==1024:
                queries=[queries[index] for index in (0,1,2,3,127,511,1025,1026,1500,2048)]
            rows.extend(row(f'affine-c{ordinal}-n{count}-q{index}',ordinal,x,y,query) for index,query in enumerate(queries))
    return rows


def flags() -> list[tuple[str,bytes,int,int]]:
    return [row(f'flag-{flag}-n{count}',2,tuple(float(index) for index in range(count)),tuple(-0. for _ in range(count)),.5,bits(-0.),flag)
            for flag in range(256) for count in (0,1,2)]


def random_tables() -> list[tuple[str,bytes,int,int]]:
    generator=random.Random(241)
    rows=[]
    for index in range(128):
        count=2+index%14;position=-1000;coordinates=[]
        for _ in range(count):
            position+=generator.randrange(1,100);coordinates.append(position/8)
        x=tuple(coordinates);y=tuple(generator.randrange(-1000,1000)/16 for _ in x)
        queries=(x[0]-1,x[-1]+1,x[count//2],(x[count//2-1]+x[count//2])/2)
        rows.extend(row(f'random-{index}-q{query_index}',(1,2,4)[index%3],x,y,query) for query_index,query in enumerate(queries))
    return rows


def extremes() -> list[tuple[str,bytes,int,int]]:
    profiles=[((0.,1.),(-MAX,MAX),.5),((0.,1.,2.),(0.,-MAX,MAX),1.),
              ((-MAX,MAX),(0.,1.),0.),((-MAX,MAX),(-MAX,MAX),0.),
              ((0.,2*MIN),(0.,1.),MIN),((-1.,1.),(-0.,-0.),0.),
              ((-MIN,MIN),(-MIN,MIN),0.),((0.,1.,2.),(1.,-0.,3.),1.),((0.,1.),(MAX,-MAX),.5)]
    return [row(f'extreme-c{ordinal}-{index}',ordinal,x,y,query) for ordinal in (1,2,4) for index,(x,y,query) in enumerate(profiles)]


def invalid_cases() -> list[tuple[str,bytes,int,int]]:
    rows=[row(f'unsupported-{ordinal}',ordinal,(),(),0.,bits(1.)) for ordinal in (-2147483648,-1,0,3,5,6,7,2147483647)]
    rows += [row(f'bad-query-{pattern:016x}',2,(0.,1.),(0.,1.),number(pattern)) for pattern in objects.PATTERNS if not math.isfinite(number(pattern))]
    for pattern in objects.PATTERNS:
        if math.isfinite(number(pattern)):
            continue
        rows.append((f'bad-scalar-{pattern:016x}',body(2,pattern,(),()),2,bits(0.)))
        rows.append((f'bad-x-{pattern:016x}',body(2,bits(0.),(bits(0.),pattern),(bits(0.),bits(1.))),2,bits(-1.)))
        rows.append((f'bad-y-{pattern:016x}',body(2,bits(0.),(bits(0.),bits(1.)),(bits(0.),pattern)),2,bits(-1.)))
    rows += [row(f'nonordered-{index}',2,x,(0.,1.),0.) for index,x in enumerate(((1.,1.),(2.,1.),(-0.,0.),(0.,-0.)))]
    return rows


def cases() -> list[tuple[str,bytes,int,int]]:
    return native_cases()+constants()+tables()+flags()+random_tables()+extremes()+invalid_cases()
