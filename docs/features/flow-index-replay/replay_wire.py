"""Owned physical carriers and exact modern terminal framing for logical replay."""
import struct
import sys
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parent.parent / 'flow-control-records'))
import control_requests as controls
ROOT = controls.ROOT
SOURCE = controls.SOURCE
INDEX, SAVED, DELETED = controls.INDEX, controls.SAVED, 0xffffff9d


def record(identifier: int, payload: bytes) -> bytes:
    return struct.pack('<IQ', identifier, len(payload)) + payload


def carrier(records: list[tuple[int, bytes]], version: int = 8) -> bytes:
    return struct.pack('<H', version) + b''.join(record(identifier, payload) for identifier, payload in records)


def terminal(records: list[tuple[int, bytes]], entries: list[tuple[int, int, int]], version: int = 8) -> bytes:
    return carrier(records + [(INDEX, controls.index_stored(controls.index_body(entries), 8))], version)


def saved(entries: list[tuple[int, int]]) -> bytes:
    return controls.zlib.compress(controls.saved_body(entries))


def footer_patch(raw: bytes, distance: int | None = None, identifier: int | None = None) -> bytes:
    result = bytearray(raw)
    if distance is not None:
        result[-12:-4] = struct.pack('<Q', distance & 0xffffffffffffffff)
    if identifier is not None:
        result[-4:] = struct.pack('<I', identifier)
    return bytes(result)


def matched_payload(decoded: bytes, compressed: bytes | None = None) -> bytes:
    member = controls.zlib.compress(decoded) if compressed is None else compressed
    return carrier([(7, b'A'), (INDEX, controls.wrap_index(member, 8))])
