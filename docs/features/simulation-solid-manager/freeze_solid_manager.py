"""Fresh complete direct SolidManager Property evidence without target execution (#148)."""
from pathlib import Path
from typing import cast
import json
import struct
import sys
import tempfile
import unittest

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
for folder in ('simulation-binary-thermal', 'model-archives'):
    sys.path.insert(0, str(FEATURE.parent / folder))
sys.path.insert(0, str(ROOT / 'target/reference-python'))
import pefile
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot
from native_carriers import SOURCE, RESEARCH, digest, recover_sample, pinned_auxiliaries
from owned_solid_manager import owned_solid_headers, solid_counted_nulls
from solid_projection import projection_case, history_case
from solid_adversarial import mutation_packet, budget_packet


def baseline(name: str, value: object) -> None:
    """Only initialize full evidence explicitly; e.g. baseline('native-reference.json', report)."""
    path = FEATURE / name
    raw = (json.dumps(value, indent=2) + '\n').encode()
    if '--write' in sys.argv: path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected unchanged complete evidence'


def retained_report() -> dict[str, object]:
    """Keep all original accepted and failed arguments; e.g. retained_report()."""
    retained = json.loads((RESEARCH / 'cosworks-solid-manager-native-evidence.json').read_bytes())
    assert retained['evidence'] == 'C-729' and not retained['target_executed']
    baseline('initial-reference.json', retained)
    return cast(dict[str, object], retained)


def case_packet(version: int, raw: bytes, parsed: dict[str, object]) -> bytes:
    """Preserve complete bodies and every original physical span; e.g. case_packet(870008, raw, parsed)."""
    spans = cast(list[dict[str, int]], parsed['spans'])
    return (struct.pack('<III', version, int(cast(bool, parsed['success'])), len(raw)) + raw
            + struct.pack('<I', len(spans)) + b''.join(struct.pack('<II', span['offset'], span['end']) for span in spans))


def native_row(row: dict[str, object], streams: dict[str, bytes]) -> tuple[bytes, bytes, bytes]:
    """Require exact full original outcomes after fresh extraction; e.g. native_row(row, streams)."""
    from cosworks_solid_manager_layout import parse
    raw = streams[cast(str, row['stream'])]
    assert len(raw) == row['bytes'] and digest(raw) == row['sha256']
    packets = [bytearray() for _ in range(3)]
    for expected in cast(list[dict[str, object]], row['attempts']):
        version = cast(int, expected['version'])
        parsed = parse(raw, version)
        assert dict(version=version, **parsed) == expected
        packets[0].extend(case_packet(version, raw, parsed))
        if parsed['success']:
            packets[1].extend(projection_case(version, raw, parsed))
            packets[2].extend(history_case(version, raw))
    return cast(tuple[bytes, bytes, bytes], tuple(bytes(packet) for packet in packets))


def replay_native(pin: Path) -> None:
    """Recover all 361 identities and retain all 395 framing arguments; e.g. replay_native(pin)."""
    rows = cast(list[dict[str, object]], retained_report()['rows'])
    packets = [bytearray() for _ in range(3)]
    for document in sorted({cast(str, row['path']) for row in rows}):
        streams = recover_sample(document, pin)
        for row in (row for row in rows if row['path'] == document):
            for packet, fragment in zip(packets, native_row(row, streams), strict=True): packet.extend(fragment)
    attempts = [attempt for row in rows for attempt in cast(list[dict[str, object]], row['attempts'])]
    assert len(rows) == 361 and sum(cast(int, row['bytes']) for row in rows) == 5136
    assert len(attempts) == 395 and sum(cast(bool, attempt['success']) for attempt in attempts) == 365
    packet = bytes(packets[0])
    (ROOT / 'target/simulation-solid-manager-native.bin').write_bytes(packet)
    baseline('native-reference.json', dict(streams=361, arguments=395, accepted=365, failed=30, bytes=len(packet), sha256=digest(packet)))

    for role, packet in zip(('projection', 'history'), packets[1:], strict=True): save_operation(bytes(packet), role, True, 365)


def replay_owned() -> None:
    """Require every independent profile to complete with raw spans intact; e.g. replay_owned()."""
    from cosworks_solid_manager_layout import parse
    cases = owned_solid_headers()
    packet = bytearray()
    projection = bytearray()
    history = bytearray()
    for _, version, raw in cases:
        parsed = parse(raw, version)
        assert parsed['success'] and parsed['end'] == len(raw), (version, parsed)
        assert parsed['reconstruction_sha256'] == digest(raw)
        packet.extend(case_packet(version, raw, parsed))
        projection.extend(projection_case(version, raw, parsed))
        history.extend(history_case(version, raw))
    save_owned_input(bytes(packet), len(cases))
    save_operation(bytes(projection), 'projection', False, len(cases))
    save_operation(bytes(history), 'history', False, len(cases))


def save_owned_input(packet: bytes, count: int) -> None:
    """Freeze all complete original owned inputs and spans; e.g. save_owned_input(packet, 68)."""
    path = ROOT / 'tests/fixtures/simulation-solid-manager/owned.bin'
    if '--write' in sys.argv:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(packet)
    assert path.read_bytes() == packet
    baseline('owned-reference.json', dict(cases=count, bytes=len(packet), sha256=digest(packet)))



def save_operation(packet: bytes, role: str, native: bool, count: int) -> None:
    """Keep native bytes outside the repository and owned expectations complete; e.g. save_operation(packet, 'history', True, 365)."""
    path = ROOT / f'target/simulation-solid-manager-native-{role}.bin' if native else ROOT / f'tests/fixtures/simulation-solid-manager/{role}.bin'
    if native or '--write' in sys.argv: path.write_bytes(packet)
    assert path.read_bytes() == packet
    baseline(f'{"native" if native else "owned"}-{role}-reference.json', dict(cases=count, bytes=len(packet), sha256=digest(packet)))


def verify_sources() -> None:
    """Pin complete original evidence dependencies and the full direct-header clause; e.g. verify_sources()."""
    for original, expected in cast(dict[str, str], retained_report()['sources']).items():
        relative = original.replace('\\', '/').split('/solidworks/research/', 1)[1]
        assert digest((RESEARCH / relative).read_bytes()) == expected
    text = (SOURCE / 'docs/solidworks-file-format/specification.md').read_bytes().decode().replace('\r\n', '\n')
    start = text.index('### 14.353 ')
    raw = (text[start:text.index('\n### ', start + 1)].rstrip() + '\n').encode()
    path = FEATURE / 'schema-14.353.md'
    if '--write' in sys.argv: path.write_bytes(raw)
    assert path.read_bytes() == raw


def verify_image(image: pefile.PE, retained: dict[str, object]) -> None:
    """Bind installed constructor/reader spans, storage pointer and thresholds; e.g. verify_image(image, retained)."""
    base = image.OPTIONAL_HEADER.ImageBase
    for function in cast(list[dict[str, str]], retained['functions']):
        start, end = int(function['address'], 16), int(function['end'], 16)
        assert digest(image.get_data(start - base, end - start)) == function['raw_sha256']
    slot = int(cast(str, retained['reader_slot']), 16)
    assert struct.unpack('<Q', image.get_data(slot - base, 8))[0] == int(cast(str, retained['reader']), 16)
    for binding in cast(list[dict[str, object]], retained['version_constants']):
        expected = bytes.fromhex(cast(str, binding['raw_hex']))
        assert image.get_data(int(cast(str, binding['address']), 16) - base, len(expected)) == expected


def verify_bindings() -> None:
    """Inspect bytes without loading or executing the target DLL; e.g. verify_bindings()."""
    retained = retained_report()
    raw = Path('C:/Program Files/SOLIDWORKS Corp/SOLIDWORKS/Simulation/cosworks.dll').read_bytes()
    assert digest(raw) == retained['module_sha256']
    image = pefile.PE(data=raw, fast_load=True)
    try:
        verify_image(image, retained)
    finally:
        image.close()


def run_checks(pin: Path) -> unittest.TestResult:
    """Combine actual original boundary tests with fresh native/owned evidence; e.g. run_checks(pin)."""
    from functools import partial, update_wrapper
    import test_cosworks_solid_manager
    replay = update_wrapper(partial(replay_native, pin), replay_native)
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in (replay, replay_owned, verify_sources, verify_bindings, verify_hardcaps, replay_adversarial))
    suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(test_cosworks_solid_manager))
    return unittest.TextTestRunner(verbosity=2).run(suite)


def replay_adversarial() -> None:
    """Keep every actual mutation and exact source-derived budget; e.g. replay_adversarial()."""
    malformed, count, accepted = mutation_packet()
    budgets = budget_packet()
    for name, packet in (('malformed', malformed), ('budgets', budgets)):
        path = ROOT / f'tests/fixtures/simulation-solid-manager/{name}.bin'
        if '--write' in sys.argv: path.write_bytes(packet)
        assert path.read_bytes() == packet
    baseline('adversarial-reference.json', dict(inputs=count, accepted=accepted, rejected=count-accepted,
             malformed_bytes=len(malformed), malformed_sha256=digest(malformed),
             budget_contexts=68, budget_bytes=len(budgets), budget_sha256=digest(budgets)))


def verify_hardcaps() -> None:
    """Distinguish count limits from truncation with complete synthetic payloads; e.g. verify_hardcaps()."""
    from cosworks_solid_manager_layout import parse
    cases = ((870004, 100000, 0, True), (870008, 100000, 100000, True),
             (870004, 100001, 0, False), (870008, 0, 100001, False))
    rows: list[dict[str, object]] = []
    for version, first, second, accepted in cases:
        raw = solid_counted_nulls(version, first, second)
        parsed = parse(raw, version)
        assert parsed['success'] == accepted
        if accepted:
            assert parsed['end'] == len(raw) and parsed['reconstruction_sha256'] == digest(raw)
            assert len(parsed['spans']) == 5 + first + (1 + second if version >= 870008 else 0)
        else: assert 'over-limit count 100001' in parsed['error']
        rows.append(dict(version=version, first_count=first, second_count=second, accepted=accepted, bytes=len(raw), sha256=digest(raw)))
    baseline('hardcap-reference.json', dict(scope='synthetic only; no populated native list fixture', cases=rows))


def run() -> bool:
    """Keep every actual reader and container adapter on an immutable copy; e.g. run()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-solid-manager-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        pinned_auxiliaries(pin, hashes)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin / 'research'))
        result = run_checks(pin)
        record_snapshot(pin, hashes, result, FEATURE, ('cosworks_solid_manager_layout', 'cosworkspersist', 'cfb', 'swcontainer'), ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
