"""Fresh seven-family SolidManager replay with the six unresolved streams retained (#154)."""
from pathlib import Path
from typing import cast
import json
import struct
import sys
import tempfile
import unittest

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
for folder in ('simulation-binary-thermal', 'model-archives', 'simulation-solid-manager', 'simulation-solid-body',
               'simulation-solid-children', 'simulation-sheet-property', 'simulation-component-persist-id', 'simulation-material', 'simulation-surface-body'):
    sys.path.insert(0, str(FEATURE.parent / folder))
sys.path.insert(0, str(ROOT / 'target/reference-python'))
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot
from native_carriers import SOURCE, RESEARCH, digest, recover_sample, pinned_auxiliaries
from freeze_body import verify_source_hash, verify_unimported_boundary
from solid_inventory_bindings import REPORTS, SolidInventoryPayload, retained_solid_inventory, solid_inventory_payloads, complete_identity_match, solid_payload_family
from solid_inventory_replay import actual_solid_inventory
from owned_solid_inventory import owned_solid_inventory


def baseline(name: str, value: object) -> None:
    """Initialize evidence only by explicit request; e.g. baseline('fresh-reference.json', report)."""
    path = FEATURE / name
    raw = (json.dumps(value, indent=2) + '\n').encode()
    if '--write' in sys.argv: path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected unchanged complete evidence'


def solid_identity(payload: SolidInventoryPayload) -> dict[str, object]:
    """Project metadata without native body bytes; e.g. solid_identity(payload)."""
    return dict(path=payload['document'], stream=payload['stream'], bytes=payload['size'], sha256=payload['sha256'])


def verify_solid_denominator() -> None:
    """Bind C-739 to all C-590 identities and all original arguments; e.g. verify_solid_denominator()."""
    retained = retained_solid_inventory()
    baseline('initial-reference.json', retained)
    inventory = json.loads((RESEARCH / 'third-party-storage-corpus.json').read_bytes())
    identities = [dict(path=sample['path'], stream=row['name'], bytes=row['bytes'], sha256=row['sha256'])
                  for sample in inventory['samples'] for row in sample['nested_streams'] if '/SolidManager/' in row['name']]
    payloads = solid_inventory_payloads()
    complete_identity_match(identities, [solid_identity(payload) for payload in payloads])
    baseline('candidate-denominator.json', payloads)
    baseline('identity-manifest-reference.json', [solid_identity(payload) for payload in payloads])
    raw = b''.join(struct.pack('<I', payload['family']) + text_frame(payload['document']) + text_frame(payload['stream'])
                   + struct.pack('<I', payload['size']) for payload in payloads)
    path = FEATURE / 'native-identities.bin'
    if '--write' in sys.argv: path.write_bytes(raw)
    assert path.read_bytes() == raw


def verify_solid_identity_failures() -> None:
    """Reject every metadata substitution before fresh replay; e.g. verify_solid_identity_failures()."""
    rows = [solid_identity(payload) for payload in solid_inventory_payloads()]
    complete_identity_match(rows, rows)
    altered = [rows[:-1], rows + [rows[0]], rows + [dict(rows[0], path='unbound-document')]]
    for key, value in (('path', 'renamed'), ('stream', 'renamed'), ('bytes', -1), ('sha256', '0' * 64)):
        altered.append([dict(rows[0], **{key: value}), *rows[1:]])
    for candidate in altered:
        try: complete_identity_match(rows, candidate)
        except AssertionError: continue
        raise AssertionError('altered identity corpus; expected exact metadata rejection')


def verify_solid_family_failures() -> None:
    """Keep unknown siblings out of supported family dispatch; e.g. verify_solid_family_failures()."""
    root = 'Cosworks/StudyManager/StudyNumber-0/SolidManager/'
    for suffix in ('Unknown', 'Solid Components/Component/Unknown', 'Solid Components/Component/Shell Bodies/Body/Property'):
        try: solid_payload_family(root + suffix)
        except AssertionError: continue
        raise AssertionError(f'{suffix}; expected unsupported family rejection')


def text_frame(value: str) -> bytes:
    """Preserve identity spelling without path normalization; e.g. text_frame('Cosworks')."""
    raw = value.encode()
    return struct.pack('<I', len(raw)) + raw


def replay_owned_solid_inventory() -> None:
    """Export all seven complete families plus an explicit failed identity to platform CI; e.g. replay_owned_solid_inventory()."""
    cases = owned_solid_inventory()
    packet = b''.join(solid_case_packet(payload, raw) for payload, raw in cases)
    path = ROOT / 'tests/fixtures/simulation-solid-inventory/owned.bin'
    if '--write' in sys.argv:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(packet)
    assert path.read_bytes() == packet
    accepted = sum(cast(bool, attempt['success']) for payload, _ in cases for attempt in payload['attempts'])
    baseline('owned-reference.json', dict(streams=len(cases), accepted=accepted, failed=len(cases) - accepted,
             logical_bytes=sum(len(raw) for _, raw in cases), packet_bytes=len(packet), packet_sha256=digest(packet)))


def solid_case_packet(payload: SolidInventoryPayload, raw: bytes) -> bytes:
    """Compare each full original outcome before exporting its argument; e.g. solid_case_packet(payload, raw)."""
    assert len(raw) == payload['size'] and digest(raw) == payload['sha256']
    packet = bytearray(struct.pack('<I', payload['family']))
    packet.extend(text_frame(payload['document']) + text_frame(payload['stream']))
    packet.extend(struct.pack('<I', len(raw)) + raw + struct.pack('<I', len(payload['attempts'])))
    for expected in payload['attempts']:
        version = cast(int, expected['version'])
        assert actual_solid_inventory(payload, raw, version) == expected, (payload['stream'], version)
        packet.extend(struct.pack('<II', version, int(cast(bool, expected['success']))))
    return bytes(packet)


def recover_solid_document(document: str, payloads: list[SolidInventoryPayload], pin: Path) -> bytes:
    """Require the full fresh nested denominator before selecting any stream; e.g. recover_solid_document(document, payloads, pin)."""
    streams = recover_sample(document, pin)
    actual = [dict(path=document, stream=name, bytes=len(raw), sha256=digest(raw))
              for name, raw in streams.items() if '/SolidManager/' in name]
    complete_identity_match([solid_identity(payload) for payload in payloads], actual)
    return b''.join(solid_case_packet(payload, streams[payload['stream']]) for payload in payloads)


def replay_solid_inventory(pin: Path) -> None:
    """Replay all seven families together without converting incomplete streams to success; e.g. replay_solid_inventory(pin)."""
    payloads = solid_inventory_payloads()
    packet = bytearray()
    for document in sorted({payload['document'] for payload in payloads}):
        selected = [payload for payload in payloads if payload['document'] == document]
        packet.extend(recover_solid_document(document, selected, pin))
    attempts = [attempt for payload in payloads for attempt in payload['attempts']]
    accepted = sum(cast(bool, attempt['success']) for attempt in attempts)
    (ROOT / 'target/simulation-solid-inventory-native.bin').write_bytes(packet)
    baseline('fresh-replay-reference.json', dict(streams=len(payloads), logical_bytes=sum(payload['size'] for payload in payloads),
             arguments=len(attempts), accepted=accepted, failed=len(attempts) - accepted, packet_bytes=len(packet), packet_sha256=digest(packet),
             complete_streams=sum(any(attempt['success'] for attempt in payload['attempts']) for payload in payloads),
             families={str(index): sum(payload['family'] == index for payload in payloads) for index in range(7)}))


def verify_solid_source_bindings() -> None:
    """Retain every original reader pin and the precise unused-boundary exception; e.g. verify_solid_source_bindings()."""
    for name in REPORTS:
        report = json.loads((RESEARCH / name).read_bytes())
        for original, expected in report['sources'].items():
            relative = original.replace('\\', '/').split('/inventor-interface/', 1)[1]
            verify_solid_dependency(relative, expected)
    verify_unimported_boundary()


def verify_solid_dependency(relative: str, expected: str) -> None:
    """Prove the sole Material source delta while retaining default exact EOF; e.g. verify_solid_dependency(relative, expected)."""
    if relative == 'solidworks/research/test_cosworks_material.py':
        verify_material_test_delta(expected)
        return
    if relative != 'solidworks/research/cosworks_material_layout.py':
        verify_source_hash(relative, expected)
        return
    raw = (SOURCE / relative).read_bytes()
    assert digest(raw) == 'd7e60364f3b4ea749c9e8b995ad881e27993341ebcf5303bf7d1cf3d29d873bc'
    assert expected == 'df9aa8356cb30dfa77b6c63fcd85ba3baf7b5f9570994afb1096ce9e57b1cbf2'
    old = raw.replace(b'def parse(data,v,*,require_eof=True):', b'def parse(data,v):')
    old = old.replace(b'if require_eof and pos!=len(data):', b'if pos!=len(data):')
    assert digest(old) == expected, 'Material reader; expected only the explicit optional prefix switch'
    baseline('material-source-reconciliation.json', dict(file=relative, original_sha256=expected,
             current_sha256=digest(raw), reconstructed_original_sha256=digest(old), require_eof=True,
             scope='Exactly two substitutions recover the original source byte for byte; no prefix mode in this inventory.'))


def verify_material_test_delta(expected: str) -> None:
    """Recover the original tests by removing only the added prefix test class; e.g. verify_material_test_delta(expected)."""
    raw = (RESEARCH / 'test_cosworks_material.py').read_bytes()
    assert digest(raw) == 'f77f36d0845aef1f1e474505b556c9dd7bebc404f6b96840eee8e20efea51b1d'
    assert expected == 'e283b59b3fdb9c77501158a7af71f9a13c93349b7b8bd43f4cfd0a0bbd2dd691'
    start, end = raw.index(b'class MaterialPrefixTests'), raw.index(b'def curve')
    old = raw[:start] + b'\n' + raw[end:]
    assert digest(old) == expected, 'Material tests; expected only the added explicit prefix test'
    baseline('material-test-reconciliation.json', dict(original_sha256=expected, current_sha256=digest(raw),
             reconstructed_original_sha256=digest(old), added_class=raw[start:end].decode()))


def verify_solid_dependency_rejections() -> None:
    """Keep the source reconciliation tied to exact file/hash pairs; e.g. verify_solid_dependency_rejections()."""
    for filename in ('cosworks_material_layout.py', 'test_cosworks_material.py', 'unreviewed.py'):
        try: verify_solid_dependency('solidworks/research/' + filename, 'unreviewed')
        except (AssertionError, FileNotFoundError): continue
        raise AssertionError(f'{filename}; expected unreviewed dependency rejection')


def verify_solid_schema() -> None:
    """Retain the complete reconciliation clause and unresolved-case boundaries; e.g. verify_solid_schema()."""
    text = (SOURCE / 'docs/solidworks-file-format/specification.md').read_bytes().decode().replace('\r\n', '\n')
    for section in ('14.361', '14.362'):
        start = text.index('### ' + section + ' ')
        raw = (text[start:text.index('\n### ', start + 1)].rstrip() + '\n').encode()
        path = FEATURE / f'schema-{section}.md'
        if '--write' in sys.argv: path.write_bytes(raw)
        assert path.read_bytes() == raw


def run_solid_checks(pin: Path) -> unittest.TestResult:
    """Run metadata rejection, full schema pins and fresh combined replay; e.g. run_solid_checks(pin)."""
    from functools import partial, update_wrapper
    import test_cosworks_material
    replay = update_wrapper(partial(replay_solid_inventory, pin), replay_solid_inventory)
    checks = (verify_solid_denominator, verify_solid_identity_failures, verify_solid_family_failures,
              verify_solid_source_bindings, verify_solid_dependency_rejections, verify_solid_schema, replay_owned_solid_inventory, replay)
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(test_cosworks_material))
    return unittest.TextTestRunner(verbosity=2).run(suite)


def run() -> bool:
    """Freeze every reader and carrier adapter before imports; e.g. run()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-solid-inventory-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        pinned_auxiliaries(pin, hashes)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin / 'research'))
        result = run_solid_checks(pin)
        verify_unimported_boundary()
        readers = ('cosworks_solid_manager_layout', 'cosworks_solid_component_layout', 'cosworks_solid_body_layout',
                   'cosworks_sheet_property_layout', 'cosworks_material_layout', 'cosworks_surface_body_layout',
                   'cosworks_component_persistid_layout', 'cosworks_embedded_body_ids', 'cosworks_embedded_const_surface', 'cfb', 'swcontainer')
        record_snapshot(pin, hashes, result, FEATURE, readers, ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
