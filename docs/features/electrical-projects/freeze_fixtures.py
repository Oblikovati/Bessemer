"""Freeze complete ZIP/member and lexical INI expectations for Rust consumers (#231)."""
import gzip
import hashlib
import json
from pathlib import Path
import struct
from archive_fixtures import ElectricalCase, ExpectedEntry, expected_entries, native_cases, owned_cases
from ini_reference import IniRecord, ini_records


def counted(payload: bytes) -> bytes:
    """Frame raw bytes without text conversion; e.g. counted(b'\0raw')."""
    return struct.pack('<I', len(payload)) + payload


def optional(payload: bytes | None) -> bytes:
    """Keep absent and empty lexical text distinct; e.g. optional(None)."""
    return struct.pack('<I', 0xffffffff) if payload is None else counted(payload)


def lexical_record(record: IniRecord) -> bytes:
    """Frame one exact lexical interpretation; e.g. lexical_record(record)."""
    return (bytes([record.kind]) + counted(record.raw) + optional(record.section)
            + optional(record.key) + optional(record.value))


def member_record(entry: ExpectedEntry) -> bytes:
    """Pin all payload bytes plus observed ZIP metadata; e.g. member_record(entry)."""
    payload = entry.payload
    header = struct.pack('<HHI', entry.flags, entry.method, entry.crc)
    record = counted(entry.name.encode('utf-8')) + header + counted(payload)
    record += counted(entry.extra) + counted(entry.comment)
    if entry.name != 'Archive.ini':
        return record + bytes([0])
    lexical = ini_records(payload)
    if lexical is None:
        return record + bytes([1])
    return record + bytes([2]) + struct.pack('<I', len(lexical)) + b''.join(map(lexical_record, lexical))


def case_record(case: ElectricalCase) -> bytes:
    """Pair original file bytes with independent complete member expectations; e.g. case_record(case)."""
    entries, comment = expected_entries(case)
    records = tuple(map(member_record, entries))
    header = counted(case.name.encode('utf-8')) + counted(case.raw)
    return header + counted(comment) + struct.pack('<I', len(records)) + b''.join(records)


def fixture_packet(fixtures: Path, manifest: Path) -> bytes:
    """Build four native and four owned complete cases; e.g. fixture_packet(fixtures,manifest)."""
    cases = native_cases(fixtures, manifest) + owned_cases()
    return b'ECP1' + struct.pack('<I', len(cases)) + b''.join(map(case_record, cases))


def freeze(fixtures: Path, manifest: Path) -> None:
    """Write deterministic original/member/INI goldens; e.g. freeze(fixtures,manifest)."""
    raw = fixture_packet(fixtures, manifest)
    compressed = gzip.compress(raw, mtime=0)
    (fixtures / 'archive-cases.bin.gz').write_bytes(compressed)
    proof = {'issue': 231, 'behavior_version': 'electrical-carrier-v1', 'cases': 8, 'native_archives': 4,
             'native_members': 49, 'native_sqlite_payloads': 24, 'native_ewg_payloads': 17,
             'raw_bytes': len(raw), 'raw_sha256': hashlib.sha256(raw).hexdigest(),
             'compressed_bytes': len(compressed), 'compressed_sha256': hashlib.sha256(compressed).hexdigest(),
             'expectations': 'Complete independent ZIP payloads/metadata and ASCII lexical records',
             'sqlite_ewg_semantics_validated': False, 'original_native_application_executed': False}
    (fixtures / 'archive-cases-integrity.json').write_text(json.dumps(proof, indent=2) + '\n')


if __name__ == '__main__':
    repository = Path(__file__).resolve().parents[3]
    freeze(repository / 'tests/fixtures/electrical-projects',
           Path(__file__).resolve().parent / 'native-sources.json')
