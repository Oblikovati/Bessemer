"""Independent extruded surfaces, ordered word/array groups and binary-empty keepers (#54)."""
from typing import Literal, TypedDict
import struct
from fillet_fixture_bytes import WireMap, body
from body_feature_fixture_bytes import end_spec, header, atom_fields, body_suffix
from shell_fixture_bytes import ShellIdentities, face_pointer

SurfaceMode = Literal['null', 'empty', 'populated']
PointerKind = Literal['face', 'root', 'null']


class SurfaceSlots(TypedDict):
    end: int
    ordered: int


def pointer_array(mapping: WireMap, version: int, identities: ShellIdentities, owner: int,
                  items: tuple[PointerKind, ...], escape: bool = False) -> bytes:
    """Retain direct counts, nulls and repeated object identities; e.g. pointer_array(mapping, 19000, identities, 2, ('face', 'root'))."""
    raw = struct.pack('<HI', 65535, len(items)) if escape else struct.pack('<H', len(items))
    for item in items:
        raw += (face_pointer(mapping, version, identities) if item == 'face'
                else struct.pack('<H', owner) if item == 'root' else bytes(2))
    return raw


def ordered_body(mapping: WireMap, version: int, rich: bool, escape: bool) -> bytes:
    """Write every group word before its arrays; e.g. ordered_body(mapping, 19000, True, True)."""
    owner = mapping.slot
    identities = ShellIdentities()
    def array(items: tuple[PointerKind, ...], large: bool = False) -> bytes:
        return pointer_array(mapping, version, identities, owner, items, large)
    if not rich:
        return array((), escape)+bytes(4)+array(())+bytes(4)
    return (array(('face', 'face', 'null'), escape)+struct.pack('<i2I', 2, 0xffffffff, 0x80000000)
        +array(('face', 'root'))+array(())+array(('root',))
        +struct.pack('<iI', 1, 0xffffff85)+array(('face',)))


def surface_pointers(mapping: WireMap, version: int, rich: bool, mode: SurfaceMode,
                     slots: SurfaceSlots, edited: bool) -> bytes:
    """Capture each object's reserved slot before nested allocations; e.g. surface_pointers(mapping, 19000, True, 'populated', slots, False)."""
    def end_contents() -> bytes:
        slots['end'] = mapping.slot
        return end_spec(mapping, version, rich)
    raw = mapping.new('moEndSpec_c', end_contents)+struct.pack('<i', 7 if edited else -1)
    def ordered_contents() -> bytes:
        slots['ordered'] = mapping.slot
        return ordered_body(mapping, version, mode == 'populated', rich)
    return raw+(mapping.new('moOrderedFaceArray_c', ordered_contents) if mode != 'null' else bytes(2))


def surface_array(mapping: WireMap, rich: bool, mode: SurfaceMode, keeper: bool,
                  slots: SurfaceSlots) -> bytes:
    """Retain cached end/helper pointers and a freshly tagged empty keeper; e.g. surface_array(mapping, True, 'empty', True, slots)."""
    references = [slots['end']]+([slots['ordered']] if mode != 'null' else [])+[slots['end']]
    count = len(references)+int(keeper)
    raw = struct.pack('<HI', 65535, count) if rich else struct.pack('<H', count)
    raw += b''.join(struct.pack('<H', reference) for reference in references)
    return raw+(mapping.new('moExtruRefSurfIdKeeper_c', lambda: b'') if keeper else b'')


def surface_body(mapping: WireMap, version: int, rich: bool, owner: int, header_keeper: bool) -> bytes:
    """Exercise the independently observed inherited keeper role; e.g. surface_body(mapping, 19000, True, 2, True)."""
    if not header_keeper:
        return body(mapping, version, rich, owner)
    prefix = header(mapping, version, rich)
    if prefix[-2:] != bytes(2):
        raise ValueError(f'inherited keeper flag {prefix[-2:].hex()}; expected a short zero flag')
    prefix = prefix[:-2]+struct.pack('<H', 1)+mapping.new('moExtruRefSurfIdKeeper_c', lambda: b'')
    return prefix+atom_fields(mapping, rich, owner)+body_suffix(rich)


def surface_object(version: int, rich: bool, mode: SurfaceMode, keeper: bool,
                    first_slot: int = 0, edited: bool = False, header_keeper: bool = False) -> tuple[bytes, int]:
    """Build a complete feature in either object-map context; e.g. surface_object(19000, True, 'populated', True, 96)."""
    if mode not in ('null', 'empty', 'populated') or first_slot < 0:
        raise ValueError(f'surface mode/slot {(mode, first_slot)}; expected a known mode and nonnegative slot')
    mapping = WireMap()
    mapping.slot = first_slot
    slots: SurfaceSlots = dict(end=0, ordered=0)
    def contents() -> bytes:
        owner = mapping.slot
        raw = surface_body(mapping, version, rich, owner, header_keeper)+surface_pointers(mapping, version, rich, mode, slots, edited)
        return raw+struct.pack('<2i', -2147483648, 2147483647)+surface_array(mapping, rich, mode, keeper, slots)
    raw = mapping.new('moExtruRefSurface_c', contents)
    return raw, mapping.slot+1


def ordered_object(version: int, rich: bool, escape: bool) -> tuple[bytes, int]:
    """Give a tagged helper its own slots and keep its internal arrays direct; e.g. ordered_object(19000, True, True)."""
    mapping = WireMap()
    raw = mapping.new('moOrderedFaceArray_c', lambda: ordered_body(mapping, version, rich, escape))
    return raw, mapping.slot+1


def keeper_object() -> tuple[bytes, int]:
    """Freeze the documented ordinary-binary empty body; e.g. keeper_object()."""
    mapping = WireMap()
    raw = mapping.new('moExtruRefSurfIdKeeper_c', lambda: b'')
    return raw, mapping.slot+1
