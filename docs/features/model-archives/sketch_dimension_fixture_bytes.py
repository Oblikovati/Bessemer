"""Independent tagged and direct sketch-dimension body assembly."""
import struct
from dimension_fixture_bytes import dimension_base
from freeze_views import object_tag, transform
from sketch_reference_fixture_bytes import handle

G1 = ['sgValG1Dim', 'sgCircleDim']
G2 = ['sgValG2Dim', 'sgLLDist', 'sgPntPntDist', 'sgPntLineDist', 'sgPntPntHorDist', 'sgPntPntVertDist', 'sgAnglDim', 'sgPoint3DPlaneDist']
OFFSETS = ['sgOffsetDim', 'sgSkOffsetDim']


def base(version: int) -> bytes:
    return dimension_base(version, False)+struct.pack('<i3H', -1, 65535, 65534, 65533)


def valued(version: int, rich: bool) -> bytes:
    return base(version)+struct.pack('<H', 65535 if rich else 0)+(transform(True, 0x1a) if rich else b'')


def entities(count: int, rich: bool, base_index: int) -> bytes:
    if not rich: return bytes(2*count)
    return handle('sgPointHandle', -1)+struct.pack('<'+'H'*(count-1), *([base_index+2]*(count-1)))


def valued_body(name: str, version: int, rich: bool, base_index: int = 2) -> bytes:
    out = valued(version, rich)
    if name in G1: return out+entities(2, rich, base_index)+struct.pack('<2i3d', -2**31, 2**31-1, -0.0, 1.25, -2.5)
    if name in OFFSETS: return out+struct.pack('<HH', 65532, 65531)
    if name == 'sgValG3Dim': return out+entities(3, rich, base_index)+struct.pack('<H', 65530)
    out += entities(2, rich, base_index)+struct.pack('<4H', 65529, 65528, 32768, 0)
    out += handle('sg3DPlaneHandle', 65535) if rich else b'\0\0'
    out += struct.pack('<3d', -0.0, 1.25, -2.5)
    return out+(struct.pack('<H', 65527) if 0x2f79 < version < 0x2fa0 else b'')


def logical(version: int) -> bytes:
    return base(version)+struct.pack('<HH', 65535, 65534)


def help_array(base_index: int) -> bytes:
    out = object_tag('su_CObArray')+struct.pack('<HI', 65535, 2)
    out += object_tag('sgDimEntityHelpData_c')+struct.pack('<2dH', -0.0, 1.25, 65535)
    return out+struct.pack('<H', base_index+6)


def logical_body(kind: int, version: int, rich: bool, base_index: int) -> bytes:
    out = logical(version)+entities(kind, rich, base_index)
    if kind == 1: return out
    if kind == 3: return out+struct.pack('<3H', base_index+2, 0, base_index+2) if rich else out+bytes(6)
    out += struct.pack('<HH', 65533, 65532)+(help_array(base_index) if rich else b'\0\0')
    out += struct.pack('<H2i2d', 65531, -2**31, 2**31-1, -0.0, 1.25)
    out += struct.pack('<3H', base_index+2, 0, base_index+2) if rich else bytes(6)
    out += struct.pack('<H', 65530)
    return out+(struct.pack('<i', -17) if version > 0x46cb else b'')


def isoparam(version: int, populated: bool) -> bytes:
    out = logical(version)+struct.pack('<4i', -2**31, 2**31-1, -1, 0)
    return out+(struct.pack('<4I3H', 2, 0x89abcdef, 0xffffffff, 3, 0, 32768, 65535) if populated else bytes(4))


def tagged(name: str, version: int, rich: bool) -> bytes:
    if name == 'sgDim': body = base(version)
    elif name == 'sgDimEntityHelpData_c': body = struct.pack('<2dH', -0.0, 1.25, 65535)
    elif name == 'sgLogDim': body = logical(version)
    elif name in ['sgLogG1Dim', 'sgLogG2Dim', 'sgLogG3Dim']: body = logical_body(int(name[6]), version, rich, 2)
    elif name == 'sgIsoParam_c': body = isoparam(version, rich)
    else: body = valued_body(name, version, rich)
    return object_tag(name)+body


def valued_collections(version: int, alias_group: int | None = None) -> bytes:
    names = ['sgCircleDim', 'sgAnglDim', 'sgValG3Dim', 'sgSkOffsetDim']
    if alias_group is None:
        return b''.join(struct.pack('<H', 1)+tagged(name, version, False) for name in names)
    return b''.join(struct.pack('<H', 2)+tagged(name, version, False)+struct.pack('<H', 2) if group == alias_group else bytes(2)
                    for group, name in enumerate(names))


def logical_collections(version: int, rich: bool) -> bytes:
    return b''.join(struct.pack('<H', 1)+logical_body(kind, version, rich if kind == 3 else False, 0) for kind in [1, 2, 3])
