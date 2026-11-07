"""Independent complete Hole Wizard wires with direct slot axes and preserved raw parameters (#54)."""
from functools import partial
import struct
from fillet_fixture_bytes import WireMap, body
from owned_name_edit import short_wide_name
from display_dimension_fixture_bytes import shared, suffix


def slot_axis(index: int) -> bytes:
    """Keep direct geometry independent of tagged map slots; e.g. slot_axis(2)."""
    return struct.pack('<9d', index, 2, 3, 4, 5, 6, 0, 0, 1)


def hole_text(offset: str, rich: bool, edited: bool) -> bytes:
    """Build an independent variable-length label edit; e.g. hole_text('828', True, True)."""
    if edited and offset == '828':
        return short_wide_name('Bessemer hole Ω')
    return short_wide_name('hole Ω '+offset) if rich else bytes(1)


def early_parameters(rich: bool, edited: bool) -> bytes:
    """Retain the documented non-member-order prefix; e.g. early_parameters(True, False)."""
    text = partial(hole_text, rich=rich, edited=edited)
    raw = struct.pack('<di', -0.0, 2147483647)
    raw += text('828')+text('830')+struct.pack('<3i', -1, 5, -2147483648)
    raw += b''.join(text(offset) for offset in ('850', '858', '860'))
    raw += struct.pack('<5d', 1, 2, 3, 4, 5)+text('868')+struct.pack('<2i', -1, 2)
    return raw+text('8a0')+text('8b0')+struct.pack('<2i', 99, -17)+text('8b8')+struct.pack('<i', 2147483647)


def late_parameters(rich: bool, slot: bool, edited: bool) -> bytes:
    """Separate pair counts from later slot counts; e.g. late_parameters(True, True, False)."""
    text = partial(hole_text, rich=rich, edited=edited)
    pairs = struct.pack('<iidid', 2, -7, 2.5, 2147483647, -0.0) if rich else bytes(4)
    raw = pairs+struct.pack('<8id3i', -1, 2, -3, 4, -5, 6, -7, 8, -0.0, 9, -10, 11)
    raw += text('8c0')+struct.pack('<8i', 1, 2, 3, 4, 5, 6, 7, 8)+text('a30')+text('a38')
    return raw+struct.pack('<iQ3i', -3 if slot else 0, 0x7ff8000000000042, 1, -2, 3)


def dimension(mapping: WireMap, version: int) -> tuple[bytes, int]:
    """Build the complete distance dimension before its alias; e.g. dimension(mapping, 19000)."""
    name = 'moDisplayDistanceDim_c'
    raw = mapping.new(name, lambda: shared(mapping, version, name, False, mapping.slot)
                      +suffix(mapping, version, name, False))
    return raw, mapping.slot


def slot_suffix(mapping: WireMap, version: int, rich: bool) -> bytes:
    """Do not insert a count or object tags into inherited direct slots; e.g. slot_suffix(mapping, 0x2b5d, True)."""
    raw, reference = dimension(mapping, version)
    axes = (2 if rich else 0) if version < 0x2b5e else (3 if rich else 1)
    if version >= 0x2b5e:
        raw += struct.pack('<i', axes)
    raw += b''.join(slot_axis(index) for index in range(axes))
    raw += struct.pack('<3iH', 21, -22, 23, reference)
    if version < 0x42ea:
        raw += struct.pack('<i', -24)
    return raw+struct.pack('<3d', 1, 2, 3)


def hole_suffix(mapping: WireMap, version: int, rich: bool, slot: bool, owner: int, edited: bool) -> bytes:
    """Preserve original references, noncanonical bytes and relocated words; e.g. hole_suffix(mapping, 19000, True, True, 98, False)."""
    pointer = mapping.new('suObList', lambda: struct.pack('<3H', 2, owner, owner)) if rich else bytes(2)
    raw = struct.pack('<i', -2147483648)+pointer+early_parameters(rich, edited)+late_parameters(rich, slot, edited)
    if version > 0x2f47:
        raw += struct.pack('<i', 17)
    if version > 0x332e:
        raw += struct.pack('<i', 18)
    if slot:
        raw += slot_suffix(mapping, version, rich)
    if version > 0x3355:
        raw += struct.pack('<d', 25)
    if version > 0x3f02:
        raw += struct.pack('<Bii', 255, 26, -27)+(struct.pack('<3H', 2, owner, owner) if rich else bytes(2))
    if version >= 0x42ea:
        raw += struct.pack('<i', -24)
    return raw


def hole_object(version: int, rich: bool, slot: bool, first_slot: int = 0, edited: bool = False) -> tuple[bytes, int]:
    """Compose the complete inherited and derived feature; e.g. hole_object(19000, True, True, 96, True)."""
    if first_slot < 0:
        raise ValueError(f'Hole Wizard first slot {first_slot}; expected a nonnegative slot')
    mapping = WireMap()
    mapping.slot = first_slot
    def contents() -> bytes:
        owner = mapping.slot
        inherited = body(mapping, version, rich, owner)
        if 0x2f46 <= version < 0x2f68:
            inherited += struct.pack('<i', -123456)
        return inherited+hole_suffix(mapping, version, rich, slot, owner, edited)
    raw = mapping.new('moHoleWzd_c', contents)
    return raw, mapping.slot+1


def slot_object() -> tuple[bytes, int]:
    """Give a separately tagged axis its own normal slots; e.g. slot_object()."""
    mapping = WireMap()
    raw = mapping.new('moSlotCenterAxisData_c', lambda: slot_axis(7))
    return raw, mapping.slot+1
