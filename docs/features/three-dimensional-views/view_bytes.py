"""Independent complete modern 3D views with direct annotation groups (#54)."""
import struct
from edge_fixture_bytes import EdgeGraph,WORDS,BITS
from freeze_views import transform
from owned_name_edit import short_wide_name
from weld_corner_bytes import counted

VERSIONS=(0x3b07,0x3b08,0x42ee,0x42ef,19000,0xffffffff)
VECTOR_COUNTS=(-1,0,3,2,-2147483648,3)
KEY_COUNTS=(-1,0,2,1,-2147483648,3)


def high_low(count: int) -> bytes:
    """Keep the high word first rather than little-endian u64; e.g. high_low(2)."""
    return struct.pack('<2I',count>>32,count&0xffffffff)


def base(graph: EdgeGraph) -> bytes:
    """Use complete base transforms and only cached/null transient-list entries."""
    s=graph.style;identity=struct.pack('<H',graph.ids['root']);flags=(7,0x1a,0x2b,0x4c,0x8d,7)[s]
    raw=short_wide_name('Ω 3D view')+transform(s%2==1,flags)+transform(s%2==0,flags)
    raw+=struct.pack('<Hidi',65535,-3,-0.0,2)+bytes(4)+struct.pack('<2i',s,-7)
    raw+=struct.pack('<i',0 if s==0 else -1)
    if s!=0:raw+=counted([bytes(2),identity,identity],s==5)
    raw+=struct.pack('<iQi',-11,BITS[s],0 if s in (0,4) else 255)
    if s not in (0,4):raw+=counted([identity,bytes(2)],s==5)
    return raw+struct.pack('<2i',-13,-17)


def annotation_data(style: int) -> bytes:
    """Emit every key before all groups, retaining signed nonpositive counts."""
    count=KEY_COUNTS[style];keys=(-7,-7,2147483647)[:max(0,count)]
    raw=struct.pack('<i',count)+b''.join(struct.pack('<i',key) for key in keys)
    for group in range(max(0,count)):
        items=(2,1,0)[group%3];raw+=high_low(items)
        raw+=b''.join(struct.pack('<4i',-2147483648,2147483647,index,-index-1) for index in range(items))
    return raw


def suffix(style: int, edited: bool) -> bytes:
    """Keep duplicate member values separate and preserve ordered duplicate map keys."""
    raw=struct.pack('<4i',WORDS[style],-1,2147483647,12345 if edited else -2147483648)
    raw+=short_wide_name('Ω annotation')+struct.pack('<2i2I2i',-1,7,0xffffffff,0x80000000,-7,8)
    count=VECTOR_COUNTS[style];raw+=struct.pack('<i',count)
    raw+=b''.join(struct.pack('<i',v) for v in (-1,-1,7)[:max(0,count)])
    raw+=struct.pack('<2Q',BITS[style],BITS[(style+1)%6])
    entries=(0,1,2,2,1,3)[style];raw+=high_low(entries)
    for index in range(entries):raw+=short_wide_name('Ω duplicate')+annotation_data(style)
    return raw+struct.pack('<I',0xfedcba98)


def view_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Reserve the actual root before cached self references; e.g. view_object(0,19000,2)."""
    graph=EdgeGraph(version,style,first_slot)
    raw=graph.shared('root','mo3dView_c',lambda:base(graph)+suffix(style,edited))
    return raw,graph.mapping.slot+1
