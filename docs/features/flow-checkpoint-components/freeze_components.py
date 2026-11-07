"""Freeze complete source projections in six independently bounded FCC1 packets."""
from __future__ import annotations
import gzip
import json
import struct
import sys
from pathlib import Path
from collections.abc import Callable
from typing import TypeVar
import checkpoint_requests as requests
sys.path.insert(0, str(Path(__file__).resolve().parent.parent / 'flow-field-values'))
from wire_packet import blob, counted
T = TypeVar("T")


def sequence(values: list[T] | tuple[T, ...], encode: Callable[[T], bytes]) -> bytes:
    return struct.pack('<I', len(values)) + b''.join(encode(value) for value in values)


def optional(value: T | None, encode: Callable[[T], bytes]) -> bytes:
    return b'\0' if value is None else b'\1' + encode(value)


def container(value: dict) -> bytes:
    result = struct.pack('<III', value['start'], value['end'], value['count'])
    result += counted(value['first_wide_patterns'], 'Q') + counted(value['index_patterns'], 'I')
    return result + counted(value['second_wide_patterns'], 'Q')


def slot(value: dict) -> bytes:
    prefix = struct.pack('<IBB', value['ordinal'], value['tag_byte'], value['canonical_writer_tag'])
    return prefix + optional(value['container'], container)


def row(value: dict) -> bytes:
    result = struct.pack('<III', value['start'], value['end'], value['count']) + counted(value['index_patterns'], 'I')
    return result + optional(value['slots'], lambda values: sequence(values, slot))


def row_presence(value: dict) -> bytes:
    return struct.pack('<IB', value['ordinal'], value['present_byte']) + optional(value['row'], row)


def group(value: dict) -> bytes:
    return struct.pack('<II', value['ordinal'], value['length']) + blob(value['presence_raw']) + sequence(value['rows'], row_presence)


def grid(value: dict) -> bytes:
    result = struct.pack('<II', value['start'], value['end'])
    result += requests.wire.integers(value['dimensions']) + requests.wire.integers(value['member_triplet_i32'])
    return result + sequence(value['groups'], group)


def chunk(value: dict) -> bytes:
    return struct.pack('<II', value['ordinal'], value['stored_bytes']) + blob(value['encoded_bytes'])


def chunk_buffer(value: dict) -> bytes:
    result = struct.pack('<iIQ', value['version'], value['chunk_count'], value['declared_raw_bytes'])
    return result + sequence(value['chunks'], chunk) + bytes([value['codec_decoded'], value['decoded_size_verified']])


def projection(kind: str, value: dict) -> bytes:
    result = struct.pack('<IBBB', value['end'], value['complete_component_framing'], value['complete_parent_record'], value['engineering_fields_assigned'])
    if kind == 'graph1':
        return result + counted(value['patterns'], 'Q')
    if kind in ('graph2', 'auxiliary'):
        return result + grid(value)
    if kind == 'trailing':
        return result + sequence(value['groups'], container)
    return result + chunk_buffer(value)


def case_packet(case: requests.Case) -> bytes:
    name, _, kind, raw = case
    parsed = requests.wire.READERS[kind](raw)
    assert parsed['end'] == len(raw) and parsed['complete_component_framing']
    assert not parsed['complete_parent_record'] and not parsed['engineering_fields_assigned']
    return blob(name.encode()) + bytes([requests.wire.KINDS[kind]]) + blob(raw) + projection(kind, parsed)


def write_packet(root: Path, rows: list[requests.Case], family: str) -> dict:
    selected = [case for case in rows if case[1] == family]
    raw = b'FCC1' + struct.pack('<I', len(selected)) + b''.join(case_packet(case) for case in selected)
    assert len(raw) <= 16_000_000, (family, len(raw))
    path = root / 'tests/fixtures/flow-checkpoint-components' / f'{family}.bin.gz'
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(gzip.compress(raw, mtime=0))
    return dict(cases=len(selected), bytes=len(raw), sha256=requests.digest(raw))


def freeze(root: Path = requests.wire.ROOT) -> None:
    rows = requests.cases()
    _, evidence = requests.reference_cases()
    families = ('graph1', 'graph2', 'graph2-containers', 'auxiliary', 'trailing', 'chunks')
    proof = dict(cases=len(rows), packets={family: write_packet(root, rows, family) for family in families}, reference=evidence,
                 source_lf_sha256=requests.digest((requests.wire.SOURCE / 'flowcheckpointcomponents.py').read_bytes().replace(b'\r\n', b'\n')),
                 source_dependencies_lf_sha256={name: requests.digest((requests.wire.SOURCE / name).read_bytes().replace(b'\r\n', b'\n')) for name in ('flowmetadataprefix.py', 'flowlegacytables.py', 'swarchive.py')},
                 compression=dict(build=requests.wire.controls.zlib.ZLIB_VERSION, runtime=requests.wire.controls.zlib.ZLIB_RUNTIME_VERSION),
                 installed_native_component_fixtures=False, complete_parent_record=False, chunk_codec_decoded=False,
                 native_execution=False, engineering_fields_assigned=False)
    path = root / 'docs/features/flow-checkpoint-components/baseline-verification.json'
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(proof, indent=2) + '\n', encoding='utf-8', newline='\n')


if __name__ == '__main__':
    freeze()
