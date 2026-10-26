"""Independent sketch handle, backup and entity-reference wire assembly."""
import struct
from freeze_views import object_tag
from freeze_model_references import wrapper
from freeze_component_references import base as component_base, boundary as component_boundary
from freeze_boundary_references import base as boundary_base, wire as boundary_wire
from owned_name_edit import short_wide_name

HANDLES = ['sgEntHandle', 'sgLineHandle', 'sgArcHandle', 'sgPointHandle', 'sgSplineHandle',
           'sgEllipseHandle', 'sgTextHandle', 'sg3DPlaneHandle', 'sg3DSphereHandle']
BACKUPS = ['moPointBackedUpData_c', 'moLineBackedUpData_c', 'moSegmentBackedUpData_c',
           'moArcBackedUpData_c', 'moEllipseBackedUpData_c', 'mo3DPlaneBackedUpData_c', 'mo3DSphereBackedUpData_c']


def handle(name: str, index: int, reference_id: int = -19) -> bytes:
    encoded = struct.pack('<H', index) if 0 <= index <= 65535 and index != 0x777f else struct.pack('<Hi', 0x777f, index)
    return object_tag(name)+encoded+struct.pack('<ii', reference_id, 2**31-1)


def point() -> bytes:
    return object_tag('moPointBackedUpData_c')+struct.pack('<3d', -0.0, 1.25, -2.5)


def backup(name: str, rich: bool) -> bytes:
    if name == 'moPointBackedUpData_c':
        return point()
    if name == 'mo3DPlaneBackedUpData_c':
        return object_tag(name)+struct.pack('<9d', -0.0, 1, 2, 3, 4, 5, 6, 7, 8)
    if name == 'mo3DSphereBackedUpData_c':
        return object_tag(name)+struct.pack('<4d', -0.0, 1.25, -2.5, 3.75)
    line_base = (point()+struct.pack('<H', 4) if rich else bytes(4))+struct.pack('<ii', -3, 5)
    if name in ('moLineBackedUpData_c', 'moSegmentBackedUpData_c'):
        return object_tag(name)+line_base
    out = object_tag(name)+line_base+struct.pack('<i', -7)
    if name == 'moEllipseBackedUpData_c':
        return out+struct.pack('<5H', *([4]*5 if rich else [0]*5))
    out += struct.pack('<HH', 4, 0) if rich else bytes(4)
    out += object_tag('moLineBackedUpData_c')+bytes(4)+struct.pack('<ii', -11, 13) if rich else b'\0\0'
    return out+struct.pack('<HH', 6, 4) if rich else out+bytes(4)


def entity(rich: bool, flags_56: int = 0x8200, flags_54: int = 0xffff) -> bytes:
    out = struct.pack('<iiHHBIHhf', 17, -9, 65530, 65529, 255, 0x89abcdef, 65528, -123, -0.0)
    out += struct.pack('<i4Hhf', 2, 0, 65527, 65526, 65525, -127, 1.25)+b'\0\0'
    out += struct.pack('<HHiHB', flags_54, flags_56, -23, 0, 255 if rich else 0)
    if rich:
        out += short_wide_name('Ω sketch entity')
    return out+struct.pack('<dII', -0.0, 0xffffffff, 0x89abcdef)


def slot(rich: bool, name: str) -> bytes:
    out = object_tag('sgSlot_c')+entity(rich)+struct.pack('<ii', -1, -2)
    return out+handle(name, -2**31)+struct.pack('<5H', 4, 4, 0, 4, 4)


def external_reference(version: int, mode: int, rich: bool) -> bytes:
    out = object_tag('moSketchExtRef_w')+wrapper(version, False)+struct.pack('<h', mode)
    return out+(point() if rich else b'\0\0')


def temporary_axis(version: int, rich: bool) -> bytes:
    out = object_tag('moTempAxisRef_w')+wrapper(version, rich)+struct.pack('<8dB', -0.0, 1, 2, 3, 4, 5, 6, 7, 255)
    if version > 0x371d:
        out += struct.pack('<6di', -0.0, 1, 2, 3, 4, 5, -17)
    return out


def intersection_info() -> bytes:
    return object_tag('moSkIntersectInfo_c')+struct.pack('<H4i', 65535, -2**31, 2**31-1, -1, 0)


def external_entity(version: int, rich: bool) -> bytes:
    out = object_tag('sgExtEnt_c')
    out += object_tag('moRefWrapper_c')+wrapper(version, False) if rich else b'\0\0'
    out += b'\0\0'+(intersection_info() if rich else b'\0\0')+b'\0\0'
    return out+short_wide_name('Ω axis')


def component_handle(version: int, name: str) -> bytes:
    return object_tag('moCompSketchEntHandle_c')+component_base(version, False)+struct.pack('<iI', -29, 0xffffffff)+handle(name, 0x777f)+struct.pack('<I', 0x89abcdef)


def silhouette_vertex(version: int, rich: bool) -> bytes:
    out = object_tag('moSilhVertexRef_c')+boundary_base(version, False)
    out += boundary_wire('moFaceRef_c', version, False)+struct.pack('<HH', 4, 0) if rich else bytes(6)
    return out+struct.pack('<I2dI', 0xffffffff, -0.0, 1.25, 0x89abcdef)


def component_vertex(version: int, rich: bool) -> bytes:
    return object_tag('moCompSilhVertex_c')+component_boundary(version, rich)
