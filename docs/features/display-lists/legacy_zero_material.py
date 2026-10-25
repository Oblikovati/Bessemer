"""Independently observed low-version light-name, material and colour omissions."""
import struct


def zero_light_reader(original: object, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, old_lights: bool = True) -> object:
        first,before = archive.p,len(archive.map)
        result = original(archive,old_lights=old_lights)
        assert old_lights and before==len(archive.map) and archive.p>first+128
        assert struct.unpack_from("<Ii",archive.b,first+120)==(result["color"],result["suppressed"])
        patches.append((first+128,archive.p,b""))
        return result
    return read


def material_double_reader(original: object, archive: object, spans: list[tuple[int,int]]) -> object:
    def read() -> float:
        first = archive.p
        result = original()
        spans.append((first,archive.p))
        return result
    return read


def zero_material_reader(original: object, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        saved,spans = archive.f64,[]
        archive.f64 = material_double_reader(saved,archive,spans)
        try:result = original(archive)
        finally:archive.f64 = saved
        assert len(spans)==6
        first,last = spans[4][0],spans[5][1]
        assert archive.b[first:last]==struct.pack("<dd",result["transparency"],result["emission"])
        patches.append((first,last,b""))
        return result
    return read


def zero_visual_reader(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, schema: int) -> object:
        first = archive.p
        result = original(archive,schema)
        for index,(threshold,name) in enumerate([(144,"color"),(211,"wire_color"),(172,"hlr_color")]):
            position = first+index*4
            assert struct.unpack_from("<I",archive.b,position)[0]==result[name]
            if version<=threshold:patches.append((position,position+4,b""))
        return result
    return read
