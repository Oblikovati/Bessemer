"""Independent literal embedded wrapper images with populated direct transforms (#134)."""
import struct
from typing import TypedDict


class EmbeddedCase(TypedDict):
    version: int
    matrix_presence: int
    flags: int
    wrapper_end: int
    first_transform_end: int
    raw_hex: str


def patterned_doubles(count: int) -> bytes:
    """Retain finite, signed zero, infinity and NaN payload bits; e.g. patterned_doubles(9)."""
    patterns = (0x3ff0000000000000, 0x8000000000000000, 0x7ff0000000000000,
                0xfff0000000000000, 0x7ff8000000000123, 0x0000000000000001)
    return b''.join(struct.pack('<Q', patterns[index % len(patterns)]) for index in range(count))


def literal_transform(presence: int, flags: int) -> bytes:
    """Build physical transform bytes independently of either reader; e.g. literal_transform(255, 0x2f)."""
    matrix = patterned_doubles(9) if presence else b''
    payload = {0x10: patterned_doubles(4), 0x20: patterned_doubles(6),
               0x40: struct.pack('<i', -2147483648) + patterned_doubles(25)}.get(flags & 0xf0, b'')
    return bytes((presence,)) + matrix + patterned_doubles(4) + bytes((flags,)) + payload


def literal_wrapper(version: int) -> bytes:
    """Encode historical gates with two present optional transforms; e.g. literal_wrapper(14000)."""
    prefix = b'\0\0' + struct.pack('<H', 0xfedc) + b'\0\0' + struct.pack('<2i', -1, 2147483647)
    optional = struct.pack('<H', 0xffff) + literal_transform(1, 0x1b)
    optional += struct.pack('<H', 2) + literal_transform(0, 0x4e)
    suffix = struct.pack('<4i', -2147483648, 0, 1, -1)
    suffix += struct.pack('<i', 74) if version > 0x2faf else b''
    suffix += struct.pack('<2i', 76, 104)
    suffix += struct.pack('<2i', 52, 48) if version > 0x1fd1 else b''
    suffix += struct.pack('<i', 144) if version >= 0x2b73 else b''
    suffix += struct.pack('<i', 56) if version >= 0x3b02 else b''
    return prefix + optional + suffix


def owned_embedded_cases() -> list[EmbeddedCase]:
    """Cover every wrapper threshold and transform selector; e.g. owned_embedded_cases()."""
    versions = (4307, 6000, 0x1fd1, 0x1fd2, 0x2b72, 0x2b73, 0x2faf, 0x2fb0, 0x3b01, 0x3b02, 19000)
    name = b'moCompXFormWrapper_c'
    result: list[EmbeddedCase] = []
    for version in versions:
        for presence in (0, 1, 255):
            for flags in (0, 0x1f, 0x2a, 0x4b, 0x80, 0xff):
                header = struct.pack('<IHHH', version, 0xffff, 0x1234, len(name)) + name
                wrapper = header + literal_wrapper(version)
                first = literal_transform(presence, flags)
                raw = wrapper + first + literal_transform(255, flags ^ 0x0f)
                result.append(dict(version=version, matrix_presence=presence, flags=flags,
                                   wrapper_end=len(wrapper), first_transform_end=len(wrapper) + len(first), raw_hex=raw.hex()))
    return result
