"""Independent direct comment lists, extra members and four-byte times (#54)."""
import struct
from fillet_fixture_bytes import WireMap
from freeze_component_references import base
from owned_name_edit import short_wide_name

NAMES = ('moNewFeatComment_c', 'moCommentExtraData_c', 'moCommentTextItem_c')


def text_item(version: int, rich: bool, edited: bool = False) -> bytes:
    """Keep the archive time width and all distinct strings; e.g. text_item(19000, True)."""
    strings = ['author Ω', 'Bessemer text Ω' if edited else 'stored text Ω', 'second\r\nline Ω', 'last Ω']
    raw = struct.pack('<i', -2147483648 if rich else 0)
    raw += short_wide_name(strings[0]) if rich else b'\0'
    raw += struct.pack('<I', 0xffffffff if rich else 0)
    raw += b''.join(short_wide_name(value) if rich else b'\0' for value in strings[1:3])
    return raw+((short_wide_name(strings[3]) if rich else b'\0') if version >= 0x3b10 else b'')


def extra(version: int, rich: bool) -> bytes:
    """Use the native +0C/+08/+10 order without a tag; e.g. extra(0x3b10, True)."""
    return struct.pack('<3i', -2147483648, 2147483647, -1) if version >= 0x3b10 and rich else bytes(12 if version >= 0x3b10 else 0)


def comment_list(mapping: WireMap, version: int, rich: bool, edited: bool) -> bytes:
    """Retain repeated/null items and a repeated runtime class; e.g. comment_list(mapping, 19000, True, False)."""
    if not rich:
        return bytes(2)
    first = mapping.new(NAMES[2], lambda: text_item(version, True, edited))
    slot = mapping.slot
    second = mapping.new(NAMES[2], lambda: text_item(version, False))
    return struct.pack('<H', 4)+first+struct.pack('<2H', slot, 0)+second


def comment(mapping: WireMap, version: int, rich: bool, owners: bool, edited: bool) -> bytes:
    """Separate direct members from two tagged owner occurrences; e.g. comment(mapping, 19000, True, True, False)."""
    raw = comment_list(mapping, version, rich, edited)+struct.pack('<i', 2 if rich else 0)+extra(version, rich)
    if version < 0x3b10:
        return raw
    if not owners:
        return raw+bytes(4)
    feature = mapping.new('moCompFeature_c', lambda: base(version, False)+struct.pack('<iI', -2147483648, 0xfedcba98))
    chain = mapping.new('moChainComponent_c', lambda: struct.pack('<HHI', 0, 0, 0xffffffff))
    return raw+feature+chain


def comment_object(kind: int, version: int, rich: bool, owners: bool = False,
                   first_slot: int = 0, edited: bool = False) -> tuple[bytes, int]:
    """Build complete tagged bodies in bare or imported maps; e.g. comment_object(0, 19000, True, True, 96)."""
    if kind not in range(3) or first_slot < 0:
        raise ValueError(f'comment fixture {(kind, first_slot)}; expected kind 0..2 and nonnegative slot')
    mapping = WireMap();mapping.slot = first_slot
    def contents() -> bytes:
        if kind == 0:
            return comment(mapping, version, rich, owners, edited)
        return extra(version, rich) if kind == 1 else text_item(version, rich, edited)
    raw = mapping.new(NAMES[kind], contents)
    return raw, mapping.slot+1
