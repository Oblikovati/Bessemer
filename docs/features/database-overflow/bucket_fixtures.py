"""Independent complete mixed-bucket images and literal native cell payloads (#229)."""
from dataclasses import dataclass
from pathlib import Path
import struct
from writer_pages import image_matrix
from native_packet import native_value


@dataclass(frozen=True)
class ExpectedCell:
    offset: int
    key_hash: int
    following: int
    first: int
    key: bytes
    value: bytes


@dataclass(frozen=True)
class BucketCase:
    extent: int
    rejected: bool
    bucket: bytes
    pages: tuple[tuple[int, bytes], ...]
    free: tuple[tuple[int, int, int], ...]
    cells: tuple[ExpectedCell, ...]


def owned_buckets() -> tuple[BucketCase, ...]:
    """Combine literal inline cells with every independent overflow image; e.g. owned_buckets()."""
    cases: list[BucketCase] = []
    for image in image_matrix():
        bucket = bytearray(image.bucket)
        bucket[80:106] = bucket[20:46]
        struct.pack_into('>HHQ', bucket, 0, 20, 12, 0)
        struct.pack_into('>HH', bucket, 12, 0, 65535)
        struct.pack_into('>IIQHQ', bucket, 20, 0x12345678, 3, 4, 80, 0)
        bucket[46:53] = b'k\0\xffv\0\x80\xfe'
        cells = (ExpectedCell(20, 0x12345678, 80, 0, b'k\0\xff', b'v\0\x80\xfe'),
                 ExpectedCell(80, 0x80000001, 0, image.pages[0][0], image.key, image.value))
        cases.append(BucketCase(image.extent, image.cursor_wrapped, bytes(bucket), image.pages,
                                ((12, 0, 65535),), cells))
    return tuple(cases)


def bucket_controls() -> tuple[BucketCase, ...]:
    """Keep empty and inline-only complete buckets independent; e.g. bucket_controls()."""
    source = owned_buckets()[0]
    inline = bytearray(source.bucket)
    struct.pack_into('>H', inline, 36, 0)
    cell = ExpectedCell(20, 0x12345678, 0, 0, b'k\0\xff', b'v\0\x80\xfe')
    empty = bytearray(inline)
    struct.pack_into('>H', empty, 0, 0)
    return (BucketCase(512, False, bytes(inline), (), source.free, (cell,)),
            BucketCase(512, False, bytes(empty), (), source.free, ()))


def native_free_records(bucket: bytes) -> tuple[tuple[int, int, int], ...]:
    """Follow native BE16 free links with exact metadata spans; e.g. native_free_records(bucket)."""
    offset = int.from_bytes(bucket[2:4], 'big')
    seen: set[int] = set()
    records: list[tuple[int, int, int]] = []
    while offset:
        assert offset not in seen and 12 <= offset <= len(bucket) - 4
        seen.add(offset)
        following, size = struct.unpack_from('>HH', bucket, offset)
        records.append((offset, following, size))
        offset = following
    return tuple(records)


def native_cell(raw: bytes, bucket: bytes, offset: int) -> ExpectedCell:
    """Use direct native inline/first-key spans and value checkpoints; e.g. native_cell(raw,bucket,12)."""
    key_hash, key_length, value_length, following, first = struct.unpack_from('>IIQHQ', bucket, offset)
    if first == 0:
        start = offset + 26
        key = bucket[start:start + key_length]
        value = bucket[start + key_length:start + key_length + value_length]
    else:
        assert key_length <= 4096 - 18
        start = first * 4096 + 18
        key = raw[start:start + key_length]
        value = native_value(raw, 4096, first, value_length)
    assert len(key) == key_length and len(value) == value_length
    return ExpectedCell(offset, key_hash, following, first, key, value)


def native_bucket(index: int, fixtures: Path) -> BucketCase:
    """Retain one explicit native bucket at8192 with its referenced pages; e.g. native_bucket(2,root)."""
    raw = (fixtures / f'native-{index}.db').read_bytes()
    return native_bucket_bytes(raw, 2)


def native_bucket_bytes(raw: bytes, page_index: int) -> BucketCase:
    """Read a selected4096-byte native bucket, without file-wide role inference; e.g. native_bucket_bytes(raw,2)."""
    bucket = raw[page_index * 4096:(page_index + 1) * 4096]
    offset = int.from_bytes(bucket[:2], 'big')
    cells: list[ExpectedCell] = []
    seen: set[int] = set()
    while offset:
        assert offset not in seen and 12 <= offset <= 4096 - 26
        seen.add(offset)
        cell = native_cell(raw, bucket, offset)
        cells.append(cell)
        offset = cell.following
    return BucketCase(4096, False, bucket, native_pages(raw, cells), native_free_records(bucket), tuple(cells))


def native_pages(raw: bytes, cells: list[ExpectedCell]) -> tuple[tuple[int, bytes], ...]:
    """Own every referenced native chain once, excluding other buckets; e.g. native_pages(raw,cells)."""
    pages: dict[int, bytes] = {}
    for cell in cells:
        identity = cell.first
        while identity:
            assert identity not in pages and 0 < identity < len(raw) // 4096
            page = raw[identity * 4096:(identity + 1) * 4096]
            pages[identity] = page
            identity = int.from_bytes(page[:8], 'big')
    return tuple(sorted(pages.items()))


def encode_bucket(case: BucketCase) -> bytes:
    """Frame independent metadata/payload expectations in BWB1; e.g. encode_bucket(case)."""
    header = struct.pack('<IBIII', case.extent, case.rejected, len(case.pages), len(case.free), len(case.cells))
    pages = b''.join(struct.pack('<Q', identity) + page for identity, page in case.pages)
    free = b''.join(struct.pack('<HHH', *record) for record in case.free)
    cells = b''.join(struct.pack('<HIHQIQ', c.offset, c.key_hash, c.following, c.first,
                                len(c.key), len(c.value)) + c.key + c.value for c in case.cells)
    return header + case.bucket + pages + free + cells


def bucket_packet(fixtures: Path) -> bytes:
    """Build all122 owned/control/native bucket images; e.g. bucket_packet(fixtures)."""
    cases = owned_buckets() + bucket_controls() + tuple(native_bucket(i, fixtures) for i in range(3))
    records = tuple(encode_bucket(case) for case in cases)
    return b'BWB1' + struct.pack('<I', len(records)) + b''.join(
        struct.pack('<I', len(record)) + record for record in records)
