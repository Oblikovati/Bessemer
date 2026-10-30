"""Pin complete approved readers and replay retained persistence carriers (#127)."""
from pathlib import Path
import hashlib
import json
import sys
import tempfile
import unittest
import zlib
from typing import TypedDict, cast


class NativeText(TypedDict):
    raw_hex: str


class NativeCarrier(TypedDict):
    raw_hex: str
    sha256: str
    name: str
    path: str
    first_text: NativeText
    items: list[NativeText]
    item_count: int
    version_text: str
    tail_hex: str


class NumericContext(TypedDict):
    path: str
    numeric_version: int


class RetainedInventory(TypedDict):
    rows: list[NativeCarrier]
    native_checks: list[NumericContext]
    parent_native_sha256: str

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
RESEARCH = ROOT.parent / 'inventor-interface/solidworks/research'
sys.path.insert(0, str(FEATURE.parent / 'model-archives'))
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot


def digest(raw: bytes) -> str:
    """Identify complete evidence bytes; e.g. digest(b'')."""
    return hashlib.sha256(raw).hexdigest()


def evidence(name: str) -> RetainedInventory:
    """Read a complete retained inventory; e.g. evidence('inventory.json')."""
    return cast(RetainedInventory, json.loads((RESEARCH / name).read_bytes()))


def payload(row: NativeCarrier) -> bytes:
    """Require the retained payload hash before parsing; e.g. payload(row)."""
    raw = bytes.fromhex(row['raw_hex'])
    assert digest(raw) == row['sha256']
    return raw


def baseline(name: str, value: object) -> None:
    """Require explicit baseline creation; e.g. baseline('proof.json', proof)."""
    path = FEATURE / name
    raw = (json.dumps(value, indent=2) + '\n').encode('utf-8')
    if '--write' in sys.argv:
        path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected unchanged reviewed baseline'


def primitive_inventory() -> None:
    """Reconstruct every native primitive and complete study-list payload."""
    import cosworkspersist as reader
    versions, studies, spans, extent = 0, 0, 0, 0
    for name in ('cosworks-small-stream-native-evidence.json', 'cosworks-root-study-order-native-evidence.json'):
        for row in evidence(name)['rows']:
            raw = payload(row)
            extent += len(raw)
            if row['name'].endswith('CosworksVersoin'):
                record = reader.read_text(raw)
                assert record.header + record.payload + raw[record.end:] == raw
                assert record.payload.hex() == row['first_text']['raw_hex']
                versions += 1
                spans += 1
                continue
            spans += verify_study(raw, row)
            studies += 1
    assert (versions, studies, spans, extent) == (262, 335, 1031, 25988)
    baseline('primitive-inventory.json', dict(versions=versions, studies=studies, text_spans=spans, bytes=extent))


def verify_study(raw: bytes, row: NativeCarrier) -> int:
    """Compare all stored item boundaries and bytes; e.g. verify_study(raw, row)."""
    import cosworkspersist as reader
    record = reader.read_study_order(raw)
    rebuilt = record.leading.header + record.leading.payload + record.count_header
    rebuilt += b''.join(item.header + item.payload for item in record.items)
    assert rebuilt == raw and record.end == len(raw)
    assert len(record.items) == row['item_count']
    assert [item.payload.hex() for item in record.items] == [item['raw_hex'] for item in row['items']]
    return 1 + len(record.items)


def primary_inventory() -> None:
    """Check both reader modes using supplied hash-bound numeric contexts."""
    import cosworkspersist as reader
    gates = evidence('cosworks-version-gates-native-evidence.json')
    small = RESEARCH / 'cosworks-small-stream-native-evidence.json'
    assert digest(small.read_bytes()) == gates['parent_native_sha256']
    versions = {row['path']: row['numeric_version'] for row in gates['native_checks']}
    rows = [row for row in evidence(small.name)['rows'] if row['name'] == 'Cosworks/CosworksVersoin']
    for row in rows:
        raw = payload(row)
        for full in (False, True):
            verify_primary(raw, versions[row['path']], full)
    assert len(rows) == 262
    baseline('primary-inventory.json', dict(carriers=len(rows), modes_per_carrier=2, numeric_conversion_supplied=True))


def verify_primary(raw: bytes, version: int, full: bool) -> None:
    """Compare independent word boundaries and assignments; e.g. verify_primary(raw, 830032, True)."""
    import cosworkspersist as reader
    record = reader.read_primary_version(raw, version, full_read=full)
    text_end = 4 + int.from_bytes(raw[:4], 'little')
    count = int(version > 800018) + int(full and version > 800076) + int(full and version >= 830032)
    words = tuple(int.from_bytes(raw[offset:offset + 4], 'little', signed=True) for offset in range(text_end, text_end + 4 * count, 4))
    updates = []
    if 800018 < version < 830032:
        updates.append((0x1c8, words[0]))
    if full and version > 800076:
        updates.append((0x1a8, words[1]))
    if full and version >= 830032:
        updates.append((0x1c8, words[2]))
    assert record.words == words and record.owner_updates == tuple(updates)
    assert record.end == text_end + 4 * count
    assert record.text.header + record.text.payload + record.word_payload + record.remainder == raw
    assert record.remainder == raw[record.end:] and (not full or record.remainder == b'')


def legacy_inventory() -> None:
    """Check every opaque legacy suffix and coexisting primary precedence."""
    import cosworkspersist as reader
    primary = {row['path']: row for row in evidence('cosworks-small-stream-native-evidence.json')['rows'] if row['name'] == 'Cosworks/CosworksVersoin'}
    versions = {row['path']: row['numeric_version'] for row in evidence('cosworks-version-gates-native-evidence.json')['native_checks']}
    rows, pairs = evidence('cosworks-legacy-version-native-evidence.json')['rows'], 0
    for row in rows:
        raw = payload(row)
        selected = reader.select_version_stream(None, raw)
        record = selected.record
        assert selected.source == 'legacy' and record.text == row['version_text']
        assert record.prefix + record.opaque_suffix == raw and record.opaque_suffix.hex() == row['tail_hex']
        if row['path'] in primary:
            chosen = reader.select_version_stream(payload(primary[row['path']]), raw, primary_numeric_version=versions[row['path']])
            assert chosen.source == 'primary' and chosen.numeric_version != record.numeric_version
            pairs += 1
    assert (len(rows), pairs) == (25, 21)
    baseline('legacy-inventory.json', dict(carriers=len(rows), primary_precedence_pairs=pairs, native_opens_executed=False))


def schema() -> None:
    """Retain complete accepted clauses rather than paraphrased byte grammars."""
    path = ROOT.parent / 'inventor-interface/docs/solidworks-file-format/specification.md'
    text = path.read_bytes().decode('utf-8').replace('\r\n', '\n')
    clauses = []
    for number in (262, 263, 329, 332):
        start = text.index(f'### 14.{number} ')
        end = text.index('\n### ', start + 1)
        clauses.append(text[start:end].rstrip() + '\n')
    raw = ('\n'.join(clauses)).encode('utf-8')
    destination = FEATURE / 'schema-snapshot.md'
    if '--write' in sys.argv:
        destination.write_bytes(raw)
    assert destination.read_bytes() == raw
    baseline('schema-reference.json', dict(clauses=[262, 263, 329, 332], bytes=len(raw), sha256=digest(raw)))


def source_inventory() -> None:
    """Bind every retained native evidence inventory to its complete bytes."""
    names = ('cosworks-small-stream-native-evidence.json', 'cosworks-root-study-order-native-evidence.json',
             'cosworks-version-gates-native-evidence.json', 'cosworks-legacy-version-native-evidence.json')
    hashes = {name: digest((RESEARCH / name).read_bytes()) for name in names}
    baseline('retained-source-reference.json', hashes)


def native_cases() -> list[tuple[int, int, int, int, bytes, bytes]]:
    """Retain explicit established version contexts for every hash-bound native carrier."""
    from owned_persistence import study_edit_cases
    versions = {row['path']: row['numeric_version'] for row in evidence('cosworks-version-gates-native-evidence.json')['native_checks']}
    cases = []
    for name in ('cosworks-small-stream-native-evidence.json', 'cosworks-root-study-order-native-evidence.json'):
        for row in evidence(name)['rows']:
            raw = payload(row)
            if row['name'].endswith('CosworksVersoin'):
                cases.append((0, 0, 0, 0, raw, b''))
                cases += [(2, versions[row['path']], full, 0, raw, b'') for full in (0, 1)]
                continue
            cases.append((1, 0, 0, 0, raw, b''))
            cases += study_edit_cases(raw)
    primary = {row['path']: row for row in evidence('cosworks-small-stream-native-evidence.json')['rows'] if row['name'] == 'Cosworks/CosworksVersoin'}
    for row in evidence('cosworks-legacy-version-native-evidence.json')['rows']:
        raw = payload(row)
        cases.append((3, 0, 0, 0, raw, b''))
        if row['path'] in primary:
            cases.append((4, versions[row['path']], 1, 0, payload(primary[row['path']]), raw))
    return cases


def corpus() -> None:
    """Freeze owned goldens and native verification inputs without shipping native payloads."""
    from owned_persistence import owned_cases, span
    from persistence_projection import packet
    owned_inputs, native_inputs = owned_cases(), native_cases()
    owned, native = packet(owned_inputs), packet(native_inputs)
    fixture = ROOT / 'tests/fixtures/cosworks-persistence/owned.bin'
    if '--write' in sys.argv:
        fixture.parent.mkdir(parents=True, exist_ok=True)
        fixture.write_bytes(zlib.compress(owned, 9))
    assert zlib.decompress(fixture.read_bytes()) == owned
    (ROOT / 'target/cosworks-persistence-native.bin').write_bytes(native)
    baseline('corpus-reference.json', dict(owned_cases=len(owned_inputs), native_cases=len(native_inputs), owned_packet_bytes=len(owned), owned_packet_sha256=digest(owned), native_packet_bytes=len(native), native_packet_sha256=digest(native), native_payloads_committed=False))
    if '--verify-rust' in sys.argv:
        actual = (ROOT / 'target/cosworks-persistence-projections.bin').read_bytes()
        expected = span(owned) + span(native)
        assert actual == expected, 'Rust complete projection; expected every owned/native byte unchanged'
        baseline('rust-projection-reference.json', dict(bytes=len(actual), sha256=digest(actual)))


def run() -> bool:
    """Test the actual immutable reader copy, e.g. run()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-cosworks-persistence-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        checks = (primitive_inventory, primary_inventory, legacy_inventory, schema, source_inventory, corpus)
        suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
        result = unittest.TextTestRunner(verbosity=2).run(suite)
        record_snapshot(pin, hashes, result, FEATURE, ('cosworkspersist',), ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
