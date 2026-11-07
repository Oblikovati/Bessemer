"""Freeze complete paired IMS-value projections without engineering column assignments."""
import gzip
import json
import struct
import sys
from pathlib import Path
import value_requests as requests
sys.path.insert(0, str(Path(__file__).resolve().parent.parent / 'flow-field-values'))
from wire_packet import blob, packed


def array_packet(array: dict) -> bytes:
    return struct.pack('<IIiI', array['start'], array['end'], array['marker'], array['count']) + blob(packed(array['patterns'], 'Q'))


def optional_array(array: dict | None) -> bytes:
    return b'\0' if array is None else b'\1' + array_packet(array)


def bl_packet(value: dict | None) -> bytes:
    if value is None:
        return b'\0'
    words = [pattern for row in value['rows'] for pattern in row]
    header = struct.pack('<IIiIB', value['start'], value['end'], value['version'], value['count'], value['native_state_byte_after_load'])
    return b'\1' + header + blob(packed(words, 'Q'))


def object_packet(value: dict) -> bytes:
    result = struct.pack('<iIIiB', value['identifier_i32'], value['start'], value['end'], value['version'], value['first_flag_byte'])
    result += optional_array(value['optional_538']) + array_packet(value['main_418'])
    result += bytes([value['bl_flag_byte']]) + bl_packet(value['bl'])
    return result + bytes([value['last_flag_byte']]) + optional_array(value['optional_690'])


def case_packet(name: str, native: bool, definitions_stored: bytes, stored: bytes) -> bytes:
    context = requests.definitions.parse(definitions_stored)
    value = requests.parse(definitions_stored, stored)
    identities = [obj['identifier_i32'] for obj in context['groups'][0]['objects']]
    result = blob(name.encode()) + bytes([native]) + blob(definitions_stored)
    result += blob(requests.definitions.reference.inflate(definitions_stored)) + blob(packed(identities, 'i'))
    result += blob(stored) + blob(requests.reference.inflate(stored)) + bytes([value['version']])
    result += struct.pack('<I', len(value['objects'])) + b''.join(object_packet(obj) for obj in value['objects'])
    flags = ('complete_decoded_framing', 'fresh_objects_assumed', 'native_runtime_state_reproduced', 'engineering_columns_assigned')
    return result + struct.pack('<I', value['end']) + bytes(value[key] for key in flags)


def freeze(root: Path = requests.ROOT) -> None:
    rows = requests.cases()
    _, native = requests.native_cases()
    packet = b'FIV1' + struct.pack('<I', len(rows)) + b''.join(case_packet(*row) for row in rows)
    assert len(packet) <= 16_000_000, len(packet)
    folder = root / 'tests/fixtures/flow-result-values'
    folder.mkdir(parents=True, exist_ok=True)
    (folder / 'record501.bin.gz').write_bytes(gzip.compress(packet, mtime=0))
    proof = dict(cases=len(rows), bytes=len(packet), sha256=requests.definitions.objects.digest(packet), native=native,
                 references={name: requests.definitions.objects.digest((requests.SOURCE / name).read_bytes().replace(b'\r\n', b'\n'))
                             for name in ('flowresultvalues.py', 'flowresultdefinitions.py', 'flowlegacytables.py')},
                 compression=dict(build=requests.zlib.ZLIB_VERSION, runtime=requests.zlib.ZLIB_RUNTIME_VERSION),
                 helper_lf_sha256={Path(module.__file__).name: requests.definitions.objects.digest(Path(module.__file__).read_bytes().replace(b'\r\n', b'\n')) for module in (requests.definitions, requests.wides, requests.wides.objects)},
                 scope='Complete version-two record501 / version-three IMS values and BL version-zero rows; explicit fresh version-two allocation-zero definitions',
                 native_runtime_state_reproduced=False, engineering_columns_assigned=False, native_application_checked=False)
    path = root / 'docs/features/flow-result-values/baseline-verification.json'
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(proof, indent=2) + '\n', encoding='utf-8', newline='\n')


if __name__ == '__main__':
    freeze()
