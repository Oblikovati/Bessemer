"""Fresh SurfaceBody framing with all unresolved C-740 prefix/tail failures retained (#153)."""
from pathlib import Path
from typing import cast
import json
import struct
import sys
import tempfile
import unittest

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
for name in ('simulation-binary-thermal', 'model-archives', 'simulation-solid-manager', 'simulation-solid-body', 'simulation-solid-children'):
    sys.path.insert(0, str(FEATURE.parent / name))
sys.path.insert(0, str(ROOT / 'target/reference-python'))
import pefile
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot
from native_carriers import SOURCE, RESEARCH, digest, recover_sample, pinned_auxiliaries
from freeze_solid_manager import verify_image
from freeze_body import body_input, verify_source_hash
from owned_surface import owned_surface_bodies
from surface_projection import surface_projection, surface_history


def baseline(name: str, value: object) -> None:
    """Initialize complete evidence only on explicit write; e.g. baseline('native-reference.json', report)."""
    path = FEATURE / name
    raw = (json.dumps(value, indent=2) + '\n').encode()
    if '--write' in sys.argv: path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected unchanged complete evidence'


def retained_report() -> dict[str, object]:
    """Keep all C-733 outcomes, including every incomplete native stream; e.g. retained_report()."""
    report = json.loads((RESEARCH / 'cosworks-surface-body-native-evidence.json').read_bytes())
    assert report['evidence'] == 'C-733' and not report['target_executed']
    baseline('initial-reference.json', report)
    return cast(dict[str, object], report)


def tail_report() -> dict[str, object]:
    """Retain the full C-740 metadata/prefix/unclassified-byte investigation; e.g. tail_report()."""
    report = json.loads((RESEARCH / 'cosworks-surface-tail-mismatch-evidence.json').read_bytes())
    assert report['evidence'] == 'C-740' and not report['target_executed']
    baseline('initial-tail-reference.json', report)
    return cast(dict[str, object], report)


def native_row(row: dict[str, object], streams: dict[str, bytes]) -> tuple[bytes, bytes, bytes]:
    """Match every exact original version result without selecting a successful candidate; e.g. native_row(row, streams)."""
    from cosworks_surface_body_layout import parse
    raw = streams[cast(str, row['stream'])]
    assert len(raw) == row['bytes'] and digest(raw) == row['sha256']
    packet = bytearray()
    projected = bytearray()
    history = bytearray()
    for expected in cast(list[dict[str, object]], row['attempts']):
        version = cast(int, expected['version'])
        parsed = parse(raw, version)
        assert dict(version=version, **parsed) == expected
        packet.extend(body_input(version, raw, parsed))
        if parsed['success']:
            projected.extend(surface_projection(version, raw))
            history.extend(surface_history(version, raw))
    return bytes(packet), bytes(projected), bytes(history)


def replay_native(pin: Path) -> None:
    """Freshly recover all 232 identities and all 237 argument outcomes; e.g. replay_native(pin)."""
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
    assert len(rows) == 232 and sum(cast(int, row['bytes']) for row in rows) == 34672
    assert len(attempts) == 237 and sum(cast(bool, item['success']) for item in attempts) == 228
    (ROOT / 'target/simulation-surface-body-native.bin').write_bytes(packet)
    save_native_oracles(bytes(projected), bytes(history))
    baseline('native-reference.json', dict(streams=232, logical_bytes=34672, complete_streams=226,
             arguments=237, accepted=228, failed=9, bytes=len(packet), sha256=digest(packet)))


def replay_owned() -> None:
    """Require complete actual framing for independent historical literals; e.g. replay_owned()."""
    from cosworks_surface_body_layout import parse
    cases = owned_surface_bodies()
    packet = bytearray()
    projected = bytearray()
    history = bytearray()
    for _, version, raw in cases:
        parsed = parse(raw, version)
        assert parsed['success'] and parsed['end'] == len(raw)
        assert parsed['reconstruction_sha256'] == digest(raw)
        packet.extend(body_input(version, raw, parsed))
        projected.extend(surface_projection(version, raw))
        history.extend(surface_history(version, raw))
    save_owned_packet('owned', bytes(packet), len(cases))
    save_owned_packet('projection', bytes(projected), len(cases))
    save_owned_packet('history', bytes(history), len(cases))


def save_native_oracles(projected: bytes, history: bytes) -> None:
    """Pin only complete arguments as field/history oracles while retaining failed inputs separately; e.g. save_native_oracles(fields, edits)."""
    records: dict[str, object] = {}
    for name, packet in (('projection', projected), ('history', history)):
        (ROOT / f'target/simulation-surface-body-native-{name}.bin').write_bytes(packet)
        records[name] = dict(accepted_arguments=228, bytes=len(packet), sha256=digest(packet))
    baseline('native-oracle-reference.json', records)


def save_owned_packet(name: str, packet: bytes, count: int) -> None:
    """Retain immutable complete owned input/field/history packets; e.g. save_owned_packet('history', packet, 96)."""
    path = ROOT / f'tests/fixtures/simulation-surface-body/{name}.bin'
    if '--write' in sys.argv:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(packet)
    assert path.read_bytes() == packet
    metadata = 'owned-reference.json' if name == 'owned' else f'owned-{name}-reference.json'
    baseline(metadata, dict(cases=count, bytes=len(packet), sha256=digest(packet)))


def verify_historical_failures() -> None:
    """Keep exact partial prefixes and four unclassified bytes incomplete; e.g. verify_historical_failures()."""
    from cosworks_surface_body_layout import parse
    retained = cast(list[dict[str, object]], retained_report()['rows'])
    failures = {(row['path'], row['stream']): row for row in retained
                if not any(item['success'] for item in cast(list[dict[str, object]], row['attempts']))}
    tail = tail_report()
    rows = cast(list[dict[str, object]], tail['rows'])
    assert len(rows) == len(failures) == 6 and tail['complete_860006_comparators'] == []
    for row in rows:
        raw = bytes.fromhex(cast(str, row['raw_hex']))
        assert digest(raw) == row['sha256'] == failures[(row['path'], row['stream'])]['sha256']
        parsed = parse(raw, 860006)
        assert not parsed['success'] and parsed['end'] == row['consumed_prefix']
        assert parsed['spans'] == row['prefix_spans'] and parsed['error'] == row['failure']
        assert raw[parsed['end']:].hex() == row['unclassified_tail_hex']
        assert len(raw) - parsed['end'] == 4 and 'need 8' in parsed['error']


def verify_sources() -> None:
    """Require both investigations' exact source/decompilation inputs and clauses; e.g. verify_sources()."""
    for report in (retained_report(), tail_report()):
        for original, expected in cast(dict[str, str], report['sources']).items():
            relative = original.replace('\\', '/').split('/inventor-interface/', 1)[1]
            verify_source_hash(relative, expected)
    text = (SOURCE / 'docs/solidworks-file-format/specification.md').read_bytes().decode().replace('\r\n', '\n')
    for section in ('14.356', '14.362'):
        start = text.index(f'### {section} ')
        raw = (text[start:text.index('\n### ', start + 1)].rstrip() + '\n').encode()
        path = FEATURE / f'schema-{section}.md'
        if '--write' in sys.argv: path.write_bytes(raw)
        assert path.read_bytes() == raw


def verify_bindings() -> None:
    """Inspect installed function/gate/storage bytes without loading target code; e.g. verify_bindings()."""
    report = retained_report()
    raw = Path('C:/Program Files/SOLIDWORKS Corp/SOLIDWORKS/Simulation/cosworks.dll').read_bytes()
    assert digest(raw) == report['module_sha256']
    image = pefile.PE(data=raw, fast_load=True)
    try:
        verify_image(image, report)
        static = json.loads((RESEARCH / 'cosworks-sheet-storage-static-evidence.json').read_bytes())
        for row in static['version_constants']:
            expected = bytes.fromhex(row['raw_hex'])
            assert image.get_data(int(row['address'], 16) - image.OPTIONAL_HEADER.ImageBase, len(expected)) == expected
    finally: image.close()


def run_checks(pin: Path) -> unittest.TestResult:
    """Keep every original boundary test and unresolved failure denominator; e.g. run_checks(pin)."""
    from functools import partial, update_wrapper
    import test_cosworks_surface_body
    replay = update_wrapper(partial(replay_native, pin), replay_native)
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in
                               (replay, replay_owned, verify_historical_failures, verify_sources, verify_bindings))
    suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(test_cosworks_surface_body))
    return unittest.TextTestRunner(verbosity=2).run(suite)


def run() -> bool:
    """Freeze all reader/container imports before native recovery; e.g. run()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-surface-body-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        pinned_auxiliaries(pin, hashes)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin / 'research'))
        result = run_checks(pin)
        record_snapshot(pin, hashes, result, FEATURE, ('cosworks_surface_body_layout', 'cosworkspersist', 'cfb', 'swcontainer'), ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
