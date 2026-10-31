"""Freeze actual lossless USD framing and explicit writer-layout views (#123)."""
from pathlib import Path
import json
import struct
import sys
import tempfile
import unittest
import zlib

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
sys.path.insert(0, str(FEATURE.parent / 'simulation-images'))
import freeze_images as shared
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot
from owned_usd import PROFILE_LABELS, record_cases, writer_packet, writer_values, canonical_unicode, edit_cases
span, words, sha = shared.span, shared.words, shared.sha


def report(name: str, value: object) -> None:
    """Require explicit review before changing a frozen baseline."""
    path = FEATURE / name
    raw = (json.dumps(value, indent=2) + '\n').encode()
    if '--write' in sys.argv:
        path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected unchanged reviewed baseline'


def fixture(name: str, packet: bytes) -> None:
    """Ship owned literal inputs and expected boundaries, never a claimed native whole file."""
    path = ROOT / 'tests/fixtures/plastics-usd' / name
    raw = zlib.compress(packet, 9)
    if '--write' in sys.argv:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(raw)
    assert path.read_bytes() == raw


def record_oracle(raw: bytes, offset: int = 0) -> bytes:
    """Retain u64 counts and raw headers/payloads without lossy text decoding."""
    import plasticsusd as reader
    record = reader.read_record(raw, offset)
    assert record.header + record.payload == raw[offset:record.end]
    if record.character_width == 2:
        assert record.unicode_text.encode('utf-16-le', 'surrogatepass') == record.payload
    else:
        assert record.unicode_text is None
    metadata = struct.pack('<QQQIQ', record.offset, record.payload_offset, record.end, record.character_width, record.declared_count)
    return metadata + span(record.header) + span(record.payload)


def freeze_records() -> None:
    """Cross literal canonical/noncanonical escapes with independent count and extent facts."""
    import plasticsusd as reader
    cases = record_cases()
    packet = bytearray(b'USD1' + words([len(cases)]))
    for case in cases:
        raw = case.header + case.payload
        actual = reader.read_record(raw)
        assert (actual.header, actual.payload, actual.character_width, actual.declared_count) == (case.header, case.payload, case.width, case.count)
        assert (actual.offset, actual.payload_offset, actual.end) == (0, len(case.header), len(raw))
        packet += span(raw) + span(record_oracle(raw))
        assert_record_truncations(raw, len(case.header))
    fixture('records.bin', bytes(packet))
    report('record-reference.json', dict(cases=len(cases), oracle_sha256=sha(bytes(packet)), full_record_native_fixtures=0))
    native_prefix_model()


def native_prefix_model() -> None:
    """Check the hash-bound native-derived literal without promoting its model to a file fixture."""
    import plasticsusd as reader
    path = ROOT.parent / 'inventor-interface/solidworks/research/plastics-usd-archive-native-evidence.json'
    evidence_bytes = path.read_bytes()
    evidence = json.loads(evidence_bytes)
    raw = bytes.fromhex(evidence['first_record_hex'])
    assert raw == b'\xff\xfe\xff\x2a' + b'\x2d\x00' * 42
    actual = reader.read_record(raw)
    assert actual.character_width == 2 and actual.declared_count == 42 and actual.end == 88
    assert actual.payload == b'\x2d\x00' * 42
    report('native-prefix-reference.json', dict(source=str(path), source_sha256=sha(evidence_bytes), prefix_model_bytes=88, prefix_model_sha256=sha(raw), is_complete_native_file_fixture=False))


def assert_record_truncations(raw: bytes, header: int) -> None:
    """Check every small prefix and every large header/terminal truncation without quadratic copying."""
    import plasticsusd as reader
    lengths = range(len(raw)) if len(raw) < 1024 else (*range(header), header, len(raw) - 1)
    for length in lengths:
        with unittest.TestCase().assertRaises(reader.USDFramingError):
            reader.read_record(raw[:length])


def freeze_guards() -> None:
    """Reject unsafe counts and malformed tails while preserving records after #EOF text."""
    import plasticsusd as reader
    for count in (0xffffffff, 0x100000000, 0xffffffffffffffff):
        for width in (1, 2):
            raw = (b'\xff\xfe\xff' if width == 2 else b'') + b'\xff' * 7 + struct.pack('<Q', count)
            with unittest.TestCase().assertRaises(reader.USDFramingError):
                reader.read_record(raw)
    raw = b'\x04#EOF' + canonical_unicode(bytes.fromhex('000000d8')) + b'\x00'
    records = list(reader.iter_records(raw))
    assert len(records) == 3 and b''.join(r.header + r.payload for r in records) == raw
    for kwargs in ({'max_records': 2}, {'max_records': -1}, {'max_payload': -1}, {'max_payload': 0}):
        with unittest.TestCase().assertRaises(reader.USDFramingError):
            list(reader.iter_records(raw, **kwargs))
    with unittest.TestCase().assertRaises(reader.USDFramingError):
        list(reader.iter_records(raw + b'\xff'))
    fixture('after-eof.bin', raw)


def layout_oracle(raw: bytes, adapters: int, payloads: list[bytes]) -> bytes:
    """Bind every writer-layout group to exact record identities and unchanged Unicode bytes."""
    import plasticsusd as reader
    layout = reader.map_writer_layout(raw)
    assert [record.payload for record in layout.records] == payloads
    assert len(layout.records) == 75 + 10 * adapters and len(layout.adapters) == adapters
    assert tuple(label for label, _ in layout.profile) == PROFILE_LABELS
    indices = {record.offset: index for index, record in enumerate(layout.records)}
    selected = [layout.host, layout.volume_serial, *(record for group in layout.adapters for record in group), *layout.customer, *(record for _, record in layout.profile)]
    actual = [indices[record.offset] for record in selected]
    start = 6 + 10 * adapters
    expected = [1, 4, *range(6, start), *range(start + 3, start + 8), *(start + 12 + 5 * index for index in range(12))]
    assert actual == expected
    labels = b''.join(span(label.encode('utf-16-le')) for label, _ in layout.profile)
    return words([len(layout.records), adapters, len(actual), *actual]) + labels


def archive_oracle(raw: bytes) -> bytes:
    """Expose every source record boundary after a literal portable edit."""
    import plasticsusd as reader
    records = list(reader.iter_records(raw))
    return words([len(records)]) + b''.join(span(record_oracle(raw, record.offset)) for record in records)


def freeze_edits() -> None:
    """Validate independent exact record replacement goldens with actual complete readers."""
    import plasticsusd as reader
    cases = edit_cases()
    packet = bytearray(b'USDE' + words([len(cases)]))
    for base, index, replacement, golden in cases:
        records = list(reader.iter_records(base))
        selected = records[1]
        assert golden == base[:selected.offset] + replacement + base[selected.end:]
        actual = list(reader.iter_records(golden))
        assert len(actual) == 3 and actual[1].header + actual[1].payload == replacement
        assert actual[0].header + actual[0].payload == base[:selected.offset]
        assert actual[2].header + actual[2].payload == base[selected.end:]
        packet += span(base) + words([index]) + span(replacement) + span(golden) + span(archive_oracle(golden))
    fixture('edits.bin', bytes(packet))
    report('edit-reference.json', dict(cases=len(cases), unchanged_cases=1, oracle_sha256=sha(bytes(packet)), operation_is_portable_record_replacement=True, native_writer_executed=False))


def verify_rust() -> None:
    """Compare complete independent record, layout and edit results across processes."""
    if '--verify-rust' not in sys.argv:
        return
    expected = bytearray(b'USDV' + words([49]))
    for case in record_cases():
        expected += span(record_oracle(case.header + case.payload))
    expected += words([4])
    for count in (0, 1, 2, 7):
        raw, payloads = writer_packet(count)
        expected += span(layout_oracle(raw, count, payloads))
    expected += words([50])
    for _, _, _, golden in edit_cases():
        expected += span(golden) + span(archive_oracle(golden))
    actual = (ROOT / 'target/plastics-usd-projections.bin').read_bytes()
    assert actual == expected
    report('rust-projection-reference.json', dict(bytes=len(actual), sha256=sha(actual), records=49, layouts=4, edits=50, inputs_are_owned_synthetic=True))


def freeze_layouts() -> None:
    """Own complete labeled layouts without executing a native save or reading host diagnostics."""
    packet = bytearray(b'USDL' + words([4]))
    for count in (0, 1, 2, 7):
        raw, payloads = writer_packet(count)
        packet += words([count]) + span(raw) + span(layout_oracle(raw, count, payloads))
    fixture('layouts.bin', bytes(packet))
    report('layout-reference.json', dict(adapter_counts=[0, 1, 2, 7], record_counts=[75, 85, 95, 145], oracle_sha256=sha(bytes(packet)), fixtures_are_synthetic=True))
    assert_layout_failures()


def assert_layout_failures() -> None:
    """Reject missing, extra, narrow or mutated fixed records without normalizing them."""
    import plasticsusd as reader
    values = writer_values(0)
    cases = [values[:-1], values + [''], values + [''] * 10]
    for index, value in ((0, '-' * 41), (7, 'Other Customer Profile'), (16, 'Wrong Label'), (-1, '#EOF\0suffix')):
        changed = values.copy()
        changed[index] = value
        cases.append(changed)
    for changed in cases:
        raw = b''.join(canonical_unicode(value.encode('utf-16-le', 'surrogatepass')) for value in changed)
        with unittest.TestCase().assertRaises(reader.USDLayoutError):
            reader.map_writer_layout(raw)
    raw, _ = writer_packet(0)
    host = list(reader.iter_records(raw))[1]
    with unittest.TestCase().assertRaises(reader.USDLayoutError):
        reader.map_writer_layout(raw[:host.offset] + b'\x01x' + raw[host.end:])


def schema() -> None:
    """Retain all supporting writer/input clauses without claiming the later runtime loader."""
    text = (ROOT.parent / 'inventor-interface/docs/solidworks-file-format/specification.md').read_text(encoding='utf-8').replace('\r\n', '\n')
    first = text[text.index('### 14.187 '):text.index('### 14.199 ')]
    second = text[text.index('### 14.201 '):text.index('### 14.202 ')]
    raw = (first.rstrip() + '\n\n' + second.rstrip() + '\n').encode()
    path = FEATURE / 'schema-snapshot.md'
    if '--write' in sys.argv:
        path.write_bytes(raw)
    assert path.read_bytes() == raw
    report('schema-reference.json', dict(issue=123, clauses=[f'14.{number}' for number in (*range(187, 199), 201)], sha256=sha(raw)))


def run() -> bool:
    """Import unchanged complete copied reader code and record its origin before cleanup."""
    with tempfile.TemporaryDirectory(prefix='bessemer-plastics-usd-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        suite = unittest.TestSuite(unittest.FunctionTestCase(test) for test in (freeze_records, freeze_guards, freeze_layouts, freeze_edits, schema, verify_rust))
        result = unittest.TextTestRunner(verbosity=2).run(suite)
        record_snapshot(pin, hashes, result, FEATURE, ('plasticsusd',), ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
