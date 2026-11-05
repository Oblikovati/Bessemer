"""Literal direct boundary bodies without importing Bessemer (#205)."""
from pathlib import Path
import struct

REFERENCE: Path | None = None
VERSIONS = (0x27cc, 0x332c, 0x332d, 0x3358, 0x3359, 19000)
SCENARIOS = ((0, 0, 0, 0), (-2147483648, 0, 0, 0), (-1, 0, 0, 0),
             (2, 0, 0, 0), (2, 1, 2, 3), (1, -1, -1, 3),
             (-1, -2147483648, 2, -2147483648), (-2147483648, 2147483647, -2147483648, -1),
             (0, 1, 0, 0), (2, 255, 2, 3))


def downstream_body(version: int, style: int, edited: bool) -> bytes:
    """Preserve duplicate pairs and zero vector entries; e.g. downstream_body(19000,4,False)."""
    _, _, pairs, values = SCENARIOS[style]
    raw = struct.pack('<i', pairs)
    raw += struct.pack('<4i', -7, -2147483648, -7, 2147483647) if pairs > 0 else b''
    raw += struct.pack('<i', values)
    raw += struct.pack('<3i', 0, -2147483648, 0) if values > 0 else b''
    raw += struct.pack('<i', 42 if edited else -17)
    return raw + (struct.pack('<2i', -2147483648, 2147483647) if version >= 0x3359 else b'')


def boundary_record(version: int, style: int, edited: bool = False) -> bytes:
    """Build a complete tagged ordinary boundary with direct suffixes; e.g. boundary_record(19000,4)."""
    temporary, presence, _, _ = SCENARIOS[style]
    name = b'moBEntRef_c'
    raw = struct.pack('<HHH', 65535, 7, len(name)) + name
    raw += struct.pack('<5I2dHIi', 0x80000000, 0xffffffff, 0, 0x87654321, 0x12345678, -0.0, 1.25, 0, 0, temporary)
    for ordinal in range(max(temporary, 0)):
        first, second = ((0xffffffff, 0x80000000), (0, 0x65))[ordinal]
        first = 0x11223344 if edited and not presence and ordinal == 0 else first
        raw += struct.pack('<II', first, second)
    if version <= 0x332c:
        return raw
    raw += struct.pack('<Ii', 0xffffffff, presence)
    return raw + (downstream_body(version, style, edited) if presence else b'')


def boundary_cases() -> list[tuple[int, int, int, bool, bytes]]:
    """Separate bare gate coverage from complete owned19000 graphs; e.g. boundary_cases()."""
    rows = []
    for version in VERSIONS:
        styles = range(4 if version <= 0x332c else 10)
        rows += [(0, version, style, False, boundary_record(version, style)) for style in styles]
        rows += [(0, version, style, True, boundary_record(version, style, True)) for style in styles if style >= 3]
    for style in range(10):
        rows.append((1, 19000, style, False, struct.pack('<iH', -123, 1) + boundary_record(19000, style)))
    for style in range(3, 10):
        rows.append((1, 19000, style, True, struct.pack('<iH', -123, 1) + boundary_record(19000, style, True)))
    return rows
