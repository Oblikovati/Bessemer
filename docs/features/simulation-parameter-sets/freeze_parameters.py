"""Freeze complete actual ParameterSets framing and C-656/C-659 evidence (#133)."""
from pathlib import Path
import hashlib
import json
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
from owned_persistence import literal_texts, word
from literal_parameters import parameter_sections, parameter_bytes


class AuditSpan(TypedDict):
    label: str
    offset: int
    end: int
    raw_hex: str


class ParameterAudit(TypedDict):
    header: list[int]
    record_count: int
    table_count: int
    spans: list[AuditSpan]


class ParameterCandidate(TypedDict):
    version: int
    success: bool
    parsed: NotRequired[ParameterAudit]


class ParameterCarrier(TypedDict):
    stream: str
    bytes: int
    raw_hex: str
    sha256: str
    successful_stored_versions: list[int]
    layout_candidates: list[ParameterCandidate]


def digest(raw: bytes) -> str:
    """Identify the complete byte image, e.g. digest(b'')."""
    return hashlib.sha256(raw).hexdigest()


def baseline(name: str, value: object) -> None:
    """Create initial reviewed evidence explicitly, then compare unchanged bytes."""
    path = FEATURE / name
    raw = (json.dumps(value, indent=2) + '\n').encode('utf-8')
    if '--write' in sys.argv:
        path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected unchanged reviewed baseline'


def carriers() -> list[ParameterCarrier]:
    """Read only exact retained ParameterSets payloads and candidate contexts."""
    inventory = json.loads((RESEARCH / 'cosworks-parameter-layout-native-evidence.json').read_bytes())
    assert inventory['evidence'] == 'C-656' and not inventory['target_executed']
    return cast(list[ParameterCarrier], inventory['rows'])


def check_candidate(raw: bytes, candidate: ParameterCandidate) -> None:
    """Retained acceptance/rejection is fixed before actual reader execution."""
    from cosworkspersist import read_parameter_sets
    if not candidate['success']:
        with unittest.TestCase().assertRaises(ValueError):
            read_parameter_sets(raw, candidate['version'])
        return
    record = read_parameter_sets(raw, candidate['version'])
    expected = candidate['parsed']
    assert record.header == tuple(expected['header'])
    assert record.record_count == expected['record_count'] and record.table_count == expected['table_count']
    assert [(span.label, span.offset, span.offset + len(span.payload), span.payload.hex()) for span in record.spans] == [(span['label'], span['offset'], span['end'], span['raw_hex']) for span in expected['spans']]
    assert record.end == len(raw) and b''.join(span.payload for span in record.spans) == raw
    assert [text.header + text.payload for text in record.texts] == [span.payload for span in record.spans if span.label.endswith('.text')]


def native_candidates() -> None:
    """Preserve all five carriers, forty-three candidate outcomes and four stored matches."""
    streams, total, accepted, rejected = 0, 0, 0, 0
    for row in carriers():
        raw = bytes.fromhex(row['raw_hex'])
        assert digest(raw) == row['sha256'] and len(raw) == row['bytes']
        for candidate in row['layout_candidates']:
            check_candidate(raw, candidate)
            accepted += int(candidate['success'])
            rejected += int(not candidate['success'])
        matched = {candidate['version'] for candidate in row['layout_candidates'] if candidate['success']}
        assert set(row['successful_stored_versions']) <= matched
        streams += bool(row['successful_stored_versions'])
        total += len(raw) if row['successful_stored_versions'] else 0
    assert (streams, total, accepted, rejected) == (4, 5468, 8, 35)
    baseline('native-reference.json', dict(carriers=5, stored_matching_streams=streams, stored_matching_bytes=total, accepted_candidate_contexts=accepted, rejected_candidate_contexts=rejected, runtime_dispatch_proven=False))


def decoder_validation() -> None:
    """Bind the independent C-659 stored-version report to complete C-656 payloads."""
    from cosworkspersist import read_parameter_sets
    report = json.loads((RESEARCH / 'cosworks-parameter-decoder-validation.json').read_bytes())
    assert report['evidence'] == 'C-659' and not report['target_executed']
    assert report['source_sha256'] == digest((RESEARCH / 'cosworks-parameter-layout-native-evidence.json').read_bytes())
    indexed = {row['sha256']: row for row in carriers()}
    for expected in report['rows']:
        row = indexed[expected['sha256']]
        assert row['bytes'] == expected['bytes'] and expected['version'] in row['successful_stored_versions']
        record = read_parameter_sets(bytes.fromhex(row['raw_hex']), expected['version'])
        assert len(record.spans) == expected['spans'] and record.end == expected['bytes']
    assert len(report['rows']) == 4 and report['exact_reconstruction_bytes'] == 5468
    legacy = next(row for row in indexed.values() if row['bytes'] == 2944 and 'BasicStudy' in row['stream'])
    assert report['incompatible_legacy_versions_rejected'] == [700021, 890116]
    for version in report['incompatible_legacy_versions_rejected']:
        assert_invalid(bytes.fromhex(legacy['raw_hex']), version)
    baseline('decoder-validation-reference.json', dict(streams=4, bytes=5468, incompatible_basic_study_bytes=2944, rejected_versions=[700021, 890116], source_inventory_identity_matches=True))


def check_literal(version: int, count: int, texts: tuple[bytes, ...]) -> None:
    """Compare every independently literal section, offset and preserved full-width pattern."""
    from cosworkspersist import read_parameter_sets
    sections = parameter_sections(version, count, texts)
    raw = b''.join(payload for _, payload in sections)
    record = read_parameter_sets(raw, version, max_table_items=max(10000, count))
    assert record.table_count == count and record.record_count == len(texts)
    assert len(record.spans) == len(sections)
    offset = 0
    for span, (label, payload) in zip(record.spans, sections, strict=True):
        assert (span.label, span.offset, span.payload) == (label, offset, payload)
        offset += len(payload)
    assert record.end == offset == len(raw)
    assert b''.join(span.payload for span in record.spans) == raw
    assert [text.header + text.payload for text in record.texts] == list(texts)


def owned_boundaries() -> None:
    """Cover every threshold/neighbor and legacy ignored first header values."""
    versions = (0, 700011, 700012, 700013, 700014, 700018, 700019, 810025, 810026, 820015, 820016, 830009, 830010, 0x7fffffff)
    texts = literal_texts()
    for version in versions:
        for count in ((100,) if version < 810026 else (0, 1, 2)):
            check_literal(version, count, ())
            check_literal(version, count, (texts[0],))
            check_literal(version, count, (texts[1], texts[3], texts[2]))


def beyond_retention() -> None:
    """Consume and preserve the complete table and record array above the native retention cap."""
    for count in (10000, 10001):
        check_literal(830010, count, (literal_texts()[2],))
    assert_invalid(parameter_bytes(830010, 10001, (literal_texts()[2],)), 830010)


def assert_invalid(raw: bytes, version: int, **limits: int) -> None:
    """Keep malformed expectations separate from reader exceptions and field iteration."""
    from cosworkspersist import read_parameter_sets
    with unittest.TestCase().assertRaises(ValueError):
        read_parameter_sets(raw, version, **limits)


def malformed() -> None:
    """Reject every representative prefix, trailing bytes, negative and huge effective counts."""
    raw = parameter_bytes(830010, 2, tuple(literal_texts()[:4]))
    for end in range(len(raw)):
        assert_invalid(raw[:end], 830010)
    assert_invalid(raw + b'\0', 830010)
    for value in (-1, 0x7fffffff):
        assert_invalid(word(value) + raw[4:], 830010, max_table_items=0x7fffffff)
        assert_invalid(raw[:28] + word(value) + raw[32:], 830010, max_records=0x7fffffff)
    assert_invalid(raw, 830010, max_records=3)
    assert_invalid(raw, 830010, max_table_items=1)
    assert_invalid(raw, 830010, max_string_bytes=2)
    assert_invalid(parameter_bytes(0, 100, ()), 0, max_table_items=99)
    assert_invalid(raw, -1)


def schema() -> None:
    """Retain complete relevant clauses, native inventory and historical decoder-validation identity."""
    text = (ROOT.parent / 'inventor-interface/docs/solidworks-file-format/specification.md').read_bytes().decode('utf-8').replace('\r\n', '\n')
    clauses = []
    for number in (278, 279, 280, 283, 284):
        start = text.index(f'### 14.{number} ')
        end = text.index('\n### ', start + 1)
        clauses.append(text[start:end].rstrip() + '\n')
    raw = '\n'.join(clauses).encode('utf-8')
    path = FEATURE / 'schema-snapshot.md'
    if '--write' in sys.argv:
        path.write_bytes(raw)
    assert path.read_bytes() == raw
    baseline('schema-reference.json', dict(clauses=[278, 279, 280, 283, 284], bytes=len(raw), sha256=digest(raw)))
    names = ('cosworks-parameter-layout-native-evidence.json', 'cosworks-parameter-decoder-validation.json')
    baseline('native-source-reference.json', {name: digest((RESEARCH / name).read_bytes()) for name in names})


def corpus() -> None:
    """Freeze complete owned bytes and ignored native inputs without requiring a Rust export yet."""
    from parameter_cases import owned_cases, native_cases
    from parameter_projection import packet
    from fixture_storage import pack, unpack
    from owned_persistence import span
    owned_inputs, native_inputs = owned_cases(), native_cases()
    owned, native = packet(owned_inputs), packet(native_inputs)
    fixture = ROOT / 'tests/fixtures/simulation-parameter-sets/owned.bin'
    if '--write' in sys.argv:
        fixture.parent.mkdir(parents=True, exist_ok=True)
        fixture.write_bytes(zlib.compress(pack(owned), 9))
    assert unpack(zlib.decompress(fixture.read_bytes())) == owned
    (ROOT / 'target/simulation-parameter-sets-native.bin').write_bytes(native)
    baseline('corpus-reference.json', dict(owned_cases=len(owned_inputs), native_cases=len(native_inputs), owned_bytes=len(owned), native_bytes=len(native), owned_sha256=digest(owned), native_sha256=digest(native), native_inputs_committed=False, large_array_edits='All fields retained; table/value edits probed at indices 0, 1, 9998, 9999, 10000 and last where present'))
    if '--verify-rust' in sys.argv:
        actual = (ROOT / 'target/simulation-parameter-sets-projections.bin').read_bytes()
        assert actual == span(owned) + span(native)
        baseline('rust-projection-reference.json', dict(bytes=len(actual), sha256=digest(actual)))


def run() -> bool:
    """Import complete actual reader modules only after a consistent whole-project source copy."""
    with tempfile.TemporaryDirectory(prefix='bessemer-parameter-sets-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        checks = (native_candidates, decoder_validation, owned_boundaries, beyond_retention, malformed, schema, corpus)
        suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
        result = unittest.TextTestRunner(verbosity=2).run(suite)
        record_snapshot(pin, hashes, result, FEATURE, ('cosworkspersist',), ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
