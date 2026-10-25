"""Insert a new CObject definition in a scene list and alias it from floor selection."""
import struct
from early_appearance_patches import convert as convert_appearance
from generate_shaded_vectors import ObjectBuilder
from ui_object_wire import body
import dl_light


def object_capture(original: object, archive: object, spans: list[tuple[int,int,int,object]]) -> object:
    def read(*args: object) -> object:
        start,index = archive.p,len(archive.map)
        result = original(*args)
        spans.append((start,archive.p,index,result))
        return result
    return read


def scene_reader(original: object, version: int, class_name: str, variant: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, kind: int) -> object:
        saved,spans = archive.object,[]
        archive.object = object_capture(saved,archive,spans)
        try:result = original(archive,kind)
        finally:archive.object = saved
        matches = [span for span in spans if isinstance(span[3],dict) and span[3].get("name")=="child name 鋼"]
        assert len(matches)==1 and spans[-1][3] is result["floor_selection"]
        start,end,index,_ = matches[0]
        replacement = ObjectBuilder(index).object(class_name,lambda _:body(version,class_name,variant))
        patches.append((start,end,replacement))
        patches.append((*spans[-1][:2],struct.pack("<H",index+1)))
        return result
    return read


def convert(document: object, path: object, raw: bytes, version: int, edited: bool, patches: list[tuple[int,int,bytes]], class_name: str, variant: int) -> bytes:
    original = dl_light.read_scene
    dl_light.read_scene = scene_reader(original,version,class_name,variant,patches)
    try:return convert_appearance(document,path,raw,version,edited,patches)
    finally:dl_light.read_scene = original


def native_root(document: object, path: object, raw: bytes, class_name: str, variant: int) -> bytes:
    from middle_range_source import displaylists
    from generate_version_range import apply_patches
    patches: list[tuple[int,int,bytes]] = []
    original = dl_light.read_scene
    dl_light.read_scene = scene_reader(original,19000,class_name,variant,patches)
    try:
        archive = displaylists.archive_for(document,str(path),raw)
        displaylists.display_lists(archive,archive.doc_kind)
        assert archive.p==len(raw)
    finally:dl_light.read_scene = original
    return apply_patches(raw,patches)
