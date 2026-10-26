"""Independent ordinary pattern bodies and explicitly loaded note/count targets."""
import struct
from freeze_views import object_tag
from sketch_reference_fixture_bytes import handle
from freeze_geometric_references import wire as geometric_wire
from sketch_dimension_fixture_bytes import logical_body

LOADED = ['moNotePattern_c', 'sgLinearPattCntDim', 'sgCircularPattCntDim']


def base(loaded: bool) -> bytes:
    return struct.pack('<3iH', -2**31, 2**31-1, -1, 1 if loaded else 0)


def construction_lines(rich: bool, base_index: int) -> bytes:
    return handle('sgLineHandle', -1)+struct.pack('<H', base_index+2) if rich else bytes(4)


def linear(version: int, rich: bool, loaded: bool, base_index: int) -> bytes:
    out = base(loaded)+struct.pack('<6H3d', 0, 1, 2, 32768, 65534, 65535, -0.0, 1.25, -2.5)
    out += construction_lines(rich, base_index)+struct.pack('<3H', 65535, 65534, 65533)
    out += geometric_wire('moLineRef_w', version, False, 0)+struct.pack('<H', base_index+4) if rich else bytes(4)
    return out+struct.pack('<HdH', 2 if loaded else 0, -0.0, 65532)


def circular(version: int, rich: bool, loaded: bool, base_index: int) -> bytes:
    out = base(loaded)+struct.pack('<4H3d', 0, 32768, 65534, 65535, -0.0, 1.25, -2.5)
    out += construction_lines(rich, base_index)+struct.pack('<H', 65535)
    out += handle('sgArcHandle', 65535) if rich else b'\0\0'
    out += struct.pack('<2H', 65534, 32768)+(handle('sgPointHandle', 0x777f) if rich else b'\0\0')
    out += struct.pack('<2H', 65533, 65532)
    out += geometric_wire('moCircleRef_w', version, False, 0) if rich else b'\0\0'
    return out+struct.pack('<H', 3 if loaded else 0)


def constraint(version: int, rich: bool, base_index: int, x_scale: int = -2**31) -> bytes:
    return logical_body(2, version, rich, base_index)+struct.pack('<6H2i', 0, 1, 2, 32768, 65534, 65535, x_scale, 2**31-1)


def tagged(name: str, version: int, rich: bool, loaded: bool) -> bytes:
    base_index = 2+(3 if loaded else 0)
    if name == 'sgLinearPattern': body = linear(version, rich, loaded, base_index)
    elif name == 'sgCircularPattern': body = circular(version, rich, loaded, base_index)
    else: body = constraint(version, rich, base_index)
    return object_tag(name)+body


def following(version: int, rich: bool, loaded: bool) -> bytes:
    out = struct.pack('<H', 1)+constraint(version, False, 0)
    out += struct.pack('<H', 1)+circular(version, rich, loaded, 3 if loaded else 0)
    return out+struct.pack('<H', 1)+constraint(version, False, 0)
