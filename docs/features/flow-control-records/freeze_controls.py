"""Freeze separate saved-span and terminal-index writer profiles with exact footers."""
import gzip
import json
import struct
import sys
from pathlib import Path
import control_requests as requests
sys.path.insert(0, str(Path(__file__).resolve().parent.parent / 'flow-field-values'))
from wire_packet import blob, optional


def entry_bytes(identifier: int, entries: list[dict]) -> bytes:
    if identifier == requests.SAVED:
        return b''.join(struct.pack('<QQ', entry['key_bits'], entry['value_bits']) for entry in entries)
    return b''.join(struct.pack('<IQQ', entry['identifier'], entry['record_start'], entry['record_span']) for entry in entries)


def case_packet(name: str, observed: bool, identifier: int, width: int, stored: bytes) -> bytes:
    value = requests.parse(identifier, width, stored)
    compressed = value['compressed_bytes']
    decoded = requests.zlib.decompress(stored if identifier == requests.SAVED else stored[8:8 + compressed])
    result = blob(name.encode()) + bytes([observed]) + struct.pack('<IB', identifier, width)
    result += blob(stored) + blob(decoded) + struct.pack('<QQ', value['writer_marker'], value['count'])
    result += optional(value['count'] if identifier == requests.INDEX else None, 'i')
    result += blob(entry_bytes(identifier, value['entries'])) + struct.pack('<II', compressed, value['decoded_end'])
    if identifier == requests.SAVED:
        result += b'\0'
    else:
        result += b'\1' + struct.pack('<IQI', value['footer_start'], value['backward_distance'], value['footer_identifier'])
    return result + bytes([value['complete_payload_framing']])


def write_packet(root: Path, rows: list[tuple], identifier: int, name: str, magic: bytes) -> dict:
    selected = [row for row in rows if row[2] == identifier]
    packet = magic + struct.pack('<I', len(selected)) + b''.join(case_packet(*row) for row in selected)
    assert len(packet) <= 16_000_000, (name, len(packet))
    path = root / 'tests/fixtures/flow-control-records' / name
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(gzip.compress(packet, mtime=0))
    return dict(cases=len(selected), bytes=len(packet), sha256=requests.definitions.objects.digest(packet))


def freeze(root: Path = requests.ROOT) -> None:
    rows = requests.cases()
    _, reference = requests.reference_cases()
    packets = {name: write_packet(root, rows, identifier, name, magic) for identifier, name, magic in
               ((requests.SAVED, 'saved-spans.bin.gz', b'FCS1'), (requests.INDEX, 'terminal-index.bin.gz', b'FCI1'))}
    proof = dict(cases=len(rows), packets=packets, reference=reference,
                 source_lf_sha256=requests.definitions.objects.digest((requests.SOURCE / 'flowcontrolrecords.py').read_bytes().replace(b'\r\n', b'\n')),
                 carrier_reader_lf_sha256=requests.definitions.objects.digest((requests.SOURCE / 'flowresults.py').read_bytes().replace(b'\r\n', b'\n')),
                 helper_lf_sha256={Path(module.__file__).name: requests.definitions.objects.digest(Path(module.__file__).read_bytes().replace(b'\r\n', b'\n')) for module in (requests.definitions, requests.wides, requests.wides.objects)},
                 compression=dict(build=requests.zlib.ZLIB_VERSION, runtime=requests.zlib.ZLIB_RUNTIME_VERSION),
                 scope='Strict zero-marker writer profiles; both footer widths explicit; raw ordered entries without adoption or target reconciliation',
                 native_payload_fixtures=False, native_execution=False, target_spans_validated=False, application_checked=False)
    path = root / 'docs/features/flow-control-records/baseline-verification.json'
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(proof, indent=2) + '\n', encoding='utf-8', newline='\n')


if __name__ == '__main__':
    freeze()
