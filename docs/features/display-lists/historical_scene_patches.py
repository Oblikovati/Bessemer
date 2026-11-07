"""Reference-guided removal of scene and component fields absent in 6000-range roots."""
from early_view_patches import convert as convert_views
from generate_transform_vectors import compact
from lower_range_patches import string_capture
import dl_light
import dl_tess
from mfcobjects import ObjectArchive


def scene_reader(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, kind: int) -> object:
        result = original(archive,kind)
        end = archive.p-20
        sizes = [(0x186c,len(compact(0,0x40))),(0x185e,4),(0x1841,8),(0x1827,12)]
        for threshold,size in sizes:
            if version<=threshold:patches.append((end-size,end,b""))
            end -= size
        return result
    return read


def tail_reader(original: object, version: int, threshold: int, size: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, *args: object) -> object:
        result = original(archive,*args)
        if version<=threshold:patches.append((archive.p-size,archive.p,b""))
        return result
    return read


def circle_reader(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        result = original(archive)
        end = archive.p
        if version<0x1842:patches.append((end-8,end,b""))
        if version<=0x1827:patches.append((end-28,end-8,b""))
        return result
    return read


def component_reader(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        saved,spans = archive.cstring,[]
        archive.cstring = string_capture(saved,archive,spans)
        try:result = original(archive)
        finally:archive.cstring = saved
        end = spans[-7][1]
        for offset,threshold in [(12,0x17f3),(16,0x17f5),(20,0x183d)]:
            if version<=threshold:patches.append((end+offset,end+offset+4,b""))
        return result
    return read


def convert(document: object, path: object, raw: bytes, version: int, edited: bool, patches: list[tuple[int,int,bytes]]) -> bytes:
    bindings = [(dl_light,"read_scene"),(dl_light,"read_swCircleWall_c"),(dl_light,"read_swSphereWall_c"),(dl_tess,"read_moViewerComponentData")]
    originals = [getattr(module,name) for module,name in bindings]
    readers = [scene_reader(originals[0],version,patches),circle_reader(originals[1],version,patches),tail_reader(originals[2],version,0x1827,32,patches),component_reader(originals[3],version,patches)]
    original_pw = ObjectArchive.readers["pwSceneData_c"]
    ObjectArchive.readers["pwSceneData_c"] = tail_reader(original_pw,version,0x17dc,4,patches)
    original_walls = dl_light.WALL_TYPES.copy()
    dl_light.WALL_TYPES[2] = (original_walls[2][0],readers[1])
    dl_light.WALL_TYPES[3] = (original_walls[3][0],readers[2])
    try:
        for (module,name),reader in zip(bindings,readers,strict=True):setattr(module,name,reader)
        return convert_views(document,path,raw,version,edited,patches)
    finally:
        dl_light.WALL_TYPES = original_walls
        ObjectArchive.readers["pwSceneData_c"] = original_pw
        for (module,name),reader in zip(bindings,originals,strict=True):setattr(module,name,reader)
