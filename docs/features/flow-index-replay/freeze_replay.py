"""Freeze canonical unsigned-key maps and exact logical recognition outcomes."""
import gzip
import json
import struct
import sys
from pathlib import Path
import index_requests as requests
sys.path.insert(0, str(Path(__file__).resolve().parent.parent / 'flow-field-values'))
from wire_packet import blob


def active_packet(active: dict[int, dict]) -> bytes:
    return struct.pack('<I', len(active)) + b''.join(struct.pack('<IQQ', key, value['record_start'], value['record_span']) for key, value in sorted(active.items()))


def saved_packet(saved: dict[int, int]) -> bytes:
    return struct.pack('<I', len(saved)) + b''.join(struct.pack('<QQ', key, value) for key, value in sorted(saved.items()))


def terminal_packet(value: dict | None, physical: dict) -> bytes:
    if value is None:
        return b'\0'
    record = physical['records'][-1]
    stored = record['payload']
    decoded = requests.wire.controls.zlib.decompress(stored[8:8 + value['compressed_bytes']])
    entries = requests.wire.controls.index_body([(e['identifier'], e['record_start'], e['record_span']) for e in value['entries']])[4:]
    result = b'\1' + struct.pack('<Ii', len(physical['records']) - 1, value['count']) + blob(decoded) + blob(entries)
    result += struct.pack('<IIIIQI', value['decoded_end'], value['compressed_bytes'], value['stored_bytes'], value['footer_start'], value['backward_distance'], value['footer_identifier'])
    return result + bytes([value['length_width'], value['complete_payload_framing']])


def outcome_packet(raw: bytes, forced: bool, physical: dict) -> bytes:
    outcome = requests.outcome(raw, forced)
    if not outcome['completed']:
        return bytes([forced, False]) + blob(outcome['cause'].encode())
    value = outcome['projection']
    result = bytes([forced, True, value['method'] == 'terminal_index'])
    result += active_packet(value['active_index']) + saved_packet(value['saved_spans'])
    result += terminal_packet(value['terminal_index'], physical)
    flags = ('ordinary_payloads_decoded', 'indexed_ordinary_targets_validated', 'complete_document_schema')
    return result + struct.pack('<HI', value['version'], value['physical_record_count']) + bytes(value[key] for key in flags)


def case_packet(name: str, family: str, raw: bytes) -> bytes:
    physical = requests.wire.controls.definitions.objects.flowresults.read(raw)
    result = blob(name.encode()) + blob(family.encode()) + blob(raw) + struct.pack('<H', physical['version'])
    result += struct.pack('<I', len(physical['records']))
    result += b''.join(struct.pack('<IQQ', row['identifier'], row['start'], row['end'] - row['start']) for row in physical['records'])
    return result + outcome_packet(raw, False, physical) + outcome_packet(raw, True, physical)


def write_packet(root: Path, rows: list[tuple[str, str, bytes]], family: str) -> dict:
    selected = [row for row in rows if row[1] == family]
    packet = b'FLR1' + struct.pack('<I', len(selected)) + b''.join(case_packet(*row) for row in selected)
    assert len(packet) <= 16_000_000, (family, len(packet))
    path = root / 'tests/fixtures/flow-index-replay' / f'{family}.bin.gz'
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(gzip.compress(packet, mtime=0))
    return dict(cases=len(selected), bytes=len(packet), sha256=requests.wire.controls.definitions.objects.digest(packet))


def freeze(root: Path = requests.ROOT) -> None:
    rows = requests.cases()
    _, reference = requests.reference_cases()
    packets = {family: write_packet(root, rows, family) for family in ('native', 'scan7', 'scan8', 'terminal')}
    proof = dict(cases=len(rows), packets=packets, reference=reference,
                 references={name: requests.wire.controls.definitions.objects.digest((requests.SOURCE / name).read_bytes().replace(b'\r\n', b'\n')) for name in ('flowindex.py', 'flowresults.py', 'flowcontrolrecords.py')},
                 map_order='Unsigned key order for deterministic map comparison; raw control-entry order is preserved separately',
                 compression=dict(build=requests.wire.controls.zlib.ZLIB_VERSION, runtime=requests.wire.controls.zlib.ZLIB_RUNTIME_VERSION),
                 native_terminal_payload_fixtures=False, native_execution=False, indexed_ordinary_targets_validated=False, complete_document_schema=False)
    path = root / 'docs/features/flow-index-replay/baseline-verification.json'
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(proof, indent=2) + '\n', encoding='utf-8', newline='\n')


if __name__ == '__main__':
    freeze()
