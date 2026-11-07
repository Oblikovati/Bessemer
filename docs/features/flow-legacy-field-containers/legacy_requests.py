"""Owned raw fields, scalar comparisons and shared one-dimensional fallback grammar."""
from __future__ import annotations
import hashlib
import json
import struct
import sys
from itertools import product
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parent.parent / 'flow-conditional-items'))
import requests as conditional
assert Path(conditional.__file__).resolve().parent.name == 'flow-conditional-items'
import snapshot
import freeze_items
from wire_packet import ROOT, SOURCE, packed
import flowlegacyfields as reference
from swarchive import ArchiveError
WIDE = tuple(dict.fromkeys([*conditional.patterns.PATTERNS, 0xffffffffffffffff]))
WORDS = tuple(dict.fromkeys(value & 0xffffffff for value in WIDE))
COUNTS = (0, 1, 2, 3, 16, 64, 257, 4096, 65536)
WordCase = tuple[str, int, bytes]


def digest(raw: bytes) -> str:
    return hashlib.sha256(raw).hexdigest()


def native_cases() -> tuple[list[WordCase], dict]:
    folder = SOURCE / 'research'
    evidence_path = folder / 'flow-legacy-fields-native-evidence.json'
    evidence = json.loads(evidence_path.read_text())
    manifest = folder / 'flow-legacy-fields-corpus.json'
    assert digest(manifest.read_bytes()) == evidence['corpus_sha256']
    for name, expected in evidence['files_sha256'].items():
        assert digest((SOURCE / name).read_bytes().replace(b'\r\n', b'\n')) == expected
    rows = [authenticate(row, index) for index, row in enumerate(json.loads(manifest.read_text()))]
    return rows, dict(evidence_sha256=digest(evidence_path.read_bytes()), corpus_sha256=digest(manifest.read_bytes()),
                      scalar_constants=evidence['scalar_constants'], limitations=evidence['limitations'])


def authenticate(row: dict, index: int) -> WordCase:
    raw = Path(row['path']).read_bytes()
    assert digest(raw) == row['source_sha256']
    start, end, identifier = row['record_start'], row['record_end'], row['identifier']
    stored = raw[start + 12:end]
    assert struct.unpack_from('<IQ', raw, start) == (identifier, len(stored))
    assert digest(stored) == row['payload_sha256']
    parsed = reference.read_record(dict(identifier=identifier, size=len(stored), payload=stored))
    assert json.loads(json.dumps(parsed)) == row['projection']
    queries = reference.queries_105(parsed) if identifier == 105 else None
    assert queries == row['scalar_queries']
    return f'native-{identifier}-{index}', identifier, stored


def word_cases() -> list[WordCase]:
    rows, _ = native_cases()
    for position, pattern in product((0, 7, 10, 319), WIDE):
        values = [0] * 320
        values[position] = pattern
        rows.append((f'fixed-{position}-{pattern:016x}', 105, packed(values, 'Q')))
    for position in range(320):
        values = [0] * 320
        values[position] = 1 << (position % 64)
        rows.append((f'walking-position-{position}', 105, packed(values, 'Q')))
    for count in COUNTS:
        values = [WIDE[index % len(WIDE)] for index in range(count)]
        rows.append((f'counted-{count}', 113, struct.pack('<i', count) + packed(values, 'Q')))
    for pattern in WIDE:
        rows.append((f'counted-pattern-{pattern:016x}', 113, struct.pack('<iQ', 1, pattern)))
    return rows


def query_domain() -> tuple[int, ...]:
    boundaries = [struct.unpack('<Q', struct.pack('<d', value))[0] for value in (253.0, 565.0)]
    adjacent = [bits + offset for bits in boundaries for offset in (-2, -1, 0, 1, 2)]
    nans = [sign | 0x7ff0000000000000 | fraction for sign, fraction in product((0, 1 << 63), [1 << bit for bit in range(52)] + [((1 << 52) - 1) ^ (1 << bit) for bit in range(52)])]
    return tuple(dict.fromkeys([*WIDE, *adjacent, *nans, 0x8000000000000001, 0xffefffffffffffff]))


def query(slot7: int, slot10: int) -> dict:
    patterns = [0] * 320
    patterns[7], patterns[10] = slot7, slot10
    return reference.queries_105(reference.read_105(packed(patterns, 'Q')))


def group_raw(spec: conditional.GroupSpec) -> bytes:
    return struct.pack('<i', 0) + conditional.group(spec, 1)


def group_snapshot(raw: bytes) -> snapshot.Snapshot:
    parsed = reference.read_155(raw)
    result = snapshot.Snapshot()
    with result.scope((), dict(start=0, end=parsed['end'])):
        result.scalar(('version',), parsed['version'])
        snapshot.group(result, ('group',), parsed['group'])
    assert bytes(result.raw) == raw
    return result


def group_active_cases() -> list[tuple[str, bytes]]:
    rows = []
    for version, active in product(range(6), range(256)):
        item = conditional.ItemSpec(version=version, active=active, mode=0 if version == 1 else active ^ 255, seed=active)
        spec = conditional.GroupSpec(presence=active or 1, items=(None, item))
        rows.append((f'active-v{version}-{active}', group_raw(spec)))
    for flag in range(256):
        items = (conditional.ItemSpec(),) if flag else (None,)
        rows.append((f'presence-{flag}', group_raw(conditional.GroupSpec(presence=flag or 1, items=items))))
    return rows


def group_branch_cases() -> list[tuple[str, bytes]]:
    rows = []
    gates = ((0, 0), (0, 1), (0, 0xffffffff), (1, 0), (2, 0))
    for version, (mode, gate), count in product(range(6), gates, (0, 1, 3)):
        item = conditional.ItemSpec(version=version, groups=count, wide_count=count, byte_count=count, word_count=count)
        rows.append((f'branch-{version}-{mode}-{gate}-{count}', group_raw(conditional.GroupSpec(mode=mode, gate=gate, items=(None, item)))))
    for pattern in WORDS:
        raw = bytearray(group_raw(conditional.GroupSpec(mode=0, gate=pattern)))
        struct.pack_into('<I', raw, 8, pattern)
        rows.append((f'header-pattern-{pattern:08x}', bytes(raw)))
    return rows


def group_boundary_cases() -> list[tuple[str, bytes]]:
    rows = []
    for version, kind in product(range(2, 6), (-0x80000000, -1, 0, 1, 2, 3, 0x7fffffff)):
        item = conditional.ItemSpec(version=version, subkind=kind)
        rows.append((f'subkind-{version}-{kind}', group_raw(conditional.GroupSpec(items=(item,)))))
    for count in COUNTS:
        item = conditional.ItemSpec(wide_count=count, byte_count=count, word_count=count)
        rows.append((f'large-{count}', group_raw(conditional.GroupSpec(items=(item,)))))
    return rows


def group_cases() -> list[tuple[str, bytes]]:
    return group_active_cases() + group_branch_cases() + group_boundary_cases()


def selection_cases() -> list[tuple[str, list[tuple[int, bytes]], dict]]:
    choices = [(113, struct.pack('<i', 0)), (113, b''), (113, struct.pack('<i', -1)),
               (155, group_raw(conditional.GroupSpec(mode=0, gate=0))), (155, b''), (155, struct.pack('<i', 1)), (999, b'opaque')]
    rows = []
    for count in range(4):
        for ordinal, records in enumerate(product(choices, repeat=count)):
            selected = selection(list(records))
            rows.append((f'selection-{count}-{ordinal}', list(records), selected))
    return rows


def selection(records: list[tuple[int, bytes]]) -> dict:
    document = dict(records=[dict(identifier=identifier, size=len(raw), payload=raw) for identifier, raw in records])
    try:
        result = reference.select_113_or_155(document)
        return dict(completed=True, identifier=None if result is None else result['identifier'], cause='')
    except ArchiveError as error:
        return dict(completed=False, identifier=None, cause=str(error))
