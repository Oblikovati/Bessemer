"""Independent common-pattern and tagged/direct plane construction (#54)."""
from collections.abc import Callable
from dataclasses import dataclass
from typing import Literal, Protocol
import struct

PlaneMode = Literal['null', 'direct', 'shared']


class MirrorWireMap(Protocol):
    slot: int
    def new(self, name: str, body: Callable[[], bytes]) -> bytes: ...


@dataclass
class PlaneReferenceIdentity:
    component: int | None = None


def plane_tail(version: int) -> bytes:
    """Reuse the independently built wrapper gates; e.g. plane_tail(19000)."""
    from freeze_model_references import wrapper
    transform = b'\0'+struct.pack('<4dB', 1, 2, 3, 1, 0)
    return wrapper(version, False)[2:]+bytes(32)+struct.pack('<6d', 1, 2, 3, 0, 0, 1)+transform+struct.pack('<3d', 4, 5, 6)


def plane_body(mapping: MirrorWireMap, version: int, identity: PlaneReferenceIdentity,
               with_face: bool) -> bytes:
    """Allocate a face once, then retain its identity; e.g. plane_body(mapping, 19000, identity, True)."""
    from freeze_component_references import wire as component
    from display_dimension_fixture_bytes import body
    if not with_face:
        return bytes(2)+plane_tail(version)
    if identity.component is not None:
        return struct.pack('<H', identity.component)+plane_tail(version)
    face = mapping.new('moCompFace_c', lambda: body(component('moCompFace_c', version, False, 0)))
    identity.component = mapping.slot
    return face+plane_tail(version)


def common_pattern(mapping: MirrorWireMap, version: int, rich: bool) -> bytes:
    """Keep the root's reserved owner through inherited records; e.g. common_pattern(mapping, 19000, True)."""
    import pattern_fixture_bytes as pattern
    owner = mapping.slot
    raw = pattern.body(mapping, version, rich, owner)+pattern.header(mapping, version, rich)
    raw += pattern.collections(rich)+pattern.shape(4, True)+b'\xff\xfe'
    return raw+pattern.variation(rich)+pattern.tail(version, rich, -2**31)


def mirror_suffix(mapping: MirrorWireMap, version: int, tagged: bool, mode: PlaneMode) -> bytes:
    """Separate tagged allocation from direct members; e.g. mirror_suffix(mapping, 19000, True, 'shared')."""
    identity = PlaneReferenceIdentity()
    pointer = b''
    if version > 0x3745:
        pointer = mapping.new('moPlaneRef_w', lambda: plane_body(mapping, version, identity, mode == 'shared')) if tagged else bytes(2)
    return pointer+plane_body(mapping, version, identity, mode in ('direct', 'shared'))


def mirror_object(version: int, rich: bool, tagged: bool, mode: PlaneMode = 'null',
                  first_slot: int = 0) -> tuple[bytes, int]:
    """Build a complete object in either map context; e.g. mirror_object(19000, True, True, 'shared', 96)."""
    from pattern_fixture_bytes import WireMap
    if mode not in ('null', 'direct', 'shared') or first_slot < 0:
        raise ValueError(f'mirror mode/first slot {(mode, first_slot)}; expected a known mode and nonnegative slot')
    if mode == 'shared' and (not tagged or version <= 0x3745):
        raise ValueError(f'shared plane version/tagged {(version, tagged)}; expected version > 0x3745 and a tagged plane')
    mapping = WireMap()
    mapping.slot = first_slot
    raw = mapping.new('moMirrorPattern_c', lambda:
        common_pattern(mapping, version, rich)+mirror_suffix(mapping, version, tagged, mode))
    return raw, mapping.slot+1


def plane_object(version: int, with_face: bool) -> tuple[bytes, int]:
    """Build a complete standalone tagged plane reference; e.g. plane_object(19000, True)."""
    from pattern_fixture_bytes import WireMap
    mapping = WireMap()
    raw = mapping.new('moPlaneRef_w', lambda: plane_body(mapping, version, PlaneReferenceIdentity(), with_face))
    return raw, mapping.slot+1
