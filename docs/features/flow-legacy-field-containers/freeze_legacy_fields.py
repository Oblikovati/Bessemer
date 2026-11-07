"""Freeze raw word layouts, scalar-query outcomes and complete record155 field ledgers."""
from __future__ import annotations
import gzip
import json
import struct
from pathlib import Path
import legacy_requests as requests
from wire_packet import blob, counted, optional


def query_packet(value: dict) -> bytes:
    return bytes([value['slot10_nonzero'], value['slot7_calls_virtual_78']]) + optional(value['slot7_precheck_status'], 'i') + bytes([value['native_fp_faults_reproduced'], value['native_virtual_result_reproduced']])


def word_packet(case: requests.WordCase) -> bytes:
    name, identifier, raw = case
    parsed = requests.reference.read_record(dict(identifier=identifier, size=len(raw), payload=raw))
    query = None if identifier != 105 else requests.reference.queries_105(parsed)
    result = blob(name.encode()) + struct.pack('<I', identifier) + blob(raw)
    result += counted(parsed['patterns'], 'Q') + struct.pack('<I', parsed['end'])
    return result + (b'\0' if query is None else b'\1' + query_packet(query)) + b'\0\0'


def scalar_packet(slot7: int, slot10: int) -> bytes:
    return struct.pack('<QQ', slot7, slot10) + query_packet(requests.query(slot7, slot10))


def group_packet(name: str, raw: bytes) -> bytes:
    value = requests.group_snapshot(raw)
    result = blob(name.encode()) + struct.pack('<IiI', 155, 0, 1) + optional(None, 'i') + blob(raw) + blob(raw)
    result += struct.pack('<I', len(value.fields)) + b''.join(requests.freeze_items.field_packet(field) for field in value.fields)
    result += struct.pack('<I', len(value.scopes)) + b''.join(requests.freeze_items.path_bytes(path) + struct.pack('<II', start, end) for path, start, end in value.scopes)
    result += counted(value.counts, 'I')
    result += struct.pack('<I', len(value.projections)) + b''.join(requests.freeze_items.path_bytes(path) + bytes([flag]) for path, flag in value.projections)
    return result + b'\1\0\0'


def selection_packet(name: str, records: list[tuple[int, bytes]], result: dict) -> bytes:
    prefix = blob(name.encode()) + struct.pack('<I', len(records))
    prefix += b''.join(struct.pack('<I', identifier) + blob(raw) for identifier, raw in records)
    return prefix + bytes([result['completed']]) + optional(result['identifier'], 'I') + blob(result['cause'].encode())


def write_packet(root: Path, name: str, magic: bytes, rows: list[bytes]) -> dict:
    raw = magic + struct.pack('<I', len(rows)) + b''.join(rows)
    assert len(raw) <= 16_000_000, (name, len(raw))
    path = root / 'tests/fixtures/flow-legacy-field-containers' / f'{name}.bin.gz'
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(gzip.compress(raw, mtime=0))
    return dict(cases=len(rows), bytes=len(raw), sha256=requests.digest(raw))


def freeze(root: Path = requests.ROOT) -> None:
    words = requests.word_cases()
    domain = requests.query_domain()
    groups = requests.group_cases()
    packets = dict(words=write_packet(root, 'words', b'FLW1', [word_packet(row) for row in words]),
                   queries=write_packet(root, 'queries', b'FLQ1', [scalar_packet(left, right) for left in domain for right in domain]),
                   groups=write_packet(root, 'groups', b'FCI1', [group_packet(*row) for row in groups]),
                   selections=write_packet(root, 'selections', b'FLS1', [selection_packet(*row) for row in requests.selection_cases()]))
    _, native = requests.native_cases()
    proof = dict(packets=packets, query_domain_patterns=len(domain), native_reference=native,
                 references={name: requests.digest((requests.SOURCE / name).read_bytes().replace(b'\r\n', b'\n')) for name in ('flowlegacyfields.py', 'flowrecord126.py', 'flowrecord315.py', 'flowlegacytables.py')},
                 reused_helpers={name: requests.digest((requests.ROOT / 'docs/features/flow-conditional-items' / name).read_bytes().replace(b'\r\n', b'\n')) for name in ('requests.py', 'snapshot.py', 'freeze_items.py')},
                 queries='All domain pairs; completed comparison branches only, without native faults or virtual calls',
                 native_record155_payload=False, native_execution=False, engineering_fields_assigned=False)
    path = root / 'docs/features/flow-legacy-field-containers/baseline-verification.json'
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(proof, indent=2) + '\n', encoding='utf-8', newline='\n')


if __name__ == '__main__':
    freeze()
