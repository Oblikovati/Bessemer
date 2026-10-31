"""Freeze actual direct manager Property framing and every C-721 context (#131)."""
from pathlib import Path
import hashlib
import json
import struct
import sys
import tempfile
import unittest
import zlib
from typing import TypedDict, NotRequired, cast

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
RESEARCH = ROOT.parent / 'inventor-interface/solidworks/research'
sys.path.insert(0, str(FEATURE.parent / 'model-archives'))
sys.path.insert(0, str(FEATURE.parent / 'cosworks-persistence'))
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot


class ManagerAttempt(TypedDict):
    version: int | None
    success: bool
    values: NotRequired[list[int]]
    destinations: NotRequired[list[int]]


class ManagerCarrier(TypedDict):
    kind: str
    bytes: int
    raw_hex: str
    sha256: str
    attempts: list[ManagerAttempt]


def digest(raw: bytes) -> str:
    """Identify the entire retained byte image, e.g. digest(b'')."""
    return hashlib.sha256(raw).hexdigest()


def baseline(name: str, value: object) -> None:
    """Create evidence only under explicit initial-write mode; otherwise compare exactly."""
    path = FEATURE / name
    raw = (json.dumps(value, indent=2) + '\n').encode('utf-8')
    if '--write' in sys.argv:
        path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected unchanged reviewed baseline'


def carriers() -> list[ManagerCarrier]:
    """Retain exact native header scope without treating nested streams as headers."""
    inventory = json.loads((RESEARCH / 'cosworks-manager-property-native-evidence.json').read_bytes())
    assert inventory['evidence'] == 'C-721' and not inventory['target_executed']
    return cast(list[ManagerCarrier], inventory['rows'])


def destinations(kind: str, version: int | None) -> tuple[int, ...]:
    """Independent literal destination order; no counter or geometric semantics."""
    if kind == 'MassElementManager':
        return (0x28,)
    assert kind == 'BeamManager' and version is not None
    if version < 810024:
        return (0x14c,)
    if version < 820006:
        return (0x14c, 0x168)
    return (0x14c, 0x168, 0x148)


def check_attempt(kind: str, raw: bytes, attempt: ManagerAttempt) -> None:
    """Fix expected size before accepting or rejecting the actual reader call."""
    from cosworkspersist import read_manager_property
    offsets = destinations(kind, attempt['version'])
    accepted = len(raw) == 4 * len(offsets)
    assert accepted == attempt['success']
    if not accepted:
        with unittest.TestCase().assertRaises(ValueError):
            read_manager_property(raw, kind, attempt['version'])
        return
    record = read_manager_property(raw, kind, attempt['version'])
    assert record.kind == kind and record.version == attempt['version']
    assert record.raw == raw and record.destinations == offsets
    assert list(record.values) == attempt['values']
    assert list(record.destinations) == attempt['destinations']
    assert b''.join(struct.pack('<i', value) for value in record.values) == raw


def native_headers() -> None:
    """Replay all 514 full payloads and 548 candidate contexts, preserving 30 errors."""
    counts: dict[str, list[int]] = {}
    for row in carriers():
        raw = bytes.fromhex(row['raw_hex'])
        assert digest(raw) == row['sha256'] and len(raw) == row['bytes']
        totals = counts.setdefault(row['kind'], [0, 0, 0, 0])
        totals[0] += 1
        totals[1] += len(raw)
        for attempt in row['attempts']:
            check_attempt(row['kind'], raw, attempt)
            totals[2 if attempt['success'] else 3] += 1
    assert counts == {'BeamManager': [353, 4228, 357, 30], 'MassElementManager': [161, 644, 161, 0]}
    baseline('native-reference.json', dict(counts=counts, four_byte_native_beam_headers=0, runtime_dispatch_proven=False))


def owned_headers() -> None:
    """Prove threshold neighbors, ordered destinations and complete signed extrema."""
    versions = (0, 810023, 810024, 810025, 820005, 820006, 820007, 0x7fffffff)
    for version in versions:
        offsets = destinations('BeamManager', version)
        values = (-0x80000000, -1, 0x7fffffff)[:len(offsets)]
        raw = b''.join(struct.pack('<i', value) for value in values)
        check_attempt('BeamManager', raw, dict(version=version, success=True, values=list(values), destinations=list(offsets)))
    for version in (None, 0, 810024, 820006, 0x7fffffff):
        for value in (-0x80000000, -1, 0, 1, 0x7fffffff):
            check_attempt('MassElementManager', struct.pack('<i', value), dict(version=version, success=True, values=[value], destinations=[0x28]))


def malformed_headers() -> None:
    """Require every prefix, surplus byte and incompatible header size to fail."""
    contexts = [('BeamManager', version) for version in (0, 810024, 820006)] + [('MassElementManager', None)]
    for kind, version in contexts:
        extent = 4 * len(destinations(kind, version))
        invalid = [bytes(size) for size in range(17) if size != extent]
        for raw in invalid:
            assert_invalid_header(raw, kind, version)


def invalid_contexts() -> None:
    """Reject missing Beam context, invalid versions and unsupported literal kinds."""
    cases = [('BeamManager', None), ('BeamManager', -1), ('BeamManager', 0x80000000), ('MassElementManager', -1), ('MassElementManager', 0x80000000), ('Other', None), ('beammanager', 820006)]
    for kind, version in cases:
        assert_invalid_header(bytes(4), kind, version)


def assert_invalid_header(raw: bytes, kind: str, version: int | None) -> None:
    """Share explicit invalid contexts without accepting unexpected parse failures."""
    from cosworkspersist import read_manager_property
    with unittest.TestCase().assertRaises(ValueError):
        read_manager_property(raw, kind, version)


def schema() -> None:
    """Pin the whole clause and exact retained native-inventory identity."""
    text = (ROOT.parent / 'inventor-interface/docs/solidworks-file-format/specification.md').read_bytes().decode('utf-8').replace('\r\n', '\n')
    start = text.index('### 14.345 ')
    end = text.index('\n### ', start + 1)
    raw = (text[start:end].rstrip() + '\n').encode('utf-8')
    path = FEATURE / 'schema-snapshot.md'
    if '--write' in sys.argv:
        path.write_bytes(raw)
    assert path.read_bytes() == raw
    baseline('schema-reference.json', dict(clause='14.345', bytes=len(raw), sha256=digest(raw)))
    name = 'cosworks-manager-property-native-evidence.json'
    baseline('native-source-reference.json', dict(filename=name, sha256=digest((RESEARCH / name).read_bytes())))


def corpus() -> None:
    """Pin owned bytes and rebuild ignored native packets before complete Rust comparison."""
    from manager_cases import owned_cases, native_cases, packet
    from owned_persistence import span
    owned_inputs, native_inputs = owned_cases(), native_cases()
    owned, native = packet(owned_inputs), packet(native_inputs)
    fixture = ROOT / 'tests/fixtures/simulation-manager-property/owned.bin'
    if '--write' in sys.argv:
        fixture.parent.mkdir(parents=True, exist_ok=True)
        fixture.write_bytes(zlib.compress(owned, 9))
    assert zlib.decompress(fixture.read_bytes()) == owned
    (ROOT / 'target/simulation-manager-property-native.bin').write_bytes(native)
    baseline('corpus-reference.json', dict(owned_cases=len(owned_inputs), native_cases=len(native_inputs), owned_bytes=len(owned), native_bytes=len(native), owned_sha256=digest(owned), native_sha256=digest(native), native_inputs_committed=False))
    if '--verify-rust' in sys.argv:
        actual = (ROOT / 'target/simulation-manager-property-projections.bin').read_bytes()
        assert actual == span(owned) + span(native)
        baseline('rust-projection-reference.json', dict(bytes=len(actual), sha256=digest(actual)))


def run() -> bool:
    """Import complete actual modules only after a stable whole-source copy."""
    with tempfile.TemporaryDirectory(prefix='bessemer-manager-property-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        checks = (native_headers, owned_headers, malformed_headers, invalid_contexts, schema, corpus)
        suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
        result = unittest.TextTestRunner(verbosity=2).run(suite)
        record_snapshot(pin, hashes, result, FEATURE, ('cosworkspersist',), ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
