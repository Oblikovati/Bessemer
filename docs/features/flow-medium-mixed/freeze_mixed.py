"""Freeze complete mixed-kind records with separate reference-key domains."""
from __future__ import annotations
import gzip
import hashlib
import json
import struct
import sys
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parent.parent / 'flow-medium-references'))
sys.path.insert(0, str(Path(__file__).resolve().parent.parent / 'flow-medium-kind-two'))
import freeze_profiles as profiles
import freeze_kind_two as two
from wire_packet import ROOT, SOURCE, blob, counted, optional


def shape(order: str, model: int) -> tuple[list[tuple[int, int, int]], list[tuple[int, int, list[int]]], list[int]]:
    if order == 'two-only':
        return [(2, 0, model), (2, 2, model)], [(2, 1, [2, 0, 2])], [1, 1, 1]
    if order == 'first-two-only':
        return [(2, 0, model)], [], []
    if order == 'empty-two-references':
        return [], [(2, 1, [])], [1, 1, 1]
    if order == 'empty-both-references':
        return [], [(2, 1, []), (1, 1, [])], [1, 1, 1]
    first = [(1, 0, 0), (2, 0, model), (1, 1, 0), (2, 2, model)]
    second = [(1, 1, [0, 1, 0]), (2, 1, [2, 0, 2])]
    if order == 'two-first':
        first.reverse()
        second.reverse()
    if order == 'duplicates':
        first += first[:2]
        second += second[:1]
    return first, second, [1, 1, 1]


def record(version: int, model: int, order: str, state: int = 255, name: str = 'empty', count: int = 0) -> bytes:
    first, second, outer = shape(order, model)
    raw = profiles.metadata.prefix(version, name, state) + b'\0\0' + struct.pack('<i', len(first))
    for ordinal, (kind, key, tag) in enumerate(first):
        body = profiles.objects.body(state, state ^ 165, count, state, b'\0\0', ordinal)
        if kind == 2:
            body = two.body(tag, count, state, ordinal, state, counts=(count, count + 1, count + 2))
        raw += struct.pack('<i', kind) + profiles.references.KEYS[key] + body
    raw += b'\0' + struct.pack('<i', len(second))
    for kind, key, refs in second:
        raw += struct.pack('<i', kind) + profiles.references.body(kind, state, [profiles.references.KEYS[index] for index in refs], profiles.references.KEYS[key])
    raw += struct.pack('<i', len(outer)) + b''.join(profiles.references.KEYS[key] for key in outer)
    return raw + (struct.pack('<iii', -2147483648, 0, 2147483647) if outer else b'') + bytes(8)


def cases() -> list[tuple[str, bytes]]:
    rows = [(f'native-{name}', raw) for name, raw in profiles.native_records(424)]
    rows += [(f'v{version}-tag{model}-{order}', record(version, model, order))
             for version in range(2, 128) for model in range(6) for order in ('one-first', 'two-first', 'duplicates')]
    rows += [(f'state-{state}', record(3, state % 6, 'two-first', state)) for state in range(256)]
    rows += [(f'name-v{version}-{name}', record(version, 5, 'duplicates', 128, name))
             for version in (2, 3, 127) for name in profiles.metadata.NAMES]
    rows += [(f'arrays-tag{model}-{count}', record(3, model, 'two-first', 255, count=count))
             for model in range(6) for count in (1, 3, 32)]
    rows += [(f'edge-tag{model}-{order}', record(3, model, order))
             for model in range(6) for order in ('two-only', 'first-two-only', 'empty-two-references', 'empty-both-references')]
    rows += [(f'kind-one-{name}', raw) for name, raw in profiles.cases() if not name.startswith('native-')]
    return rows


def read(raw: bytes) -> dict:
    return profiles.references.schema.read_record_424_liquid_profile(
        dict(identifier=424, size=len(raw), payload=raw), string_mode=0, fresh_backend=True)


def first_packet(entry: dict, raw: bytes) -> bytes:
    body = raw[entry['start'] + 20:entry['end']]
    out = struct.pack('<IIIi', entry['ordinal'], entry['start'], entry['end'], entry['kind']) + blob(entry['key_raw'])
    encode = profiles.objects.case_packet if entry['kind'] == 1 else two.case_packet
    return out + encode(f"first-{entry['ordinal']}", body)


def second_packet(entry: dict) -> bytes:
    body = entry['body']
    out = struct.pack('<IIIi', entry['ordinal'], entry['start'], entry['end'], entry['kind']) + blob(body['key_raw'])
    out += optional(body.get('state_byte'), 'B') + optional(body.get('native_state_after_load'), 'B')
    return out + profiles.references.key_list(body['references_raw'])


def case_packet(name: str, raw: bytes) -> bytes:
    value = read(raw)
    out = blob(name.encode()) + blob(raw) + blob(raw[:value['backend_start']])
    out += struct.pack('<II', value['backend_start'], value['backend_end'])
    out += struct.pack('<I', len(value['first_table'])) + b''.join(first_packet(entry, raw) for entry in value['first_table'])
    out += struct.pack('<I', len(value['second_table'])) + b''.join(map(second_packet, value['second_table']))
    out += profiles.references.key_list(value['outer_reference_keys_raw']) + counted(value['outer_attribute_i32'], 'i')
    out += struct.pack('<III', *value['final_collection_counts'], value['end'])
    return out + bytes(value[key] for key in ('complete_record_framing', 'full_engineering_schema', 'fresh_backend_assumed', 'native_runtime_state_reproduced', 'installed_kind2_fixture_validated'))


def freeze(output_root: Path = ROOT) -> None:
    rows = cases()
    packet = b'FML1' + struct.pack('<I', len(rows)) + b''.join(case_packet(*row) for row in rows)
    destination = output_root / 'tests/fixtures/flow-medium-mixed/profiles.bin.gz'
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_bytes(gzip.compress(packet, mtime=0))
    proof = dict(cases=len(rows), bytes=len(packet), sha256=hashlib.sha256(packet).hexdigest(),
                 reference_sha256=hashlib.sha256((SOURCE / 'flowmediumliquids.py').read_bytes()).hexdigest(),
                 scope='Complete kind-one/two record framing with independently decoded nested bodies, typed reference domains, duplicate physical entries, ordered outer references, signed attributes and exact EOF',
                 installed_kind2_fixture_validated=False, full_engineering_schema=False)
    report = output_root / 'docs/features/flow-medium-mixed/baseline-verification.json'
    report.parent.mkdir(parents=True, exist_ok=True)
    report.write_text(json.dumps(proof, indent=2) + '\n', encoding='utf-8', newline='\n')


if __name__ == '__main__':
    freeze()
