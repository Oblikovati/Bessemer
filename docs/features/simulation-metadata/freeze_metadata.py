"""Freeze actual GUID-string/scalar readers and independent CWR fixtures (#116)."""
from pathlib import Path
import json
import struct
import sys
import tempfile
import unittest
import zlib

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
sys.path.insert(0, str(FEATURE.parent / 'simulation-out'))
sys.path.insert(0, str(FEATURE.parent / 'simulation-images'))
import freeze_reports as shared
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot
from owned_metadata import utf16, stored_fields, guid_cases, writer_cases, carrier, KEEP, REPLACEMENT

span, words, sha = shared.span, shared.words, shared.sha
KINDS = ('TimeStamp', 'RunState')
PATTERNS = (0, 1, 213, 0x7fffffff, 0x80000000, 0xffffffff, 0xdeadbeef)
STAMPS = (-0x80000000, -213, -1, 0, 1, 213, 0x7fffffff)


def report(name: str, record: object) -> None:
    """Require explicit review intent before replacing owned/reference facts."""
    path = FEATURE / name
    raw = (json.dumps(record, indent=2) + '\n').encode()
    if '--write' in sys.argv:
        path.write_bytes(raw)
        return
    assert path.read_bytes() == raw, f'{path}; expected unchanged reviewed baseline'


def nullable_units(value: tuple[int, ...] | None) -> bytes:
    """Distinguish null from allocated empty in portable packet framing."""
    return words([0]) if value is None else words([1]) + span(utf16(value))


def guid_oracle(raw: bytes) -> bytes:
    """Retain raw fields and both surrogate-preserving stored/caller-visible views."""
    import simulationguids
    parsed = simulationguids.read(raw)
    expected = words([len(parsed['fields'])])
    for field in parsed['fields']:
        expected += words([field['index'], field['offset'], field['byte_length'], int(field['content'] is not None)])
        if field['content'] is not None:
            expected += span(field['content'])
            expected += span(field['text'].encode('utf-16-le', errors='surrogatepass'))
            expected += span(field['caller_visible_text'].encode('utf-16-le', errors='surrogatepass'))
        expected += span(field['raw'])
        assert field['role'] is None
    assert b''.join(field['raw'] for field in parsed['fields']) + parsed['tail'] == raw
    return expected + words([parsed['end'], int(parsed['stream_complete'])]) + span(parsed['tail'])


def scalar_oracle(kind: int, raw: bytes) -> bytes:
    """Keep signed/unsigned patterns and explicitly unassigned clock/enum metadata."""
    import simulationscalars
    parsed = simulationscalars.read(KINDS[kind], raw)
    assert parsed['enum'] is None and parsed['wall_clock_unit'] is None
    return span(parsed['name'].encode()) + words([parsed['pattern'], parsed['signed_value'], int(parsed['meaning'] == 'model_document_update_stamp'), int(parsed['field_semantics_complete']), int(parsed['transfer_prefix'])]) + span(parsed['raw'])


def freeze_owned() -> None:
    """Own every stream and literal field edit before checking actual reader projections."""
    records, facts = [], []
    for label, fields, tail in guid_cases():
        raw = stored_fields(fields, tail)
        expected = guid_oracle(raw)
        goldens = [stored_fields(fields[:index] + (REPLACEMENT,) + fields[index + 1:], tail) for index in range(4)]
        for golden in goldens:
            guid_oracle(golden)
        records.append(span(raw) + span(expected) + b''.join(map(span, goldens)) + span(carrier(raw)) + span(carrier(goldens[0])) + span(carrier(raw, stamp=0x87654321)) + span(carrier(raw, stamp=0x87654321, state=1)))
        facts.append(dict(label=label, bytes=len(raw), sha256=sha(raw), oracle_sha256=sha(expected), complete=not tail, field_goldens=[sha(golden) for golden in goldens]))
    packet = b'SIMO' + words([len(records)]) + b''.join(records)
    packet += freeze_writers() + freeze_scalars() + freeze_helpers()
    compressed = zlib.compress(packet, 9)
    path = ROOT / 'tests/fixtures/simulation-metadata/owned.bin'
    if '--write' in sys.argv:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(compressed)
    assert path.read_bytes() == compressed
    report('owned-reference.json', dict(fixture_sha256=sha(compressed), decoded_bytes=len(packet), guid_cases=facts, writer_cases=10, scalar_cases=14, helper_cases=7))


def freeze_writers() -> bytes:
    """Check nullable caller strings and the creation gate without invented storage operations."""
    import simulationguids
    cases = writer_cases()
    raw = words([len(cases)])
    for label, fields in cases:
        strings = [None if value is None else utf16(value).decode('utf-16-le', errors='surrogatepass') for value in fields]
        projected = simulationguids.project_writer(strings)
        raw += b''.join(nullable_units(value) for value in fields) + words([int(projected['stream_created'])])
        if projected['stream_created']:
            expected = stored_fields(fields)
            assert projected['raw'] == expected, label
            raw += span(expected) + span(guid_oracle(expected))
        else:
            assert projected['raw'] is None
    return raw


def freeze_scalars() -> bytes:
    """Bind all raw/signed scalar patterns to actual named-reader output."""
    raw = words([len(KINDS) * len(PATTERNS)])
    for kind in range(2):
        for pattern in PATTERNS:
            encoded = struct.pack('<I', pattern)
            raw += words([kind, pattern]) + span(encoded) + span(scalar_oracle(kind, encoded))
    return raw


def freeze_helpers() -> bytes:
    """Use independent literal scalar words for the supplied-stamp helper projection."""
    import simulationscalars
    raw = words([len(STAMPS)])
    for stamp in STAMPS:
        projected = simulationscalars.project_helper(stamp)
        state, timestamp = struct.pack('<i', 1), struct.pack('<i', stamp)
        assert projected['streams'] == dict(RunState=state, TimeStamp=timestamp)
        assert not projected['run_state_enum_complete'] and not projected['native_io_executed']
        raw += words([stamp]) + span(state) + span(timestamp)
    return raw


def freeze_carriers() -> None:
    """Reopen independent complete carriers through the actual frozen CFB implementation."""
    from cfb import CFB
    with tempfile.TemporaryDirectory(prefix='bessemer-metadata-owned-') as temporary:
        path = Path(temporary) / 'owned.cwr'
        for _, fields, tail in guid_cases():
            guid = stored_fields(fields, tail)
            path.write_bytes(carrier(guid))
            document = CFB(path)
            assert document.read('Keep') == KEEP
            assert document.read('TimeStampGUIDs') == guid
            assert document.read('TimeStamp') == struct.pack('<I', 0xdeadbeef)
            assert document.read('RunState') == struct.pack('<I', 0x12345678)
            guid_oracle(document.read('TimeStampGUIDs'))


def freeze_native() -> None:
    """Retain every installed scalar byte and all three actual GUID-stream absences."""
    from cfb import CFB
    catalog = json.loads((FEATURE.parent / 'compound-files/native-reference.json').read_text())['records']
    inputs = [item for item in catalog if Path(item['path']).suffix.lower() == '.cwr']
    assert len(inputs) == 3
    packets, facts = [], []
    for item in inputs:
        raw = Path(item['path']).read_bytes()
        assert sha(raw) == item['sha256']
        document = CFB(Path(item['path']))
        names = {entry.path for entry in document.walk() if entry.type == 2}
        assert 'TimeStampGUIDs' not in names
        encoded = span(raw)
        fields = []
        for kind, name in enumerate(KINDS):
            encoded += words([int(name in names)])
            if name not in names:
                continue
            scalar = document.read(name)
            encoded += span(scalar) + span(scalar_oracle(kind, scalar))
            fields.append(dict(name=name, bytes=len(scalar), sha256=sha(scalar), pattern=struct.unpack('<I', scalar)[0]))
        packets.append(encoded)
        facts.append(dict(path=item['path'], carrier_sha256=sha(raw), guid_stream_present=False, scalars=fields))
    assert sum(len(item['scalars']) for item in facts) == 4
    report('native-reference.json', dict(carriers=3, guid_streams_present=0, scalar_streams=4, scalar_bytes=16, records=facts))
    if '--export' in sys.argv:
        packet = b'SIMN' + words([len(packets)]) + b''.join(packets)
        (ROOT / 'target/simulation-metadata-native.bin').write_bytes(packet)
        report('native-probe-reference.json', dict(bytes=len(packet), sha256=sha(packet)))


def schema() -> None:
    """Pin complete string/scalar clauses without inferring additional identity or clock roles."""
    text = (ROOT.parent / 'inventor-interface/docs/solidworks-file-format/specification.md').read_text().replace('\r\n', '\n')
    begin, end = text.index('### 14.110 '), text.index('### 14.112 ')
    raw = (text[begin:end].rstrip() + '\n').encode()
    path = FEATURE / 'schema-snapshot.md'
    if '--write' in sys.argv:
        path.write_bytes(raw)
    assert path.read_bytes() == raw
    report('schema-reference.json', dict(issue=116, clauses=['14.110', '14.111'], sha256=sha(raw)))


def run() -> bool:
    """Freeze and verify actual reader origins before any reference invocation."""
    with tempfile.TemporaryDirectory(prefix='bessemer-simulation-metadata-source-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        shared.compound.pin_reader(pin, hashes)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        suite = unittest.TestSuite(unittest.FunctionTestCase(test) for test in (freeze_owned, freeze_carriers, freeze_native, schema))
        result = unittest.TextTestRunner(verbosity=2).run(suite)
        record_snapshot(pin, hashes, result, FEATURE, ('cfb', 'simulationguids', 'simulationscalars'), ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
