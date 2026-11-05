"""Freeze complete native selected-chain inputs with independent expected spans (#229)."""
import hashlib
import struct
from pathlib import Path
from typing import TypedDict


class NativeCensusRow(TypedDict):
    """Typed retained cell boundary; e.g. row['extent'] is an integer page extent."""
    ordinal: int
    extent: int
    cell: list[int]
    key_page_ids: list[int]
    value_page_ids: list[int]
    database_sha256: str
    key_sha256: str
    value_sha256: str


def native_value(raw: bytes, extent: int, first: int, length: int) -> bytes:
    """Read the retained value checkpoint with bounded traversal; e.g. native_value(raw,4096,4,4380)."""
    page, cursor = struct.unpack_from('>QH', raw, first * extent + 8)
    visited: set[int] = set()
    pieces: list[bytes] = []
    remaining = length
    while remaining:
        assert 0 < page < len(raw) // extent and page not in visited
        assert 8 <= cursor <= extent
        visited.add(page)
        count = min(remaining, extent - cursor)
        pieces.append(raw[page * extent + cursor:page * extent + cursor + count])
        remaining -= count
        page = struct.unpack_from('>Q', raw, page * extent)[0]
        cursor = 8
    assert page == 0, f'tail link {page}; expected complete terminal chain'
    return b''.join(pieces)


def checked_payloads(row: NativeCensusRow, raw: bytes) -> tuple[bytes, bytes]:
    """Verify independent payload hashes from retained census fields; e.g. checked_payloads(row,raw)."""
    assert hashlib.sha256(raw).hexdigest() == row['database_sha256']
    extent = row['extent']
    _, _, key_length, value_length, _, first = row['cell']
    assert key_length in (3, 36), f'key length {key_length}; expected retained cohort3/36'
    key = raw[first * extent + 18:first * extent + 18 + key_length]
    value = native_value(raw, extent, first, value_length)
    assert hashlib.sha256(key).hexdigest() == row['key_sha256']
    assert hashlib.sha256(value).hexdigest() == row['value_sha256']
    return key, value


def native_record(row: NativeCensusRow, raw: bytes) -> bytes:
    """Encode one BOV1 native chain and independent payloads; e.g. native_record(row,raw)."""
    key, value = checked_payloads(row, raw)
    extent = row['extent']
    first = row['cell'][-1]
    ids = sorted(set(row['key_page_ids'] + row['value_page_ids']))
    header = struct.pack('<IIIQQI', row['ordinal'], extent, len(key), len(value), first, len(ids))
    pages = b''.join(struct.pack('<Q', identity) + raw[identity * extent:(identity + 1) * extent]
                     for identity in ids)
    return header + pages + key + value


def native_packet(rows: list[NativeCensusRow], image_root: Path) -> bytes:
    """Build from an injected image root, authenticating every source; e.g. native_packet(rows,root)."""
    assert len(rows) == 5140, f'census rows {len(rows)}; expected5140'
    records: list[bytes] = []
    images: dict[int, bytes] = {}
    for row in rows:
        ordinal = row['ordinal']
        if ordinal not in images:
            images[ordinal] = (image_root / f'{ordinal}.bin').read_bytes()
        record = native_record(row, images[ordinal])
        records.append(struct.pack('<I', len(record)) + record)
    return b'BOV1' + struct.pack('<I', len(rows)) + b''.join(records)
