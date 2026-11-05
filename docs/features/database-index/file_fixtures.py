"""Owned full-file mutation expectations independent of the Rust codecs (#230)."""
from dataclasses import dataclass
import struct
from hash_reference import default_hash


@dataclass(frozen=True)
class FileValueCase:
    bucket: int
    cell: int
    key: bytes
    value: bytes
    spans: tuple[tuple[int, int], ...]


def inline_cell(raw: bytearray, page: int, key: bytes, value: bytes) -> FileValueCase:
    """Place one literal inline cell at12; e.g. inline_cell(raw,2,b'a',b'value')."""
    base = page * 4096
    struct.pack_into('>H', raw, base, 12)
    struct.pack_into('>IIQHQ', raw, base + 12, default_hash(key), len(key), len(value), 0, 0)
    start = base + 38
    raw[start:start + len(key)] = key
    start += len(key)
    raw[start:start + len(value)] = value
    return FileValueCase(page, 12, key, value, ((start, start + len(value)),))


def file_header(raw: bytearray) -> None:
    """Assign literal header/index/free/continuation roles; e.g. file_header(raw)."""
    raw[:11] = b"unqlite\xdb\x7c\x27\x12"
    struct.pack_into('>I', raw, 19, 4096)
    struct.pack_into('>IIQQQQI', raw, 4096, 0xfa782dcb, 0x03f523f5, 4, 0, 2, 3, 1)
    struct.pack_into('>QQ', raw, 4096 + 44, 0, 2)
    struct.pack_into('>QIQQ', raw, 3 * 4096, 0, 1, 1, 5)
    struct.pack_into('>Q', raw, 4 * 4096, 7)
    struct.pack_into('>Q', raw, 2 * 4096 + 4, 6)


def overflow_cell(raw: bytearray) -> FileValueCase:
    """Assign exact cross-page value spans and terminal links; e.g. overflow_cell(raw)."""
    key = b'e'
    value = bytes((index * 17 + 3) % 256 for index in range(5000))
    struct.pack_into('>H', raw, 2 * 4096 + 12 + 16, 80)
    struct.pack_into('>IIQHQ', raw, 2 * 4096 + 80, default_hash(key), 1, 5000, 0, 8)
    struct.pack_into('>QQH', raw, 8 * 4096, 9, 8, 19)
    raw[8 * 4096 + 18] = key[0]
    spans = ((8 * 4096 + 19, 9 * 4096), (9 * 4096 + 8, 9 * 4096 + 931))
    cursor = 0
    for start, end in spans:
        raw[start:end] = value[cursor:cursor + end - start]
        cursor += end - start
    assert cursor == len(value)
    return FileValueCase(2, 80, key, value, spans)


def complete_file() -> tuple[bytes, tuple[FileValueCase, ...]]:
    """Construct all selected page roles and four mutable values; e.g. complete_file()."""
    raw = bytearray(4096 * 10)
    file_header(raw)
    cells = (inline_cell(raw, 2, b'a', b'first\0value'),
             inline_cell(raw, 5, b'b', b'second\xffvalue'),
             inline_cell(raw, 6, b'c', b''), overflow_cell(raw))
    return bytes(raw), cells


def expected_edit(raw: bytes, cell: FileValueCase, replacement: bytes) -> bytes:
    """Apply only independently declared value spans; e.g. expected_edit(raw,cell,cell.value)."""
    assert len(replacement) == len(cell.value)
    result = bytearray(raw)
    cursor = 0
    for start, end in cell.spans:
        result[start:end] = replacement[cursor:cursor + end - start]
        cursor += end - start
    assert cursor == len(replacement)
    return bytes(result)
