"""Independent wire assembly for ordinary dimension and handle fixtures."""
import struct
from freeze_component_references import base as component_base
from freeze_node_units import node_wire
from freeze_views import object_tag, transform
from owned_name_edit import short_wide_name


def tolerance(version: int) -> bytes:
    out = struct.pack('<ii2dB3d', 2, -3, .1, -.2, 255, 1.5, -0.0, 3.5)
    out += b'\x01U\x01L'+struct.pack('<BiB3dBBi', 254, -4, 253, 4.5, -0.0, 6.5, 252, 251, -5)
    if version > 0x2b6d:
        out += b'\x01u\x01l\x01g'
    if version > 0x2b76:
        out += struct.pack('<i', -6)
    return object_tag('moDimTolerance_c')+out


def attachment() -> bytes:
    return object_tag('moAttachPoint_c')+struct.pack('<6diH6dB', -0.0, 1, 2, 3, 4, 5, -7, 0, 6, 7, 8, 9, 10, 11, 255)


def integer_parameter(version: int, present: int) -> bytes:
    out = object_tag('moIntegerParameter_c')+node_wire(version)+b'\0\0'
    out += struct.pack('<iB', -2**31, present)
    return out+(struct.pack('<i', 2**31-1) if present else b'')


def dimension_base(version: int, rich: bool, reference_count: int = -7) -> bytes:
    out = struct.pack('<i', reference_count)
    out += integer_parameter(version, 255)+tolerance(version) if rich else bytes(4)
    out += struct.pack('<4i', 19, 1, -2, -3)
    out += attachment() if rich else b'\0\0'
    out += struct.pack('<H', 65535)
    if version > 0x42ee:
        out += b'\xff'+short_wide_name('Ω dimension') if rich else b'\0'
    return out


def dimension(name: str, version: int, flags: int) -> bytes:
    out = dimension_base(version, bool(flags & 1))
    if name == 'Dimension_c':
        return object_tag(name)+out
    out += struct.pack('<9dH', -0.0, 1, 2, 3, 4, 5, 6, 7, 8, 65535 if flags & 2 else 0)
    if flags & 2:
        out += struct.pack('<6d', -0.0, 10, 11, 12, 13, 14)
    out += struct.pack('<H', 65534 if flags & 4 else 0)+(transform(True, 0x1a) if flags & 4 else b'')
    out += struct.pack('<H', 0x8000 | ((flags >> 3) & 3))
    for bit in [8, 16]:
        if flags & bit:
            out += struct.pack('<3d', -0.0, bit, -bit)
    if name == 'AngleDim_c':
        out += struct.pack('<H', 65530)
    if name == 'moRevolvedRadiusDim_c':
        out += b'\0\0'+struct.pack('<i', -23)
    return object_tag(name)+out


def handle_base(version: int, external: bool, flags: int) -> bytes:
    out = component_base(version, False)+struct.pack('<H', 65535 if flags & 1 else 0)
    if flags & 1:
        out += transform(False, 0x2b)
    out += bytes(8)+struct.pack('<BH', 255, 1 if external else 0)
    out += dimension('Dimension_c', version, 0) if flags & 2 else b'\0\0'
    if flags & 2:
        out += transform(True, 7)+struct.pack('<i', -1)+short_wide_name('Ω handle')
    out += struct.pack('<iH', -29, 65534 if flags & 4 else 0)
    if flags & 4:
        out += transform(True, 0x1a)
    return out+struct.pack('<i', -31)


def handle(name: str, version: int, external: bool, flags: int) -> bytes:
    out = object_tag(name)+handle_base(version, external, flags)
    if name in ('moFeatureDimHandle_c', 'moRefDimHandle_c'):
        return out if external else out+dimension('AngleDim_c', version, 0)
    return out+(struct.pack('<H', 65535) if external else b'\x01\0')+struct.pack('<i', -37)
