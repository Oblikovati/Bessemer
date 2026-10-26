"""Independent Toolbox arrays, shared items and nonfinite words (#54)."""
from fillet_fixture_bytes import WireMap
from owned_name_edit import short_wide_name
import struct

FLOAT_BITS = (0x8000000000000000, 0x7ff8000000001234, 0xfff0000000000000)


def ansi_string(value: str) -> bytes:
    """Keep short and extended ANSI framing; e.g. ansi_string('café')."""
    encoded = value.encode('cp1252')
    if len(encoded) > 65534:
        raise ValueError(f'ANSI fixture length {len(encoded)}; expected <=65534 bytes')
    prefix = bytes([len(encoded)]) if len(encoded) < 255 else b'\xff'+struct.pack('<H', len(encoded))
    return prefix+encoded


def item_body(rich: bool, bits: int) -> bytes:
    """Preserve the exact double word; e.g. item_body(True, FLOAT_BITS[1])."""
    strings = short_wide_name('Item Ω')+ansi_string('café') if rich else bytes(2)
    return struct.pack('<i', -2147483648)+strings+struct.pack('<Qii', bits, -1, 2147483647)


def array_suffix(rich: bool, edited: bool) -> bytes:
    """Construct the independent Unicode edit without patching bytes; e.g. array_suffix(True, True)."""
    strings = [ansi_string('café'), short_wide_name('Ω_tool'), b'\0', ansi_string('A'*300),
        ansi_string('duplicate'), ansi_string('duplicate'), short_wide_name('工具'), short_wide_name('')]
    raw = b''.join(strings) if rich else bytes(8)
    name = short_wide_name('Bessemer 工具 Ω' if edited else '工具 Ω') if rich else b'\0'
    return raw+struct.pack('<i', -9)+name+struct.pack('<i', 2147483647)


def toolbox_object(kind: int, mode: int, bits: int, first_slot: int = 0, edited: bool = False) -> tuple[bytes, int]:
    """Build complete bodies with actual inherited slots; e.g. toolbox_object(0, 2, FLOAT_BITS[1], 96)."""
    if kind not in (0, 1) or mode not in (0, 1, 2) or first_slot < 0 or bits not in FLOAT_BITS:
        raise ValueError(f'Toolbox fixture {(kind, mode, bits, first_slot)}; expected known kind/mode/float and nonnegative slot')
    mapping = WireMap();mapping.slot = first_slot
    def contents() -> bytes:
        owner = mapping.slot
        count = 4 if mode else 0
        raw = struct.pack('<HI', 65535, count) if mode == 2 else struct.pack('<H', count)
        if mode:
            raw += mapping.new('moToolboxPartSpec_c', lambda: item_body(True, bits))
            raw += struct.pack('<3H', mapping.slot, 0, owner)
        return raw+array_suffix(bool(mode), edited)
    name = 'moToolboxPartSpec_c' if kind else 'moToolboxPartSpecArray_c'
    raw = mapping.new(name, (lambda: item_body(bool(mode), bits)) if kind else contents)
    return raw, mapping.slot+1
