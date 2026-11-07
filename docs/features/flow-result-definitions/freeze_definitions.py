"""Freeze complete IMS-definition wire framing and reference projections."""
import gzip
import json
import struct
import sys
from pathlib import Path
import definition_requests as requests
sys.path.insert(0, str(Path(__file__).resolve().parent.parent / 'flow-field-values'))
from wire_packet import blob, packed


def array_packet(value: dict | None) -> bytes:
    if value is None:
        return b'\0'
    return b'\1' + struct.pack('<IIiI', value['start'], value['end'], value['marker'], value['count']) + blob(packed(value['patterns'], 'I'))


def object_packet(value: dict) -> bytes:
    body = value['definition']
    result = struct.pack('<i', value['identifier_i32']) + blob(value['guid_raw'])
    result += struct.pack('<IIi', body['start'], body['end'], body['version'])
    flag = body['allocation_byte']
    result += b'\0' if flag is None else bytes([1, flag])
    arrays = [body['first_array'], body['second_array']] + body['extra_arrays']
    arrays += [None] * (4 - len(arrays))
    return result + b''.join(array_packet(array) for array in arrays)


def group_packet(group: dict) -> bytes:
    return struct.pack('<III', group['start'], group['end'], group['count']) + b''.join(object_packet(value) for value in group['objects'])


def case_packet(name: str, native: bool, stored: bytes) -> bytes:
    value = requests.parse(stored)
    result = blob(name.encode()) + bytes([native]) + blob(stored) + blob(requests.reference.inflate(stored))
    result += bytes([value['version']]) + struct.pack('<I', len(value['groups']))
    result += b''.join(group_packet(group) for group in value['groups'])
    return result + struct.pack('<I', value['end']) + bytes([value['complete_decoded_framing'], value['native_runtime_state_reproduced']])


def freeze(root: Path = requests.ROOT) -> None:
    rows = requests.cases()
    _, native = requests.native_cases()
    packet = b'FDF1' + struct.pack('<I', len(rows)) + b''.join(case_packet(*row) for row in rows)
    assert len(packet) <= 16_000_000
    folder = root / 'tests/fixtures/flow-result-definitions'
    folder.mkdir(parents=True, exist_ok=True)
    (folder / 'record500.bin.gz').write_bytes(gzip.compress(packet, mtime=0))
    proof = dict(cases=len(rows), bytes=len(packet), sha256=requests.objects.digest(packet), native=native,
                 references={name: requests.objects.digest((requests.SOURCE / name).read_bytes().replace(b'\r\n', b'\n'))
                             for name in ('flowresultdefinitions.py', 'flowlegacytables.py')},
                 scope='Complete version-one record500 / IMS versions zero through three; secondary collections empty',
                 compression=dict(build=requests.zlib.ZLIB_VERSION, runtime=requests.zlib.ZLIB_RUNTIME_VERSION),
                 native_runtime_state_reproduced=False, engineering_fields_assigned=False, native_application_checked=False)
    path = root / 'docs/features/flow-result-definitions/baseline-verification.json'
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(proof, indent=2) + '\n', encoding='utf-8', newline='\n')


if __name__ == '__main__':
    freeze()
