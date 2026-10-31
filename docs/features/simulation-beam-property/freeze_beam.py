"""Fresh complete beam Property reference replay without target execution (#142)."""
from pathlib import Path
from typing import cast
import json
import struct
import sys
import tempfile
import unittest

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
sys.path.insert(0, str(FEATURE.parent / 'simulation-binary-thermal'))
sys.path.insert(0, str(FEATURE.parent / 'model-archives'))
sys.path.insert(0, str(FEATURE.parent / 'simulation-inertial-loads'))
sys.path.insert(0, str(ROOT / 'target/reference-python'))
import pefile
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot
from native_carriers import SOURCE, RESEARCH, digest, recover_sample, pinned_auxiliaries
from owned_beam import owned_beam_cases
from beam_projection import beam_parser, projection_case, history_case, nested_history_case
from beam_adversarial import mutation_packet, budget_packet


def freeze(name: str, value: object) -> None:
    """Compare complete evidence unless initializing explicitly; e.g. freeze('native.json', report)."""
    path = FEATURE / name
    raw = (json.dumps(value, indent=2) + '\n').encode()
    if '--write' in sys.argv: path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected unchanged complete evidence'


def beam_report() -> dict[str, object]:
    """Read the full retained denominator; e.g. beam_report()."""
    report = json.loads((RESEARCH / 'cosworks-beam-property-native-evidence.json').read_bytes())
    assert report['evidence'] == 'C-725' and not report['target_executed']
    freeze('initial-reference.json', report)
    return cast(dict[str, object], report)


def replay_row(row: dict[str, object], streams: dict[str, bytes]) -> bytes:
    """Match every original outcome and physical span; e.g. replay_row(row, streams)."""
    from cosworks_beam_property_layout import parse
    from cosworks_binary_thermal_transform_audit import archive_reader
    raw = streams[cast(str, row['stream'])]
    assert len(raw) == row['bytes'] and digest(raw) == row['sha256']
    packet = bytearray()
    for expected in cast(list[dict[str, object]], row['attempts']):
        version = cast(int, expected['version'])
        actual = parse(raw, version, 'Beam', archive_reader=archive_reader)
        assert dict(version=version, **actual) == expected
        spans = cast(list[dict[str, int]], actual['spans'])
        packet.extend(struct.pack('<III', version, int(actual['success']), len(raw)) + raw)
        packet.extend(struct.pack('<I', len(spans)))
        packet.extend(b''.join(struct.pack('<II', span['offset'], span['end']) for span in spans))
    return bytes(packet)


def replay_native(pin: Path) -> None:
    """Freshly recover all 163 complete native occurrences; e.g. replay_native(pin)."""
    rows = cast(list[dict[str, object]], beam_report()['rows'])
    packet = bytearray()
    for path in sorted({cast(str, row['path']) for row in rows}):
        streams = recover_sample(path, pin)
        for row in (row for row in rows if row['path'] == path):
            packet.extend(replay_row(row, streams))
    assert len(rows) == 163 and sum(cast(int, row['bytes']) for row in rows) == 43516
    attempts = sum(len(cast(list[object], row['attempts'])) for row in rows)
    assert attempts == 163
    (ROOT / 'target/simulation-beam-property-native.bin').write_bytes(packet)
    freeze('native-reference.json', dict(streams=len(rows), attempts=attempts,
           bytes=len(packet), sha256=digest(packet)))


def save_operation_packet(packet: bytes, role: str, native: bool, count: int) -> None:
    """Keep native payloads outside the repository; e.g. save_operation_packet(packet, 'history', True, 163)."""
    path = ROOT / f'target/simulation-beam-property-native-{role}.bin' if native else ROOT / f'tests/fixtures/simulation-beam-property/{role}.bin'
    if native or '--write' in sys.argv: path.write_bytes(packet)
    assert path.read_bytes() == packet
    freeze(f'{"native" if native else "owned"}-{role}-reference.json', dict(cases=count, bytes=len(packet), sha256=digest(packet)))


def operation_packets(cases: list[tuple[int, bytes]], native: bool) -> None:
    """Export every complete typed field and edited outer relation; e.g. operation_packets(cases, False)."""
    projection = bytearray()
    history = bytearray()
    nested = bytearray()
    for version, raw in cases:
        projection.extend(projection_case(version, raw, beam_parser(raw, version)))
        history.extend(history_case(version, raw))
        nested.extend(nested_history_case(version, raw))
    save_operation_packet(bytes(projection), 'projection', native, len(cases))
    save_operation_packet(bytes(history), 'history', native, len(cases))
    save_operation_packet(bytes(nested), 'nested-history', native, len(cases))


def replay_operations(pin: Path) -> None:
    """Use freshly recovered originals for all retained candidate versions; e.g. replay_operations(pin)."""
    operation_packets([(version, raw) for _, version, raw in owned_beam_cases()], False)
    rows = cast(list[dict[str, object]], beam_report()['rows'])
    cases: list[tuple[int, bytes]] = []
    for path in sorted({cast(str, row['path']) for row in rows}):
        streams = recover_sample(path, pin)
        for row in (row for row in rows if row['path'] == path):
            raw = streams[cast(str, row['stream'])]
            assert digest(raw) == row['sha256'] and len(raw) == row['bytes']
            cases.extend((cast(int, attempt['version']), raw) for attempt in cast(list[dict[str, object]], row['attempts']) if attempt['success'])
    assert len(cases) == 163
    operation_packets(cases, True)


def verify_sources() -> None:
    """Bind all original dependencies and the complete clause; e.g. verify_sources()."""
    report = beam_report()
    for original, expected in cast(dict[str, str], report['sources']).items():
        relative = original.replace('\\', '/').split('/solidworks/research/', 1)[1]
        assert digest((RESEARCH / relative).read_bytes()) == expected
    text = (SOURCE / 'docs/solidworks-file-format/specification.md').read_bytes().decode().replace('\r\n', '\n')
    start = text.index('### 14.349 ')
    raw = (text[start:text.index('\n### ', start + 1)].rstrip() + '\n').encode()
    path = FEATURE / 'schema-14.349.md'
    if '--write' in sys.argv: path.write_bytes(raw)
    assert path.read_bytes() == raw


def replay_owned() -> None:
    """Require complete actual-source spans for every independent literal; e.g. replay_owned()."""
    from cosworks_beam_property_layout import parse
    from cosworks_binary_thermal_transform_audit import archive_reader
    cases = owned_beam_cases()
    packet = bytearray()
    for profile, version, raw in cases:
        parsed = parse(raw, version, 'Beam', archive_reader=archive_reader)
        assert parsed['success'] and parsed['end'] == len(raw), (profile, version, parsed)
        assert parsed['reconstruction_sha256'] == digest(raw)
        spans = parsed['spans']
        packet.extend(struct.pack('<III', profile, version, len(raw)) + raw)
        packet.extend(struct.pack('<I', len(spans)))
        packet.extend(b''.join(struct.pack('<II', span['offset'], span['end']) for span in spans))
    path = ROOT / 'tests/fixtures/simulation-beam-property/owned.bin'
    if '--write' in sys.argv:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(packet)
    assert path.read_bytes() == packet
    freeze('owned-reference.json', dict(cases=len(cases), bytes=len(packet), sha256=digest(packet)))


def verify_image(image: pefile.PE, report: dict[str, object]) -> None:
    """Check installed code spans, slots and threshold bytes; e.g. verify_image(image, report)."""
    base = image.OPTIONAL_HEADER.ImageBase
    for function in cast(list[dict[str, str]], report['functions']):
        start, end = int(function['address'], 16), int(function['end'], 16)
        assert digest(image.get_data(start - base, end - start)) == function['raw_sha256']
    for slot in cast(list[dict[str, str]], report['serializer_slots']):
        actual = struct.unpack('<Q', image.get_data(int(slot['address'], 16) - base, 8))[0]
        assert actual == int(slot['target'], 16)
    for binding in cast(list[dict[str, object]], report['version_constants']):
        expected = bytes.fromhex(cast(str, binding['raw_hex']))
        assert image.get_data(int(cast(str, binding['address']), 16) - base, len(expected)) == expected


def verify_bindings() -> None:
    """Inspect installed bytes without loading or executing the DLL; e.g. verify_bindings()."""
    report = beam_report()
    raw = Path('C:/Program Files/SOLIDWORKS Corp/SOLIDWORKS/Simulation/cosworks.dll').read_bytes()
    assert digest(raw) == report['module_sha256']
    image = pefile.PE(data=raw, fast_load=True)
    try:
        verify_image(image, report)
    finally:
        image.close()


def run_checks(pin: Path) -> unittest.TestResult:
    """Run original boundary tests alongside fresh evidence checks; e.g. run_checks(pin)."""
    from functools import partial, update_wrapper
    import test_cosworks_beam_property
    replay = update_wrapper(partial(replay_native, pin), replay_native)
    suite = unittest.TestSuite(unittest.FunctionTestCase(check)
                               for check in (replay, verify_sources, verify_bindings, replay_owned))
    suite.addTest(unittest.FunctionTestCase(update_wrapper(partial(replay_operations, pin), replay_operations)))
    suite.addTest(unittest.FunctionTestCase(replay_adversarial))
    suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(test_cosworks_beam_property))
    return unittest.TextTestRunner(verbosity=2).run(suite)


def replay_adversarial() -> None:
    """Retain every actual mutation result and independent exact budget; e.g. replay_adversarial()."""
    malformed, count, accepted = mutation_packet()
    budgets = budget_packet()
    for name, packet in (('malformed', malformed), ('budgets', budgets)):
        path = ROOT / f'tests/fixtures/simulation-beam-property/{name}.bin'
        if '--write' in sys.argv: path.write_bytes(packet)
        assert path.read_bytes() == packet
    freeze('adversarial-reference.json', dict(inputs=count, accepted=accepted, rejected=count-accepted,
           malformed_bytes=len(malformed), malformed_sha256=digest(malformed),
           budget_cases=104, budget_bytes=len(budgets), budget_sha256=digest(budgets)))


def run() -> bool:
    """Use an immutable copy for every actual parser import; e.g. run()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-beam-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        pinned_auxiliaries(pin, hashes)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin / 'research'))
        result = run_checks(pin)
        readers = ('cosworks_beam_property_layout', 'cosworks_binary_thermal_transform_audit', 'cfb', 'swcontainer')
        record_snapshot(pin, hashes, result, FEATURE, readers, ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
