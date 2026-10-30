"""Replay retained LBC candidate outcomes without inferring a replacement context (#128)."""
from pathlib import Path
import hashlib
import json
import struct
import sys
import tempfile
import unittest
import zlib
from typing import TypedDict, cast
from owned_lbc_property import BOUNDARIES, LAYOUTS
from lbc_cases import LbcCase, field_cases

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
RESEARCH = ROOT.parent / 'inventor-interface/solidworks/research'
sys.path.insert(0, str(FEATURE.parent / 'model-archives'))
sys.path.insert(0, str(FEATURE.parent / 'cosworks-persistence'))
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot


class CandidateAttempt(TypedDict):
    version: int
    success: bool
    destinations: list[int]
    values: list[int]


class PropertyCarrier(TypedDict):
    path: str
    stream: str
    sha256: str
    raw_hex: str
    attempts: list[CandidateAttempt]


def digest(raw: bytes) -> str:
    """Identify the complete original bytes, e.g. digest(b'')."""
    return hashlib.sha256(raw).hexdigest()


def baseline(name: str, value: object) -> None:
    """Require reviewed creation rather than silently replacing a baseline."""
    path = FEATURE / name
    raw = (json.dumps(value, indent=2) + '\n').encode('utf-8')
    if '--write' in sys.argv:
        path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected unchanged reviewed baseline'


def carriers() -> list[PropertyCarrier]:
    """Load retained carriers while preserving their externally recorded candidate outcomes."""
    inventory = json.loads((RESEARCH / 'cosworks-lbc-property-native-evidence.json').read_bytes())
    return cast(list[PropertyCarrier], inventory['rows'])


def verify_attempt(raw: bytes, attempt: CandidateAttempt) -> None:
    """Require expected validity before handling a documented failure."""
    from cosworkspersist import read_lbc_property
    if not attempt['success']:
        with unittest.TestCase().assertRaises(ValueError):
            read_lbc_property(raw, attempt['version'])
        return
    record = read_lbc_property(raw, attempt['version'])
    assert record.raw == raw and list(record.values) == attempt['values']
    assert list(record.destinations) == attempt['destinations']
    assert record.destinations == LAYOUTS[len(raw)]
    assert b''.join(struct.pack('<i', value) for value in record.values) == raw


def retained_candidates() -> None:
    """Preserve all 390 attempts, including every one of the 52 expected failures."""
    rows, accepted, rejected, unmatched, extent = carriers(), 0, 0, 0, 0
    for row in rows:
        raw = bytes.fromhex(row['raw_hex'])
        assert digest(raw) == row['sha256']
        extent += len(raw)
        unmatched += int(not any(attempt['success'] for attempt in row['attempts']))
        for attempt in row['attempts']:
            verify_attempt(raw, attempt)
            accepted += int(attempt['success'])
            rejected += int(not attempt['success'])
    assert (len(rows), extent, accepted, rejected, unmatched) == (356, 46568, 338, 52, 18)
    baseline('native-candidate-reference.json', dict(streams=len(rows), bytes=extent, accepted_contexts=accepted, rejected_contexts=rejected, unmatched_streams=unmatched, context_inferred_from_length=False))


def literal_boundaries() -> None:
    """Compare explicit literal layouts and signed patterns on both sides of every gate."""
    from cosworkspersist import read_lbc_property
    cases = 0
    for version, size in BOUNDARIES:
        for pattern in (0, -1, -0x80000000, 0x7fffffff):
            raw = struct.pack('<i', pattern) * (size // 4)
            record = read_lbc_property(raw, version)
            assert record.destinations == LAYOUTS[size]
            assert record.values == (pattern,) * (size // 4)
            assert record.raw == raw
            cases += 1
    assert cases == 72
    baseline('boundary-reference.json', dict(version_contexts=len(BOUNDARIES), complete_signed_word_cases=cases, exact_layout_sizes=sorted(LAYOUTS)))


def fixed_writer() -> None:
    """Compare the independent writer order and all eight retained modern payloads."""
    path = RESEARCH / 'cosworks-lbc-property-writer-native-evidence.json'
    writer = json.loads(path.read_bytes())
    parent = RESEARCH / 'cosworks-lbc-property-native-evidence.json'
    expected_parent = writer['sources'][str(parent.resolve())]
    assert digest(parent.read_bytes()) == expected_parent
    assert tuple(writer['ordered_destinations']) == LAYOUTS[216]
    assert (writer['word_count'], writer['byte_count']) == (54, 216)
    rows = {(row['path'], row['stream']): row for row in carriers()}
    identities = writer['retained_modern_payloads_reconstructed']
    for identity in identities:
        row = rows[(identity['path'], identity['stream'])]
        raw = bytes.fromhex(row['raw_hex'])
        assert len(raw) == 216 and digest(raw) == identity['sha256'] == row['sha256']
        assert b''.join(struct.pack('<i', value[0]) for value in struct.iter_unpack('<i', raw)) == raw
    assert len(identities) == 8
    baseline('writer-reference.json', dict(retained_payloads=len(identities), ordered_members=writer['ordered_destinations'], evidence_sha256=digest(path.read_bytes()), native_writer_executed=False))


def schema() -> None:
    """Retain complete reader, threshold/mismatch and writer clauses."""
    source = ROOT.parent / 'inventor-interface/docs/solidworks-file-format/specification.md'
    text = source.read_bytes().decode('utf-8').replace('\r\n', '\n')
    clauses = []
    for number in (330, 334, 335):
        start = text.index(f'### 14.{number} ')
        end = text.index('\n### ', start + 1)
        clauses.append(text[start:end].rstrip() + '\n')
    raw = '\n'.join(clauses).encode('utf-8')
    path = FEATURE / 'schema-snapshot.md'
    if '--write' in sys.argv:
        path.write_bytes(raw)
    assert path.read_bytes() == raw
    baseline('schema-reference.json', dict(clauses=[330, 334, 335], bytes=len(raw), sha256=digest(raw)))


def native_cases() -> list[LbcCase]:
    """Keep every established native candidate and explicit writer identity as supplied."""
    cases = []
    rows = carriers()
    for row in rows:
        raw = bytes.fromhex(row['raw_hex'])
        assert digest(raw) == row['sha256']
        for attempt in row['attempts']:
            original = LbcCase(0, False, attempt['version'], raw, 0, 0, attempt['success'])
            cases.append(original)
            if attempt['success']:
                cases += field_cases(original)
    indexed = {(row['path'], row['stream']): row for row in rows}
    writer = json.loads((RESEARCH / 'cosworks-lbc-property-writer-native-evidence.json').read_bytes())
    for identity in writer['retained_modern_payloads_reconstructed']:
        row = indexed[(identity['path'], identity['stream'])]
        raw = bytes.fromhex(row['raw_hex'])
        assert digest(raw) == identity['sha256']
        original = LbcCase(0, True, 0, raw, 0, 0, True)
        cases += [original, *field_cases(original)]
    return cases


def corpus() -> None:
    """Freeze synthetic packets and untracked native inputs, then check complete Rust output."""
    from lbc_cases import owned_cases
    from lbc_projection import packet
    from owned_persistence import span
    owned_inputs, native_inputs = owned_cases(), native_cases()
    owned, native = packet(owned_inputs), packet(native_inputs)
    fixture = ROOT / 'tests/fixtures/simulation-lbc-property/owned.bin'
    if '--write' in sys.argv:
        fixture.parent.mkdir(parents=True, exist_ok=True)
        fixture.write_bytes(zlib.compress(owned, 9))
    assert zlib.decompress(fixture.read_bytes()) == owned
    (ROOT / 'target/simulation-lbc-property-native.bin').write_bytes(native)
    baseline('corpus-reference.json', dict(owned_cases=len(owned_inputs), native_cases=len(native_inputs), owned_bytes=len(owned), owned_sha256=digest(owned), native_bytes=len(native), native_sha256=digest(native), native_inputs_committed=False, writer_numeric_version_inferred=False))
    if '--verify-rust' in sys.argv:
        actual = (ROOT / 'target/simulation-lbc-property-projections.bin').read_bytes()
        assert actual == span(owned) + span(native)
        baseline('rust-projection-reference.json', dict(bytes=len(actual), sha256=digest(actual)))


def run() -> bool:
    """Test complete actual frozen modules, e.g. run()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-lbc-property-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        checks = (retained_candidates, literal_boundaries, fixed_writer, schema, corpus)
        suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
        result = unittest.TextTestRunner(verbosity=2).run(suite)
        record_snapshot(pin, hashes, result, FEATURE, ('cosworkspersist',), ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
