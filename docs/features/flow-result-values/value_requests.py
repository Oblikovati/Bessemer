"""Complete paired IMS values with explicit fresh-definition provenance."""
from __future__ import annotations
import json
import struct
import sys
import zlib
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parent.parent / 'flow-result-definitions'))
sys.path.insert(0, str(Path(__file__).resolve().parent.parent / 'flow-aux-wide-values'))
import definition_requests as definitions
import wide_requests as wides
import flowresultvalues as reference
ROOT = definitions.ROOT
SOURCE = definitions.SOURCE


def context(identifiers: list[int], lengths: tuple[int, int] = (0, 0)) -> bytes:
    arrays = [list(range(lengths[0])), list(range(lengths[1])), [], []]
    body = definitions.definition(2, arrays, 0)
    return zlib.compress(definitions.decoded([(identifier, body) for identifier in identifiers]))


def shared(values: list[int]) -> bytes:
    return struct.pack('<iQ', 1, len(values)) + struct.pack('<' + 'Q' * len(values), *values)


def value_body(flags: tuple[int, int, int], arrays: list[list[int]], rows: list[list[int]]) -> bytes:
    assert len(arrays) == 3 and all(len(row) == 18 for row in rows)
    raw = struct.pack('<iB', 3, flags[0])
    if flags[0]:
        raw += shared(arrays[0])
    raw += shared(arrays[1]) + bytes([flags[1]])
    if flags[1]:
        raw += struct.pack('<iQ', 0, len(rows)) + b''.join(struct.pack('<18Q', *row) for row in rows)
    return raw + bytes([flags[2]]) + (shared(arrays[2]) if flags[2] else b'')


def parse(definitions_stored: bytes, values_stored: bytes) -> dict:
    return reference.read_record(dict(identifier=501, size=len(values_stored), payload=values_stored),
                                 definitions=definitions.parse(definitions_stored), fresh_objects=True)


def native_cases() -> tuple[list[tuple[str, bool, bytes, bytes]], dict]:
    definition_rows, definition_proof = definitions.native_cases()
    manifest = SOURCE / 'research/flow-result-pair-corpus.json'
    row = next(row for row in json.loads(manifest.read_text(encoding='utf-8')) if row['identifier'] == 501)
    source = Path(row['path']).read_bytes()
    assert definitions.objects.digest(source) == row['source_sha256']
    record = definitions.objects.flowresults.latest(definitions.objects.flowresults.read(source), 501)
    assert (record['start'], record['end']) == (row['record_start'], row['record_end'])
    stored = bytes(record['payload'])
    assert stored == source[row['payload_start']:row['record_end']]
    assert definitions.objects.digest(stored) == row['payload_sha256']
    definitions_stored = definition_rows[0][2]
    assert definitions.objects.normalize(parse(definitions_stored, stored)) == row['projection']
    proof = {key: row[key] for key in ('source_sha256', 'payload_sha256', 'record_start', 'record_end', 'stored_bytes')}
    proof['definitions'] = definition_proof
    return [('native-fld-paired-ims-values', True, definitions_stored, stored)], proof


def flag_cases() -> list[tuple[str, bool, bytes, bytes]]:
    rows = []
    definition_stored = context([0])
    for position in range(3):
        for value in range(256):
            flags = [0, 0, 0]
            flags[position] = value
            body = value_body(tuple(flags), [[], [0], []], [])
            rows.append((f'flag-{position}-{value}', False, definition_stored, zlib.compress(b'\2' + body)))
    return rows


def pattern_cases() -> list[tuple[str, bool, bytes, bytes]]:
    rows = []
    definition_stored = context([0])
    for position in range(21):
        for pattern in wides.RAW_PATTERNS:
            body = pattern_body(position, pattern)
            rows.append((f'pattern-{position}-{pattern:016x}', False, definition_stored, zlib.compress(b'\2' + body)))
    return rows


def pattern_body(position: int, pattern: int) -> bytes:
    arrays = [[0], [0], [0]]
    bl_row = [0] * 18
    if position < 3:
        arrays[position][0] = pattern
    else:
        bl_row[position - 3] = pattern
    return value_body((1, 1, 1), arrays, [bl_row])


def object_cases() -> list[tuple[str, bool, bytes, bytes]]:
    rows = []
    for count in (0, 1, 2, 3, 16, 64, 257):
        identifiers = [7 if ordinal % 2 == 0 else 0xffffffff for ordinal in range(count)]
        bodies = [value_body((ordinal % 2, 0, ordinal % 3), [[], [ordinal], []], []) for ordinal in range(count)]
        rows.append((f'objects-{count}', False, context(identifiers), zlib.compress(b'\2' + b''.join(bodies))))
    for pattern in definitions.PATTERNS:
        body = value_body((0, 0, 0), [[], [pattern], []], [])
        rows.append((f'identifier-{pattern:08x}', False, context([pattern], (3, 7)), zlib.compress(b'\2' + body)))
    return rows


def length_cases() -> list[tuple[str, bool, bytes, bytes]]:
    rows = []
    definition_stored = context([0])
    for position in range(3):
        for count in (0, 1, 2, 3, 7, 8, 15, 16, 31, 32, 63, 64, 255, 256, 257, 65535, 65536):
            arrays = [[], [], []]
            arrays[position] = [(index * 0x9e3779b97f4a7c15) & 0xffffffffffffffff for index in range(count)]
            body = value_body((255, 0, 128), arrays, [])
            rows.append((f'length-{position}-{count}', False, definition_stored, zlib.compress(b'\2' + body)))
    for count in (0, 1, 2, 3, 16, 64, 257, 4096):
        bl_rows = [[(ordinal * 18 + column) ^ 0x8000000000000000 for column in range(18)] for ordinal in range(count)]
        body = value_body((0, 128, 0), [[], [], []], bl_rows)
        rows.append((f'bl-rows-{count}', False, definition_stored, zlib.compress(b'\2' + body)))
    return rows


def compression_cases() -> list[tuple[str, bool, bytes, bytes]]:
    definition_stored = context([0])
    raw = b'\2' + value_body((255, 128, 254), [[0, 1], [2], [3]], [list(range(18))])
    rows = [(f'level-{level}', False, definition_stored, zlib.compress(raw, level)) for level in range(10)]
    for strategy in (zlib.Z_DEFAULT_STRATEGY, zlib.Z_FILTERED, zlib.Z_HUFFMAN_ONLY, zlib.Z_RLE, zlib.Z_FIXED):
        for window in (9, 15):
            encoder = zlib.compressobj(6, zlib.DEFLATED, window, 8, strategy)
            rows.append((f'compression-{strategy}-{window}', False, definition_stored, encoder.compress(raw) + encoder.flush()))
    return rows


def cases() -> list[tuple[str, bool, bytes, bytes]]:
    native, _ = native_cases()
    return native + flag_cases() + pattern_cases() + object_cases() + length_cases() + compression_cases()
