"""Complete native-bound folder layers and direct custom-property identities (#54)."""
import struct
from fillet_fixture_bytes import WireMap
from freeze_environment import feature
from freeze_component_references import base as component_base
from owned_name_edit import short_wide_name
from reference_plane_fixture_bytes import plane_reference
from sweep_fixture_bytes import shared_object
from weldment_fixture_bytes import custom_list

CLASSES = ('moAtomBodyFolder_c', 'moFeatSolidBodyFolder_c', 'moFeatSurfaceBodyFolder_c',
    'moSubAtomFolder_c', 'moSubWeldFolder_c', 'moCutListFolder_c', 'moRefAxisFtrFolder_c',
    'moCoordSysFtrFolder_c', 'moCurveFtrFolder_c', 'moMirroredCompFeatFtrFolder_c',
    'moProfileFtrFolder_c', 'moRefPlaneFtrFolder_c', 'moRefPointFtrFolder_c', 'moStockFtrFolder_c')
FLAGS = (0, 1, 2, 3, 128, 255)
VERSIONS = (0x27cc, 19000, 0xffffffff)


def common_folder(mapping: WireMap, version: int, rich: bool) -> bytes:
    """Reuse the complete feature and reserve its actual node-name slots; e.g. common_folder(mapping, 19000, True)."""
    mapping.slot += 2
    return feature(version, rich)+struct.pack('<2i', -2147483648, 2147483647)


def component_pointer(mapping: WireMap, ids: dict[str, int], version: int) -> bytes:
    """Share one complete component feature across all list and payload occurrences; e.g. component_pointer(mapping, ids, 19000)."""
    return shared_object(mapping, ids, 'component-feature', 'moCompFeature_c',
        lambda: component_base(version, False)+struct.pack('<iI', -2147483648, 0xffffffff))


def pointer_list(mapping: WireMap, ids: dict[str, int], version: int) -> bytes:
    """Retain an escaped count and separate repeated/null occurrences; e.g. pointer_list(mapping, ids, 19000)."""
    first = component_pointer(mapping, ids, version)
    return struct.pack('<HI', 65535, 3)+first+bytes(2)+component_pointer(mapping, ids, version)


def body_folder(mapping: WireMap, ids: dict[str, int], version: int, style: int) -> bytes:
    """Keep the atom-body folder's complete inherited material and sort references; e.g. body_folder(mapping, ids, 19000, 1)."""
    raw = common_folder(mapping, version, style != 0)+struct.pack('<2i', -2147483648, 2147483647)
    if style == 0:
        return raw+bytes(4)
    material = shared_object(mapping, ids, 'material', 'moMaterialFolder_c',
        lambda: common_folder(mapping, version, True)+short_wide_name('材料 Ω')+struct.pack('<I', 0xffffffff))
    sorting = shared_object(mapping, ids, 'sorting', 'moCutListSortOptions_c',
        lambda: struct.pack('<i', -2147483648)+pointer_list(mapping, ids, version))
    return raw+material+sorting


def sub_atom_suffix(mapping: WireMap, ids: dict[str, int], version: int, style: int) -> bytes:
    """The +3D8 list is tagged; the separately declared unknown feature-wrapper dependency remains null; e.g. sub_atom_suffix(mapping, ids, 19000, 1)."""
    listing = shared_object(mapping, ids, 'sub-list', 'suObList', lambda: pointer_list(mapping, ids, version)) if style else bytes(2)
    return listing+struct.pack('<iH2i', -2147483648, 0, 2147483647, -1)


def cut_payload(mapping: WireMap, ids: dict[str, int], version: int, style: int) -> bytes:
    """Keep the raw byte directly between signed words and the plane pointer; e.g. cut_payload(mapping, ids, 19000, 2)."""
    component = component_pointer(mapping, ids, version) if style else bytes(2)
    plane = shared_object(mapping, ids, 'plane', 'moPlaneRef_w', lambda: plane_reference(version)) if style else bytes(2)
    raw = component+struct.pack('<2iB', -2147483648, 2147483647, FLAGS[style])+plane
    return raw+struct.pack('<3Q2i', 0x8000000000000000, 0x7ff8000000001234, 0x7ff0000000000000, -1, -2147483648)


def folder_contents(mapping: WireMap, ids: dict[str, int], kind: int, version: int, style: int, edited: bool) -> bytes:
    """Compose every intermediate layer before the selected suffix; e.g. folder_contents(mapping, ids, 5, 19000, 2, False)."""
    if kind >= 6:
        prefix = common_folder(mapping, version, style != 0)
        owner = component_pointer(mapping, ids, version) if style else bytes(2)
        return prefix+owner+struct.pack('<3i', -2147483648, 2147483647, -1)
    raw = body_folder(mapping, ids, version, style)
    if kind >= 3:
        raw += sub_atom_suffix(mapping, ids, version, style)
    if kind >= 4:
        raw += custom_list(style % 4, edited)
    return raw+(cut_payload(mapping, ids, version, style) if kind == 5 else b'')


def cut_list_object(kind: int, version: int, style: int, edited: bool = False, first_slot: int = 0) -> tuple[bytes, int]:
    """Keep equal folders and direct properties distinct, with shared tagged dependencies; e.g. cut_list_object(5, 19000, 2)."""
    mapping = WireMap();mapping.slot = first_slot;ids: dict[str, int] = {}
    def record(key: str) -> bytes:
        return shared_object(mapping, ids, key, CLASSES[kind], lambda: folder_contents(mapping, ids, kind, version, style, edited and key == 'first'))
    def array() -> bytes:
        return struct.pack('<H', 3)+record('first')+record('second')+record('first')
    raw = mapping.new('suObArray', array) if style in (2, 5) else record('first')
    return raw, mapping.slot+1
