"""Native complete-file cases with independently addressed literal value spans (#230)."""
import struct
from index_reference import inspect_default_file
from bucket_fixtures import ExpectedCell, native_bucket_bytes


def value_spans(raw: bytes, physical: int, cell: ExpectedCell) -> tuple[tuple[int, int], ...]:
    """Bind complete value bytes to physical intervals; e.g. value_spans(raw,2,cell)."""
    if cell.first == 0:
        start = physical * 4096 + cell.offset + 26 + len(cell.key)
        return ((start, start + len(cell.value)),)
    current, cursor = struct.unpack_from('>QH', raw, cell.first * 4096 + 8)
    remaining = len(cell.value)
    spans: list[tuple[int, int]] = []
    while remaining:
        count = min(remaining, 4096 - cursor)
        start = current * 4096 + cursor
        spans.append((start, start + count))
        remaining -= count
        current = struct.unpack_from('>Q', raw, current * 4096)[0]
        cursor = 8
    assert current == 0
    assert b''.join(raw[start:end] for start, end in spans) == cell.value
    return tuple(spans)


def corpus_cell(raw: bytes, physical: int, cell: ExpectedCell) -> bytes:
    """Frame persisted metadata, complete payloads and value intervals; e.g. corpus_cell(raw,2,cell)."""
    spans = value_spans(raw, physical, cell)
    header = struct.pack('<HQIIQ', cell.offset, physical, cell.key_hash, len(cell.key), len(cell.value))
    locations = struct.pack('<I', len(spans)) + b''.join(struct.pack('<QQ', *span) for span in spans)
    return header + cell.key + cell.value + locations


def corpus_file(raw: bytes, ordinal: int, physical: tuple[int, ...]) -> tuple[bytes, dict[str, int]]:
    """Check all roles before framing every cell in the file; e.g. corpus_file(raw,0,(2,))."""
    checked = inspect_default_file(raw)
    buckets = tuple(native_bucket_bytes(raw, identity) for identity in physical)
    cells = tuple(corpus_cell(raw, identity, cell)
                  for identity, bucket in zip(physical, buckets) for cell in bucket.cells)
    assert len(cells) == checked['cells']
    header = struct.pack('<III', ordinal, len(raw), len(cells))
    return header + raw + b''.join(cells), checked
