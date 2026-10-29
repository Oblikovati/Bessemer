"""Independent complete pattern-extension bodies and signed-count branches (#54)."""
import struct
from edge_fixture_bytes import EdgeGraph,WORDS
from fillet_fixture_bytes import body
import pattern_fixture_bytes as pattern
from freeze_node_units import node_wire
from owned_name_edit import short_wide_name
from member_reference_bytes import profile_body,MemberFixtureOptions
from freeze_model_references import wrapper
from freeze_views import transform

CLASSES=('moTablePattern_c','moSketchPattern_c','moDimPattern_c','moDimPatternFeatInstance_c','moSketchPatternSurfIdRep_c','moAllRangeDim_c')
VERSIONS=(0x27cc,0x2f86,0x2f87,0x2f93,0x2f94,0x3745,0x3746,0x42ce,0x42cf,0x42db,0x42dc,19000,0xffffffff)
COUNTS=(0,-1,2,1,-2147483648,3)
FLAGS=(0,0x800,0xfffff7ff,0xffffffff,0,0x800)


def count_words(style: int) -> bytes:
    """Retain negative declarations with zero iterations; e.g. count_words(1)."""
    count=COUNTS[style]
    return struct.pack('<i',count)+b''.join(struct.pack('<i',WORDS[i]) for i in range(max(0,count)))


def variation(graph: EdgeGraph, advanced: bool) -> bytes:
    """Insert advanced indices at their native position before originals."""
    rich=graph.style in (3,5)
    raw=struct.pack('<i',2 if rich else 0)+(pattern.increment(True)*2 if rich else b'')+b'\xff\xfe'
    if advanced:raw+=count_words(graph.style)
    for populated in (rich,False):raw+=struct.pack('<i',int(populated))+(pattern.original(True) if populated else b'')
    return raw+pattern.original(rich)*2


def common(graph: EdgeGraph, owner: int, advanced: bool, edited: bool) -> bytes:
    """Patch only the containing node flags using its independently known prefix."""
    v=graph.version;rich=graph.style in (3,5);raw=body(graph.mapping,v,rich,owner)
    node=node_wire(v);suffix=struct.pack('<I',0xfffffffe)+short_wide_name('tail')
    assert raw.startswith(node) and node.endswith(suffix)
    offset=len(node)-len(suffix)
    raw=raw[:offset]+struct.pack('<I',FLAGS[graph.style])+raw[offset+4:]
    raw+=pattern.header(graph.mapping,v,rich)+pattern.collections(rich)+pattern.shape(graph.style%5,rich)+b'\xff\xfe'
    return raw+variation(graph,advanced)+pattern.tail(v,rich,12345 if edited else WORDS[graph.style])


def transforms(style: int) -> bytes:
    """Use raw payload words, including nonzero bits in perspective positions."""
    width=(0,120,240,120,120,360)[style];count=(0,0,2,1,0,2)[style]
    if width==0:return bytes(4)
    return struct.pack('<4I',width,0xffffffff,7,count)+b''.join(struct.pack('<I',(0xfedcba98+i)&0xffffffff) for i in range(count*width//4))


def sketch_payload(graph: EdgeGraph) -> bytes:
    """Keep actual profile bodies, bulk arrays and containing-node map branches."""
    profile=bytes(2) if graph.style in (0,4) else graph.shared('outer-profile-wrapper','moProfileRef_w',lambda:profile_body(graph.mapping,graph.version,graph.ids,MemberFixtureOptions(rich=graph.style in (3,5))))
    raw=profile+graph.reference()+struct.pack('<3d',-0.0,1.25,-2.5)+transforms(graph.style)
    if not FLAGS[graph.style]&0x800:return raw
    count=COUNTS[graph.style]
    return raw+struct.pack('<2i',WORDS[graph.style],count)+b''.join(struct.pack('<2i',i,-i-1) for i in range(max(0,count)))


def feature_status(graph: EdgeGraph, index: int) -> bytes:
    """Exercise both conditional strings and shared feature lists."""
    string_flag=index%2;feature_flag=(index+graph.style)%2
    raw=struct.pack('<2i',WORDS[index],string_flag)
    if string_flag==0:raw+=short_wide_name('Ω regeneration')
    raw+=struct.pack('<i',feature_flag)
    return raw+(graph.listing() if feature_flag==0 else b'')


def statuses(graph: EdgeGraph) -> bytes:
    """Preserve independent signed counts at both regeneration levels."""
    count=COUNTS[graph.style];raw=struct.pack('<i',count)
    for index in range(max(0,count)):
        flag=index%2;raw+=struct.pack('<3i',WORDS[index],-1,flag)
        if flag==0:raw+=graph.listing()
        raw+=struct.pack('<i',count)+b''.join(feature_status(graph,i) for i in range(max(0,count)))
    return raw


def dimension_payload(graph: EdgeGraph, edited: bool) -> bytes:
    """Share complete display targets without inventing regeneration semantics."""
    raw=b''.join(graph.display() for _ in range(4))+b'\xff'+graph.display()+graph.display(True)
    raw+=graph.marker()+graph.marker()+struct.pack('<i',12345 if edited else WORDS[graph.style])+graph.listing()+b'\xfe'+struct.pack('<i',-1)
    raw+=statuses(graph)*2
    count=COUNTS[graph.style]
    return raw+struct.pack('<i',count)+b''.join(graph.marker() for _ in range(max(0,count)))


def contents(graph: EdgeGraph, kind: int, owner: int, edited: bool) -> bytes:
    """Compose each entire documented class; e.g. contents(graph,0,2,False)."""
    if kind==4:return struct.pack('<3Hi',owner,0,owner,12345 if edited else WORDS[graph.style])
    if kind==5:return graph.display()+bytes([123 if edited else graph.style*41])
    if kind==3:
        raw=body(graph.mapping,graph.version,graph.style in (3,5),owner)+struct.pack('<i',12345 if edited else WORDS[graph.style])
        parent=bytes(2) if graph.style in (0,4) else graph.shared('parent',CLASSES[2],lambda:contents(graph,2,graph.ids['parent'],False))
        return raw+parent+struct.pack('<3i',-1,2147483647,-2147483648)
    raw=common(graph,owner,kind==2,edited and kind!=2)
    if kind==1:return raw+sketch_payload(graph)
    if kind==2:return raw+dimension_payload(graph,edited)
    coordinate=bytes(2) if graph.style in (0,4) else graph.shared('coordinate','moCoordSysRef_w',lambda:wrapper(graph.version,False)+transform(False,7))
    array=struct.pack('<i',0 if graph.style==0 else -1)
    if graph.style!=0:array+=graph.array()
    return raw+coordinate+graph.reference()+array+struct.pack('<3d',-0.0,1.25,-2.5)


def pattern_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Reserve real root identity before any inherited target; e.g. pattern_object(0,19000,2)."""
    graph=EdgeGraph(version,style,first_slot)
    raw=graph.shared('root',CLASSES[kind],lambda:contents(graph,kind,graph.ids['root'],edited))
    return raw,graph.mapping.slot+1
