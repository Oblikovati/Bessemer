"""Complete roots with ordinary text widths physically absent at their boundary."""
import struct
from resumed_inline_wire import resumed_root
from legacy_edge_reference import edge_reference
from legacy_edge_roots import span_reader
from generate_version_range import apply_patches
from generate_item_vectors import record
from generate_component_vectors import integers
from generate_transform_vectors import doubles
from generate_named_vectors import cstring
import dl_doc
import dl_light


def ordinary_text(version: int, kind: int, enabled: bool, edited: bool) -> bytes:
    text = ("B" if edited else "A")+"😀e\u0301鋼" if enabled else ""
    raw = doubles(5)+integers(-1,2)+cstring(text,True)+integers(-3)+doubles(2)
    if version>0xa03:
        raw += struct.pack("<h",-2 if enabled else 0)
        if enabled:raw += integers(-1+int(edited),-2147483648,2147483647,0,1073741824)
    return record(kind,raw)


def ordinary_root(version: int, kind: str, scenario: int) -> tuple[bytes,tuple[str,...]]:
    raw,identities = classified_root(version,kind,scenario)
    original,spans = dl_doc.read_display_item_list,[]
    dl_doc.read_display_item_list = span_reader(original,spans)
    try:edge_reference(raw,max(version,2564),kind,identities)
    finally:dl_doc.read_display_item_list = original
    assert spans,spans
    first,last = spans[-1]
    count, = struct.unpack_from("<H",raw,first)
    edited = scenario==4 or scenario%16==1
    extra = b"".join(ordinary_text(version,t,e,edited) for t in (7,15) for e in (False,True))
    replacement = struct.pack("<H",count+4)+raw[first+2:last]+extra
    return apply_patches(raw,[(first,last,replacement)]),identities


def material_reader(original: object, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        saved_string,saved_properties = archive.cstring,dl_light.read_moUserProperty_c
        strings,boundaries = [],[]
        archive.cstring = string_reader(archive,saved_string,strings)
        dl_light.read_moUserProperty_c = span_reader(saved_properties,boundaries)
        try:result = original(archive)
        finally:archive.cstring,dl_light.read_moUserProperty_c = saved_string,saved_properties
        first,last = [span for span in strings if span[1]<=boundaries[0][0]][-1]
        patches.append((first,last,b""))
        return result
    return read


def classified_root(version: int, kind: str, scenario: int) -> tuple[bytes,tuple[str,...]]:
    raw,identities = resumed_root(2596,kind,scenario)
    if version>0xa05:return raw,identities
    original,patches = dl_light.read_uoMaterialProperties,[]
    dl_light.read_uoMaterialProperties = material_reader(original,patches)
    try:edge_reference(raw,2596,kind,identities)
    finally:dl_light.read_uoMaterialProperties = original
    return apply_patches(raw,patches),identities


def string_reader(archive: object, original: object, spans: list[tuple[int,int]]) -> object:
    def read() -> str:
        first = archive.p
        result = original()
        spans.append((first,archive.p))
        return result
    return read
