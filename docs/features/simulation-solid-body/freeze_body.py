"""Fresh complete C-734 solid-body boundaries with original C-731 replay (#150)."""
from pathlib import Path
from typing import cast
import json
import struct
import sys
import tempfile
import unittest

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
for folder in ('simulation-binary-thermal', 'model-archives', 'simulation-solid-manager'):
    sys.path.insert(0, str(FEATURE.parent / folder))
sys.path.insert(0, str(ROOT / 'target/reference-python'))
import pefile
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot
from native_carriers import SOURCE, RESEARCH, digest, recover_sample, pinned_auxiliaries
from freeze_solid_manager import verify_image
from owned_body import owned_solid_bodies


def baseline(name: str, value: object) -> None:
    """Initialize evidence only on explicit write; e.g. baseline('native-reference.json', report)."""
    path = FEATURE / name
    raw = (json.dumps(value, indent=2) + '\n').encode()
    if '--write' in sys.argv: path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected unchanged complete evidence'


def retained_report() -> dict[str, object]:
    """Retain every original C-734 alternative and installed binding; e.g. retained_report()."""
    report = json.loads((RESEARCH / 'cosworks-solid-body-expanded-native-evidence.json').read_bytes())
    assert report['evidence'] == 'C-734' and not report['target_executed']
    baseline('initial-reference.json', report)
    return cast(dict[str, object], report)


def actual_body(raw: bytes, version: int) -> dict[str, object]:
    """Use the unchanged actual body reader and full embedded callback; e.g. actual_body(raw, 880004)."""
    from cosworks_solid_body_layout import parse
    from cosworks_embedded_body_ids import archive_reader
    return cast(dict[str, object], parse(raw, version, archive_reader=archive_reader))


def body_input(version: int, raw: bytes, parsed: dict[str, object]) -> bytes:
    """Preserve complete original outcomes and every source boundary; e.g. body_input(version, raw, parsed)."""
    spans = cast(list[dict[str, int]], parsed['spans'])
    return (struct.pack('<III', version, int(cast(bool, parsed['success'])), len(raw)) + raw
            + struct.pack('<I', len(spans)) + b''.join(struct.pack('<II', span['offset'], span['end']) for span in spans))


def replay_body_row(row: dict[str, object], streams: dict[str, bytes]) -> bytes:
    """Keep every exact original argument, including failures; e.g. replay_body_row(row, streams)."""
    raw = streams[cast(str, row['stream'])]
    assert len(raw) == row['bytes'] and digest(raw) == row['sha256']
    packet = bytearray()
    for expected in cast(list[dict[str, object]], row['attempts']):
        version = cast(int, expected['version'])
        parsed = actual_body(raw, version)
        assert dict(version=version, **parsed) == expected
        packet.extend(body_input(version, raw, parsed))
    return bytes(packet)


def replay_native(pin: Path) -> None:
    """Recover all 680 original solid bodies independently; e.g. replay_native(pin)."""
    rows = cast(list[dict[str, object]], retained_report()['rows'])
    packet = bytearray()
    for document in sorted({cast(str, row['path']) for row in rows}):
        streams = recover_sample(document, pin)
        for row in (row for row in rows if row['path'] == document):
            packet.extend(replay_body_row(row, streams))
    attempts = [attempt for row in rows for attempt in cast(list[dict[str, object]], row['attempts'])]
    assert len(rows) == 680 and sum(cast(int, row['bytes']) for row in rows) == 855786
    assert len(attempts) == 745 and sum(cast(bool, attempt['success']) for attempt in attempts) == 691
    (ROOT / 'target/simulation-solid-body-native.bin').write_bytes(packet)
    baseline('native-reference.json', dict(streams=680, logical_bytes=855786, arguments=745,
             accepted=691, failed=54, bytes=len(packet), sha256=digest(packet)))


def replay_owned() -> None:
    """Require actual complete framing of independent historical literals; e.g. replay_owned()."""
    packet = bytearray()
    cases = owned_solid_bodies()
    for _, version, raw in cases:
        parsed = actual_body(raw, version)
        assert parsed['success'] and parsed['end'] == len(raw), (version, parsed)
        assert parsed['reconstruction_sha256'] == digest(raw)
        packet.extend(body_input(version, raw, parsed))
    path = ROOT / 'tests/fixtures/simulation-solid-body/owned.bin'
    if '--write' in sys.argv:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(packet)
    assert path.read_bytes() == packet
    baseline('owned-reference.json', dict(cases=len(cases), bytes=len(packet), sha256=digest(packet)))


def verify_prior_successes() -> None:
    """Preserve every earlier C-731 success through the expanded callback; e.g. verify_prior_successes()."""
    previous = json.loads((RESEARCH / 'cosworks-solid-body-native-evidence.json').read_bytes())
    baseline('initial-c731-reference.json', previous)
    current = {(row['path'], row['stream']): row for row in cast(list[dict[str, object]], retained_report()['rows'])}
    assert len(current) == len(previous['rows']) == 680
    for row in previous['rows']:
        expanded = current[(row['path'], row['stream'])]
        assert expanded['sha256'] == row['sha256']
        for original in row['attempts']:
            if not original['success']: continue
            selected = next(item for item in cast(list[dict[str, object]], expanded['attempts']) if item['version'] == original['version'])
            assert selected['success'] and selected['end'] == original['end']
            assert selected['reconstruction_sha256'] == original['reconstruction_sha256']


def verify_sources() -> None:
    """Bind full source/decompilation evidence and three complete clauses; e.g. verify_sources()."""
    for original, expected in cast(dict[str, str], retained_report()['sources']).items():
        normalized = original.replace('\\', '/')
        relative = normalized.split('/inventor-interface/', 1)[1]
        verify_source_hash(relative, expected)
    text = (SOURCE / 'docs/solidworks-file-format/specification.md').read_bytes().decode().replace('\r\n', '\n')
    for section in ('14.352', '14.355', '14.357'):
        start = text.index(f'### {section} ')
        raw = (text[start:text.index('\n### ', start + 1)].rstrip() + '\n').encode()
        path = FEATURE / f'schema-{section}.md'
        if '--write' in sys.argv: path.write_bytes(raw)
        assert path.read_bytes() == raw


def verify_source_hash(relative: str, original: str) -> None:
    """Allow only the reviewed, unimported dependency pin; e.g. verify_source_hash(relative, original)."""
    current = digest((SOURCE / relative).read_bytes())
    if current == original: return
    require_unused_pin(relative, original, current)


def require_unused_pin(relative: str, original: str, current: str) -> None:
    """Require every dimension of the single reviewed pin; e.g. require_unused_pin(relative, original, current)."""
    reconciliation = json.loads((FEATURE / 'source-reconciliation.json').read_bytes())
    assert relative == reconciliation['file'], relative
    assert original == reconciliation['original_c734_sha256']
    assert current == reconciliation['current_sha256']
    verify_unimported_boundary()


def verify_reconciliation_failures() -> None:
    """Ensure no other file or hash can use this reconciliation; e.g. verify_reconciliation_failures()."""
    pin = json.loads((FEATURE / 'source-reconciliation.json').read_bytes())
    valid = (pin['file'], pin['original_c734_sha256'], pin['current_sha256'])
    require_unused_pin(*valid)
    for index in range(3):
        altered = list(valid)
        altered[index] = 'unreviewed'
        try: require_unused_pin(*altered)
        except AssertionError: continue
        raise AssertionError(f'{altered}; expected rejection outside the exact reviewed pin')


def verify_unimported_boundary() -> None:
    """Reject future callback use of the independently changed general reader; e.g. verify_unimported_boundary()."""
    assert 'modelboundary' not in sys.modules, 'modelboundary; expected no imported general boundary reader in this scoped replay'


def verify_model_bindings(image: pefile.PE, report: dict[str, object]) -> None:
    """Check primary and suffix slots for all four added identities; e.g. verify_model_bindings(image, report)."""
    base = image.OPTIONAL_HEADER.ImageBase
    for binding in cast(list[dict[str, str]], report['id_bindings']):
        table = int(binding['vtable'], 16)
        for offset, key in ((0x28, 'serializer'), (0x2a0, 'suffix')):
            assert struct.unpack('<Q', image.get_data(table + offset - base, 8))[0] == int(binding[key], 16)


def verify_bindings() -> None:
    """Read installed images without loading target code; e.g. verify_bindings()."""
    report = retained_report()
    root = Path('C:/Program Files/SOLIDWORKS Corp/SOLIDWORKS')
    for filename, hash_key in (('Simulation/cosworks.dll', 'module_sha256'), ('sldmodu.dll', 'model_sha256')):
        raw = (root / filename).read_bytes()
        assert digest(raw) == report[hash_key]
        image = pefile.PE(data=raw, fast_load=True)
        try:
            if hash_key == 'module_sha256': verify_image(image, report)
            else: verify_model_bindings(image, report)
        finally:
            image.close()


def run_checks(pin: Path) -> unittest.TestResult:
    """Keep original reader/suffix tests and fresh corpus checks; e.g. run_checks(pin)."""
    from functools import partial, update_wrapper
    import test_cosworks_solid_body
    import test_cosworks_embedded_body_ids
    replay = update_wrapper(partial(replay_native, pin), replay_native)
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in
                               (replay, replay_owned, verify_prior_successes, verify_sources, verify_bindings, verify_unimported_boundary, verify_reconciliation_failures))
    for module in (test_cosworks_solid_body, test_cosworks_embedded_body_ids):
        suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(module))
    return unittest.TextTestRunner(verbosity=2).run(suite)


def run() -> bool:
    """Freeze all reader and container imports before native recovery; e.g. run()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-solid-body-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        pinned_auxiliaries(pin, hashes)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin / 'research'))
        result = run_checks(pin)
        verify_unimported_boundary()
        record_snapshot(pin, hashes, result, FEATURE, ('cosworks_solid_body_layout', 'cosworks_embedded_body_ids', 'cosworkspersist', 'cfb', 'swcontainer'), ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
