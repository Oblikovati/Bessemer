"""Insert a complete self-referencing transient viewer and relocate later MFC tags."""
import struct
from generate_shaded_vectors import ObjectBuilder
from old_shaded_wire import component
from middle_range_source import displaylists,component_trace
from generate_version_range import apply_patches
import dl_tess


def observed_objects(archive: object, original: object, records: list[tuple[int,int,int]]) -> object:
    def read(expect: object = None) -> object:
        start,index = archive.p,len(archive.map)
        result = original(expect)
        records.append((start,archive.p,index))
        return result
    return read


def viewer_location(original: object, records: list[tuple[int,int,int]], target: list[tuple[int,int,int]]) -> object:
    def read(archive: object) -> object:
        result = original(archive)
        assert result["unknown_obj"] is None
        target.append(records[-1])
        return result
    return read


def locations(document: object, path: object, raw: bytes, version: int) -> tuple[list[tuple[int,int,int]],tuple[int,int,int],int]:
    archive = displaylists.archive_for(document,str(path),raw)
    archive.version,archive.doc_kind,archive.face_tess_comp_ids = version,"assembly",set()
    records,target = [],[]
    archive.object = observed_objects(archive,archive.object,records)
    original = dl_tess.read_moViewerComponentData
    dl_tess.read_moViewerComponentData = viewer_location(original,records,target)
    try:
        root = displaylists.display_lists(archive,"assembly")
    finally:
        dl_tess.read_moViewerComponentData = original
    assert archive.p==len(raw) and len(target)==1
    viz = next(index for index,value in enumerate(archive.map) if value is root["visual_properties"])
    return sorted(records),target[0],viz


def shifted_tag(raw: bytes, start: int, first: int, delta: int) -> bytes | None:
    tag, = struct.unpack_from("<H",raw,start)
    if tag in (0,0xffff):return None
    big = tag==0x7fff
    value = struct.unpack_from("<I",raw,start+2)[0] if big else tag
    mask = 0x80000000 if big else 0x8000
    if value&(mask-1)<first:return None
    adjusted = (value&mask)|((value&(mask-1))+delta)
    return struct.pack("<HI",tag,adjusted) if big else struct.pack("<H",adjusted)


def populate(document: object, path: object, raw: bytes, version: int, edited: bool) -> bytes:
    records,(start,end,index),viz = locations(document,path,raw,version)
    assert raw[start:end]==bytes(2)
    builder = ObjectBuilder(index)
    body = component(version,viz)
    body = body[:-4]+struct.pack("<H",index+1)+struct.pack("<i",23 if edited else 22)
    payload = builder.object("moViewerComponentData_c",lambda _:body)
    patches = [(start,end,payload)]
    for position,_,_ in records:
        if position<end:continue
        replacement = shifted_tag(raw,position,index,builder.next_index-index)
        if replacement is not None:patches.append((position,position+len(replacement),replacement))
    return apply_patches(raw,patches)


def transient_trace(root: dict[str,object]) -> bytes:
    child = root["tessellation"][0]["viewer_comp_data"].get("unknown_obj")
    if child is None:return b""
    shadow = dict(root,tessellation=[dict(root["tessellation"][0],viewer_comp_data=child)])
    prefix = b"root.tessellation.components[0].viewer_comp_data"
    return b"".join(line.replace(prefix,prefix+b".unknown_obj",1)+b"\n" for line in component_trace(shadow,"assembly").splitlines() if line.startswith(prefix))
