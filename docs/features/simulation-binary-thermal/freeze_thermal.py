"""Replay complete actual C-716 callbacks from immutable source modules (#134)."""
from pathlib import Path
import json
import sys
import tempfile
import unittest
from functools import partial, update_wrapper
from typing import cast

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
sys.path.insert(0, str(FEATURE.parent / 'model-archives'))
sys.path.insert(0, str(ROOT / 'target/reference-python'))
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot
from native_carriers import recover_carriers, pinned_auxiliaries, digest, RESEARCH, SOURCE, ThermalCarrier
from embedded_reference import verify_owned_embedded, verify_embedded_rejections
from owned_components import owned_holders
from owned_chains import owned_chain_cases, tagged_component
from owned_references import owned_reference_cases, nested_representation_bytes
from native_embedded import detached_archive_packet, component_guard_rejections
from registry_reference import declared_registry
from owned_thermal import owned_thermal_cases, THERMAL_KINDS
from thermal_edit_oracle import owned_scalar_edit_packet, native_scalar_edit_packet
from thermal_malformed import malformed_packet, budget_packet
from thermal_projection import owned_projection_packet, native_projection_packet
from thermal_history_projection import owned_history_projection, native_history_projection
from embedded_map_projection import owned_map_packet, native_map_packet
from embedded_field_projection import owned_field_packet, native_field_packet
from thermal_nested_history import owned_nested_history_packet, native_nested_history_packet


def baseline(name: str, value: object) -> None:
    """Create only initial feature evidence explicitly; e.g. baseline('native-reference.json', {})."""
    path = FEATURE / name
    raw = (json.dumps(value, indent=2) + '\n').encode('utf-8')
    if '--write' in sys.argv:
        path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected unchanged complete baseline'


def replay_candidates(pin: Path) -> None:
    """Compare all 75 complete retained attempts, including failed alternatives; e.g. replay_candidates(pin)."""
    from cosworks_binary_thermal_transform_audit import parse_native, archive_reader
    carriers = recover_carriers(pin)
    embedded = bytearray()
    counts = {'success': 0, 'failed': 0, 'archives': 0, 'transforms': 0}
    for row, raw in carriers:
        for expected in row['attempts']:
            version = cast(int, expected['version'])
            actual = dict(version=version, **parse_native(raw, version, row['stream'].split('/')[-1], archive_reader))
            assert actual == expected, f"{row['path']}:{row['stream']}:{version}; expected complete retained outcome"
            counts['success' if actual['success'] else 'failed'] += 1
            counts['archives'] += len(actual['archive_records']) if actual['success'] else 0
            counts['transforms'] += len(actual['transform_wrappers']) if actual['success'] else 0
            if actual['success']:
                embedded.extend(detached_archive_packet(raw, actual))
    assert counts == dict(success=68, failed=7, archives=242, transforms=19)
    save_native_packets(carriers, embedded, counts)


def save_native_packets(carriers: list[tuple[ThermalCarrier, bytes]], embedded: bytearray, counts: dict[str, int]) -> None:
    """Retain exact full native input packets and their identities; e.g. save_native_packets(carriers, embedded, counts)."""
    payload = b''.join(len(raw).to_bytes(4, 'little') + raw for _, raw in carriers)
    (ROOT / 'target/simulation-binary-thermal-native.bin').write_bytes(payload)
    (ROOT / 'target/simulation-embedded-native.bin').write_bytes(embedded)
    baseline('native-embedded-reference.json', dict(archives=242, packet_bytes=len(embedded), packet_sha256=digest(embedded)))
    baseline('native-reference.json', dict(**counts, streams=len(carriers), bytes=267820, packet_bytes=len(payload), packet_sha256=digest(payload)))
    save_thermal_candidates(carriers)
    save_native_edit_oracle(carriers)
    save_native_projection(carriers)
    save_native_history_projection(carriers)
    save_native_map_projection(embedded)
    save_native_field_projection(embedded)
    save_native_nested_history(carriers)


def save_native_nested_history(carriers: list[tuple[ThermalCarrier, bytes]]) -> None:
    """Record complete actual native nested scalar-edit provenance; e.g. save_native_nested_history(carriers)."""
    packet = native_nested_history_packet(carriers)
    (ROOT / 'target/simulation-thermal-native-nested-history.bin').write_bytes(packet)
    baseline('thermal-native-nested-history-reference.json', dict(cases=len(carriers), bytes=len(packet), sha256=digest(packet)))


def save_native_field_projection(embedded: bytearray) -> None:
    """Record complete actual native primitive fields; e.g. save_native_field_projection(embedded)."""
    packet = native_field_packet(bytes(embedded))
    (ROOT / 'target/simulation-embedded-native-fields.bin').write_bytes(packet)
    baseline('embedded-native-field-reference.json', dict(cases=242, bytes=len(packet), sha256=digest(packet)))


def save_native_map_projection(embedded: bytearray) -> None:
    """Record exact actual native pointer identities; e.g. save_native_map_projection(embedded)."""
    packet = native_map_packet(bytes(embedded))
    (ROOT / 'target/simulation-embedded-native-maps.bin').write_bytes(packet)
    baseline('embedded-native-map-reference.json', dict(cases=242, bytes=len(packet), sha256=digest(packet)))


def save_native_history_projection(carriers: list[tuple[ThermalCarrier, bytes]]) -> None:
    """Record complete native edited-field and outer-history expectations; e.g. save_native_history_projection(carriers)."""
    packet = native_history_projection(carriers)
    (ROOT / 'target/simulation-thermal-native-history.bin').write_bytes(packet)
    baseline('thermal-native-history-reference.json', dict(cases=len(carriers), bytes=len(packet), sha256=digest(packet)))


def save_native_projection(carriers: list[tuple[ThermalCarrier, bytes]]) -> None:
    """Record complete independent native primitives; e.g. save_native_projection(carriers)."""
    packet = native_projection_packet(carriers)
    (ROOT / 'target/simulation-thermal-native-projection.bin').write_bytes(packet)
    baseline('thermal-native-projection-reference.json', dict(cases=len(carriers), bytes=len(packet), sha256=digest(packet)))


def save_native_edit_oracle(carriers: list[tuple[ThermalCarrier, bytes]]) -> None:
    """Record all complete native scalar-edit expectations from actual source replay; e.g. save_native_edit_oracle(carriers)."""
    packet = native_scalar_edit_packet(carriers)
    (ROOT / 'target/simulation-thermal-native-edits.bin').write_bytes(packet)
    baseline('thermal-native-edit-reference.json', dict(cases=len(carriers), bytes=len(packet), sha256=digest(packet)))


def save_thermal_candidates(carriers: list[tuple[ThermalCarrier, bytes]]) -> None:
    """Retain all successful and failed outer contexts without substituting versions; e.g. save_thermal_candidates(carriers)."""
    packet = bytearray()
    kinds = ('Temperature', 'Convection', 'HeatFlux', 'HeatPower', 'Radiation')
    count = 0
    for row, raw in carriers:
        kind = kinds.index(row['stream'].split('/')[-1])
        for attempt in row['attempts']:
            words = (kind, cast(int, attempt['version']), int(cast(bool, attempt['success'])), len(raw))
            packet.extend(b''.join(value.to_bytes(4, 'little') for value in words) + raw)
            count += 1
    assert count == 75
    (ROOT / 'target/simulation-thermal-native-cases.bin').write_bytes(packet)
    baseline('thermal-native-cases-reference.json', dict(contexts=count, successful=68, rejected=7, packet_bytes=len(packet), packet_sha256=digest(packet)))


def source_evidence() -> None:
    """Pin both retained inventories independently of replay; e.g. source_evidence()."""
    names = ('cosworks-binary-thermal-transform-native-evidence.json', 'third-party-storage-corpus.json')
    baseline('native-source-reference.json', {name: digest((RESEARCH / name).read_bytes()) for name in names})
    names = ('reader/cfb.py', 'solidworks/cache/keytable.json')
    baseline('auxiliary-source-reference.json', {name: digest((SOURCE / name).read_bytes()) for name in names})


def owned_archives() -> None:
    """Freeze independent complete populated wrapper images and source boundaries; e.g. owned_archives()."""
    cases = verify_owned_embedded()
    baseline('owned-embedded-reference.json', cases)
    packet = bytearray()
    for case in cases:
        raw = bytes.fromhex(cast(str, case['raw_hex']))
        for value in (len(raw), cast(int, case['wrapper_end']), cast(int, case['first_transform_end'])):
            packet.extend(value.to_bytes(4, 'little'))
        packet.extend(raw)
    path = ROOT / 'tests/fixtures/simulation-binary-thermal/embedded.bin'
    if '--write' in sys.argv:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(packet)
    assert path.read_bytes() == packet


def schema_evidence() -> None:
    """Freeze complete relevant clauses without importing live code; e.g. schema_evidence()."""
    specification = SOURCE / 'docs/solidworks-file-format/specification.md'
    text = specification.read_bytes().decode('utf-8').replace('\r\n', '\n')
    numbers = (282, 292, 293, 296, 298, 299, 301, 304, 305, 308, 309, 310, 311, 312, 313, 315, 316, 319, 320, 339, 340)
    clauses: dict[str, str] = {}
    for section in ('11.6', *(f'14.{number}' for number in numbers)):
        start = text.index(f'### {section} ')
        end = text.find('\n### ', start + 1)
        raw = (text[start:end if end >= 0 else len(text)].rstrip() + '\n').encode('utf-8')
        path = FEATURE / f'schema-{section}.md'
        if '--write' in sys.argv:
            path.write_bytes(raw)
        assert path.read_bytes() == raw, f'{path}; expected unchanged complete clause'
        clauses[section] = digest(raw)
    baseline('schema-reference.json', clauses)


def owned_component_holders() -> None:
    """Replay complete literal holders with the actual C-716 callback; e.g. owned_component_holders()."""
    from cosworks_binary_thermal_transform_audit import archive_reader
    outcomes: list[dict[str, object]] = []
    packet = bytearray()
    for version, raw in owned_holders():
        end, details = archive_reader(raw + b'TAIL', 0)
        assert end == len(raw) and details['object_class'] == 'moCompHolder_c'
        assert details['body_counts']['component_bodies'] == 1
        assert details['body_counts']['component_prefixes'] == 1
        outcomes.append(dict(version=version, raw_hex=raw.hex(), archive=details))
        packet.extend(len(raw).to_bytes(4, 'little') + raw)
    baseline('owned-holders-reference.json', outcomes)
    path = ROOT / 'tests/fixtures/simulation-binary-thermal/holders.bin'
    if '--write' in sys.argv:
        path.write_bytes(packet)
    assert path.read_bytes() == packet


def owned_reference_chains() -> None:
    """Replay all nested chain roots through the actual additive registry; e.g. owned_reference_chains()."""
    from cosworks_binary_thermal_transform_audit import archive_reader
    outcomes: list[dict[str, object]] = []
    packet = bytearray()
    for version, name, raw in owned_chain_cases():
        end, details = archive_reader(raw + b'TAIL', 0)
        assert end == len(raw) and details['object_class'] == name
        outcomes.append(dict(version=version, raw_hex=raw.hex(), archive=details))
        packet.extend(len(raw).to_bytes(4, 'little') + raw)
    baseline('owned-chains-reference.json', outcomes)
    path = ROOT / 'tests/fixtures/simulation-binary-thermal/chains.bin'
    if '--write' in sys.argv:
        path.write_bytes(packet)
    assert path.read_bytes() == packet


def owned_reference_families() -> None:
    """Replay complete independent roots absent from native samples too; e.g. owned_reference_families()."""
    from cosworks_binary_thermal_transform_audit import archive_reader
    outcomes: list[dict[str, object]] = []
    packet = bytearray()
    for version, name, raw in owned_reference_cases():
        end, details = archive_reader(raw + b'TAIL', 0)
        assert end == len(raw) and details['object_class'] == name
        outcomes.append(dict(version=version, raw_hex=raw.hex(), archive=details))
        packet.extend(len(raw).to_bytes(4, 'little') + raw)
    baseline('owned-references-reference.json', outcomes)
    path = ROOT / 'tests/fixtures/simulation-binary-thermal/references.bin'
    if '--write' in sys.argv:
        path.write_bytes(packet)
    assert path.read_bytes() == packet


def representation_depth_boundary() -> None:
    """Keep the reference's fixed representation cap independent of caller budgets; e.g. representation_depth_boundary()."""
    from cosworks_binary_thermal_transform_audit import archive_reader
    from swarchive import ArchiveError
    packet = bytearray()
    prior = sys.getrecursionlimit()
    try:
        sys.setrecursionlimit(10000)
        for depth in (128, 129):
            raw = nested_representation_bytes(depth)
            if depth == 128:
                assert archive_reader(raw, 0)[0] == len(raw)
            else:
                with unittest.TestCase().assertRaises(ArchiveError): archive_reader(raw, 0)
            packet.extend(depth.to_bytes(4, 'little') + len(raw).to_bytes(4, 'little') + raw)
    finally:
        sys.setrecursionlimit(prior)
    path = ROOT / 'tests/fixtures/simulation-binary-thermal/depth.bin'
    if '--write' in sys.argv: path.write_bytes(packet)
    assert path.read_bytes() == packet


def complete_registry_evidence() -> None:
    """Pin the actual full additive class denominator; e.g. complete_registry_evidence()."""
    baseline('registry-reference.json', dict(declared_classes=declared_registry()))


def owned_thermal_collections() -> None:
    """Freeze complete outer layouts and independent actual span boundaries; e.g. owned_thermal_collections()."""
    from cosworks_binary_thermal_transform_audit import parse_native, archive_reader
    packet = bytearray()
    cases = owned_thermal_cases()
    for kind, version, raw in cases:
        result = parse_native(raw, version, THERMAL_KINDS[kind], archive_reader)
        assert result['success'] and result['end'] == len(raw) and len(result['complete_records']) == 2
        assert len(result['archive_records']) == (4 if version >= 840002 else 0)
        packet.extend(b''.join(value.to_bytes(4, 'little') for value in (kind, version, 1, len(raw))) + raw)
        packet.extend(len(result['spans']).to_bytes(4, 'little'))
        for span in result['spans']:
            packet.extend(span['offset'].to_bytes(4, 'little') + span['end'].to_bytes(4, 'little'))
    path = ROOT / 'tests/fixtures/simulation-binary-thermal/thermal-owned.bin'
    if '--write' in sys.argv: path.write_bytes(packet)
    assert path.read_bytes() == packet
    baseline('thermal-owned-reference.json', dict(cases=len(cases), bytes=len(packet), sha256=digest(packet), oracle='Complete actual C-716 parser with full additive registry; exact every-span boundaries'))


def owned_thermal_edits() -> None:
    """Freeze complete independent edited images, rather than deriving expectations in Rust; e.g. owned_thermal_edits()."""
    packet = owned_scalar_edit_packet()
    path = ROOT / 'tests/fixtures/simulation-binary-thermal/thermal-edits.bin'
    if '--write' in sys.argv: path.write_bytes(packet)
    assert path.read_bytes() == packet
    baseline('thermal-owned-edit-reference.json', dict(cases=295, bytes=len(packet), sha256=digest(packet)))


def owned_thermal_adversarial() -> None:
    """Freeze actual acceptance and independent budget boundaries; e.g. owned_thermal_adversarial()."""
    malformed, count, accepted = malformed_packet()
    budgets = budget_packet()
    for name, packet in (('thermal-malformed.bin', malformed), ('thermal-budgets.bin', budgets)):
        path = ROOT / 'tests/fixtures/simulation-binary-thermal' / name
        if '--write' in sys.argv: path.write_bytes(packet)
        assert path.read_bytes() == packet
    baseline('thermal-adversarial-reference.json', dict(cases=count, accepted=accepted,
             malformed_bytes=len(malformed), malformed_sha256=digest(malformed),
             budget_cases=295, budget_bytes=len(budgets), budget_sha256=digest(budgets)))


def owned_thermal_projection() -> None:
    """Freeze every independent primitive and physical field path; e.g. owned_thermal_projection()."""
    packet = owned_projection_packet()
    path = ROOT / 'tests/fixtures/simulation-binary-thermal/thermal-projection.bin'
    if '--write' in sys.argv: path.write_bytes(packet)
    assert path.read_bytes() == packet
    baseline('thermal-owned-projection-reference.json', dict(cases=295, bytes=len(packet), sha256=digest(packet)))


def owned_thermal_history_projection() -> None:
    """Freeze every complete independent edited field and outer parent; e.g. owned_thermal_history_projection()."""
    packet = owned_history_projection()
    path = ROOT / 'tests/fixtures/simulation-binary-thermal/thermal-history.bin'
    if '--write' in sys.argv: path.write_bytes(packet)
    assert path.read_bytes() == packet
    baseline('thermal-owned-history-reference.json', dict(cases=295, bytes=len(packet), sha256=digest(packet)))


def owned_embedded_map_projection() -> None:
    """Freeze every owned actual pointer call across the complete registry; e.g. owned_embedded_map_projection()."""
    packet = owned_map_packet()
    path = ROOT / 'tests/fixtures/simulation-binary-thermal/embedded-maps.bin'
    if '--write' in sys.argv: path.write_bytes(packet)
    assert path.read_bytes() == packet
    baseline('embedded-owned-map-reference.json', dict(cases=1595, bytes=len(packet), sha256=digest(packet)))


def owned_embedded_field_projection() -> None:
    """Freeze every independent owned nested primitive field; e.g. owned_embedded_field_projection()."""
    packet = owned_field_packet()
    path = ROOT / 'tests/fixtures/simulation-binary-thermal/embedded-fields.bin'
    if '--write' in sys.argv: path.write_bytes(packet)
    assert path.read_bytes() == packet
    baseline('embedded-owned-field-reference.json', dict(cases=1595, bytes=len(packet), sha256=digest(packet)))


def owned_nested_history_projection() -> None:
    """Freeze independently observed complete nested parents for owned scalar edits; e.g. owned_nested_history_projection()."""
    packet = owned_nested_history_packet()
    path = ROOT / 'tests/fixtures/simulation-binary-thermal/thermal-nested-history.bin'
    if '--write' in sys.argv: path.write_bytes(packet)
    assert path.read_bytes() == packet
    baseline('thermal-owned-nested-history-reference.json', dict(cases=295, bytes=len(packet), sha256=digest(packet)))


def embedded_string_framing_reference() -> None:
    """Check both nonminimal CString forms with the actual full registry (#137); e.g. embedded_string_framing_reference()."""
    from cosworks_binary_thermal_transform_audit import archive_reader
    from swarchive import Archive
    cases: list[dict[str, object]] = []
    for body in (b'\x01x', b'\xff\x01\x00x'):
        raw = (14000).to_bytes(4, 'little') + tagged_component('moCStringHandle_c', body)
        end, details = archive_reader(raw, 0)
        assert end == len(raw) and details['object_class'] == 'moCStringHandle_c'
        text = Archive(raw, 14000, len(raw) - len(body))
        assert text.cstring() == 'x' and text.p == len(raw)
        cases.append(dict(bytes=len(raw), raw_hex=raw.hex(), sha256=digest(raw), archive=details))
    assert [case['bytes'] for case in cases] == [29, 31]
    baseline('framing-noop-reference.json', dict(issue=137, same_decoded_text=True, different_retained_wire=True, cases=cases))


def run() -> bool:
    """Run actual unmodified modules from a consistent source copy; e.g. run()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-binary-thermal-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        pinned_auxiliaries(pin, hashes)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin / 'research'))
        replay = update_wrapper(partial(replay_candidates, pin), replay_candidates)
        checks = (replay, source_evidence, owned_archives, verify_embedded_rejections, schema_evidence, owned_component_holders, owned_reference_chains, component_guard_rejections, owned_reference_families, representation_depth_boundary, complete_registry_evidence, owned_thermal_collections, owned_thermal_edits, owned_thermal_adversarial, owned_thermal_projection, owned_thermal_history_projection, owned_embedded_map_projection, owned_embedded_field_projection, owned_nested_history_projection, embedded_string_framing_reference)
        suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
        result = unittest.TextTestRunner(verbosity=2).run(suite)
        readers = ('cosworks_binary_thermal_transform_audit', 'cosworks_binary_thermal_layout', 'cosworks_force_optional_audit', 'cfb', 'swcontainer')
        record_snapshot(pin, hashes, result, FEATURE, readers, ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
