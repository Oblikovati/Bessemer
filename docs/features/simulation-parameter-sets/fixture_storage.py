"""Lossless owned-fixture packing; complete independent golden bytes remain authoritative (#133)."""
import struct
from owned_persistence import word, span

RawCase = tuple[bytes, bytes, bytes, bytes]


def take_span(raw: bytes, cursor: int) -> tuple[bytes, int]:
    """Read exactly one bounded trusted golden span and its following byte position."""
    extent = struct.unpack_from('<I', raw, cursor)[0]
    end = cursor + 4 + extent
    assert end <= len(raw)
    return raw[cursor + 4:end], end


def cases(raw: bytes) -> list[RawCase]:
    """Retain every original caller/header/result byte, without interpreting case semantics."""
    assert raw[:4] == b'PARM'
    count = struct.unpack_from('<I', raw, 4)[0]
    cursor, entries = 8, []
    for _ in range(count):
        header = raw[cursor:cursor + 28]
        cursor += 28
        original, cursor = take_span(raw, cursor)
        replacement, cursor = take_span(raw, cursor)
        expected, cursor = take_span(raw, cursor)
        entries.append((header, original, replacement, expected))
    assert cursor == len(raw)
    return entries


def xor_equal(first: bytes, second: bytes) -> bytes:
    """Bijective fixed-length XOR, preserving every byte including leading/trailing zeros."""
    assert len(first) == len(second)
    return (int.from_bytes(first, 'little') ^ int.from_bytes(second, 'little')).to_bytes(len(first), 'little')


def pack(raw: bytes) -> bytes:
    """Share complete baselines only within identical input contexts and expected lengths."""
    bases: list[bytes] = []
    indices: dict[tuple[bytes, int], int] = {}
    entries = []
    for header, original, replacement, expected in cases(raw):
        key = (header[:16] + original, len(expected))
        if key not in indices:
            indices[key] = len(bases)
            bases.append(expected)
        index = indices[key]
        delta = xor_equal(expected, bases[index])
        entries.append(header + span(original) + span(replacement) + word(index) + span(delta))
    return b'PADX' + word(2) + word(len(raw)) + word(len(bases)) + b''.join(span(base) for base in bases) + word(len(entries)) + b''.join(entries)


def unpack(storage: bytes) -> bytes:
    """Recover the complete original PARM packet for byte-for-byte comparison."""
    assert storage[:8] == b'PADX\x02\0\0\0'
    expanded_size, base_count = struct.unpack_from('<2I', storage, 8)
    cursor, bases = 16, []
    for _ in range(base_count):
        base, cursor = take_span(storage, cursor)
        bases.append(base)
    count = struct.unpack_from('<I', storage, cursor)[0]
    cursor += 4
    entries, cursor = unpack_cases(storage, cursor, bases, count)
    assert cursor == len(storage)
    raw = b'PARM' + word(count) + b''.join(entries)
    assert len(raw) == expanded_size
    return raw


def unpack_cases(storage: bytes, cursor: int, bases: list[bytes], count: int) -> tuple[list[bytes], int]:
    """Expand every exact result from its stored complete baseline and full XOR pattern."""
    entries = []
    for _ in range(count):
        header = storage[cursor:cursor + 28]
        cursor += 28
        original, cursor = take_span(storage, cursor)
        replacement, cursor = take_span(storage, cursor)
        index = struct.unpack_from('<I', storage, cursor)[0]
        cursor += 4
        delta, cursor = take_span(storage, cursor)
        expected = xor_equal(delta, bases[index])
        entries.append(header + span(original) + span(replacement) + span(expected))
    return entries, cursor
