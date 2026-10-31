"""Fresh full C-726 evidence and independent joint/group/list/folder literals (#144)."""
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
from owned_groups import KINDS, owned_group_cases
from group_projection import projection_case, history_case
from group_adversarial import mutation_packet, budget_packet


def baseline(name: str, value: object) -> None:
    """Compare complete evidence outside explicit initialization; e.g. baseline('native.json', report)."""
    path = FEATURE / name
    raw = (json.dumps(value, indent=2) + '\n').encode()
    if '--write' in sys.argv: path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected complete unchanged evidence'


def report() -> dict[str, object]:
    """Retain the exact original denominator and every outcome; e.g. report()."""
    value = json.loads((RESEARCH / 'cosworks-beam-group-native-evidence.json').read_bytes())
    assert value['evidence'] == 'C-726' and not value['target_executed']
    assert value['beam_folder_payloads'] == 0
    baseline('initial-reference.json', value)
    return cast(dict[str, object], value)


def actual_parser(raw: bytes, version: int, kind: str, expected: int | None) -> dict[str, object]:
    """Use the actual full parser and explicit count/archive dependencies; e.g. actual_parser(raw, 810030, 'Joint', None)."""
    from cosworks_beam_group_layout import parse
    from cosworks_binary_thermal_transform_audit import archive_reader
    return cast(dict[str, object], parse(raw, version, kind, archive_reader=archive_reader, expected_joint_count=expected))


def packet_case(kind: int, version: int, expected: int | None, raw: bytes, parsed: dict[str, object]) -> bytes:
    """Retain complete bytes and original physical boundaries; e.g. packet_case(2, 891107, 2, raw, parsed)."""
    assert parsed['success'] and parsed['end'] == len(raw)
    assert parsed['reconstruction_sha256'] == digest(raw)
    spans = cast(list[dict[str, int]], parsed['spans'])
    prefix = struct.pack('<IIiI', kind, version, -1 if expected is None else expected, len(raw)) + raw
    return prefix + struct.pack('<I', len(spans)) + b''.join(struct.pack('<II', span['offset'], span['end']) for span in spans)


def native_row(row: dict[str, object], streams: dict[str, bytes]) -> bytes:
    """Match all original candidate details as well as accepted framing; e.g. native_row(row, streams)."""
    raw = streams[cast(str, row['stream'])]
    assert len(raw) == row['bytes'] and digest(raw) == row['sha256']
    kind = cast(str, row['kind'])
    expected = cast(int | None, row['expected_joint_count'])
    packet = bytearray()
    for retained in cast(list[dict[str, object]], row['attempts']):
        version = cast(int, retained['version'])
        parsed = actual_parser(raw, version, kind, expected)
        assert dict(version=version, **parsed) == retained
        packet.extend(packet_case(KINDS.index(kind), version, expected, raw, parsed))
    return bytes(packet)


def replay_native(pin: Path) -> None:
    """Recover every envelope/nested stream and full retained argument; e.g. replay_native(pin)."""
    rows = cast(list[dict[str, object]], report()['rows'])
    packet = bytearray()
    for path in sorted({cast(str, row['path']) for row in rows}):
        streams = recover_sample(path, pin)
        for row in (row for row in rows if row['path'] == path):
            packet.extend(native_row(row, streams))
    assert len(rows) == 414 and sum(cast(int, row['bytes']) for row in rows) == 50872
    attempts = sum(len(cast(list[object], row['attempts'])) for row in rows)
    (ROOT / 'target/simulation-beam-groups-native.bin').write_bytes(packet)
    baseline('native-reference.json', dict(streams=414, attempts=attempts, bytes=len(packet), sha256=digest(packet), native_folders=0))


def replay_owned() -> None:
    """Validate complete independent literals for all kinds and boundaries; e.g. replay_owned()."""
    cases = owned_group_cases()
    packet = bytearray()
    for kind, version, expected, raw in cases:
        parsed = actual_parser(raw, version, KINDS[kind], expected)
        packet.extend(packet_case(kind, version, expected, raw, parsed))
    path = ROOT / 'tests/fixtures/simulation-beam-groups/owned.bin'
    if '--write' in sys.argv:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(packet)
    assert path.read_bytes() == packet
    baseline('owned-reference.json', dict(cases=len(cases), bytes=len(packet), sha256=digest(packet)))


def save_operation(packet: bytes, role: str, native: bool, count: int) -> None:
    """Retain complete expectations with native bytes outside the repository; e.g. save_operation(packet, 'history', True, 414)."""
    path = ROOT / f'target/simulation-beam-groups-native-{role}.bin' if native else ROOT / f'tests/fixtures/simulation-beam-groups/{role}.bin'
    if native or '--write' in sys.argv: path.write_bytes(packet)
    assert path.read_bytes() == packet
    baseline(f'{"native" if native else "owned"}-{role}-reference.json', dict(cases=count, bytes=len(packet), sha256=digest(packet)))


def operation_packets(cases: list[tuple[int, int, int | None, bytes]], native: bool) -> None:
    """Keep every complete physical field and all edit parents; e.g. operation_packets(cases, False)."""
    for role, project in (('projection', projection_case), ('history', history_case)):
        packet = b''.join(project(kind, version, expected, raw) for kind, version, expected, raw in cases)
        save_operation(packet, role, native, len(cases))


def replay_operations(pin: Path) -> None:
    """Freshly extract all native inputs for full independent operation expectations; e.g. replay_operations(pin)."""
    operation_packets(owned_group_cases(), False)
    rows = cast(list[dict[str, object]], report()['rows'])
    cases: list[tuple[int, int, int | None, bytes]] = []
    for path in sorted({cast(str, row['path']) for row in rows}):
        streams = recover_sample(path, pin)
        for row in (row for row in rows if row['path'] == path):
            raw = streams[cast(str, row['stream'])]
            assert digest(raw) == row['sha256'] and len(raw) == row['bytes']
            for attempt in cast(list[dict[str, object]], row['attempts']):
                assert attempt['success']
                cases.append((KINDS.index(cast(str, row['kind'])), cast(int, attempt['version']), cast(int | None, row['expected_joint_count']), raw))
    assert len(cases) == 414
    operation_packets(cases, True)


def verify_sources() -> None:
    """Bind every original source/decompilation and full clause; e.g. verify_sources()."""
    for original, expected in cast(dict[str, str], report()['sources']).items():
        relative = original.replace('\\', '/').split('/solidworks/research/', 1)[1]
        assert digest((RESEARCH / relative).read_bytes()) == expected
    text = (SOURCE / 'docs/solidworks-file-format/specification.md').read_bytes().decode().replace('\r\n', '\n')
    start = text.index('### 14.350 ')
    raw = (text[start:text.index('\n### ', start + 1)].rstrip() + '\n').encode()
    path = FEATURE / 'schema-14.350.md'
    if '--write' in sys.argv: path.write_bytes(raw)
    assert path.read_bytes() == raw


def verify_image(image: pefile.PE, retained: dict[str, object]) -> None:
    """Inspect installed function, threshold and literal bytes; e.g. verify_image(image, retained)."""
    base = image.OPTIONAL_HEADER.ImageBase
    for function in cast(list[dict[str, str]], retained['functions']):
        start, end = int(function['address'], 16), int(function['end'], 16)
        assert digest(image.get_data(start - base, end - start)) == function['raw_sha256']
    for group in ('version_constants', 'literals'):
        for binding in cast(list[dict[str, object]], retained[group]):
            expected = bytes.fromhex(cast(str, binding['raw_hex']))
            assert image.get_data(int(cast(str, binding['address']), 16) - base, len(expected)) == expected


def verify_bindings() -> None:
    """Never load or execute target code while checking its identity; e.g. verify_bindings()."""
    retained = report()
    raw = Path('C:/Program Files/SOLIDWORKS Corp/SOLIDWORKS/Simulation/cosworks.dll').read_bytes()
    assert digest(raw) == retained['module_sha256']
    image = pefile.PE(data=raw, fast_load=True)
    try:
        verify_image(image, retained)
    finally:
        image.close()


def run_checks(pin: Path) -> unittest.TestResult:
    """Combine original boundary cases with fresh native/owned evidence; e.g. run_checks(pin)."""
    from functools import partial, update_wrapper
    import test_cosworks_beam_group
    replay = update_wrapper(partial(replay_native, pin), replay_native)
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in (replay, replay_owned, verify_sources, verify_bindings))
    suite.addTest(unittest.FunctionTestCase(update_wrapper(partial(replay_operations, pin), replay_operations)))
    suite.addTest(unittest.FunctionTestCase(replay_adversarial))
    suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(test_cosworks_beam_group))
    return unittest.TextTestRunner(verbosity=2).run(suite)


def replay_adversarial() -> None:
    """Preserve all actual mutation outcomes and independent exact budgets; e.g. replay_adversarial()."""
    malformed, count, accepted = mutation_packet()
    budgets = budget_packet()
    for name, packet in (('malformed', malformed), ('budgets', budgets)):
        path = ROOT / f'tests/fixtures/simulation-beam-groups/{name}.bin'
        if '--write' in sys.argv: path.write_bytes(packet)
        assert path.read_bytes() == packet
    baseline('adversarial-reference.json', dict(inputs=count, accepted=accepted, rejected=count-accepted,
             malformed_bytes=len(malformed), malformed_sha256=digest(malformed),
             budget_contexts=560, budget_bytes=len(budgets), budget_sha256=digest(budgets)))


def run() -> bool:
    """Keep every source import on an immutable copy; e.g. run()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-beam-groups-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        pinned_auxiliaries(pin, hashes)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin / 'research'))
        result = run_checks(pin)
        readers = ('cosworks_beam_group_layout', 'cosworks_binary_thermal_transform_audit', 'cfb', 'swcontainer')
        record_snapshot(pin, hashes, result, FEATURE, readers, ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
