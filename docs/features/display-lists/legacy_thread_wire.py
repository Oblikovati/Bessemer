"""Owned populated buckets before and after thread-pattern storage began."""
import struct
from legacy_width_wire import width_root
from generate_legacy_panes import physical_reference
from earlier_bucket_wire import polygon_encoder
from generate_shaded_vectors import ObjectBuilder
from generate_component_vectors import integers
from generate_named_vectors import cstring
from generate_version_range import apply_patches
from mfcobjects import ObjectArchive
import legacy_bucket_wire as wire
import bucket_vector_wire as polygon_wire
import dl_doc


def thread_poly(version: int, dimensions: int, count: int = 1, kind: int = 1, line: bool = True, smooth: int = -17) -> bytes:
    original = polygon_wire.line_property
    polygon_wire.line_property = legacy_line_property
    try:raw = polygon_encoder(ObjectBuilder(1),version)(count,dimensions,kind,False,line)
    finally:polygon_wire.line_property = original
    raw = raw[:8]+integers(smooth)+raw[12:]
    return raw[:-4] if version<=0xad2 and count>0 else raw


def legacy_line_property() -> bytes:
    style = cstring("line 鋼",True)
    width = integers(-2147483648)
    return style+width+integers(-4,5)


def thread_map_reader(original: object) -> object:
    def encode(builder: ObjectBuilder, version: int, identity: int, empty: bool) -> bytes:
        if version<=0xad2:return b""
        return original(builder,version,identity,empty)
    return encode


def bucket_body_reader(original: object) -> object:
    def encode(builder: ObjectBuilder, version: int, identity: int, two_dimensional: bool, empty: bool, edited: bool) -> bytes:
        raw = original(builder,version,identity,two_dimensional,empty,edited)
        halo = wire.poly_list([] if empty else [thread_poly(version,3,kind=9,line=False,smooth=0)])
        assert raw.endswith(halo) and raw[-len(halo)-4:-len(halo)]==integers(-5)
        return raw[:-len(halo)-4]
    return encode


def view_body_reader(original: object) -> object:
    def encode(builder: ObjectBuilder, version: int, identity: int, two_dimensional: bool, empty: bool, edited: bool) -> bytes:
        raw = original(builder,version,identity,two_dimensional,empty,edited)
        assert raw[-2:]==bytes(2)
        return raw[:-2]
    return encode


def populated_bucket(version: int, first_index: int, scenario: int) -> tuple[bytes,int]:
    names = ["poly","bucket","view_bucket","thread_maps","LIST_ORDER"]
    originals = [getattr(wire,name) for name in names]
    wire.poly,wire.bucket = thread_poly,bucket_body_reader(originals[1])
    wire.view_bucket,wire.thread_maps = view_body_reader(originals[2]),thread_map_reader(originals[3])
    wire.LIST_ORDER = [name for name in originals[4] if name not in {"weldbead_v","weldbead_h"}]
    builder = ObjectBuilder(first_index)
    try:
        raw = builder.object("uiViewBucket_c",lambda state:wire.view_bucket(state,version,first_index+1,scenario%32>=16,scenario%16==2,scenario%16==1))
        return raw,builder.next_index-first_index
    finally:
        for name,original in zip(names,originals,strict=True):setattr(wire,name,original)


def tag_reader(original: object, tags: list[tuple[int,int]]) -> object:
    def read(archive: object, expected: object = None) -> object:
        start = archive.p
        tag, = struct.unpack_from("<H",archive.b,start)
        result = original(archive,expected)
        tags.append((start,tag))
        return result
    return read


def bucket_slot(raw: bytes, kind: str, identities: tuple[str,...]) -> tuple[tuple[int,int,int],list[tuple[int,int]]]:
    original,slots = dl_doc.read_display_item_list,[]
    object_reader,tags = ObjectArchive.object,[]
    def capture(archive: object) -> object:
        slots.append((archive.p-2,archive.p,len(archive.map)))
        return original(archive)
    dl_doc.read_display_item_list = capture
    ObjectArchive.object = tag_reader(object_reader,tags)
    try:physical_reference(raw,2771,kind,identities)
    finally:dl_doc.read_display_item_list,ObjectArchive.object = original,object_reader
    assert len(slots)==1
    first,last,_ = slots[0]
    assert raw[first:last]==bytes(2),"view bucket must be null; expected a replaceable owned pointer"
    return slots[0],tags


def relocated_tags(end: int, first_index: int, added: int, tags: list[tuple[int,int]]) -> list[tuple[int,int,bytes]]:
    patches = []
    for position,tag in tags:
        if position<end or tag in (0,0xffff):continue
        assert tag!=0x7fff,f"tag {tag:#x}; expected short tags in these owned roots"
        index = tag & 0x7fff
        if index<first_index:continue
        assert index+added<0x7fff,f"index {index}+{added}; expected a short reference"
        patches.append((position,position+2,struct.pack("<H",(tag & 0x8000)|(index+added))))
    return patches


def thread_root(version: int, kind: str, scenario: int) -> tuple[bytes,tuple[str,...]]:
    if scenario<32:return width_root(version,kind,scenario)
    base = (3 if kind=="assembly" else 0)+int(scenario%16==1) if scenario>=64 else 2
    raw,identities = width_root(version,kind,base)
    (first,last,index),tags = bucket_slot(raw,kind,identities)
    replacement,added = populated_bucket(version,index,scenario)
    patches = [(first,last,replacement)]+relocated_tags(last,index,added,tags)
    return apply_patches(raw,patches),identities


def thread_cached_root(version: int, kind: str, scenario: int) -> tuple[bytes,tuple[str,...]]:
    assert kind=="assembly" and scenario in (3,4),(kind,scenario)
    target = 64+int(scenario==4)
    return thread_root(version,kind,target)
