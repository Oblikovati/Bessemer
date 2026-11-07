"""Strict writer-derived control profiles; all payload examples are synthetic."""
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
import flowcontrolrecords as reference
ROOT = definitions.ROOT
SOURCE = definitions.SOURCE
SAVED = 0xffffff9c
INDEX = 0xffffff9b


def saved_body(entries: list[tuple[int, int]]) -> bytes:
    return struct.pack('<QQ', 0, len(entries)) + b''.join(struct.pack('<QQ', *entry) for entry in entries)


def index_body(entries: list[tuple[int, int, int]]) -> bytes:
    return struct.pack('<i', len(entries)) + b''.join(struct.pack('<IQQ', *entry) for entry in entries)


def index_stored(raw: bytes, width: int, level: int = 6) -> bytes:
    return wrap_index(zlib.compress(raw, level), width)


def wrap_index(compressed: bytes, width: int) -> bytes:
    distance = 4 + width + 8 + len(compressed)
    return bytes(8) + compressed + distance.to_bytes(width, 'little') + struct.pack('<I', INDEX)


def parse(identifier: int, width: int, stored: bytes) -> dict:
    record = dict(identifier=identifier, size=len(stored), payload=stored)
    return reference.read_record(record, length_width=width)


def reference_cases() -> tuple[list[tuple[str, bool, int, int, bytes]], dict]:
    path = SOURCE / 'research/flow-control-records-synthetic.json'
    evidence_path = SOURCE / 'research/flow-control-records-native-evidence.json'
    evidence = json.loads(evidence_path.read_text(encoding='utf-8'))
    assert definitions.objects.digest(path.read_bytes()) == evidence['synthetic_sha256']
    assert definitions.objects.digest((SOURCE / 'flowcontrolrecords.py').read_bytes().replace(b'\r\n', b'\n')) == evidence['files_sha256']['flowcontrolrecords.py']
    rows = []
    for ordinal, entry in enumerate(json.loads(path.read_text(encoding='utf-8'))):
        stored = bytes.fromhex(entry['bytes_hex'])
        assert definitions.objects.digest(stored) == entry['sha256']
        identifier, width = entry['identifier'], entry.get('length_width', 0)
        assert parse(identifier, width, stored) == entry['projection']
        rows.append((f'reference-synthetic-{ordinal}', True, identifier, width, stored))
    proof = dict(synthetic_sha256=evidence['synthetic_sha256'], evidence_sha256=definitions.objects.digest(evidence_path.read_bytes()),
                 limitations=evidence['limitations'], installed_fixture_audit=audit_installed(evidence['installed_fixture_audit']),
                 carrier_corpus_sha256=definitions.objects.digest((SOURCE / 'research/flow-result-carrier-corpus.json').read_bytes()))
    return rows, proof


def audit_installed(expected: list[dict]) -> list[dict]:
    corpus = json.loads((SOURCE / 'research/flow-result-carrier-corpus.json').read_text(encoding='utf-8'))
    expected_counts = {Path(row['path']): row['control_record_count'] for row in expected}
    result = []
    for entry in corpus:
        path = Path(entry['path'])
        raw = path.read_bytes()
        digest = definitions.objects.digest(raw)
        assert digest == entry['source_sha256']
        records = definitions.objects.flowresults.read(raw)['records']
        count = sum(row['identifier'] in (SAVED, INDEX) for row in records)
        assert count == expected_counts[path]
        result.append(dict(path=str(path), source_sha256=digest, control_record_count=count))
    assert {Path(row['path']) for row in result} == set(expected_counts)
    return result


def saved_cases() -> list[tuple[str, bool, int, int, bytes]]:
    rows = []
    for position in range(2):
        for pattern in wides.RAW_PATTERNS:
            entry = [0, 0]
            entry[position] = pattern
            rows.append((f'saved-pattern-{position}-{pattern:016x}', False, SAVED, 0, zlib.compress(saved_body([tuple(entry)]))))
    for count in counts():
        entries = [(index * 0x9e3779b97f4a7c15 & 0xffffffffffffffff, (index ^ 0x8000000000000000)) for index in range(count)]
        rows.append((f'saved-count-{count}', False, SAVED, 0, zlib.compress(saved_body(entries))))
    entries = [(7, 3), (7, 0), (0, 0xffffffffffffffff), (7, 1)]
    rows.append(('saved-unordered-duplicates', False, SAVED, 0, zlib.compress(saved_body(entries))))
    return rows


def counts() -> tuple[int, ...]:
    return (0, 1, 2, 3, 7, 8, 15, 16, 31, 32, 63, 64, 255, 256, 257, 65535, 65536)


def index_cases(width: int) -> list[tuple[str, bool, int, int, bytes]]:
    rows = []
    for pattern in definitions.PATTERNS:
        rows.append((f'index-{width}-identifier-{pattern:08x}', False, INDEX, width, index_stored(index_body([(pattern, 0, 0)]), width)))
    for position in (1, 2):
        for pattern in wides.RAW_PATTERNS:
            entry = [0, 0, 0]
            entry[position] = pattern
            rows.append((f'index-{width}-pattern-{position}-{pattern:016x}', False, INDEX, width, index_stored(index_body([tuple(entry)]), width)))
    for count in counts():
        entries = [(index % 3, index * 0x9e3779b97f4a7c15 & 0xffffffffffffffff, index ^ 0x8000000000000000) for index in range(count)]
        rows.append((f'index-{width}-count-{count}', False, INDEX, width, index_stored(index_body(entries), width)))
    entries = [(12, 123, 25), (12, 9, 0), (0xffffff9d, 0xffffffffffffffff, 1), (12, 0, 0xffffffffffffffff)]
    rows.append((f'index-{width}-unordered-duplicates', False, INDEX, width, index_stored(index_body(entries), width)))
    return rows


def compression_cases(identifier: int, width: int) -> list[tuple[str, bool, int, int, bytes]]:
    raw = saved_body([(7, 0xffffffffffffffff)]) if identifier == SAVED else index_body([(12, 7, 0xffffffffffffffff)])
    rows = []
    for level in range(10):
        compressed = zlib.compress(raw, level)
        stored = compressed if identifier == SAVED else wrap_index(compressed, width)
        rows.append((f'compression-{identifier}-{width}-level-{level}', False, identifier, width, stored))
    for strategy in (zlib.Z_DEFAULT_STRATEGY, zlib.Z_FILTERED, zlib.Z_HUFFMAN_ONLY, zlib.Z_RLE, zlib.Z_FIXED):
        for window in (9, 15):
            encoder = zlib.compressobj(6, zlib.DEFLATED, window, 8, strategy)
            compressed = encoder.compress(raw) + encoder.flush()
            stored = compressed if identifier == SAVED else wrap_index(compressed, width)
            rows.append((f'compression-{identifier}-{width}-{strategy}-{window}', False, identifier, width, stored))
    return rows


def cases() -> list[tuple[str, bool, int, int, bytes]]:
    observed, _ = reference_cases()
    return observed + saved_cases() + index_cases(4) + index_cases(8) + compression_cases(SAVED, 0) + compression_cases(INDEX, 4) + compression_cases(INDEX, 8)
