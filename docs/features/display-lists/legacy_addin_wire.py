"""Observe body pointers and add-in fields before removing their physical storage."""
import struct
from legacy_visual_wire import visual_root
from generate_legacy_panes import physical_reference
from lower_range_patches import string_capture
from generate_version_range import apply_patches
from generate_named_vectors import cstring
from mfcobjects import ObjectArchive


def addin_reader(original: object, version: int, edited: bool, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, schema: int) -> object:
        saved,spans = archive.cstring,[]
        archive.cstring = string_capture(saved,archive,spans)
        try:result = original(archive,schema)
        finally:archive.cstring = saved
        first,last = spans[-1]
        assert last+4==archive.p and result["addin_identifier"]=="" and result["valid_only_for_addin"]==0
        replacement = cstring("addin 鋼"+(" edited" if edited else ""),True)+struct.pack("<i",-2147483648+int(edited)) if version>0xb2b else b""
        patches.append((first,last+4,replacement))
        return result
    return read


def object_capture(original: object, archive: object, spans: list[tuple[int,int,object]]) -> object:
    def read(*args: object) -> object:
        start = archive.p
        result = original(*args)
        spans.append((start,archive.p,result))
        return result
    return read


def body_reader(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, schema: int) -> object:
        saved,spans = archive.object,[]
        archive.object = object_capture(saved,archive,spans)
        try:result = original(archive,schema)
        finally:archive.object = saved
        first,last,value = spans[-1]
        assert last==archive.p and result["body_prop_infos"][0][1]["viz_prop"] is value
        tag, = struct.unpack_from("<H",archive.b,first)
        # Removing a reference or null keeps every later MFC slot at its original index.
        assert tag<0x7fff and last-first==2,"body property pointer must not allocate an MFC slot"
        if version<0xb4b:patches.append((first,last,b""))
        return result
    return read


def addin_root(version: int, kind: str, scenario: int) -> tuple[bytes,tuple[str,...]]:
    raw,identities = visual_root(version,kind,scenario)
    names,patches = ["moVisualProperties_c","uoTempBodyTessData_c"],[]
    originals = [ObjectArchive.readers[name] for name in names]
    ObjectArchive.readers[names[0]] = addin_reader(originals[0],version,scenario in (1,4),patches)
    ObjectArchive.readers[names[1]] = body_reader(originals[1],version,patches)
    try:physical_reference(raw,2891,kind,identities)
    finally:
        for name,original in zip(names,originals,strict=True):ObjectArchive.readers[name] = original
    assert patches or scenario==2,"observed patches were empty; expected shaded appearances"
    return apply_patches(raw,patches),identities
