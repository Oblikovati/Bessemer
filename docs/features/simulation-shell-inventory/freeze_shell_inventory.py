"""Fresh combined replay of the retained C-746 denominator without target execution (#160)."""
from pathlib import Path
from typing import cast
import json
import struct
import sys
import tempfile
import unittest
from functools import partial, update_wrapper

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
for folder in ('simulation-binary-thermal', 'model-archives', 'simulation-solid-manager', 'simulation-solid-body',
               'simulation-solid-inventory', 'simulation-shell-property', 'simulation-shell-manager',
               'simulation-material', 'simulation-component-persist-id', 'simulation-sheet-property', 'simulation-shell-composite'):
    sys.path.insert(0, str(FEATURE.parent / folder))
sys.path.insert(0, str(ROOT / 'target/reference-python'))
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot
from native_carriers import SOURCE, RESEARCH, digest, recover_sample, pinned_auxiliaries
from freeze_body import verify_unimported_boundary
from freeze_solid_inventory import verify_solid_dependency
from shell_property_evidence import REFINEMENTS, require_shell_refinement
from shell_inventory_bindings import REPORTS, ShellInventoryPayload, complete_identity_match, shell_inventory_payloads, shell_inventory_identity
from freeze_inventory_bindings import ShellInventoryBindingTests, inventory_baseline
from shell_inventory_replay import actual_shell_inventory
from owned_shell_inventory import OwnedInventoryCase, owned_shell_inventory


def verify_inventory_dependencies() -> None:
    """Reuse reviewed exact source deltas and reject all other changes; e.g. verify_inventory_dependencies()."""
    for name in REPORTS:
        report = json.loads((RESEARCH / name).read_bytes())
        for original, expected in report['sources'].items():
            relative = original.replace('\\', '/').split('/inventor-interface/', 1)[1]
            current = digest((SOURCE / relative).read_bytes())
            if current == expected: continue
            if relative in REFINEMENTS and current != expected:
                require_shell_refinement(relative, expected, current)
                continue
            verify_solid_dependency(relative, expected)
    verify_unimported_boundary()


def verify_inventory_clause() -> None:
    """Retain the complete reconciliation and its exclusions; e.g. verify_inventory_clause()."""
    text = (SOURCE / 'docs/solidworks-file-format/specification.md').read_bytes().decode().replace('\r\n', '\n')
    start = text.index('### 14.367 ')
    raw = (text[start:text.index('\n### ', start + 1)].rstrip() + '\n').encode()
    path = FEATURE / 'schema-14.367.md'
    if '--write' in sys.argv: path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected complete retained clause'


def inventory_text_frame(value: str) -> bytes:
    """Preserve exact storage spelling in the typed-reader packet; e.g. inventory_text_frame('Shells')."""
    raw = value.encode()
    return struct.pack('<I', len(raw)) + raw


def verify_inventory_manifest() -> None:
    """Bind Rust replay to exact complete identities before decoding; e.g. verify_inventory_manifest()."""
    packet = b''.join(struct.pack('<I', payload['family']) + inventory_text_frame(payload['document'])
                      + inventory_text_frame(payload['stream']) + struct.pack('<I', payload['size'])
                      for payload in shell_inventory_payloads())
    path = FEATURE / 'native-identities.bin'
    if '--write' in sys.argv: path.write_bytes(packet)
    assert path.read_bytes() == packet, f'{path}; expected complete original identity manifest'


def shell_inventory_case_packet(payload: ShellInventoryPayload, raw: bytes) -> bytes:
    """Compare every original complete outcome before exporting its argument; e.g. shell_inventory_case_packet(payload, raw)."""
    assert len(raw) == payload['size'] and digest(raw) == payload['sha256'], payload['stream']
    packet = bytearray(struct.pack('<I', payload['family']))
    packet.extend(inventory_text_frame(payload['document']) + inventory_text_frame(payload['stream']))
    packet.extend(struct.pack('<I', len(raw)) + raw + struct.pack('<I', len(payload['attempts'])))
    for expected in payload['attempts']:
        assert actual_shell_inventory(payload, raw, expected) == expected, (payload['stream'], expected['version'])
        profile = expected.get('wire_profile', 'reader')
        assert profile in ('reader', 'writer'), f'{profile}; expected an explicit original profile'
        packet.extend(struct.pack('<III', cast(int, expected['version']), int(profile == 'writer'), int(cast(bool, expected['success']))))
    return bytes(packet)


def typed_inventory_reference_packet(payload: ShellInventoryPayload, raw: bytes) -> bytes:
    """Frame independent expected complete outputs for each original argument; e.g. typed_inventory_reference_packet(payload, raw)."""
    packet = bytearray()
    for attempt in payload['attempts']:
        accepted = cast(bool, attempt['success'])
        profile = attempt.get('wire_profile', 'reader')
        packet.extend(struct.pack('<IIII', payload['family'], cast(int, attempt['version']), int(profile == 'writer'), int(accepted)))
        packet.extend(inventory_text_frame(payload['document']) + inventory_text_frame(payload['stream']))
        result = raw if accepted else b''
        packet.extend(struct.pack('<I', len(result)) + result)
    return bytes(packet)


def recover_shell_inventory_document(document: str, payloads: list[ShellInventoryPayload], pin: Path) -> tuple[bytes, bytes]:
    """Check every freshly recovered sibling before dispatching any one codec; e.g. recover_shell_inventory_document(document, payloads, pin)."""
    streams = recover_sample(document, pin)
    actual = [dict(path=document, stream=name, bytes=len(raw), sha256=digest(raw))
              for name, raw in streams.items() if '/ShellManager/' in name]
    complete_identity_match([shell_inventory_identity(payload) for payload in payloads], actual)
    native = b''.join(shell_inventory_case_packet(payload, streams[payload['stream']]) for payload in payloads)
    typed = b''.join(typed_inventory_reference_packet(payload, streams[payload['stream']]) for payload in payloads)
    return native, typed


def replay_shell_inventory(pin: Path) -> None:
    """Replay every identity together while retaining all 46 rejected arguments; e.g. replay_shell_inventory(pin)."""
    payloads = shell_inventory_payloads()
    packet = bytearray()
    typed = bytearray()
    for document in sorted({payload['document'] for payload in payloads}):
        selected = [payload for payload in payloads if payload['document'] == document]
        native_fragment, typed_fragment = recover_shell_inventory_document(document, selected, pin)
        packet.extend(native_fragment)
        typed.extend(typed_fragment)
    (ROOT / 'target/simulation-shell-inventory-native.bin').write_bytes(packet)
    (ROOT / 'target/simulation-shell-inventory-typed-reference.bin').write_bytes(typed)
    inventory_baseline('fresh-replay-reference.json', dict(streams=len(payloads), packet_bytes=len(packet),
                       packet_sha256=digest(packet), complete_streams=935, incomplete_streams=1,
                       original_arguments=1008, accepted=962, rejected=46, target_executed=False))
    inventory_baseline('typed-output-reference.json', dict(arguments=1008, packet_bytes=len(typed), packet_sha256=digest(typed)))


def replay_owned_shell_inventory() -> None:
    """Export independently built bodies with independently declared acceptance; e.g. replay_owned_shell_inventory()."""
    cases = owned_shell_inventory()
    packet = b''.join(owned_inventory_packet(index, case) for index, case in enumerate(cases))
    path = ROOT / 'tests/fixtures/simulation-shell-inventory/owned.bin'
    if '--write' in sys.argv:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(packet)
    assert path.read_bytes() == packet, f'{path}; expected complete owned packet'
    outcomes = [success for case in cases for _, success in case['outcomes']]
    inventory_baseline('owned-reference.json', dict(streams=len(cases), logical_bytes=sum(len(case['raw']) for case in cases),
                       arguments=len(outcomes), accepted=sum(outcomes), rejected=len(outcomes) - sum(outcomes),
                       incomplete=sum(not any(success for _, success in case['outcomes']) for case in cases),
                       packet_bytes=len(packet), packet_sha256=digest(packet)))


def owned_inventory_packet(index: int, case: OwnedInventoryCase) -> bytes:
    """Bind each independent acceptance control to its full actual outcome; e.g. owned_inventory_packet(0, case)."""
    payload = ShellInventoryPayload(document='owned-shell-inventory', stream=f'family-{case["family"]}/case-{index}',
                                    size=len(case['raw']), sha256=digest(case['raw']), family=case['family'], attempts=[])
    for profile, success in case['outcomes']:
        argument = dict(version=case['version'], wire_profile=profile)
        actual = actual_shell_inventory(payload, case['raw'], argument)
        assert actual['success'] == success, (index, profile, actual)
        payload['attempts'].append(actual)
    return shell_inventory_case_packet(payload, case['raw'])


def run_inventory_checks(pin: Path) -> unittest.TestResult:
    """Run original identity rejection and fresh combined reference acceptance; e.g. run_inventory_checks(pin)."""
    replay = update_wrapper(partial(replay_shell_inventory, pin), replay_shell_inventory)
    checks = (verify_inventory_dependencies, verify_inventory_clause, verify_inventory_manifest, replay_owned_shell_inventory, replay)
    suite = unittest.defaultTestLoader.loadTestsFromTestCase(ShellInventoryBindingTests)
    suite.addTests(unittest.FunctionTestCase(check) for check in checks)
    return unittest.TextTestRunner(verbosity=2).run(suite)


def run() -> bool:
    """Freeze every parser and carrier before imports, then record their origins; e.g. run()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-shell-inventory-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        pinned_auxiliaries(pin, hashes)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin / 'research'))
        result = run_inventory_checks(pin)
        verify_unimported_boundary()
        readers = ('cosworks_shell_manager_layout', 'cosworks_shell_property_layout', 'cosworks_material_layout',
                   'cosworks_composite_layout', 'cosworks_embedded_const_surface', 'cosworks_embedded_body_ids', 'cfb', 'swcontainer')
        record_snapshot(pin, hashes, result, FEATURE, readers, ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
