"""Complete inline part and assembly payloads at the last pre-object facet version."""
import struct
from resumed_inline_facets import VERSION,facet_bytes
from legacy_edge_roots import edge_root
from legacy_edge_reference import edge_reference
from generate_shaded_vectors import ObjectBuilder
from generate_component_vectors import integers
from generate_version_range import apply_patches
from mfcobjects import ObjectArchive
import dl_tess


def body_payload(builder: ObjectBuilder, edited: bool) -> bytes:
    faces = struct.pack("<H",2)
    for enabled in (False,True):
        payload = facet_bytes(enabled,edited)+bytes(2)
        faces += builder.object("uoTempFaceTessData_c",lambda _,raw=payload:raw)
    return faces+integers(-20,-24,-44)


def part_payload(first_index: int, edited: bool) -> bytes:
    builder = ObjectBuilder(first_index)
    raw = struct.pack("<H",1)+builder.object("uoTempBodyTessData_c",lambda state:body_payload(state,edited))
    return raw+bytes(2)+struct.pack("<I",0xffffffff)+integers(2,-1,2,-3,4)


def part_slot(raw: bytes) -> tuple[int,int,int]:
    original,spans = dl_tess.read_uoTempPartTessData,[]
    def read(archive: object) -> object:
        first,index = archive.p,len(archive.map)
        result = original(archive)
        spans.append((first,archive.p,index))
        return result
    dl_tess.read_uoTempPartTessData = read
    try:edge_reference(raw,2606,"part",())
    finally:dl_tess.read_uoTempPartTessData = original
    assert len(spans)==1 and raw[spans[0][1]:]==bytes(4),spans
    return spans[0]


def face_slots(raw: bytes, identities: tuple[str,...]) -> list[int]:
    original,positions = ObjectArchive.readers["uoTempFaceTessData_c"],[]
    def read(archive: object, schema: int) -> object:
        positions.append(archive.p)
        return original(archive,schema)
    ObjectArchive.readers["uoTempFaceTessData_c"] = read
    try:edge_reference(raw,2606,"assembly",identities)
    finally:ObjectArchive.readers["uoTempFaceTessData_c"] = original
    assert len(positions)==2,positions
    return positions


def inline_root(kind: str, scenario: int) -> tuple[bytes,tuple[str,...]]:
    edited = scenario%16==1
    base = (80 if scenario>=112 else 64)+int(edited)
    raw,identities = edge_root(2606,kind,base)
    if kind=="part":
        first,last,index = part_slot(raw)
        return apply_patches(raw,[(first,last,part_payload(index,edited))]),()
    positions = face_slots(raw,identities)
    patches = [(position,position,facet_bytes(bool(index),edited)) for index,position in enumerate(positions)]
    return apply_patches(raw,patches),()


def resumed_root(version: int, kind: str, scenario: int) -> tuple[bytes,tuple[str,...]]:
    assert VERSION<=version<=2606,(version,scenario)
    if scenario<96:return edge_root(2606,kind,scenario)
    assert version==VERSION,(version,scenario)
    return inline_root(kind,scenario)
