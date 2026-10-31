"""Fresh complete study-manager Material replay with every original candidate retained (#140)."""
from pathlib import Path
from typing import cast
from functools import partial, update_wrapper
import json
import struct
import sys
import tempfile
import unittest

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
sys.path.insert(0, str(FEATURE.parent / 'simulation-binary-thermal'))
sys.path.insert(0, str(FEATURE.parent / 'model-archives'))
sys.path.insert(0, str(ROOT / 'target/reference-python'))
import pefile
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot
from native_carriers import SOURCE, RESEARCH, digest, recover_sample, pinned_auxiliaries
from owned_material import owned_material_cases, material_literal
from material_reference import actual_material_parser, retained_outcome
from material_projection import material_projection_case
from material_history import material_history_case
from material_adversarial import material_mutation_packet, material_budget_packet


def baseline(name: str, value: object) -> None:
    """Initialize only requested new evidence and compare complete bytes; e.g. baseline('owned-reference.json', report)."""
    path = FEATURE / name
    raw = (json.dumps(value, indent=2) + '\n').encode('utf-8')
    if '--write' in sys.argv: path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected unchanged complete reference baseline'


def initial_reports() -> tuple[dict[str, object], dict[str, object]]:
    """Require the complete retained historical reports before replay; e.g. initial_reports()."""
    initial = json.loads((FEATURE / 'initial-reference.json').read_bytes())
    reports: list[dict[str, object]] = []
    for name, reference in initial['native_reports'].items():
        raw = (RESEARCH / name).read_bytes()
        assert digest(raw) == reference['sha256'] and json.loads(raw) == reference['report']
        report = cast(dict[str, object], reference['report'])
        assert not report['target_executed']
        reports.append(report)
    assert len(reports) == 2
    return reports[0], reports[1]


def case_packet(raw: bytes, version: int, parsed: dict[str, object]) -> bytes:
    """Keep full inputs and all actual consumed spans, including failed prefixes; e.g. case_packet(raw, 830036, parsed)."""
    spans = cast(list[dict[str, int]], parsed['spans'])
    header = struct.pack('<IIII', version, int(cast(bool, parsed['success'])), len(raw), cast(int, parsed['end']))
    return header + raw + struct.pack('<I', len(spans)) + b''.join(
        struct.pack('<II', span['offset'], span['end']) for span in spans)


def verify_payload(row: dict[str, object], streams: dict[str, bytes]) -> bytes:
    """Check fresh envelope-derived payload identity before any parser call; e.g. verify_payload(row, streams)."""
    raw = streams[cast(str, row['stream'])]
    assert len(raw) == row['bytes'] and digest(raw) == row['sha256']
    return raw


def replay_row(row: dict[str, object], raw: bytes, prior: dict[str, object] | None) -> tuple[bytes, bytes, bytes]:
    """Match every original success/failure and the complete earlier mass-child outcomes; e.g. replay_row(row, raw, prior)."""
    packet = bytearray()
    projection = bytearray()
    history = bytearray()
    for expected in cast(list[dict[str, object]], row['attempts']):
        version = cast(int, expected['version'])
        parsed = actual_material_parser(raw, version)
        assert retained_outcome(version, parsed) == expected, (row['path'], row['stream'], version)
        packet.extend(case_packet(raw, version, parsed))
        if parsed['success']:
            projection.extend(material_projection_case(raw, version, parsed))
            history.extend(material_history_case(raw, version, parsed))
    if prior is not None:
        assert prior['sha256'] == row['sha256']
        for expected in cast(list[dict[str, object]], prior['attempts']):
            version = cast(int, expected['version'])
            assert dict(version=version, **actual_material_parser(raw, version)) == expected
    return bytes(packet), bytes(projection), bytes(history)


def replay_native(pin: Path) -> None:
    """Recover every complete original occurrence and keep excluded carrier identities separate; e.g. replay_native(pin)."""
    earlier, corpus = initial_reports()
    rows = cast(list[dict[str, object]], corpus['rows'])
    excluded = cast(list[dict[str, object]], corpus['excluded_awdocument'])
    prior = {(row['path'], row['stream']): row for row in cast(list[dict[str, object]], earlier['rows'])}
    packet = bytearray()
    projection = bytearray()
    history = bytearray()
    paths = sorted({cast(str, row['path']) for row in rows + excluded})
    for path in paths:
        image, fields, edits = replay_document(path, pin, rows, excluded, prior)
        packet.extend(image)
        projection.extend(fields)
        history.extend(edits)
    save_native(packet, rows, excluded)
    save_projection(bytes(projection), True)
    save_history(bytes(history), True)


def replay_document(path: str, pin: Path, rows: list[dict[str, object]], excluded: list[dict[str, object]], prior: dict[tuple[object, object], dict[str, object]]) -> tuple[bytes, bytes, bytes]:
    """Recover one document and verify all retained/excluded identities; e.g. replay_document(path, pin, rows, excluded, prior)."""
    streams = recover_sample(path, pin)
    packet, projection, history = bytearray(), bytearray(), bytearray()
    for row in (row for row in rows if row['path'] == path):
        raw = verify_payload(row, streams)
        image, fields, edits = replay_row(row, raw, prior.get((row['path'], row['stream'])))
        packet.extend(image)
        projection.extend(fields)
        history.extend(edits)
    for row in (row for row in excluded if row['path'] == path):
        verify_payload(row, streams)
    return bytes(packet), bytes(projection), bytes(history)


def save_native(packet: bytearray, rows: list[dict[str, object]], excluded: list[dict[str, object]]) -> None:
    """Freeze all candidate counts and full source input/span bytes; e.g. save_native(packet, rows, excluded)."""
    attempts = [attempt for row in rows for attempt in cast(list[dict[str, object]], row['attempts'])]
    accepted = sum(bool(attempt['success']) for attempt in attempts)
    assert len(rows) == 1226 and sum(cast(int, row['bytes']) for row in rows) == 4278214
    assert len(attempts) == 1297 and accepted == 1230
    assert len(excluded) == 6 and sum(cast(int, row['bytes']) for row in excluded) == 150
    (ROOT / 'target/simulation-material-native.bin').write_bytes(packet)
    baseline('native-reference.json', dict(streams=1226, bytes=4278214, attempts=1297, accepted=accepted,
             failed=67, excluded_occurrences=6, excluded_bytes=150, packet_bytes=len(packet), packet_sha256=digest(packet)))
    verify_representative_totals(rows)


def verify_representative_totals(rows: list[dict[str, object]]) -> None:
    """Keep the original first-success counting convention distinct from runtime selection; e.g. verify_representative_totals(rows)."""
    selected = [next(attempt for attempt in cast(list[dict[str, object]], row['attempts']) if attempt['success']) for row in rows]
    totals = {name: sum(cast(int, attempt[name]) for attempt in selected) for name in
              ('property_count', 'delegated_count', 'populated_property_tables', 'populated_delegated_tables')}
    assert list(totals.values()) == [8712, 18390, 18, 38]
    baseline('native-representative-counts.json', dict(convention='First retained success per occurrence; not native runtime selection', **totals))


def replay_owned() -> None:
    """Replay complete independent empty/populated/count-only historical bodies; e.g. replay_owned()."""
    packet = bytearray()
    projection = bytearray()
    history = bytearray()
    cases = owned_material_cases()
    for version, raw in cases:
        parsed = actual_material_parser(raw, version)
        assert parsed['success'] and parsed['end'] == len(raw)
        assert parsed['reconstruction_sha256'] == digest(raw)
        packet.extend(case_packet(raw, version, parsed))
        projection.extend(material_projection_case(raw, version, parsed))
        history.extend(material_history_case(raw, version, parsed))
        check_prefix(raw, version)
    path = ROOT / 'tests/fixtures/simulation-material/owned.bin'
    if '--write' in sys.argv: path.parent.mkdir(exist_ok=True); path.write_bytes(packet)
    assert path.read_bytes() == packet
    baseline('owned-reference.json', dict(cases=len(cases), packet_bytes=len(packet), packet_sha256=digest(packet)))
    save_projection(bytes(projection), False)
    save_history(bytes(history), False)


def save_projection(packet: bytes, native: bool) -> None:
    """Keep complete typed fields for every accepted candidate, including ambiguous versions; e.g. save_projection(packet, True)."""
    path = ROOT / 'target/simulation-material-native-projection.bin' if native else ROOT / 'tests/fixtures/simulation-material/projection.bin'
    if native or '--write' in sys.argv: path.write_bytes(packet)
    assert path.read_bytes() == packet
    baseline(f'{"native" if native else "owned"}-projection-reference.json', dict(cases=1230 if native else 300, bytes=len(packet), sha256=digest(packet)))


def save_history(packet: bytes, native: bool) -> None:
    """Keep complete edited images/fields and all ordered parents; e.g. save_history(packet, True)."""
    path = ROOT / 'target/simulation-material-native-history.bin' if native else ROOT / 'tests/fixtures/simulation-material/history.bin'
    if native or '--write' in sys.argv: path.write_bytes(packet)
    assert path.read_bytes() == packet
    baseline(f'{"native" if native else "owned"}-history-reference.json', dict(cases=1230 if native else 300, bytes=len(packet), sha256=digest(packet)))


def check_prefix(raw: bytes, version: int) -> None:
    """Require a complete explicit prefix and reject the same standalone trailing input; e.g. check_prefix(raw, 891107)."""
    suffix = b'\xa5\x7fnext Property body\0\xff'
    combined = raw + suffix
    parsed = actual_material_parser(combined, version, True)
    assert parsed['success'] and parsed['end'] == len(raw)
    assert parsed['reconstruction_sha256'] == digest(raw) and combined[len(raw):] == suffix
    assert not actual_material_parser(combined, version)['success']


def verify_schema() -> None:
    """Match complete copied clauses and reader hashes against the initial baseline; e.g. verify_schema()."""
    initial = json.loads((FEATURE / 'initial-reference.json').read_bytes())
    text = (SOURCE / 'docs/solidworks-file-format/specification.md').read_bytes().decode('utf-8').replace('\r\n', '\n')
    for name, expected in initial['schemas'].items():
        number = name.removeprefix('schema-').removesuffix('.md')
        start = text.index('### ' + number + ' ')
        end = text.find('\n### ', start + 1)
        raw = (text[start:end if end >= 0 else len(text)].rstrip() + '\n').encode('utf-8')
        assert (FEATURE / name).read_bytes() == raw and digest(raw) == expected
    for name, expected in initial['reader_sha256'].items():
        assert digest((RESEARCH / name).read_bytes()) == expected


def verify_presence_and_empty_shapes() -> None:
    """Check declared-but-absent points separately from populated and empty tables; e.g. verify_presence_and_empty_shapes()."""
    cases: list[dict[str, object]] = []
    for present in (-1, 0, 1, 2):
        parsed = actual_material_parser(material_literal(891107, present), 891107)
        records = cast(list[dict[str, object]], parsed['records'])
        spans = cast(list[dict[str, object]], parsed['spans'])
        assert parsed['success'] and len(records) == 2
        assert records[0]['name'] == 'A\0B' and records[0]['curve_present'] == present
        assert records[0]['pair_count'] == (3 if present else 10000)
        assert any(span['label'] == 'property0.pairs' for span in spans) == bool(present)
        assert records[1]['pair_count'] == 7 and not records[1]['curve_present']
        assert not any(span['label'] == 'property1.pairs' for span in spans)
        cases.append(dict(presence=present, records=records, curves=parsed['curves']))
    empty = actual_material_parser(material_literal(891107, 0, True), 891107)
    assert empty['success'] and empty['records'] == [] and empty['curves'] == []
    baseline('presence-reference.json', dict(cases=cases, empty_body_completed=True))


def verify_utf16_boundary() -> None:
    """Retain unpaired UTF-16 units and raw odd-byte frames without a decoded view (#141); e.g. verify_utf16_boundary()."""
    from cosworkspersist import read_text
    raw = material_literal(891107, 1)
    even = struct.pack('<I', 4) + b'\0\xd8\0\0' + raw[4:]
    odd = struct.pack('<I', 3) + b'A\0\0' + raw[4:]
    assert read_text(even).text == '\ud800'
    assert actual_material_parser(even, 891107)['success']
    odd_record = read_text(odd)
    assert not odd_record.is_null and odd_record.text is None and odd_record.payload == b'A\0\0'
    assert actual_material_parser(odd, 891107)['success']
    baseline('text-boundary-reference.json', dict(issue=141, unpaired_accepted=True, odd_accepted=True, odd_is_null=False, odd_utf16_view=None,
             even_sha256=digest(even), odd_sha256=digest(odd)))


def verify_installed_bindings() -> None:
    """Check current installed module/function/threshold bytes without target execution; e.g. verify_installed_bindings()."""
    earlier, corpus = initial_reports()
    raw = Path('C:/Program Files/SOLIDWORKS Corp/SOLIDWORKS/Simulation/cosworks.dll').read_bytes()
    assert digest(raw) == earlier['module_sha256'] == corpus['module_sha256']
    image = pefile.PE(data=raw, fast_load=True)
    try:
        verify_spans(image, cast(list[dict[str, str]], earlier['functions']))
        verify_spans(image, cast(list[dict[str, str]], corpus['body_storage_bindings']))
        for binding in cast(list[dict[str, object]], earlier['version_constants']):
            expected = bytes.fromhex(cast(str, binding['raw_hex']))
            offset = int(cast(str, binding['address']), 16) - image.OPTIONAL_HEADER.ImageBase
            assert image.get_data(offset, len(expected)) == expected
    finally:
        image.close()


def verify_spans(image: pefile.PE, spans: list[dict[str, str]]) -> None:
    """Match every retained complete function extent; e.g. verify_spans(image, spans)."""
    for span in spans:
        start, end = int(span['address'], 16), int(span['end'], 16)
        assert digest(image.get_data(start - image.OPTIONAL_HEADER.ImageBase, end - start)) == span['raw_sha256']


def replay_adversarial() -> None:
    """Preserve every actual accepted/rejected mutation and independent exact budget; e.g. replay_adversarial()."""
    malformed, count, accepted = material_mutation_packet()
    budgets = material_budget_packet()
    for name, packet in (('malformed.bin', malformed), ('budgets.bin', budgets)):
        path = ROOT / 'tests/fixtures/simulation-material' / name
        if '--write' in sys.argv: path.write_bytes(packet)
        assert path.read_bytes() == packet
    baseline('adversarial-reference.json', dict(cases=count, accepted=accepted, rejected=count-accepted,
             malformed_bytes=len(malformed), malformed_sha256=digest(malformed),
             budget_cases=300, budget_bytes=len(budgets), budget_sha256=digest(budgets)))


def run() -> bool:
    """Use only immutable copies of actual source modules; e.g. run()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-material-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        pinned_auxiliaries(pin, hashes)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin / 'research'))
        replay = update_wrapper(partial(replay_native, pin), replay_native)
        suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in (replay, replay_owned, verify_schema, verify_installed_bindings, verify_presence_and_empty_shapes, verify_utf16_boundary, replay_adversarial))
        result = unittest.TextTestRunner(verbosity=2).run(suite)
        record_snapshot(pin, hashes, result, FEATURE, ('cosworks_material_layout', 'cosworkspersist', 'cfb', 'swcontainer'), ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
