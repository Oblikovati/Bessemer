"""Authenticated reference components and independent count/bit/presence requests."""
from __future__ import annotations
import hashlib
import json
import checkpoint_wire as wire
Case = tuple[str, str, str, bytes]


def digest(raw: bytes) -> str:
    return hashlib.sha256(raw).hexdigest()


def normalize(value: object) -> object:
    return wire.controls.definitions.objects.normalize(value)


def reference_cases() -> tuple[list[Case], dict]:
    folder = wire.SOURCE / 'research'
    path = folder / 'flow-checkpoint-components-native-evidence.json'
    evidence = json.loads(path.read_text(encoding='utf-8'))
    synthetic = folder / 'flow-checkpoint-components-synthetic.json'
    assert digest(synthetic.read_bytes()) == evidence['synthetic_sha256']
    for name, expected in evidence['files_sha256'].items():
        assert digest((wire.SOURCE / name).read_bytes().replace(b'\r\n', b'\n')) == expected
    rows = []
    for entry in json.loads(synthetic.read_text(encoding='utf-8')):
        kind = 'chunks' if entry['name'] == 'chunk_framing_only' else entry['name']
        raw = bytes.fromhex(entry['source_hex'])
        assert digest(raw) == entry['sha256'] and normalize(wire.READERS[kind](raw)) == entry['projection']
        assert entry['synthetic'] and not entry['native_encoded_fixture']
        rows.append((f"reference-{kind}", kind, kind, raw))
    return rows, dict(evidence_sha256=digest(path.read_bytes()), synthetic_sha256=digest(synthetic.read_bytes()), limitations=evidence['limitations'])


def graph1_cases() -> list[Case]:
    rows = []
    for position in range(7):
        for pattern in wire.WIDE_PATTERNS:
            values = [0] * 7
            values[position] = pattern
            rows.append((f'graph1-{position}-{pattern:016x}', 'graph1', 'graph1', wire.words(values, 8)))
    return rows


def dimension_cases(kind: str) -> list[Case]:
    rows = []
    for axis in range(3):
        for count in wire.COUNTS:
            dimensions = [0, 0, 0]
            dimensions[axis] = count
            raw = wire.grid(dimensions, [-0x80000000, -1, 0x7fffffff], wire.empty_groups(dimensions))
            rows.append((f'{kind}-dimension-{axis}-{count}', kind, kind, raw))
    return rows


def row_cases(kind: str) -> list[Case]:
    rows = []
    for axis in range(3):
        for count in wire.COUNTS:
            indices = [(n * 0x80000001) & 0xffffffff for n in range(count)]
            body = wire.row(indices, bytes(count) if kind == 'graph2' else b'')
            rows.append((f'{kind}-row-{axis}-{count}', kind, kind, wire.one_row(axis, body)))
    return rows


def presence_cases(kind: str) -> list[Case]:
    rows = []
    for flag in range(256):
        groups = [(bytes([flag]), [wire.row([])] if flag else []) for _ in range(3)]
        rows.append((f'{kind}-presence-{flag}', kind, kind, wire.grid([1, 1, 1], [4, 5, 6], groups)))
    return rows


def member_cases(kind: str) -> list[Case]:
    rows = []
    for position in range(3):
        for pattern in wire.WORD_PATTERNS:
            members = [0, 0, 0]
            members[position] = wire.signed_pattern(pattern)
            raw = wire.one_row(1, wire.row([]), members=members)
            rows.append((f'{kind}-member-{position}-{pattern:08x}', kind, kind, raw))
    return rows


def tag_cases() -> list[Case]:
    rows = []
    for tag in range(256):
        body = wire.integers([0]) if tag not in (0, 1) else b''
        raw = wire.one_row(2, wire.row([0xffffffff], bytes([tag]) + body))
        rows.append((f'graph2-tag-{tag}', 'graph2', 'graph2', raw))
    return rows


def container_count_cases() -> list[Case]:
    rows = []
    for position in range(3):
        for count in wire.COUNTS:
            body = wire.patterned_container(count)
            raw = wire.one_row(position, wire.row([7], b'\2' + body))
            rows.append((f'graph2-container-{position}-{count}', 'graph2-containers', 'graph2', raw))
            containers = [wire.integers([0])] * 3
            containers[position] = body
            rows.append((f'trailing-container-{position}-{count}', 'trailing', 'trailing', b''.join(containers)))
    return rows


def container_pattern_cases() -> list[Case]:
    rows = []
    for position in range(4):
        patterns = wire.WORD_PATTERNS if position == 1 else wire.WIDE_PATTERNS
        for pattern in patterns:
            values = [0, 0, 0, 0]
            values[position] = pattern
            body = wire.container(values[:1], values[1:2], values[2:])
            raw = wire.one_row(0, wire.row([0x80000000], b'\xff' + body))
            rows.append((f'container-pattern-{position}-{pattern:016x}', 'graph2-containers', 'graph2', raw))
    return rows


def chunk_cases() -> list[Case]:
    rows = []
    for count in wire.COUNTS[1:]:
        rows.append((f'chunks-count-{count}', 'chunks', 'chunks', wire.chunks([b''] * count, 0xffffffffffffffff)))
    for count in wire.COUNTS:
        encoded = bytes(n % 256 for n in range(count))
        rows.append((f'chunks-length-{count}', 'chunks', 'chunks', wire.chunks([encoded, b'', b'\0\xff'], 1)))
    for pattern in wire.WIDE_PATTERNS:
        rows.append((f'chunks-declared-{pattern:016x}', 'chunks', 'chunks', wire.chunks([b'', bytes(range(256))], pattern)))
    return rows


def cases() -> list[Case]:
    observed, _ = reference_cases()
    rows = observed + graph1_cases() + tag_cases() + container_count_cases() + container_pattern_cases() + chunk_cases()
    for kind in ('graph2', 'auxiliary'):
        rows.extend(dimension_cases(kind) + row_cases(kind) + presence_cases(kind) + member_cases(kind))
    assert len({name for name, _, _, _ in rows}) == len(rows)
    return rows
