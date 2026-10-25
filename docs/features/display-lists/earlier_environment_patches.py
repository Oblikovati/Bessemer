"""Observe the environment word and terminal scene omitted in earlier roots."""
from earliest_visual_patches import convert as convert_visuals
from mfcobjects import ObjectArchive
import struct
import dl_light
import middle_range_patches


def environment_reader(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, schema: int) -> object:
        result = original(archive,schema)
        if version<=0xf55:
            assert archive.b[archive.p-8:archive.p-4]==struct.pack("<i",result["unknown_1c0"])
            patches.append((archive.p-8,archive.p-4,b""))
        return result
    return read


def scene_reader(original: object, version: int, spans: list[tuple[int,int]]) -> object:
    def read(archive: object, kind: int) -> object:
        start = archive.p-4
        result = original(archive,kind)
        if version<=0xf5e:spans.append((start,archive.p))
        return result
    return read


def convert(document: object, path: object, raw: bytes, version: int, edited: bool, patches: list[tuple[int,int,bytes]]) -> bytes:
    spans = []
    apply = middle_range_patches.apply_patches
    middle_range_patches.apply_patches = lambda raw,patches:apply_absent_records(raw,patches,spans,apply)
    environment = ObjectArchive.readers["uiUserModelEnv_c"]
    scene = dl_light.read_scene
    rv,texture = dl_light.read_uoRVAppearanceProperties,dl_light.read_uoMaterialTextureProperties
    dl_light.read_uoRVAppearanceProperties = rv_reader(rv,version,spans)
    dl_light.read_uoMaterialTextureProperties = texture_reader(texture,version,patches)
    ObjectArchive.readers["uiUserModelEnv_c"] = environment_reader(environment,version,patches)
    dl_light.read_scene = scene_reader(scene,version,spans)
    try:return convert_visuals(document,path,raw,version,edited,patches)
    finally:
        ObjectArchive.readers["uiUserModelEnv_c"] = environment
        dl_light.read_scene = scene
        middle_range_patches.apply_patches = apply
        dl_light.read_uoRVAppearanceProperties,dl_light.read_uoMaterialTextureProperties = rv,texture


def apply_absent_records(raw: bytes, patches: list[tuple[int,int,bytes]], spans: list[tuple[int,int]], apply: object) -> bytes:
    # Nested conversions have no wire representation when their enclosing record is absent.
    spans = maximal_omissions(spans)
    kept = [patch for patch in patches if not any(start<=patch[0] and patch[1]<=end for start,end in spans)]
    return apply(raw,kept+[(start,end,b"") for start,end in spans])


def maximal_omissions(spans: list[tuple[int,int]]) -> list[tuple[int,int]]:
    unique = sorted(set(spans))
    return [(start,end) for start,end in unique if not any(first<=start and end<=last and (first,last)!=(start,end) for first,last in unique)]


def rv_reader(original: object, version: int, spans: list[tuple[int,int]]) -> object:
    def read(archive: object) -> object:
        start = archive.p-4
        result = original(archive)
        if version<=0xf1d:spans.append((start,archive.p))
        return result
    return read


def texture_reader(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        result = original(archive)
        if version<=0xf1d:
            expected = struct.pack("<dd",result["texture_trans_x"],result["texture_trans_y"])
            assert archive.b[archive.p-20:archive.p-4]==expected
            patches.append((archive.p-20,archive.p-4,b""))
        return result
    return read
