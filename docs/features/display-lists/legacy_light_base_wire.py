"""Independent old-light bases and derived payloads inside complete archives."""
import struct
from generate_transform_vectors import doubles
from generate_component_vectors import integers
from generate_named_vectors import cstring
from legacy_edge_roots import span_reader
import dl_light


def old_light_base_bytes(version: int, result: dict[str,object], properties: bytes, edited: bool) -> bytes:
    ambient = struct.pack("<d",23.0 if edited else result["ambient"])
    diffuse_specular = struct.pack("<dd",result["diffuse"],result["specular"])
    if version<=0x427:
        raw = ambient+struct.pack("<d",-0.0)+diffuse_specular+doubles(2,4)+doubles(9)
    else:
        raw = ambient+diffuse_specular
    raw += struct.pack("<I",result["color"])+integers(result["suppressed"])
    if version>0x427:
        raw += integers(result["in_model_space"],result["unknown_364"])
        if version>=0x452:raw += integers(result["use_cart_display"])
    raw += cstring(result["id_name"],True)
    if version<0x407 or version>0x453:raw += cstring(result["user_name"],True)
    return raw+properties


def old_light_base_reader(original: object, version: int, edited: bool, ends: dict[int,int], patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, old_lights: bool = True) -> object:
        first,before = archive.p,len(archive.map)
        saved,properties = dl_light.read_moUserProperty_c,[]
        dl_light.read_moUserProperty_c = span_reader(saved,properties)
        try:result = original(archive,old_lights=old_lights)
        finally:dl_light.read_moUserProperty_c = saved
        assert old_lights and len(properties)==1 and before==len(archive.map)
        start,last = properties[0]
        ends[first] = archive.p
        payload = old_light_base_bytes(version,result,archive.b[start:last],edited)
        patches.append((first,archive.p,payload))
        return result
    return read


def old_light_derived_reader(original: object, name: str, version: int, edited: bool, ends: dict[int,int], patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, schema: int) -> object:
        first = archive.p
        result = original(archive,schema)
        start,last = ends[first],archive.p
        size = 96 if name=="moSpotLight_c" else 24
        assert last-start==size,(name,last-start)
        length = size
        if version<=0x427:length = 0
        elif name=="moSpotLight_c" and version<0x43f:length = 56
        payload = archive.b[start:start+length]
        if edited and payload:payload = struct.pack("<d",23.0)+payload[8:]
        patches.append((start,last,payload))
        return result
    return read
