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
    middle_range_patches.apply_patches = lambda raw,patches:apply_scene_omissions(raw,patches,spans,apply)
    environment = ObjectArchive.readers["uiUserModelEnv_c"]
    scene = dl_light.read_scene
    ObjectArchive.readers["uiUserModelEnv_c"] = environment_reader(environment,version,patches)
    dl_light.read_scene = scene_reader(scene,version,spans)
    try:return convert_visuals(document,path,raw,version,edited,patches)
    finally:
        ObjectArchive.readers["uiUserModelEnv_c"] = environment
        dl_light.read_scene = scene
        middle_range_patches.apply_patches = apply


def apply_scene_omissions(raw: bytes, patches: list[tuple[int,int,bytes]], spans: list[tuple[int,int]], apply: object) -> bytes:
    # Nested scene conversions have no wire representation when the whole scene is absent.
    kept = [patch for patch in patches if not any(start<=patch[0] and patch[1]<=end for start,end in spans)]
    return apply(raw,kept+[(start,end,b"") for start,end in spans])
