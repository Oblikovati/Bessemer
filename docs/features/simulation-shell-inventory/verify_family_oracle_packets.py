"""Bind complete native family oracles to every accepted C-746 argument (#160)."""
from collections import Counter
from pathlib import Path
from typing import Literal, NamedTuple, cast
import hashlib
import json
import struct
import sys
import tempfile
import unittest
from shell_inventory_bindings import FEATURE, shell_inventory_payloads
from freeze_inventory_bindings import inventory_baseline

ROOT = FEATURE.parents[2]
PacketShape = Literal['simple', 'property', 'composite', 'material']
ArgumentKey = tuple[int, int, int, int, str]
SPECS: tuple[tuple[int, PacketShape, str, str], ...] = (
    (0, 'simple', 'simulation-shell-header-native-input.bin', 'simulation-shell-manager/header-native-packets.json'),
    (1, 'simple', 'simulation-shell-group-native-input.bin', 'simulation-shell-manager/group-native-packets.json'),
    (2, 'property', 'simulation-shell-property-native.bin', 'simulation-shell-property/native-reference.json'),
    (4, 'material', 'simulation-material-native.bin', 'simulation-material/native-reference.json'),
    (5, 'composite', 'simulation-shell-composite-native.bin', 'simulation-shell-composite/native-reference.json'),
)


class NativeArgumentFrame(NamedTuple):
    family: int
    version: int
    profile: int
    accepted: int
    size: int
    body_offset: int


class FakeOraclePacketStore:
    def __init__(self, root: Path) -> None:
        self.root = root

    def packet(self, raw: bytes) -> Path:
        """Create an isolated packet for metadata rejection controls; e.g. store.packet(b'A')."""
        path = self.root / 'oracle.bin'
        path.write_bytes(raw)
        return path


def packet_words(packet: bytes, offset: int, count: int) -> tuple[int, ...]:
    """Reject partial structural extents before unpacking; e.g. packet_words(b'\0' * 4, 0, 1)."""
    end = offset + count * 4
    assert 0 <= offset <= end <= len(packet), f'{offset}:{end}; expected extent within {len(packet)} bytes'
    return cast(tuple[int, ...], struct.unpack_from(f'<{count}I', packet, offset))


def native_argument_frame(packet: bytes, offset: int, family: int, shape: PacketShape) -> NativeArgumentFrame:
    """Normalize each persisted packet grammar once; e.g. native_argument_frame(packet, 0, 0, 'simple')."""
    if shape == 'simple':
        version, accepted, size = packet_words(packet, offset, 3)
        return NativeArgumentFrame(family, version, 0, accepted, size, offset + 12)
    if shape == 'property':
        kind, version, accepted, size = packet_words(packet, offset, 4)
        assert kind <= 1, f'{kind}; expected Shells 0 or MidSurfShells 1'
        return NativeArgumentFrame(kind + 2, version, 0, accepted, size, offset + 16)
    if shape == 'composite':
        kind, profile, version, accepted, size = packet_words(packet, offset, 5)
        assert kind <= 1 and profile <= 1, f'{kind}/{profile}; expected kind and profile in 0..1'
        return NativeArgumentFrame(kind + 5, version, profile, accepted, size, offset + 20)
    assert shape == 'material', f'{shape}; expected an explicit native packet grammar'
    version, accepted, size, end = packet_words(packet, offset, 4)
    assert end <= size, f'{end}; expected consumed prefix within {size} body bytes'
    return NativeArgumentFrame(family, version, 0, accepted, size, offset + 16)


def accepted_native_arguments(packet: bytes, family: int, shape: PacketShape) -> Counter[ArgumentKey]:
    """Retain duplicate occurrences and complete byte/context identities; e.g. accepted_native_arguments(packet, 0, 'simple')."""
    counts: Counter[ArgumentKey] = Counter()
    offset = 0
    while offset < len(packet):
        frame = native_argument_frame(packet, offset, family, shape)
        end = frame.body_offset + frame.size
        assert end <= len(packet), f'{end}; expected body extent within {len(packet)} bytes'
        assert frame.accepted <= 1, f'{frame.accepted}; expected acceptance Boolean 0 or 1'
        raw = packet[frame.body_offset:end]
        offset = end
        if shape == 'material':
            count, = packet_words(packet, offset, 1)
            offset += 4 + count * 8
            assert offset <= len(packet), f'{offset}; expected complete Material span table within {len(packet)} bytes'
        if frame.accepted:
            counts[(frame.family, frame.version, frame.profile, frame.size, hashlib.sha256(raw).hexdigest())] += 1
    return counts


def accepted_inventory_arguments() -> Counter[ArgumentKey]:
    """Bind the original accepted multiset without collapsing equal payloads; e.g. accepted_inventory_arguments()."""
    counts: Counter[ArgumentKey] = Counter()
    for payload in shell_inventory_payloads():
        for attempt in payload['attempts']:
            if not attempt['success']: continue
            key = (payload['family'], cast(int, attempt['version']), int(attempt.get('wire_profile', 'reader') == 'writer'),
                   payload['size'], payload['sha256'])
            counts[key] += 1
    return counts


def require_argument_coverage(expected: Counter[ArgumentKey], actual: Counter[ArgumentKey], allow_extra: bool) -> None:
    """Require every physical occurrence, not just matching totals; e.g. require_argument_coverage(expected, actual, False)."""
    missing = expected - actual
    assert not missing, f'{missing}; expected complete accepted byte/context multiset'
    assert allow_extra or actual == expected, f'{actual - expected}; expected no unassigned extra arguments'


def checked_packet(path: Path, reference: dict[str, object]) -> bytes:
    """Require the reviewed packet length and digest before interpreting it; e.g. checked_packet(path, reference)."""
    raw = path.read_bytes()
    assert len(raw) == reference['bytes'], f'{path}: {len(raw)}; expected {reference["bytes"]} bytes'
    digest = hashlib.sha256(raw).hexdigest()
    assert digest == reference['sha256'], f'{path}: {digest}; expected hash {reference["sha256"]}'
    return raw


def oracle_input_reference(reference: dict[str, object], family: int) -> dict[str, object]:
    """Normalize retained report metadata without replacing a baseline; e.g. oracle_input_reference(reference, 0)."""
    if family < 2:
        return cast(dict[str, dict[str, object]], reference['packets'])['input']
    return dict(bytes=reference['packet_bytes'], sha256=reference['packet_sha256'])


def checked_projection_and_history(family: int, folder: Path, prefix: str) -> dict[str, object]:
    """Bind both operation oracles to their reviewed family packet references; e.g. checked_projection_and_history(0, folder, prefix)."""
    if family < 2:
        filename = 'header-native-packets.json' if family == 0 else 'group-native-packets.json'
        references = json.loads((folder / filename).read_bytes())['packets']
    elif family == 4:
        references = {name: json.loads((folder / f'native-{name}-reference.json').read_bytes()) for name in ('projection', 'history')}
    else:
        filename = 'native-packets-reference.json' if family == 2 else 'native-oracles-reference.json'
        references = json.loads((folder / filename).read_bytes())['packets']
    for name in ('projection', 'history'):
        checked_packet(ROOT / f'target/{prefix}-{name}.bin', references[name])
    return {name: references[name] for name in ('projection', 'history')}


def verify_family_oracle_coverage() -> None:
    """Prove all 962 accepted original arguments are covered by complete family oracles; e.g. verify_family_oracle_coverage()."""
    expected = accepted_inventory_arguments()
    summaries: list[dict[str, object]] = []
    for family, shape, filename, report_path in SPECS:
        path = ROOT / 'target' / filename
        reference = json.loads((FEATURE.parent / report_path).read_bytes())
        raw = checked_packet(path, oracle_input_reference(reference, family))
        actual = accepted_native_arguments(raw, family, shape)
        families = (2, 3) if family == 2 else (5, 6) if family == 5 else (family,)
        selected = Counter({key: count for key, count in expected.items() if key[0] in families})
        require_argument_coverage(selected, actual, family == 4)
        prefix = filename.removesuffix('-input.bin').removesuffix('.bin')
        oracles = checked_projection_and_history(family, (FEATURE.parent / report_path).parent, prefix)
        summaries.append(dict(families=families, required_arguments=sum(selected.values()),
                              supplied_arguments=sum(actual.values()), packet=oracle_input_reference(reference, family), oracles=oracles))
    assert sum(row['required_arguments'] for row in summaries) == 962
    inventory_baseline('family-oracle-coverage-reference.json', summaries)


class FamilyOracleCoverageTests(unittest.TestCase):
    def test_complete_original_multiset_has_all_seven_families(self) -> None:
        counts = accepted_inventory_arguments()
        self.assertEqual(sum(counts.values()), 962)
        self.assertEqual([sum(count for key, count in counts.items() if key[0] == family) for family in range(7)],
                         [365, 79, 236, 15, 237, 22, 8])

    def test_simple_property_composite_and_material_frames(self) -> None:
        controls: tuple[tuple[int, PacketShape, bytes, ArgumentKey], ...] = (
            (0, 'simple', struct.pack('<III', 840002, 1, 1) + b'A', (0, 840002, 0, 1, hashlib.sha256(b'A').hexdigest())),
            (2, 'property', struct.pack('<IIII', 1, 840002, 1, 0), (3, 840002, 0, 0, hashlib.sha256(b'').hexdigest())),
            (5, 'composite', struct.pack('<IIIII', 1, 1, 840002, 1, 0), (6, 840002, 1, 0, hashlib.sha256(b'').hexdigest())),
            (4, 'material', struct.pack('<5I', 840002, 1, 0, 0, 0), (4, 840002, 0, 0, hashlib.sha256(b'').hexdigest())),
        )
        for family, shape, packet, key in controls:
            self.assertEqual(accepted_native_arguments(packet, family, shape), Counter({key: 1}))

    def test_missing_duplicate_and_substituted_contexts_fail(self) -> None:
        key = (0, 840002, 0, 1, 'original')
        other = (0, 840003, 0, 1, 'original')
        expected = Counter({key: 2})
        for actual in (Counter({key: 1}), Counter({key: 3}), Counter({key: 1, other: 1})):
            with self.assertRaises(AssertionError): require_argument_coverage(expected, actual, False)
        require_argument_coverage(expected, Counter({key: 2, other: 1}), True)
        with self.assertRaises(AssertionError): require_argument_coverage(expected, Counter({other: 2}), True)

    def test_partial_headers_bodies_span_tables_and_unknown_profiles_fail(self) -> None:
        controls: tuple[tuple[int, PacketShape, bytes], ...] = (
            (0, 'simple', b'\0' * 11), (0, 'simple', struct.pack('<III', 1, 1, 2) + b'A'),
            (0, 'simple', struct.pack('<III', 1, 2, 0)), (2, 'property', struct.pack('<IIII', 2, 1, 1, 0)),
            (5, 'composite', struct.pack('<IIIII', 0, 2, 1, 1, 0)),
            (4, 'material', struct.pack('<IIIII', 1, 1, 0, 0, 1)),
            (4, 'material', struct.pack('<IIII', 1, 1, 0, 1)),
        )
        for family, shape, packet in controls:
            with self.assertRaises(AssertionError): accepted_native_arguments(packet, family, shape)

    def test_rejected_arguments_are_not_promoted_to_operation_oracles(self) -> None:
        packet = struct.pack('<III', 840002, 0, 0)
        self.assertEqual(accepted_native_arguments(packet, 0, 'simple'), Counter())

    def test_packet_length_and_hash_both_bind_before_interpretation(self) -> None:
        with tempfile.TemporaryDirectory(prefix='bessemer-oracle-packet-') as temporary:
            path = FakeOraclePacketStore(Path(temporary)).packet(b'A')
            reference = dict(bytes=1, sha256=hashlib.sha256(b'A').hexdigest())
            self.assertEqual(checked_packet(path, reference), b'A')
            for altered in (dict(reference, bytes=0), dict(reference, sha256='0' * 64)):
                with self.assertRaises(AssertionError): checked_packet(path, altered)


if __name__ == '__main__':
    suite = unittest.defaultTestLoader.loadTestsFromTestCase(FamilyOracleCoverageTests)
    suite.addTest(unittest.FunctionTestCase(verify_family_oracle_coverage))
    raise SystemExit(0 if unittest.TextTestRunner(verbosity=2).run(suite).wasSuccessful() else 1)
