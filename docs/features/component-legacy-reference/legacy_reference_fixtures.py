"""Literal ordinary component prefixes with direct legacy references (#206)."""
from pathlib import Path
from itertools import product
import struct

REFERENCE: Path | None = None
VERSIONS = (0x27cc, 0x2b60, 0x2b61, 0x2bb8, 0x2bb9, 0x2f55, 0x2f56, 0x3ef5, 0x3ef6, 19000)
PRESENCES = (0, 1, -1, -2147483648, 2147483647)
TIMES = (0, 0x80000000, 0xffffffff)


def class_header(name: str, schema: int) -> bytes:
    """Keep class framing separate from the untagged wrapper; e.g. class_header('moCompRef_c',7)."""
    encoded = name.encode('ascii')
    return struct.pack('<HHH', 65535, schema, len(encoded)) + encoded


def legacy_object(kind: int, identity: int) -> bytes:
    """Use null, mapped self or a fresh supported node name; e.g. legacy_object(2,98)."""
    if kind < 2:
        return struct.pack('<H', 0 if kind == 0 else identity)
    text = 'legacy Ω'
    return class_header('moNodeName_c', 1) + b'\xff\xfe\xff' + bytes([len(text)]) + text.encode('utf-16le')


def component_tail(version: int) -> bytes:
    """Retain following-field sentinels and every existing prefix gate; e.g. component_tail(19000)."""
    raw = struct.pack('<iI5i2i', -19, 0xfedcba98, -23, 29, -31, 37, -41, 0, 0)
    for threshold in (0x2b60, 0x2bb8, 0x2f55):
        if version > threshold:
            raw += bytes(4)
    return raw + (struct.pack('<I', 0x89abcdef) if version >= 0x3ef6 else b'')


def component_record(version: int, presence: int, kind: int, edited: bool, owned: bool) -> bytes:
    """Reserve only actual tagged objects and verify aliases after the root; e.g. component_record(19000,-1,2,False,True)."""
    identity = 98 if owned else 2
    raw = class_header('moCompRef_c', 7)
    raw += struct.pack('<H7i', 0, -3, 5, -7, 11, -13, 17, presence)
    if presence:
        raw += legacy_object(kind, identity) + struct.pack('<I', TIMES[kind] ^ int(edited))
    raw += component_tail(version)
    if presence and kind:
        raw += struct.pack('<H', identity + (2 if kind == 2 else 0))
    return raw


def legacy_reference_cases() -> list[tuple[int, int, int, int, bool, bytes]]:
    """Separate ten bare gate profiles from complete owned streams; e.g. legacy_reference_cases()."""
    rows = []
    for version, presence, kind in product(VERSIONS, PRESENCES, range(3)):
        for edited in (False, True) if presence else (False,):
            rows.append((0, version, presence, kind, edited, component_record(version, presence, kind, edited, False)))
    for presence, kind in product(PRESENCES, range(3)):
        for edited in (False, True) if presence else (False,):
            count = 2 if presence and kind else 1
            raw = struct.pack('<iH', -123, count) + component_record(19000, presence, kind, edited, True)
            rows.append((1, 19000, presence, kind, edited, raw))
    return rows
