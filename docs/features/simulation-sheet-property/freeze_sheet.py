"""Fresh inherited-sheet boundaries with complete C-735/C-736/C-737 provenance (#151)."""
from pathlib import Path
from typing import cast
import json
import struct
import sys
import tempfile
import unittest

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
for name in ('simulation-binary-thermal', 'model-archives', 'simulation-solid-manager', 'simulation-solid-body'):
    sys.path.insert(0, str(FEATURE.parent / name))
sys.path.insert(0, str(ROOT / 'target/reference-python'))
import pefile
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot
from native_carriers import SOURCE, RESEARCH, digest, recover_sample, pinned_auxiliaries
from freeze_solid_manager import verify_image
from freeze_body import body_input, verify_model_bindings, verify_source_hash, verify_unimported_boundary
from owned_sheet import owned_sheets
from owned_sheet_embedded import sheet_embedded_literals
from sheet_projection import sheet_projection, sheet_history
from sheet_adversarial import sheet_mutations, sheet_budgets, sheet_blob_cap


def baseline(name: str, value: object) -> None:
    """Initialize only complete reviewed evidence explicitly; e.g. baseline('native-reference.json', report)."""
    path = FEATURE / name
    raw = (json.dumps(value, indent=2) + '\n').encode()
    if '--write' in sys.argv: path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected unchanged complete evidence'


def retained_report() -> dict[str, object]:
    """Preserve the complete C-737 report and every version alternative; e.g. retained_report()."""
    report = json.loads((RESEARCH / 'cosworks-sheet-property-final-native-evidence.json').read_bytes())
    assert report['evidence'] == 'C-737' and not report['target_executed']
    baseline('initial-reference.json', report)
    return cast(dict[str, object], report)


def actual_sheet(raw: bytes, version: int) -> dict[str, object]:
    """Use the actual inherited reader and exact C-737 embedded callback; e.g. actual_sheet(raw, 840002)."""
    from cosworks_sheet_property_layout import parse
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
        parsed = actual_sheet(raw, version)
        assert dict(version=version, **parsed) == expected
        packet.extend(body_input(version, raw, parsed))
        if parsed['success']:
            projected.extend(sheet_projection(version, raw, parsed))
            history.extend(sheet_history(version, raw))
    return bytes(packet), bytes(projected), bytes(history)


def replay_native(pin: Path) -> None:
    """Recover all retained inherited-sheet streams independently; e.g. replay_native(pin)."""
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
    assert len(rows) == 238 and sum(cast(int, row['bytes']) for row in rows) == 186372
    assert len(attempts) == 243 and sum(cast(bool, item['success']) for item in attempts) == 240
    (ROOT / 'target/simulation-sheet-property-native.bin').write_bytes(packet)
    save_native_oracles(bytes(projected), bytes(history))
    baseline('native-reference.json', dict(streams=238, logical_bytes=186372, arguments=243,
             accepted=240, failed=3, bytes=len(packet), sha256=digest(packet)))


def replay_owned() -> None:
    """Check independent raw/historical literals with the unchanged reader; e.g. replay_owned()."""
    packet = bytearray()
    projected = bytearray()
    history = bytearray()
    cases = owned_sheets()
    for _, version, raw in cases:
        parsed = actual_sheet(raw, version)
        assert parsed['success'] and parsed['end'] == len(raw), (version, parsed)
        assert parsed['reconstruction_sha256'] == digest(raw)
        packet.extend(body_input(version, raw, parsed))
        projected.extend(sheet_projection(version, raw, parsed))
        history.extend(sheet_history(version, raw))
    save_owned_packet('owned', bytes(packet), len(cases))
    save_owned_packet('projection', bytes(projected), len(cases))
    save_owned_packet('history', bytes(history), len(cases))


def save_native_oracles(projected: bytes, history: bytes) -> None:
    """Keep full native field and outer/nested history packets separate from input outcomes; e.g. save_native_oracles(fields, edits)."""
    records: dict[str, object] = {}
    for name, packet in (('projection', projected), ('history', history)):
        (ROOT / f'target/simulation-sheet-property-native-{name}.bin').write_bytes(packet)
        records[name] = dict(cases=240, bytes=len(packet), sha256=digest(packet))
    baseline('native-oracle-reference.json', records)


def save_owned_packet(name: str, packet: bytes, count: int) -> None:
    """Retain complete immutable independent packets; e.g. save_owned_packet('history', packet, 32)."""
    path = ROOT / f'tests/fixtures/simulation-sheet-property/{name}.bin'
    if '--write' in sys.argv:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(packet)
    assert path.read_bytes() == packet
    metadata = 'owned-reference.json' if name == 'owned' else f'owned-{name}-reference.json'
    baseline(metadata, dict(cases=count, bytes=len(packet), sha256=digest(packet)))


def verify_owned_embedded() -> None:
    """Validate exact inner profiles and explicit unsupported geometry stops; e.g. verify_owned_embedded()."""
    from cosworks_embedded_const_surface import archive_reader
    packet = bytearray()
    accepted = 0
    for raw in sheet_embedded_literals():
        try: end, _ = archive_reader(raw, 0)
        except ValueError: end = -1
        success = end == len(raw)
        accepted += success
        packet.extend(struct.pack('<II', int(success), len(raw)) + raw)
    assert accepted == 23
    assert digest(packet[:16675]) == "d63e050c0253dc867bc9324c70ec773e1e9b4fc7d3b506aea6c1280bf8535659"
    path = ROOT / 'tests/fixtures/simulation-sheet-property/embedded.bin'
    if '--write' in sys.argv: path.write_bytes(packet)
    assert path.read_bytes() == packet
    baseline('owned-embedded-reference.json', dict(cases=61, accepted=23, failed=38, bytes=len(packet), sha256=digest(packet)))


def verify_adversarial() -> None:
    """Retain all framing outcomes and exact independently measured budgets; e.g. verify_adversarial()."""
    packet, count, accepted = sheet_mutations(actual_sheet)
    save_owned_packet('malformed', packet, count)
    save_owned_packet('budgets', sheet_budgets(actual_sheet), 32)
    baseline('adversarial-reference.json', dict(inputs=count, accepted=accepted, failed=count - accepted, budget_cases=32))


def verify_blob_cap() -> None:
    """Keep sufficient-payload fixed-cap checks distinct from truncated input tests; e.g. verify_blob_cap()."""
    sheet_blob_cap(actual_sheet, baseline)


def verify_prior_successes() -> None:
    """Preserve all earlier C-735/C-736 successful reconstructions; e.g. verify_prior_successes()."""
    current = {(row['path'], row['stream']): row for row in cast(list[dict[str, object]], retained_report()['rows'])}
    for name in ('cosworks-sheet-property-native-evidence.json', 'cosworks-sheet-property-expanded-native-evidence.json'):
        previous = json.loads((RESEARCH / name).read_bytes())
        baseline('initial-' + cast(str, previous['evidence']).lower() + '-reference.json', previous)
        assert len(current) == len(previous['rows']) == 238
        for row in previous['rows']:
            expanded = current[(row['path'], row['stream'])]
            assert expanded['sha256'] == row['sha256']
            assert_prior_attempts(row['attempts'], cast(list[dict[str, object]], expanded['attempts']))


def assert_prior_attempts(previous: list[dict[str, object]], current: list[dict[str, object]]) -> None:
    """Require complete older successes to survive without changing bytes; e.g. assert_prior_attempts(previous, current)."""
    for original in previous:
        if not original['success']: continue
        selected = next(item for item in current if item['version'] == original['version'])
        assert selected['success'] and selected['end'] == original['end']
        assert selected['reconstruction_sha256'] == original['reconstruction_sha256']


def verify_sources() -> None:
    """Keep original dependency hashes and the precise unused-file reconciliation; e.g. verify_sources()."""
    for original, expected in cast(dict[str, str], retained_report()['sources']).items():
        relative = original.replace('\\', '/').split('/inventor-interface/', 1)[1]
        verify_source_hash(relative, expected)
    text = (SOURCE / 'docs/solidworks-file-format/specification.md').read_bytes().decode().replace('\r\n', '\n')
    for section in ('14.356', '14.358', '14.359', '14.360'):
        start = text.index(f'### {section} ')
        raw = (text[start:text.index('\n### ', start + 1)].rstrip() + '\n').encode()
        path = FEATURE / f'schema-{section}.md'
        if '--write' in sys.argv: path.write_bytes(raw)
        assert path.read_bytes() == raw
    verify_unimported_boundary()


def verify_surface_bindings(image: pefile.PE) -> None:
    """Bind constant-surface and plane-section serializers without executing either; e.g. verify_surface_bindings(image)."""
    base = image.OPTIONAL_HEADER.ImageBase
    reference = json.loads((RESEARCH / 'thicken-native-evidence.json').read_bytes())['references']
    assert reference['sha256'] == retained_report()['model_sha256']
    for address, row in reference['functions'].items():
        raw = bytes.fromhex(row['native_bytes'])
        assert image.get_data(int(address, 16) - base, len(raw)) == raw
    for binding in reference['bindings']:
        for row in binding['rtti']['tables']:
            if row['object_offset'] != 0: continue
            slot = int(row['vftable'], 16) + 0x28 - base
            assert struct.unpack('<Q', image.get_data(slot, 8))[0] == int(row['slots'][5], 16)
    for table, reader in ((0x4ce941e8, 0x4bbbbad0), (0x4cec3d18, 0x4bbba2d0)):
        assert struct.unpack('<Q', image.get_data(table + 0x28 - base, 8))[0] == reader


def verify_bindings() -> None:
    """Check full installed function spans and corrected storage/base dispatch; e.g. verify_bindings()."""
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
    """Require the corrected +60 reader and its actual inherited call target; e.g. verify_storage_binding(image, report)."""
    verify_image(image, report)
    base = image.OPTIONAL_HEADER.ImageBase
    assert struct.unpack('<Q', image.get_data(0x18205a610 - base, 8))[0] == 0x1813314f0
    call = image.get_data(0x181331517 - base, 5)
    assert call[0] == 0xe8 and 0x18133151c + struct.unpack('<i', call[1:])[0] == 0x180a32600


def run_checks(pin: Path) -> unittest.TestResult:
    """Combine full recovery with every original inherited/identity/surface test; e.g. run_checks(pin)."""
    from functools import partial, update_wrapper
    import test_cosworks_sheet_property
    import test_cosworks_embedded_body_ids
    import test_cosworks_embedded_const_surface
    replay = update_wrapper(partial(replay_native, pin), replay_native)
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in
                               (replay, replay_owned, verify_owned_embedded, verify_adversarial, verify_blob_cap, verify_prior_successes, verify_sources, verify_bindings, verify_unimported_boundary))
    for module in (test_cosworks_sheet_property, test_cosworks_embedded_body_ids, test_cosworks_embedded_const_surface):
        suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(module))
    return unittest.TextTestRunner(verbosity=2).run(suite)


def run() -> bool:
    """Freeze all reader imports before inspecting any native stream; e.g. run()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-sheet-property-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        pinned_auxiliaries(pin, hashes)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin / 'research'))
        result = run_checks(pin)
        verify_unimported_boundary()
        record_snapshot(pin, hashes, result, FEATURE, ('cosworks_sheet_property_layout', 'cosworks_embedded_const_surface', 'cosworkspersist', 'cfb', 'swcontainer'), ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
