"""Independent raw-name and memory-workspace cases; no stored bits are pointers (#121)."""
from itertools import product
import struct

COUNTS=(-0x80000000,-1,0,1,17,0x1fffffff,0x20000000,0x7fffffff)
IDENTIFIERS=(0,1,2,7,0x7fffffff,0x80000000,0xffffffff)
LOGICAL=(1,2,7,0x7fffffff)
RESULTS=(None,0,1,0x123456789abcdef0,0xffffffffffffffff)


def raw_units(units: tuple[int,...]) -> bytes:
    """Encode literal u16 patterns without normalizing pairs or replacing surrogates."""
    return b''.join(struct.pack('<H',unit) for unit in units)


def row(stride: int,identifier: int,count: int,units: tuple[int,...],seed: int=37) -> bytes:
    """Keep every unknown byte distinctive around independently placed stored fields."""
    assert len(units)==128
    raw=bytearray((i*23+seed)&255 for i in range(stride))
    struct.pack_into('<Ii',raw,0,identifier,count)
    start=20 if stride==284 else 24
    struct.pack_into('<2i',raw,start-8,2,5)
    raw[start:start+256]=raw_units(units)
    struct.pack_into('<2I',raw,start+256,0xdeadbeef,0x01234567)
    return bytes(raw)


def name_buffers() -> list[tuple[int,...]]:
    """Exercise ignored unit 128, residual data, spaces and deliberately split surrogate pairs."""
    return [(0,)*128,(65,0)+(0xabcd,)*126,(0x4e2d,0xd83d,0xde00,32,0)+(0xfeed,)*123,
        (65,)*127+(0,),(65,)*127+(90,),(0xd800,0xdc00,0xdfff,32,0)+(0xbeef,)*123,
        (65,)*126+(0xd800,0xdc00),(65,32,32,0)+(0xfffd,)*124]


def caller_names() -> list[tuple[int,...]]:
    """Own code-unit limits rather than Unicode code-point limits or MFC trimming policy."""
    return [(),(65,),(65,0,66),(0,65),(65,)*126,(65,)*127,(65,)*128,
        (65,)*126+(0,66),(65,)*126+(0xd800,0xdc00),(0x4e2d,0xd83d,0xde00,32,32),(0xd800,),(0xdc00,)]


def name_cases() -> list[bytes]:
    """Vary signed metadata while holding only the documented row framing fixed."""
    buffers=name_buffers()
    return [row(stride,IDENTIFIERS[index%len(IDENTIFIERS)],COUNTS[index],units)
            for stride in (284,288) for index,units in enumerate(buffers)]


def name_golden(raw: bytes,caller: tuple[int,...]) -> tuple[bytes,int]:
    """Patch only the copied prefix and its terminator, retaining the entire supplied tail."""
    limit=min(127,len(caller))
    stop=next((i for i in range(limit) if caller[i]==0),limit)
    out=bytearray(raw)
    start=20 if len(raw)==284 else 24
    out[start:start+2*(stop+1)]=raw_units(caller[:stop])+b'\0\0'
    return bytes(out),stop


def workspace_cases() -> list[tuple[bytes,bytes]]:
    """Use explicit nonzero prior images for every legacy conversion and all count/ID boundaries."""
    result=[]
    for stride,identifier,count in product((284,288),IDENTIFIERS,COUNTS):
        raw=row(stride,identifier,count,(65,0)+(0xabcd,)*126,seed=(count&255)+13)
        prior=bytes((i*11+97+(identifier&255))&255 for i in range(288))
        result.append((raw,prior))
    return result


def normalized_golden(raw: bytes,initial: bytes) -> bytes:
    """Copy only reviewed spans; the legacy discarded word never supplies the retained slot."""
    if len(raw)==288:
        return raw
    output=bytearray(initial)
    output[:8]=raw[:8]
    output[16:24]=raw[12:20]
    output[24:288]=raw[20:284]
    return bytes(output)


def plan_golden(raw: bytes,logical: int) -> tuple[bool,int,int | None]:
    """Separate equality selection from signed count consumption and the zero-byte call."""
    identifier,count=struct.unpack_from('<ii',raw)
    selected=identifier==logical
    if selected and not 0<=count<=0x1fffffff:
        raise ValueError('selected count outside signed read-width profile')
    return selected,count,count*4 if selected else None


def binding_golden(raw: bytes,logical: int,result: int | None) -> bytes:
    """Replace only eight numeric bytes after an explicit result, never allocate or dereference."""
    selected,_,_=plan_golden(raw,logical)
    if not selected:
        return raw
    if result is None:
        raise ValueError('selected row requires explicit result bits even at zero length')
    output=bytearray(raw)
    struct.pack_into('<Q',output,8,result)
    return bytes(output)
