"""Independent complete shell bodies with preserved lists and concrete dependencies (#54)."""
from collections.abc import Callable
from dataclasses import dataclass
from typing import Literal, Protocol
import struct

ShellMode = Literal['null', 'shared', 'unequal', 'populated']


class ShellWireMap(Protocol):
    slot: int
    def new(self, name: str, body: Callable[[], bytes]) -> bytes: ...


@dataclass
class ShellIdentities:
    face: int | None = None
    first_list: int | None = None


def face_pointer(mapping: ShellWireMap, version: int, identities: ShellIdentities) -> bytes:
    """Share a serialized face across lists; e.g. face_pointer(mapping, 19000, identities)."""
    from freeze_component_references import wire
    from display_dimension_fixture_bytes import body
    if identities.face is not None:
        return struct.pack('<H', identities.face)
    raw = mapping.new('moCompFace_c', lambda: body(wire('moCompFace_c', version, False, 0)))
    identities.face = mapping.slot
    return raw


def face_list(mapping: ShellWireMap, version: int, identities: ShellIdentities, count: int) -> bytes:
    """Preserve list length and shared items; e.g. face_list(mapping, 19000, identities, 3)."""
    def contents() -> bytes:
        if identities.first_list is None:
            identities.first_list = mapping.slot
        raw = struct.pack('<H', count)
        for index in range(count):
            raw += bytes(2) if count > 1 and index == count-1 else face_pointer(mapping, version, identities)
        return raw
    return mapping.new('suObList', contents)


def distance_dimension(mapping: ShellWireMap, version: int) -> bytes:
    """Use the existing independent complete dimension body; e.g. distance_dimension(mapping, 19000)."""
    from display_dimension_fixture_bytes import shared, suffix
    name = 'moDisplayDistanceDim_c'
    return mapping.new(name, lambda: shared(mapping, version, name, False, mapping.slot)
                       +suffix(mapping, version, name, False))


def component_pointer(mapping: ShellWireMap, version: int, name: str) -> bytes:
    """Build a complete concrete edge or solid body; e.g. component_pointer(mapping, 19000, 'moCompEdge_c')."""
    from freeze_component_references import wire
    from display_dimension_fixture_bytes import body
    return mapping.new(name, lambda: body(wire(name, version, False, 0)))


def shell_suffix(mapping: ShellWireMap, version: int, mode: ShellMode, flag: int,
                 silhouette: bool) -> bytes:
    """Keep all six pointer positions, including unaligned tags; e.g. shell_suffix(mapping, 19000, 'unequal', 255, False)."""
    if mode == 'null':
        return bytes(4)+bytes([flag])+bytes(8)
    identities = ShellIdentities()
    first = face_list(mapping, version, identities, 3)
    if identities.first_list is None:
        raise ValueError('shell first list slot None; expected a newly allocated list')
    dimension = distance_dimension(mapping, version) if mode == 'populated' else bytes(2)
    aliases = struct.pack('<2H', identities.first_list, identities.first_list)
    if mode == 'unequal':
        aliases = face_list(mapping, version, identities, 3)+face_list(mapping, version, identities, 1)
    edge = component_pointer(mapping, version, 'moCompSilhEdge_c' if silhouette else 'moCompEdge_c') if mode == 'populated' else bytes(2)
    solid = component_pointer(mapping, version, 'moCompSolidBody_c') if mode == 'populated' else bytes(2)
    return first+dimension+bytes([flag])+aliases+edge+solid


def shell_object(version: int, rich: bool, mode: ShellMode, flag: int | None = None,
                 silhouette: bool = False, first_slot: int = 0) -> tuple[bytes, int]:
    """Construct a full feature in either map context; e.g. shell_object(19000, True, 'populated', first_slot=96)."""
    from fillet_fixture_bytes import WireMap, body
    actual_flag = (0 if mode == 'null' else 255) if flag is None else flag
    if mode not in ('null', 'shared', 'unequal', 'populated') or not 0 <= actual_flag <= 255 or first_slot < 0:
        raise ValueError(f'shell mode/flag/slot {(mode, actual_flag, first_slot)}; expected a known mode, u8 flag and nonnegative slot')
    if silhouette and mode != 'populated':
        raise ValueError(f'silhouette mode {mode!r}; expected populated')
    mapping = WireMap()
    mapping.slot = first_slot
    def contents() -> bytes:
        owner = mapping.slot
        return body(mapping, version, rich, owner)+shell_suffix(mapping, version, mode, actual_flag, silhouette)
    raw = mapping.new('moShell_c', contents)
    return raw, mapping.slot+1
