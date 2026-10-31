"""Independent literal historical component holders for the complete base body (#134)."""
import struct


def signed_words(*values: int) -> bytes:
    """Encode complete literal signed words; e.g. signed_words(-1, 2)."""
    return struct.pack('<' + 'i' * len(values), *values)


def holder_words(version: int) -> bytes:
    """Retain the short-lived historical word and modern replacement; e.g. holder_words(10144)."""
    raw = signed_words(-1, 2, -3, 4, -5, 6, 0, -7, -2147483648)
    if version > 0x11ab:
        raw += signed_words(-9)
    if 0x279f < version < 0x27a8:
        raw += signed_words(-94)
    if version > 0x27a7:
        raw += signed_words(90, -94, 98, -99)
    return b'\0\0' + raw


def holder_strings(version: int, populated: bool) -> bytes:
    """Keep legacy direct text distinct from presence-prefixed text; e.g. holder_strings(10187, True)."""
    raw = b''
    if version > 0x27b1:
        raw += signed_words(-1 if populated else 0) + (b'\0\0' if populated else b'')
    if 0x27ca < version < 0x27cc:
        raw += b'\x03old'
    if version > 0x27cb:
        raw += signed_words(2 if populated else 0) + (b'\x03b8!' if populated else b'')
    return raw


def holder_suffix(version: int, populated: bool) -> bytes:
    """Encode guarded absent sub-bodies and populated late strings; e.g. holder_suffix(19000, True)."""
    raw = b''
    for threshold in (0x2b60, 0x2bb8):
        if version > threshold:
            raw += signed_words(0)
    if version > 0x2f55:
        raw += signed_words(-3 if populated else 0) + (b'\x03d0!' if populated else b'')
    if version >= 0x3ef6:
        raw += signed_words(-76)
    return raw


def owned_holders() -> list[tuple[int, bytes]]:
    """Cover every historical holder threshold and immediate neighbors; e.g. owned_holders()."""
    thresholds = (0x11ab, 0x279f, 0x27a7, 0x27a8, 0x27b1, 0x27ca, 0x27cb, 0x27cc, 0x2b60, 0x2bb8, 0x2f55, 0x3ef6)
    versions = sorted({4307, 6000, 19000, *(value + delta for value in thresholds for delta in (-1, 0, 1))})
    name = b'moCompHolder_c'
    cases: list[tuple[int, bytes]] = []
    for version in versions:
        for populated in (False, True):
            body = holder_words(version) + holder_strings(version, populated) + holder_suffix(version, populated)
            raw = struct.pack('<IHHH', version, 0xffff, 0x1234, len(name)) + name + body
            cases.append((version, raw))
    return cases
