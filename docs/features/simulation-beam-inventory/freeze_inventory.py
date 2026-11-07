"""Fresh combined BeamManager replay with all inventory identities and arguments (#146)."""
from pathlib import Path
from typing import cast
import json
import struct
import sys
import tempfile
import unittest

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
for folder in ('simulation-binary-thermal', 'model-archives', 'simulation-manager-property',
               'simulation-material', 'simulation-beam-property', 'simulation-beam-groups'):
    sys.path.insert(0, str(FEATURE.parent / folder))
sys.path.insert(0, str(ROOT / 'target/reference-python'))
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot
from native_carriers import SOURCE, digest, recover_sample, pinned_auxiliaries
from inventory_bindings import BoundPayload, reconcile, retained_inventory
from inventory_replay import verify_attempt
from owned_inventory import owned_inventory_cases


def baseline(name: str, value: object) -> None:
    """Only initialize complete evidence on explicit request; e.g. baseline('fresh-replay-reference.json', report)."""
    path = FEATURE / name
    raw = (json.dumps(value, indent=2) + '\n').encode()
    if '--write' in sys.argv: path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected unchanged complete evidence'


def text_frame(value: str) -> bytes:
    """Bind the full original path without path normalization or identity loss; e.g. text_frame('Cosworks/StudyManager')."""
    raw = value.encode()
    return struct.pack('<I', len(raw)) + raw


def payload_case(payload: BoundPayload, raw: bytes) -> bytes:
    """Verify each original argument before exporting the complete body once; e.g. payload_case(payload, raw)."""
    assert len(raw) == payload['size'] and digest(raw) == payload['sha256']
    packet = bytearray(struct.pack('<I', payload['family']))
    packet.extend(text_frame(payload['document']) + text_frame(payload['stream']))
    packet.extend(struct.pack('<I', len(raw)) + raw)
    packet.extend(struct.pack('<I', len(payload['attempts'])))
    for expected in payload['attempts']:
        verify_attempt(payload, raw, expected)
        version = cast(int, expected['version'])
        assert type(version) is int and 0 <= version <= 0x7fffffff
        count = payload['expected_joint_count']
        packet.extend(struct.pack('<IIi', version, int(cast(bool, expected['success'])), -1 if count is None else count))
    return bytes(packet)


def replay_combined(pin: Path) -> None:
    """Freshly replay all six families in one frozen-source process; e.g. replay_combined(pin)."""
    payloads = reconcile()
    packet = bytearray()
    for document in sorted({payload['document'] for payload in payloads}):
        streams = recover_sample(document, pin)
        for payload in (payload for payload in payloads if payload['document'] == document):
            packet.extend(payload_case(payload, streams[payload['stream']]))
    attempts = [attempt for payload in payloads for attempt in payload['attempts']]
    accepted = sum(cast(bool, attempt['success']) for attempt in attempts)
    assert (len(payloads), len(attempts), accepted) == (1080, 1114, 1084)
    (ROOT / 'target/simulation-beam-inventory-native.bin').write_bytes(packet)
    baseline('fresh-replay-reference.json', dict(streams=1080, logical_bytes=620654, arguments=1114,
             accepted=1084, failed=30, packet_bytes=len(packet), packet_sha256=digest(packet),
             families={str(index): sum(payload['family'] == index for payload in payloads) for index in range(6)}))


def verify_identity_reconciliation() -> None:
    """Require exact one-to-one C-590/C-727 bindings before payload acceptance; e.g. verify_identity_reconciliation()."""
    baseline('initial-reference.json', retained_inventory())
    payloads = reconcile()
    assert len(payloads) == 1080 and len({(row['document'], row['stream']) for row in payloads}) == 1080
    assert sum(payload['size'] for payload in payloads) == 620654
    assert [sum(payload['family'] == index for payload in payloads) for index in range(6)] == [353, 163, 150, 36, 11, 367]
    save_identity_manifest(payloads)


def save_identity_manifest(payloads: list[BoundPayload]) -> None:
    """Commit metadata identities without native payload bytes; e.g. save_identity_manifest(payloads)."""
    packet = b''.join(struct.pack('<I', payload['family']) + text_frame(payload['document'])
                      + text_frame(payload['stream']) + struct.pack('<I', payload['size']) for payload in payloads)
    path = FEATURE / 'native-identities.bin'
    if '--write' in sys.argv: path.write_bytes(packet)
    assert path.read_bytes() == packet
    baseline('identity-manifest-reference.json', dict(identities=len(payloads), bytes=len(packet), sha256=digest(packet), native_payload_bytes=0))


def verify_schema() -> None:
    """Keep the entire reconciliation clause and its stated limits; e.g. verify_schema()."""
    text = (SOURCE / 'docs/solidworks-file-format/specification.md').read_bytes().decode().replace('\r\n', '\n')
    start = text.index('### 14.351 ')
    raw = (text[start:text.index('\n### ', start + 1)].rstrip() + '\n').encode()
    path = FEATURE / 'schema-14.351.md'
    if '--write' in sys.argv: path.write_bytes(raw)
    assert path.read_bytes() == raw


def replay_owned() -> None:
    """Give platform gates a complete independent six-family corpus; e.g. replay_owned()."""
    cases = owned_inventory_cases()
    packet = b''.join(payload_case(payload, raw) for payload, raw in cases)
    assert len(cases) == 6 and sum(len(payload['attempts']) for payload, _ in cases) == 8
    path = ROOT / 'tests/fixtures/simulation-beam-inventory/owned.bin'
    if '--write' in sys.argv:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(packet)
    assert path.read_bytes() == packet
    baseline('owned-reference.json', dict(streams=6, arguments=8, accepted=7, failed=1,
             logical_bytes=sum(len(raw) for _, raw in cases), packet_bytes=len(packet), packet_sha256=digest(packet)))


def run_checks(pin: Path) -> unittest.TestResult:
    """Combine identity/schema checks and all fresh parser outcomes; e.g. run_checks(pin)."""
    from functools import partial, update_wrapper
    replay = update_wrapper(partial(replay_combined, pin), replay_combined)
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in (verify_identity_reconciliation, verify_schema, replay_owned, replay))
    return unittest.TextTestRunner(verbosity=2).run(suite)


def run() -> bool:
    """Keep every actual reader and container adapter on an immutable copy; e.g. run()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-beam-inventory-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        pinned_auxiliaries(pin, hashes)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin / 'research'))
        result = run_checks(pin)
        readers = ('cosworkspersist', 'cosworks_material_layout', 'cosworks_beam_property_layout',
                   'cosworks_beam_group_layout', 'cosworks_binary_thermal_transform_audit', 'cfb', 'swcontainer')
        record_snapshot(pin, hashes, result, FEATURE, readers, ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
