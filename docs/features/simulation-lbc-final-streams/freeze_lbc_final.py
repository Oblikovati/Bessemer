"""Freeze complete actual final-LBC readers and retained native payloads (#130)."""
from pathlib import Path
import hashlib
import json
import struct
import sys
import tempfile
import unittest
import zlib
from typing import Literal, TypedDict, cast

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
RESEARCH = ROOT.parent / 'inventor-interface/solidworks/research'
sys.path.insert(0, str(FEATURE.parent / 'model-archives'))
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot

LegacyRecord = tuple[int, int, int, int]
LegacyGroup = tuple[LegacyRecord, ...]
LegacyGroups = tuple[LegacyGroup, LegacyGroup, LegacyGroup]


class NameCarrier(TypedDict):
    kind: Literal['LbcListOrder']
    raw_hex: str
    sha256: str
    count: int
    text_payloads: list[str]


class LegacyCarrier(TypedDict):
    kind: Literal['MidsurfMap']
    raw_hex: str
    sha256: str
    groups: list[list[list[int]]]


def digest(raw: bytes) -> str:
    """Identify complete retained bytes, e.g. digest(b'')."""
    return hashlib.sha256(raw).hexdigest()


def baseline(name: str, value: object) -> None:
    """Require reviewed explicit baseline creation rather than replacing evidence silently."""
    path = FEATURE / name
    raw = (json.dumps(value, indent=2) + '\n').encode('utf-8')
    if '--write' in sys.argv:
        path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected unchanged reviewed baseline'


def carriers() -> list[NameCarrier | LegacyCarrier]:
    """Read the exact C-712 scope without substituting newer persistent-ID map bodies."""
    inventory = json.loads((RESEARCH / 'cosworks-lbc-final-native-evidence.json').read_bytes())
    assert inventory['evidence'] == 'C-712'
    return cast(list[NameCarrier | LegacyCarrier], inventory['rows'])


def native_names() -> None:
    """Compare every original count, text payload and complete reconstruction."""
    from cosworkspersist import read_lbc_order
    count, extent, strings = 0, 0, 0
    for row in carriers():
        if row['kind'] != 'LbcListOrder':
            continue
        raw = bytes.fromhex(row['raw_hex'])
        assert digest(raw) == row['sha256']
        record = read_lbc_order(raw)
        assert record.raw == raw and record.count_header == raw[:4]
        assert len(record.items) == row['count']
        assert [item.payload.hex() for item in record.items] == row['text_payloads']
        assert record.count_header + b''.join(item.header + item.payload for item in record.items) == raw
        count += 1
        extent += len(raw)
        strings += len(record.items)
    assert (count, strings, extent) == (336, 1137, 32396)
    baseline('native-name-reference.json', dict(streams=count, strings=strings, bytes=extent, leading_selection_string=False))


def native_maps() -> None:
    """Retain all five actual empty maps without claiming populated native records."""
    from cosworkspersist import read_legacy_midsurf_map
    count, extent = 0, 0
    for row in carriers():
        if row['kind'] != 'MidsurfMap':
            continue
        raw = bytes.fromhex(row['raw_hex'])
        assert digest(raw) == row['sha256']
        assert row['groups'] == [[], [], []]
        assert read_legacy_midsurf_map(raw) == ((), (), ())
        assert raw == bytes(12)
        count += 1
        extent += len(raw)
    assert (count, extent) == (5, 60)
    baseline('native-map-reference.json', dict(streams=count, bytes=extent, populated_native_records=0, persistent_id_map_decoded=False))


def literal_names() -> tuple[bytes, ...]:
    """Independent null, allocated-empty, odd and raw-surrogate text frames."""
    return (bytes(4), b'\x02\0\0\0\0\0', b'\x03\0\0\0\x81\0\0', b'\x08\0\0\0\0\xd8\0\0\xff\xdf\0\0')


def owned_names() -> None:
    """Prove no leading text is required and preserve duplicate/null/odd strings."""
    from cosworkspersist import read_lbc_order
    texts = literal_names()
    for selected in ((), (texts[0],), (texts[1], texts[0], texts[1]), texts):
        raw = struct.pack('<i', len(selected)) + b''.join(selected)
        record = read_lbc_order(raw, max_items=len(selected))
        assert len(record.items) == len(selected)
        assert [item.header + item.payload for item in record.items] == list(selected)
        assert record.count_header + b''.join(selected) == raw


def legacy_bytes(groups: LegacyGroups) -> bytes:
    """Encode the independent three-group/four-word literal grammar."""
    return b''.join(struct.pack('<i', len(group)) + b''.join(struct.pack('<4i', *record) for record in group) for group in groups)


def owned_maps() -> None:
    """Check populated signed records independently, without native-observation credit."""
    from cosworkspersist import read_legacy_midsurf_map
    groups: LegacyGroups = (((-0x80000000, -1, 0, 0x7fffffff), (1, 2, 3, 4)), (), ((7, 8, -9, 10),))
    raw = legacy_bytes(groups)
    assert read_legacy_midsurf_map(raw, max_records=2) == groups
    assert len(raw) == 60
    for empty in (((), (), ()), ((), ((1, 2, 3, 4),), ())):
        assert read_legacy_midsurf_map(legacy_bytes(empty)) == empty


def malformed_names() -> None:
    """Reject every representative prefix, signed count, trailing byte and reduced limit."""
    from cosworkspersist import read_lbc_order
    raw = struct.pack('<i', 4) + b''.join(literal_names())
    invalid = [raw[:end] for end in range(len(raw))] + [raw + b'\0', struct.pack('<i', -1)]
    for malformed in invalid:
        with unittest.TestCase().assertRaises(ValueError):
            read_lbc_order(malformed)
    with unittest.TestCase().assertRaises(ValueError):
        read_lbc_order(raw, max_items=3)
    with unittest.TestCase().assertRaises(ValueError):
        read_lbc_order(raw, max_string_bytes=2)


def malformed_maps() -> None:
    """Reject incomplete groups, negative counts, surplus bytes and per-group overrun."""
    from cosworkspersist import read_legacy_midsurf_map
    groups: LegacyGroups = (((1, 2, 3, 4),), (), ((-1, -2, -3, -4),))
    raw = legacy_bytes(groups)
    invalid = [raw[:end] for end in range(len(raw))] + [raw + b'\0', struct.pack('<i', -1) + bytes(8)]
    for malformed in invalid:
        with unittest.TestCase().assertRaises(ValueError):
            read_legacy_midsurf_map(malformed)
    with unittest.TestCase().assertRaises(ValueError):
        read_legacy_midsurf_map(raw, max_records=0)


def schema() -> None:
    """Retain the entire shared clause and exact current native inventory identity."""
    text = (ROOT.parent / 'inventor-interface/docs/solidworks-file-format/specification.md').read_bytes().decode('utf-8').replace('\r\n', '\n')
    start = text.index('### 14.336 ')
    end = text.index('\n### ', start + 1)
    raw = (text[start:end].rstrip() + '\n').encode('utf-8')
    path = FEATURE / 'schema-snapshot.md'
    if '--write' in sys.argv:
        path.write_bytes(raw)
    assert path.read_bytes() == raw
    baseline('schema-reference.json', dict(clause='14.336', bytes=len(raw), sha256=digest(raw)))
    source = RESEARCH / 'cosworks-lbc-final-native-evidence.json'
    baseline('native-source-reference.json', dict(filename=source.name, sha256=digest(source.read_bytes())))


def corpus() -> None:
    """Pin complete owned packets and replay all retained native inputs without committing them."""
    from final_cases import owned_cases, native_cases
    from final_projection import packet, span
    owned_inputs, native_inputs = owned_cases(), native_cases()
    owned, native = packet(owned_inputs), packet(native_inputs)
    fixture = ROOT / 'tests/fixtures/simulation-lbc-final-streams/owned.bin'
    if '--write' in sys.argv:
        fixture.parent.mkdir(parents=True, exist_ok=True)
        fixture.write_bytes(zlib.compress(owned, 9))
    assert zlib.decompress(fixture.read_bytes()) == owned
    (ROOT / 'target/simulation-lbc-final-native.bin').write_bytes(native)
    baseline('corpus-reference.json', dict(owned_cases=len(owned_inputs), native_cases=len(native_inputs), owned_bytes=len(owned), native_bytes=len(native), owned_sha256=digest(owned), native_sha256=digest(native), native_inputs_committed=False))
    if '--verify-rust' in sys.argv:
        actual = (ROOT / 'target/simulation-lbc-final-projections.bin').read_bytes()
        assert actual == span(owned) + span(native)
        baseline('rust-projection-reference.json', dict(bytes=len(actual), sha256=digest(actual)))


def run() -> bool:
    """Import complete actual modules only after a consistent immutable source copy."""
    with tempfile.TemporaryDirectory(prefix='bessemer-lbc-final-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        checks = (native_names, native_maps, owned_names, owned_maps, malformed_names, malformed_maps, schema, corpus)
        suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
        result = unittest.TextTestRunner(verbosity=2).run(suite)
        record_snapshot(pin, hashes, result, FEATURE, ('cosworkspersist',), ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
