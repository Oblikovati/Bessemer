"""Independent complete imported/base-body wires with explicit kernel and vector framing (#54)."""
from typing import Literal
import struct
from fillet_fixture_bytes import WireMap, body
from owned_name_edit import short_wide_name
from kernel_geometry_fixture import frame

ImportedClass = Literal['moImported_c', 'moBaseBody_c']
BodyMode = Literal['external', 'suppressed', 'compressed']


def imported_suffix(rich: bool, mode: BodyMode, edited: bool) -> bytes:
    """Retain external markers and exact direct body boundaries; e.g. imported_suffix(True, 'compressed', False)."""
    raw = short_wide_name('cad/Δ.x_t') if rich else bytes(1)
    marker = (-2147483648 if rich else 1) if mode == 'external' else 0
    raw += struct.pack('<i', marker)
    raw += b'' if mode == 'external' else b'\xff' if mode == 'suppressed' else frame()
    name = 'Bessemer Body Ω' if edited else ('Body Ω' if rich else '')
    return raw+(short_wide_name(name) if name else bytes(1))


def base_suffix(name: ImportedClass, version: int, rich: bool) -> bytes:
    """Keep the vector separate from imported fields; e.g. base_suffix('moBaseBody_c', 0x2bcd, True)."""
    if name != 'moBaseBody_c' or version <= 0x2bcc:
        return b''
    return struct.pack('<4i', 3, -2147483648, 0, 2147483647) if rich else bytes(4)


def imported_object(name: ImportedClass, version: int, rich: bool, mode: BodyMode,
                    first_slot: int = 0, edited: bool = False) -> tuple[bytes, int]:
    """Compose each complete class in either map context; e.g. imported_object('moBaseBody_c', 19000, True, 'external', 96)."""
    if name not in ('moImported_c', 'moBaseBody_c') or mode not in ('external', 'suppressed', 'compressed') or first_slot < 0:
        raise ValueError(f'imported class/mode/slot {(name, mode, first_slot)}; expected known class/mode and a nonnegative slot')
    mapping = WireMap()
    mapping.slot = first_slot
    def contents() -> bytes:
        owner = mapping.slot
        return body(mapping, version, rich, owner)+imported_suffix(rich, mode, edited)+base_suffix(name, version, rich)
    raw = mapping.new(name, contents)
    return raw, mapping.slot+1
