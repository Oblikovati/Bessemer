"""Independent auxiliary bytes and explicit exclusive body-type contexts (#120)."""
from itertools import product
import struct
from owned_selection import nodes,elements,record

TypeCase = tuple[bytes,bytes,int,int,int,int,int | None]


def auxiliary(stride: int, body: int, low: int, high: int) -> bytes:
    """Retain distinctive unknown bytes around the three independently placed fields."""
    raw=bytearray((i*23+37)&255 for i in range(stride))
    struct.pack_into('<I',raw,0,body)
    struct.pack_into('<2i',raw,12 if stride==284 else 16,low,high)
    return bytes(raw)


def auxiliary_cases() -> list[bytes]:
    """Include both profiles, skipped/negative/reversed bounds and decrement overflow evidence."""
    bounds=((0,0),(1,1),(2,5),(0,1),(1,0),(-1,5),(5,-1),(-0x80000000,1),
            (1,-0x80000000),(0x7fffffff,0x7fffffff),(10,2),(-0x80000000,-0x80000000))
    return [auxiliary(s,b,l,h) for s,b,(l,h) in product((284,288),(0,1,7,65535,0x80000000,0xffffffff),bounds)]


def edit_golden(raw: bytes) -> bytes:
    """Patch only identifier/L/H at literal reviewed offsets, without reconstructing unknown regions."""
    out=bytearray(raw)
    struct.pack_into('<I',out,0,0x89abcdef)
    struct.pack_into('<2i',out,12 if len(raw)==284 else 16,-7,0x7fffffff)
    return bytes(out)


def type_cases() -> list[TypeCase]:
    """Exercise dispatch precedence before scan-only safety guards, including mode-one callback use."""
    bounds=((-1,8),(0,-1),(-0x80000000,0x7fffffff),(0,0),(0,1),(0,8),(1,3),(8,8),(99,98),
            (0x7fffffff,0x7fffffff),(0x7fffffff,0),(0,0x7fffffff))
    domains=((0,1,2,65535,0x80000000,0xffffffff),(-0x80000000,-1,0,1,2,3,4,5,6,7,0x7fffffff),
             (-32768,-1,0,32767,None),bounds)
    return [(nodes(),elements(),body,start,end,mode,callback)
            for body,mode,callback,(start,end) in product(*domains)]+special_type_cases()


def special_type_cases() -> list[TypeCase]:
    """Make early-stop, last reference slot and empty/negative-bound bypass independently visible."""
    cases=[(b'',b'',0,0,0,3,-1),(b'',b'',0,-1,0,3,None)]
    rows=(record(4,3,1,ids=(0,2,3)),record(4,11,1,ids=(0,)*10+(6,)),
          record(4,2,1,ids=(1,-1)),record(7,1,1,ids=(1,)),record(4,0,1,ids=()),
          record(4,255,1,activity=0),record(4,3,1,ids=(0,2,0)))
    tail=record(4,1,1,ids=(6,))
    cases.extend((nodes(),row+tail,1,0,2,3,-1) for row in rows)
    return cases


def expected_type(case: TypeCase) -> dict[str,object]:
    """Separate dispatch from the ordered source relation, preserving exclusive endpoints."""
    n,e,body,start,end,mode,callback=case
    if start<0 or end<0:
        return dict(flag=-1,visited=[],first=None,selected=0,references=0,completion=0,check=0,callback=None)
    if callback is None:
        raise ValueError('missing callback')
    if callback!=-1 or mode!=3:
        flag=16 if callback!=-1 else {0:1,1:2,2:2,4:4,6:8}.get(mode,-1)
        return dict(flag=flag,visited=[],first=None,selected=0,references=0,completion=1 if callback!=-1 else 2,check=1,callback=callback)
    if start==0x7fffffff or end==0x7fffffff or (start<end and end>len(e)//140):
        raise ValueError('unsafe exclusive range')
    return expected_scan(n,e,body,start,end)


def expected_scan(nodes_raw: bytes,elements_raw: bytes,body: int,start: int,end: int) -> dict[str,object]:
    """Consume only the prefix ending at the first successful node association."""
    visited,selected,references=[],0,0
    for index in range(start,end):
        visited.append(index)
        row=elements_raw[index*140:(index+1)*140]
        if not row[0] or struct.unpack_from('<H',row,28)[0]!=body:
            continue
        selected+=1
        first,count=first_active_reference(nodes_raw,row,index)
        references+=count
        if first is not None:
            flag={4:1,8:2,3:2,2:4}.get(row[20],-1)
            return dict(flag=flag,visited=visited,first=first,selected=selected,references=references,completion=4,check=2,callback=-1)
    return dict(flag=-1,visited=visited,first=None,selected=selected,references=references,completion=3,check=2,callback=-1)


def first_active_reference(nodes_raw: bytes,row: bytes,index: int) -> tuple[tuple[int,int,int] | None,int]:
    """Validate only consumed signed references; a later invalid slot is outside early-stop coverage."""
    if row[23]>11:
        raise ValueError('selected count')
    references=struct.unpack_from('<11i',row,96)[:row[23]]
    for slot,node in enumerate(references):
        if node==0:
            continue
        if not 1<=node<=len(nodes_raw)//52:
            raise ValueError('selected node')
        if nodes_raw[(node-1)*52]:
            return (index,slot,node-1),slot+1
    return None,len(references)
