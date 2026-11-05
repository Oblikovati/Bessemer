"""Independent installed-writer literals for both declared control states (#227)."""
from pathlib import Path
import struct
import hashlib
import json

ROOT = Path(__file__).resolve().parents[3]
LENGTHS = (0, 1, 2, 3, 4, 7, 8, 15, 16, 17, 31, 32, 255, 256, 65535, 65536)


def word(control: int, value: int, width: int) -> bytes:
    """Transfer low native bits in the declared order; e.g. word(0, 256, 2)."""
    return (value % (1 << (8 * width))).to_bytes(width, 'big' if control == 0 else 'little')


def integer(control: int, value: int) -> bytes:
    """Use independent protocol thresholds; e.g. integer(0, -129)."""
    if -32 <= value < 128:
        return bytes([value % 256])
    if value < 0:
        index = next(i for i, minimum in enumerate((-128, -32768, -2147483648, -9223372036854775808)) if value >= minimum)
        return bytes([0xd0 + index]) + word(control, value, (1, 2, 4, 8)[index])
    index = next(i for i, maximum in enumerate((255, 65535, 4294967295, 18446744073709551615)) if value <= maximum)
    return bytes([0xcc + index]) + word(control, value, (1, 2, 4, 8)[index])


def count(control: int, length: int, fix: tuple[int, int] | None, tags: tuple[int, ...]) -> bytes:
    """Keep the string/binary/container threshold differences; e.g. count(0, 16, (0x90,15), (0xdc,0xdd))."""
    if fix is not None and length <= fix[1]:
        return bytes([fix[0] | length])
    widths = (1, 2, 4) if len(tags) == 3 else (2, 4)
    index = next(i for i, width in enumerate(widths) if length < (1 << (width * 8)))
    return bytes([tags[index]]) + word(control, length, widths[index])


def payload(length: int) -> bytes:
    """Keep NUL and nontext bytes in all variable bodies; e.g. payload(32)."""
    return bytes(((index * 73) ^ 0xa5) % 256 for index in range(length))


def variable(control: int, kind: int, length: int, type_byte: int = 0xff) -> bytes:
    """Encode all five variable/container families; e.g. variable(0, 16, 4)."""
    if kind == 12:
        return count(control, length, (0xa0, 31), (0xd9, 0xda, 0xdb)) + payload(length)
    if kind == 13:
        return count(control, length, None, (0xc4, 0xc5, 0xc6)) + payload(length)
    if kind == 14:
        return count(control, length, (0x90, 15), (0xdc, 0xdd)) + bytes(i % 128 for i in range(length))
    if kind == 15:
        pairs = b''.join(b'\xa1k' + bytes([i % 128]) for i in range(length))
        return count(control, length, (0x80, 15), (0xde, 0xdf)) + pairs
    fixed = {1: 0xd4, 2: 0xd5, 4: 0xd6, 8: 0xd7, 16: 0xd8}
    prefix = bytes([fixed[length]]) if length in fixed else count(control, length, None, (0xc7, 0xc8, 0xc9))
    return prefix + bytes([type_byte]) + payload(length)


def integer_cases(control: int) -> list[tuple[int, int, bytes]]:
    """Cover each concrete signed/unsigned constructor's actual range; e.g. integer_cases(0)."""
    boundaries = (-9223372036854775808, -2147483649, -2147483648, -32769, -32768,
        -129, -128, -33, -32, -1, 0, 1, 127, 128, 255, 256, 32767, 32768,
        65535, 65536, 2147483647, 2147483648, 4294967295, 4294967296, 9223372036854775807, 18446744073709551615)
    result = []
    for kind, width in enumerate((8, 16, 32, 64), 4):
        values = [v for v in boundaries if -(1 << (width - 1)) <= v < (1 << (width - 1))]
        result.extend((kind, value % (1 << 64), integer(control, value)) for value in values)
    for kind, width in enumerate((8, 16, 32, 64), 8):
        result.extend((kind, value, integer(control, value)) for value in boundaries if 0 <= value < (1 << width))
    return result


def cases() -> list[tuple[int, int, int, bytes]]:
    """Supply every installed variant and all count boundaries in both profiles; e.g. cases()."""
    result = []
    for control in (0, 1):
        rows = [(0, 0, b'\xc0')] + [(1, value, bytes([0xc2 if value == 0 else 0xc3])) for value in (0, 1, 2, 255)]
        floats = (0, 1, 0x80000000, 0x7f7fffff, 0x7f800000, 0xff800000, 0x7fc01234, 0x7f800001, 0xffffffff)
        doubles = (0, 1, 0x8000000000000000, 0x7fefffffffffffff, 0x7ff0000000000000, 0xfff0000000000000, 0x7ff8000000001234, 0x7ff0000000000001, 0xffffffffffffffff)
        rows += [(2, bits, b'\xca' + word(control, bits, 4)) for bits in floats]
        rows += [(3, bits, b'\xcb' + word(control, bits, 8)) for bits in doubles]
        rows += integer_cases(control)
        rows += [(kind, length, variable(control, kind, length)) for kind in range(12, 16) for length in LENGTHS]
        rows += [(16, (length << 8) | type_byte, variable(control, 16, length, type_byte)) for length in LENGTHS for type_byte in (0, 1, 127, 128, 255)]
        result.extend((control, kind, meta, raw) for kind, meta, raw in rows)
    return result


def packet() -> bytes:
    """Freeze metadata and exact encodings with bounded record lengths; e.g. packet()."""
    rows = cases()
    return b'MPK1' + struct.pack('<I', len(rows)) + b''.join(struct.pack('<BBQI', control, kind, meta, len(raw)) + raw for control, kind, meta, raw in rows)


def nested_pairs(control: int) -> bytes:
    """Pin repeated recursive keys and mixed extension/float values; e.g. nested_pairs(0)."""
    key = b'\x92\xa2\0\xff' + integer(control, 256)
    value = b'\x82\xc3\xc7\x03\xff\x01\0\x02\xc3\xca' + word(control, 0x7f800001, 4)
    return b'\x82' + key + value + key + b'\xc0'


def verify_packet() -> None:
    """Reproduce all frozen bytes and native-kind/control coverage; e.g. python fixture_bytes.py."""
    raw = packet()
    reference = json.loads(Path(__file__).with_name('fixture-reference.json').read_text(encoding='utf-8'))
    assert raw == (ROOT / 'tests/fixtures/deferred-values/cases.bin').read_bytes()
    assert hashlib.sha256(raw).hexdigest() == reference['packet_sha256']
    assert len(cases()) == reference['cases']
    assert {kind for _, kind, _, _ in cases()} == set(range(17))
    for control in (0, 1):
        blob = nested_pairs(control)
        assert blob == (ROOT / f'tests/fixtures/deferred-values/nested-{control}.bin').read_bytes()
        assert hashlib.sha256(blob).hexdigest() == reference['nested_sha256'][str(control)]


if __name__ == '__main__':
    verify_packet()
