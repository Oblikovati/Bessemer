"""Fresh standalone Component PersistID boundaries and complete C-738 provenance (#152)."""
from pathlib import Path
from typing import cast
import json
import struct
import sys
import tempfile
import unittest

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
for name in ('simulation-binary-thermal', 'model-archives', 'simulation-solid-manager', 'simulation-solid-body', 'simulation-sheet-property'):
    sys.path.insert(0, str(FEATURE.parent / name))
sys.path.insert(0, str(ROOT / 'target/reference-python'))
import pefile
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot
from native_carriers import SOURCE, RESEARCH, digest, recover_sample, pinned_auxiliaries
from freeze_solid_manager import verify_image
from freeze_body import body_input, verify_model_bindings, verify_source_hash, verify_unimported_boundary
from owned_persist import owned_component_persist_ids
from freeze_sheet import verify_surface_bindings
from persist_projection import persist_id_projection, persist_id_history
from persist_adversarial import persist_id_mutations, persist_id_budgets, persist_id_blob_cap


def baseline(name: str, value: object) -> None:
    """Initialize only complete reviewed evidence explicitly; e.g. baseline('native-reference.json', report)."""
    path = FEATURE / name
    raw = (json.dumps(value, indent=2) + '\n').encode()
    if '--write' in sys.argv: path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected unchanged complete evidence'


def retained_report() -> dict[str, object]:
    """Preserve the complete C-738 report and every version alternative; e.g. retained_report()."""
    report = json.loads((RESEARCH / 'cosworks-component-persistid-native-evidence.json').read_bytes())
    assert report['evidence'] == 'C-738' and not report['target_executed']
    baseline('initial-reference.json', report)
    return cast(dict[str, object], report)


def actual_persist_id(raw: bytes, version: int) -> dict[str, object]:
    """Use the actual standalone reader and exact C-737 embedded callback; e.g. actual_persist_id(raw, 840002)."""
    from cosworks_component_persistid_layout import parse
    from cosworks_embedded_const_surface import archive_reader
    return cast(dict[str, object], parse(raw, version, archive_reader=archive_reader))


def native_row(row: dict[str, object], streams: dict[str, bytes]) -> tuple[bytes, bytes, bytes]:
    """Keep every original accepted and failed argument unchanged; e.g. native_row(row, streams)."""
    raw = streams[cast(str, row['stream'])]
    assert len(raw) == row['bytes'] and digest(raw) == row['sha256']
    packet = bytearray()
    projected = bytearray()
    history = bytearray()
    for expected in cast(list[dict[str, object]], row['attempts']):
        version = cast(int, expected['version'])
        parsed = actual_persist_id(raw, version)
        assert dict(version=version, **parsed) == expected
        packet.extend(body_input(version, raw, parsed))
        if parsed['success']:
            projected.extend(persist_id_projection(version, raw, parsed))
            history.extend(persist_id_history(version, raw))
    return bytes(packet), bytes(projected), bytes(history)


def replay_native(pin: Path) -> None:
    """Recover all retained standalone persist-ID streams independently; e.g. replay_native(pin)."""
    rows = cast(list[dict[str, object]], retained_report()['rows'])
    packet = bytearray()
    projected = bytearray()
    history = bytearray()
    for document in sorted({cast(str, row['path']) for row in rows}):
        streams = recover_sample(document, pin)
        for row in (row for row in rows if row['path'] == document):
            inputs, fields, edits = native_row(row, streams)
            packet.extend(inputs)
            projected.extend(fields)
            history.extend(edits)
    attempts = [item for row in rows for item in cast(list[dict[str, object]], row['attempts'])]
    assert len(rows) == 406 and sum(cast(int, row['bytes']) for row in rows) == 267786
    assert len(attempts) == 453 and sum(cast(bool, item['success']) for item in attempts) == 427
    (ROOT / 'target/simulation-component-persist-id-native.bin').write_bytes(packet)
    save_native_oracles(bytes(projected), bytes(history))
    baseline('native-reference.json', dict(streams=406, logical_bytes=267786, arguments=453,
             accepted=427, failed=26, bytes=len(packet), sha256=digest(packet)))


def replay_owned() -> None:
    """Check independent raw/historical literals with the unchanged reader; e.g. replay_owned()."""
    packet = bytearray()
    projected = bytearray()
    history = bytearray()
    cases = owned_component_persist_ids()
    for _, version, raw in cases:
        parsed = actual_persist_id(raw, version)
        assert parsed['success'] and parsed['end'] == len(raw), (version, parsed)
        assert parsed['reconstruction_sha256'] == digest(raw)
        packet.extend(body_input(version, raw, parsed))
        projected.extend(persist_id_projection(version, raw, parsed))
        history.extend(persist_id_history(version, raw))
    save_owned_packet('owned', bytes(packet), len(cases))
    save_owned_packet('projection', bytes(projected), len(cases))
    save_owned_packet('history', bytes(history), len(cases))


def save_native_oracles(projected: bytes, history: bytes) -> None:
    """Keep complete native fields and outer/nested histories independently pinned; e.g. save_native_oracles(fields, history)."""
    records: dict[str, object] = {}
    for name, packet in (('projection', projected), ('history', history)):
        (ROOT / f'target/simulation-component-persist-id-native-{name}.bin').write_bytes(packet)
        records[name] = dict(cases=427, bytes=len(packet), sha256=digest(packet))
    baseline('native-oracle-reference.json', records)


def save_owned_packet(name: str, packet: bytes, count: int) -> None:
    """Retain immutable standalone input/field/history packets; e.g. save_owned_packet('history', packet, 20)."""
    path = ROOT / f'tests/fixtures/simulation-component-persist-id/{name}.bin'
    if '--write' in sys.argv:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(packet)
    assert path.read_bytes() == packet
    metadata = 'owned-reference.json' if name == 'owned' else f'owned-{name}-reference.json'
    baseline(metadata, dict(cases=count, bytes=len(packet), sha256=digest(packet)))


def verify_adversarial() -> None:
    """Retain all complete outcomes and exact independently observed budgets; e.g. verify_adversarial()."""
    packet, count, accepted = persist_id_mutations(actual_persist_id)
    save_owned_packet('malformed', packet, count)
    save_owned_packet('budgets', persist_id_budgets(actual_persist_id), 20)
    baseline('adversarial-reference.json', dict(inputs=count, accepted=accepted, failed=count - accepted, budget_cases=20))


def verify_blob_cap() -> None:
    """Keep complete sufficient-payload controls separate from malformed counts; e.g. verify_blob_cap()."""
    persist_id_blob_cap(actual_persist_id, baseline)


def verify_sources() -> None:
    """Keep original dependency hashes and the precise unused-file reconciliation; e.g. verify_sources()."""
    for original, expected in cast(dict[str, str], retained_report()['sources']).items():
        relative = original.replace('\\', '/').split('/inventor-interface/', 1)[1]
        verify_source_hash(relative, expected)
    text = (SOURCE / 'docs/solidworks-file-format/specification.md').read_bytes().decode().replace('\r\n', '\n')
    for section in ('14.361',):
        start = text.index(f'### {section} ')
        raw = (text[start:text.index('\n### ', start + 1)].rstrip() + '\n').encode()
        path = FEATURE / f'schema-{section}.md'
        if '--write' in sys.argv: path.write_bytes(raw)
        assert path.read_bytes() == raw
    verify_unimported_boundary()


def verify_bindings() -> None:
    """Check full installed function spans and component storage/literal dispatch; e.g. verify_bindings()."""
    report = retained_report()
    root = Path('C:/Program Files/SOLIDWORKS Corp/SOLIDWORKS')
    for filename, key in (('Simulation/cosworks.dll', 'module_sha256'), ('sldmodu.dll', 'model_sha256')):
        raw = (root / filename).read_bytes()
        assert digest(raw) == report[key]
        image = pefile.PE(data=raw, fast_load=True)
        try:
            if key == 'model_sha256':
                verify_model_bindings(image, report)
                verify_surface_bindings(image)
            else: verify_storage_binding(image, report)
        finally: image.close()


def verify_storage_binding(image: pefile.PE, report: dict[str, object]) -> None:
    """Bind the actual component caller and separate stream literal; e.g. verify_storage_binding(image, report)."""
    verify_image(image, report)
    base = image.OPTIONAL_HEADER.ImageBase
    assert struct.unpack('<Q', image.get_data(0x18205eb98 - base, 8))[0] == 0x1813b5fd0
    expected = 'Component PersistID\0'.encode('utf-16le')
    address = int(cast(str, report['stream_literal']), 16)
    assert image.get_data(address - base, len(expected)) == expected


def run_checks(pin: Path) -> unittest.TestResult:
    """Combine full recovery with every original standalone/identity/surface test; e.g. run_checks(pin)."""
    from functools import partial, update_wrapper
    import test_cosworks_component_persistid
    import test_cosworks_embedded_body_ids
    import test_cosworks_embedded_const_surface
    replay = update_wrapper(partial(replay_native, pin), replay_native)
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in
                               (replay, replay_owned, verify_adversarial, verify_blob_cap, verify_sources, verify_bindings, verify_unimported_boundary))
    for module in (test_cosworks_component_persistid, test_cosworks_embedded_body_ids, test_cosworks_embedded_const_surface):
        suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(module))
    return unittest.TextTestRunner(verbosity=2).run(suite)


def run() -> bool:
    """Freeze all reader imports before inspecting any native stream; e.g. run()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-component-persist-id-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        pinned_auxiliaries(pin, hashes)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin / 'research'))
        result = run_checks(pin)
        verify_unimported_boundary()
        record_snapshot(pin, hashes, result, FEATURE, ('cosworks_component_persistid_layout', 'cosworks_embedded_const_surface', 'cosworkspersist', 'cfb', 'swcontainer'), ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
