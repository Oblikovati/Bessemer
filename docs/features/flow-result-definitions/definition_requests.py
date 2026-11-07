"""Owned complete record-500 IMS definitions, authenticated against the native pair."""
from __future__ import annotations
import json
import struct
import sys
import zlib
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parent.parent / 'flow-object-records'))
import object_requests as objects
ROOT = objects.ROOT
SOURCE = objects.SOURCE
import flowresultdefinitions as reference
PATTERNS = [0, 0xffffffff] + [value for bit in range(32) for value in (1 << bit, 0xffffffff ^ (1 << bit))]


def shared(values: list[int]) -> bytes:
    return struct.pack('<iQ', 1, len(values)) + struct.pack('<' + 'I' * len(values), *values)


def definition(version: int, arrays: list[list[int]], flag: int = 0) -> bytes:
    assert len(arrays) == 4 and 0 <= version <= 3
    raw = struct.pack('<i', version)
    if version:
        raw += shared(arrays[0])
    if version >= 2:
        raw += shared(arrays[1]) + bytes([flag])
    if version == 3:
        raw += shared(arrays[2]) + shared(arrays[3])
    return reference.IMS_GUID_RAW + raw


def decoded(entries: list[tuple[int, bytes]]) -> bytes:
    identifiers = b''.join(struct.pack('<I', identifier & 0xffffffff) for identifier, _ in entries)
    return b'\1' + struct.pack('<i', len(entries)) + identifiers + b''.join(body for _, body in entries) + bytes(16)


def parse(stored: bytes) -> dict:
    return reference.read_record(dict(identifier=500, size=len(stored), payload=stored))


def native_cases() -> tuple[list[tuple[str, bool, bytes]], dict]:
    manifest = SOURCE / 'research/flow-result-pair-corpus.json'
    evidence = json.loads((SOURCE / 'research/flow-result-pair-native-evidence.json').read_text(encoding='utf-8'))
    assert objects.digest(manifest.read_bytes()) == evidence['corpus_sha256']
    rows = [row for row in json.loads(manifest.read_text(encoding='utf-8')) if row['identifier'] == 500]
    assert len(rows) == 1
    row = rows[0]
    source = Path(row['path']).read_bytes()
    assert objects.digest(source) == row['source_sha256']
    record = objects.flowresults.latest(objects.flowresults.read(source), 500)
    assert (record['start'], record['end']) == (row['record_start'], row['record_end'])
    stored = bytes(record['payload'])
    assert stored == source[row['payload_start']:row['record_end']]
    assert objects.digest(stored) == row['payload_sha256']
    assert objects.normalize(parse(stored)) == row['projection']
    proof = {key: row[key] for key in ('source_sha256', 'payload_sha256', 'record_start', 'record_end', 'stored_bytes')}
    proof['manifest_sha256'] = objects.digest(manifest.read_bytes())
    return [('native-cfld-ims-definitions', True, stored)], proof


def signed_cases() -> list[tuple[str, bool, bytes]]:
    rows = []
    for version in range(4):
        body = definition(version, [[0, 0xffffffff], [7], [], [0x80000000]], 255)
        rows.extend((f'identifier-{version}-{pattern:08x}', False, zlib.compress(decoded([(pattern, body)]))) for pattern in PATTERNS)
    return rows


def pattern_cases() -> list[tuple[str, bool, bytes]]:
    rows = []
    for ordinal in range(4):
        for pattern in PATTERNS:
            arrays = [[], [], [], []]
            arrays[ordinal] = [pattern]
            rows.append((f'pattern-{ordinal}-{pattern:08x}', False, zlib.compress(decoded([(0, definition(3, arrays))]))))
    return rows


def flag_cases() -> list[tuple[str, bool, bytes]]:
    return [(f'flag-{version}-{flag}', False, zlib.compress(decoded([(0, definition(version, [[], [], [], []], flag))])))
            for version in (2, 3) for flag in range(256)]


def shape_cases() -> list[tuple[str, bool, bytes]]:
    rows = []
    for count in (0, 1, 2, 3, 16, 64, 257):
        entries = [(0xffffffff if index % 2 else 7, definition(index % 4, [[], [], [], []])) for index in range(count)]
        rows.append((f'objects-{count}', False, zlib.compress(decoded(entries))))
    for ordinal in range(4):
        for count in (0, 1, 2, 3, 7, 8, 15, 16, 31, 32, 63, 64, 255, 256, 257, 65535, 65536):
            arrays = [[], [], [], []]
            arrays[ordinal] = [(index * 0x9e3779b9) & 0xffffffff for index in range(count)]
            rows.append((f'length-{ordinal}-{count}', False, zlib.compress(decoded([(0, definition(3, arrays))]))))
    return rows


def compression_cases() -> list[tuple[str, bool, bytes]]:
    raw = decoded([(0, definition(3, [[0, 1, 0xffffffff]] * 4, 128))])
    rows = [(f'level-{level}', False, zlib.compress(raw, level)) for level in range(10)]
    for strategy in (zlib.Z_DEFAULT_STRATEGY, zlib.Z_FILTERED, zlib.Z_HUFFMAN_ONLY, zlib.Z_RLE, zlib.Z_FIXED):
        for window in (9, 15):
            encoder = zlib.compressobj(6, zlib.DEFLATED, window, 8, strategy)
            rows.append((f'compression-{strategy}-{window}', False, encoder.compress(raw) + encoder.flush()))
    return rows


def cases() -> list[tuple[str, bool, bytes]]:
    native, _ = native_cases()
    return native + signed_cases() + pattern_cases() + flag_cases() + shape_cases() + compression_cases()
