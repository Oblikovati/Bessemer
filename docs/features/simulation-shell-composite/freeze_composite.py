"""Exact CompositeFlipTurn/CompositeExtra reference foundation with the historical failure retained (#157)."""
from pathlib import Path
from typing import cast
import json
import struct
import sys
import tempfile
import unittest

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
for folder in ('simulation-binary-thermal', 'model-archives', 'simulation-solid-manager', 'simulation-solid-body', 'simulation-component-persist-id'):
    sys.path.insert(0, str(FEATURE.parent / folder))
sys.path.insert(0, str(ROOT / 'target/reference-python'))
import pefile
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot
from native_carriers import SOURCE, RESEARCH, digest, recover_sample, pinned_auxiliaries
from freeze_solid_manager import verify_image
from freeze_body import verify_source_hash, verify_unimported_boundary, verify_model_bindings
from owned_composite import owned_composite_cases, DOUBLE_BITS


def composite_baseline(name: str, value: object) -> None:
    """Initialize complete evidence only by explicit request; e.g. composite_baseline('native-reference.json', report)."""
    path = FEATURE / name
    raw = (json.dumps(value, indent=2) + '\n').encode()
    if '--write' in sys.argv: path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected unchanged complete evidence'


def composite_report() -> dict[str, object]:
    """Retain both named profiles and every original failure prefix; e.g. composite_report()."""
    report = json.loads((RESEARCH / 'cosworks-composite-native-evidence.json').read_bytes())
    assert report['evidence'] == 'C-742' and not report['target_executed']
    composite_baseline('initial-reference.json', report)
    return cast(dict[str, object], report)


def verify_composite_sources() -> None:
    """Keep all original source pins and the full asymmetric grammar clause; e.g. verify_composite_sources()."""
    for original, expected in cast(dict[str, str], composite_report()['sources']).items():
        relative = original.replace('\\', '/').split('/inventor-interface/', 1)[1]
        verify_source_hash(relative, expected)
    text = (SOURCE / 'docs/solidworks-file-format/specification.md').read_bytes().decode().replace('\r\n', '\n')
    start = text.index('### 14.364 ')
    raw = (text[start:text.index('\n### ', start + 1)].rstrip() + '\n').encode()
    path = FEATURE / 'schema-14.364.md'
    if '--write' in sys.argv: path.write_bytes(raw)
    assert path.read_bytes() == raw


def verify_composite_literals(image: pefile.PE) -> None:
    """Check independent stream names without substituting Property tails; e.g. verify_composite_literals(image)."""
    base = image.OPTIONAL_HEADER.ImageBase
    for address, name in ((0x18205b028, 'CompositeFlipTurn'), (0x18205b050, 'CompositeExtra')):
        expected = (name + '\x00').encode('utf-16-le')
        assert image.get_data(address - base, len(expected)) == expected


def verify_composite_bindings() -> None:
    """Read installed primary/suffix slots and functions without loading target code; e.g. verify_composite_bindings()."""
    report = composite_report()
    root = Path('C:/Program Files/SOLIDWORKS Corp/SOLIDWORKS')
    for filename, hash_key in (('Simulation/cosworks.dll', 'module_sha256'), ('sldmodu.dll', 'model_sha256')):
        raw = (root / filename).read_bytes()
        assert digest(raw) == report[hash_key]
        image = pefile.PE(data=raw, fast_load=True)
        try:
            verify_composite_image(image, report, hash_key)
        finally:
            image.close()


def verify_composite_image(image: pefile.PE, report: dict[str, object], hash_key: str) -> None:
    """Keep primary composite and scoped model-ID bindings distinct; e.g. verify_composite_image(image, report, 'module_sha256')."""
    if hash_key == 'module_sha256':
        verify_image(image, report)
        verify_composite_literals(image)
        return
    assert hash_key == 'model_sha256', hash_key
    verify_model_bindings(image, report)


def actual_composite(raw: bytes, version: int, kind: str, profile: str) -> dict[str, object]:
    """Use the unchanged actual parser and exact installed-ID callback; e.g. actual_composite(raw, 840002, 'CompositeExtra', 'writer')."""
    from cosworks_composite_layout import parse
    from cosworks_embedded_const_surface import archive_reader
    return cast(dict[str, object], parse(raw, version, kind, wire_profile=profile, archive_reader=archive_reader))


def composite_row_packet(row: dict[str, object], streams: dict[str, bytes]) -> bytes:
    """Keep all original named profiles without a first-success selector; e.g. composite_row_packet(row, streams)."""
    stream = cast(str, row['stream'])
    raw = streams[stream]
    assert len(raw) == row['bytes'] and digest(raw) == row['sha256']
    kind = stream.rsplit('/', 1)[-1]
    assert kind in ('CompositeFlipTurn', 'CompositeExtra'), kind
    packet = bytearray()
    for expected in cast(list[dict[str, object]], row['attempts']):
        version, profile = cast(int, expected['version']), cast(str, expected['wire_profile'])
        assert profile in ('reader', 'writer'), profile
        parsed = actual_composite(raw, version, kind, profile)
        assert dict(version=version, wire_profile=profile, **parsed) == expected
        packet.extend(struct.pack('<IIIII', int(kind == 'CompositeExtra'), int(profile == 'writer'), version, int(cast(bool, parsed['success'])), len(raw)) + raw)
    return bytes(packet)


def verify_composite_owned() -> None:
    """Match independently constructed signed/bit/selector/ID expectations against the actual callback; e.g. verify_composite_owned()."""
    outcomes: list[dict[str, object]] = []
    for case in owned_composite_cases():
        raw = case['raw']
        parsed = actual_composite(raw, case['version'], case['kind'], case['wire_profile'])
        assert parsed['success'] and parsed['end'] == len(raw) and parsed['values'] == case['values']
        assert parsed['reconstruction_sha256'] == digest(raw)
        assert b''.join(bytes.fromhex(cast(str, span['raw_hex'])) for span in cast(list[dict[str, object]], parsed['spans'])) == raw
        archives = cast(list[object], parsed['archives'])
        assert len(archives) == (2 if case['persist_present'] and case['version'] >= 840002 else 0)
        if 'member3f8' in case['values']: assert raw[16:64] == struct.pack('<6Q', *DOUBLE_BITS)
        outcomes.append(dict(kind=case['kind'], wire_profile=case['wire_profile'], version=case['version'],
                        persist_present=case['persist_present'], raw_hex=raw.hex(), expected=parsed))
    assert len(outcomes) == 270
    composite_baseline('owned-reference.json', outcomes)


def replay_composite_native(pin: Path) -> None:
    """Recover the complete 20-stream denominator and every failed alternative; e.g. replay_composite_native(pin)."""
    rows = cast(list[dict[str, object]], composite_report()['rows'])
    packet = bytearray()
    for document in sorted({cast(str, row['path']) for row in rows}):
        streams = recover_sample(document, pin)
        packet.extend(b''.join(composite_row_packet(row, streams) for row in rows if row['path'] == document))
    attempts = [attempt for row in rows for attempt in cast(list[dict[str, object]], row['attempts'])]
    complete = [row for row in rows if any(attempt['success'] for attempt in cast(list[dict[str, object]], row['attempts']))]
    assert len(rows) == 20 and sum(cast(int, row['bytes']) for row in rows) == 736
    assert len(complete) == 19 and sum(cast(int, row['bytes']) for row in complete) == 732
    accepted = sum(cast(bool, attempt['success']) for attempt in attempts)
    (ROOT / 'target/simulation-shell-composite-native.bin').write_bytes(packet)
    composite_baseline('native-reference.json', dict(streams=20, bytes=736, complete=19, complete_bytes=732,
                       arguments=len(attempts), accepted=accepted, failed=len(attempts) - accepted,
                       packet_bytes=len(packet), packet_sha256=digest(packet)))


def verify_composite_historical_failure() -> None:
    """Keep the sole four-byte historical occurrence outside every successful result; e.g. verify_composite_historical_failure()."""
    rows = cast(list[dict[str, object]], composite_report()['rows'])
    failures = [row for row in rows if not any(attempt['success'] for attempt in cast(list[dict[str, object]], row['attempts']))]
    assert len(failures) == 1 and failures[0]['bytes'] == 4
    attempts = cast(list[dict[str, object]], failures[0]['attempts'])
    assert {attempt['version'] for attempt in attempts} == {840030}
    assert {attempt['wire_profile'] for attempt in attempts} == {'reader', 'writer'}
    assert all(not attempt['success'] for attempt in attempts)
    composite_baseline('historical-failure-reference.json', failures)


def run_composite_checks(pin: Path) -> unittest.TestResult:
    """Run original callback/grammar tests with exact full native and historical checks; e.g. run_composite_checks(pin)."""
    from functools import partial, update_wrapper
    import test_cosworks_composite
    import test_cosworks_embedded_body_ids
    import test_cosworks_embedded_const_surface
    replay = update_wrapper(partial(replay_composite_native, pin), replay_composite_native)
    checks = (verify_composite_sources, verify_composite_bindings, verify_composite_historical_failure, verify_composite_owned, replay, verify_unimported_boundary)
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    for module in (test_cosworks_composite, test_cosworks_embedded_body_ids, test_cosworks_embedded_const_surface):
        suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(module))
    return unittest.TextTestRunner(verbosity=2).run(suite)


def run() -> bool:
    """Freeze all native-carrier and grammar imports before replay; e.g. run()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-shell-composite-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        pinned_auxiliaries(pin, hashes)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin / 'research'))
        result = run_composite_checks(pin)
        verify_unimported_boundary()
        record_snapshot(pin, hashes, result, FEATURE, ('cosworks_composite_layout', 'cosworks_embedded_const_surface', 'cosworks_embedded_body_ids', 'cosworkspersist', 'cfb', 'swcontainer'), ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
