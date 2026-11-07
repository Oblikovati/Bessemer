"""Independent supplied-context mesh, selection and mark-history oracles (#119)."""
from itertools import product
import struct

MASKS = tuple(range(32)) + (0x80000000, 0xffffffff, 0x10000, 0x80000012, 0x10000001)
CALLER_KEYS = (-0x80000000, -1, 0, 1, 2, 65535, 65536, 0x7fffffff)
QueryCase = tuple[bytes,bytes,int,int,int | None,tuple[int,int] | None,list[int | None] | None]
WriteHistory = tuple[int,int,int,int,int | None,int]


def record(family: int, count: int, key: int, activity: int = 1,
           ids: tuple[int, ...] = (1, 3, 0, 3, 2)) -> bytes:
    """Place native fields at reviewed offsets while retaining distinctive opaque bytes."""
    raw = bytearray((i*13+29) & 255 for i in range(140))
    raw[0], raw[20], raw[23] = activity, family, count
    struct.pack_into('<H', raw, 28, key)
    struct.pack_into('<11i', raw, 96, *(ids + (-999,)*(11-len(ids))))
    return bytes(raw)


def nodes() -> bytes:
    """Keep one inactive destination alongside noncanonical nonzero activity markers."""
    rows = [bytearray((i*7+j*19)&255 for i in range(52)) for j in range(6)]
    for row, activity in zip(rows, (1, 0, 255, 1, 2, 1)):
        row[0] = activity
    return b''.join(rows)


def elements(first_family: int = 7) -> bytes:
    """Make first-success resolution, repeated writes, clamping and bypass observably distinct."""
    return b''.join((record(first_family, 2, 1, ids=(4, 5)), record(8, 5, 1),
        record(4, 3, 1, ids=(6, 1, 3)), record(2, 3, 2, ids=(1, 6, 3)),
        record(4, 255, 65535, activity=0), record(4, 1, 65535, ids=(5,)),
        record(3, 2, 0, ids=(4, 1)), record(4, 0, 2, ids=())))


def mask_expected(family: int, mask: int) -> bool:
    """Describe acceptance as native family bit sets rather than engineering enum names."""
    accepted = {4: (1,), 8: (2,16), 2: (4,)}
    return any(mask & bit for bit in accepted.get(family, ()))


def scalar_expected(raw: bytes, mask: int, key: int) -> bool:
    """Keep the caller zero-mask bypass and full signed equality independent of helper rejection."""
    stored = struct.unpack_from('<H', raw, 28)[0]
    return bool(raw[0] and raw[20] == 4 and raw[23] == 10 and
                (not mask or mask_expected(raw[20], mask)) and (not key or stored == key))


def predicate_cases() -> list[tuple[bytes, int, int]]:
    """Cross the inspected gates with unsigned stored keys and signed caller boundaries."""
    domains = ((0,2,4,8,255), (0,1,255), (0,10,11,255), (0,1,2,32768,65535), MASKS, CALLER_KEYS)
    return [(record(f,n,k,a),mask,caller) for f,a,n,k,mask,caller in product(*domains)]


def query_cases() -> list[QueryCase]:
    """Own all mode branches, callback precedence, empty ranges and prior unknown values."""
    cases = []
    domains = ((0,1,2,65535,0x80000000,0xffffffff), (-0x80000000,-1,0,1,2,3,4,5,6,7,0x7fffffff),
               (-32768,-1,0,32767), (None,[None,0,0x80000000,9,None,0xffffffff]))
    for body,mode,callback,initial in product(*domains):
        ranges = ((0,7),(1,2),(100,99),(-1,-2)) if 2 <= mode <= 5 else (None,)
        cases.extend((nodes(),elements(),body,mode,callback,bounds,initial) for bounds in ranges)
    cases.extend((nodes(),elements(f),1,3,-1,(0,7),None) for f in (2,3,4,8,7))
    cases.append((nodes(),elements(),0xffffffff,1,None,(0x7fffffff,0x7fffffff),None))
    cases.append((b'',b'',0,3,-1,(0,-1),None))
    return cases+degenerate_queries()


def degenerate_queries() -> list[QueryCase]:
    """Separate selected/visited records from the first successful active-node mark."""
    empty = [(b'',record(4,0,0,ids=()),0,3,-1,(0,0),None),
             (b'',record(4,1,0,ids=(0,)),0,0,-1,None,None)]
    inactive = b''.join(b'\0'+nodes()[i+1:i+52] for i in range(0,len(nodes()),52))
    empty += [(inactive,elements(),1,3,-1,(0,7),None),(inactive,elements(),0,1,None,None,None)]
    for ids in ((2,2),(0,0)):
        changed = record(7,2,1,ids=ids)+elements()[140:]
        empty.append((nodes(),changed,1,3,-1,(0,7),None))
    return empty


def selected_rows(elements_raw: bytes, body: int, mode: int, bounds: tuple[int,int] | None) -> tuple[list[tuple[int, bytes]], int]:
    """Resolve inclusive selection before building an ordered reference relation."""
    rows = list(enumerate(elements_raw[i:i+140] for i in range(0,len(elements_raw),140)))
    if 2 <= mode <= 5:
        start,end = bounds
        rows = [(i,row) for i,row in rows if start <= i <= end]
    selected = [(i,row) for i,row in rows if row[0] and struct.unpack_from('<H',row,28)[0] == body
                and (2 <= mode <= 5 or row[20] == 4)]
    return selected,len(rows)


def candidates(nodes_raw: bytes, rows: list[tuple[int,bytes]]) -> list[tuple[int,int,int]]:
    """Bind each active destination to its physical element/reference parents, preserving repeats."""
    visits = []
    for element,row in rows:
        assert row[23] <= 11
        refs = struct.unpack_from('<11i',row,96)[:row[23]]
        assert all(0 <= node <= len(nodes_raw)//52 for node in refs)
        visits.extend((node-1,element,slot) for slot,node in enumerate(refs) if node and nodes_raw[(node-1)*52])
    return visits


def independent_query(nodes_raw: bytes, elements_raw: bytes, body: int, mode: int,
    callback: int | None, bounds: tuple[int,int] | None, initial: list[int | None] | None) -> dict[str, object]:
    """Derive mark/history relations independently before comparing the actual native-reader projection."""
    selected,visited = ([],0) if mode == 1 else selected_rows(elements_raw,body,mode,bounds)
    refs = [(i,-1,-1) for i in range(len(nodes_raw)//52) if nodes_raw[i*52]] if mode == 1 else candidates(nodes_raw,selected)
    signed = body if body < 0x80000000 else body-0x100000000
    pattern = body if mode == 1 else max(1,signed)
    output = [None]*(len(nodes_raw)//52) if initial is None else list(initial)
    history = []
    for node,element,slot in refs:
        history.append((int(mode != 1),node,element,slot,output[node],pattern))
        output[node] = pattern
    flag,resolved = expected_flag(mode,callback,refs,elements_raw)
    return dict(output=output,writes=[r[0] for r in refs],history=history,flag=flag,resolved=resolved,
        visited=visited,selected=len(selected),references=sum(row[23] for _,row in selected))


def expected_flag(mode: int, callback: int | None, refs: list[tuple[int,int,int]], elements_raw: bytes) -> tuple[int,bool]:
    """Resolve only the first successful range mark, including an unknown family's terminal -1."""
    if mode == 1:
        return 2,True
    if callback != -1:
        return 16,True
    initial = {0:1,2:2,4:4,5:2,6:8}
    if mode in initial:
        return initial[mode],True
    if mode == 3 and refs:
        family = elements_raw[refs[0][1]*140+20]
        return {4:1,8:2,3:2,2:4}.get(family,-1),True
    return -1,False
