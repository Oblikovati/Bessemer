"""Freeze both approved scalar-array readers and current retained inventories (#129)."""
from pathlib import Path
import hashlib
import json
import struct
import sys
import tempfile
import unittest
import zlib
from typing import TypedDict, cast
from scalar_cases import ScalarCase, edits

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
RESEARCH = ROOT.parent / 'inventor-interface/solidworks/research'
sys.path.insert(0, str(FEATURE.parent / 'model-archives'))
sys.path.insert(0, str(FEATURE.parent / 'cosworks-persistence'))
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot


class StudyResult(TypedDict):
    versions: list[int]
    count: int
    raw_hex: str
    sha256: str


class StudyCarrier(TypedDict):
    result: StudyResult
    sha256: str
    bytes: int


class UniqueCarrier(TypedDict):
    raw_hex: str
    sha256: str
    used: int
    allocation_count: int
    words_u32: list[int]
    adjacent_word_pairs: list[list[int]]


def digest(raw: bytes) -> str:
    """Bind the complete retained bytes, e.g. digest(b'')."""
    return hashlib.sha256(raw).hexdigest()


def baseline(name: str, value: object) -> None:
    """Require explicit reviewed baseline creation or replacement."""
    path = FEATURE / name
    raw = (json.dumps(value, indent=2) + '\n').encode('utf-8')
    if '--write' in sys.argv:
        path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected unchanged reviewed baseline'


def studies() -> list[StudyCarrier]:
    """Require current C-824 nested result framing, without a legacy-shape fallback."""
    inventory = json.loads((RESEARCH / 'cosworks-study-versions-native-evidence.json').read_bytes())
    assert inventory['evidence'] == 'C-824'
    return cast(list[StudyCarrier], inventory['rows'])


def unique_tables() -> list[UniqueCarrier]:
    """Bind C-648 to its retained independent C-652 decoder checkpoint."""
    path = RESEARCH / 'cosworks-unique-ids-native-evidence.json'
    checkpoint = json.loads((RESEARCH / 'cosworks-unique-decoder-validation.json').read_bytes())
    assert digest(path.read_bytes()) == checkpoint['source_sha256']
    inventory = json.loads(path.read_bytes())
    assert inventory['evidence'] == 'C-648'
    return cast(list[UniqueCarrier], inventory['rows'])


def check_integer(raw: bytes, expected: tuple[int, ...], maximum: int) -> None:
    """Compare both actual unchanged readers under the same explicit limit."""
    from cosworkspersist import read_integer_array
    import cosworks_study_versions as newer
    record = read_integer_array(raw, max_items=maximum)
    other = newer.parse(raw, max_items=maximum)
    assert other['success'] and other['complete'] and other['end'] == len(raw)
    assert record.values == expected and other['versions'] == list(expected)
    assert record.count_header + record.payload == raw and record.end == len(raw)
    assert other['raw_hex'] == raw.hex() and other['sha256'] == digest(raw)


def native_integers() -> None:
    """Replay all current hash-bound version arrays through both complete readers."""
    rows, count, extent = studies(), 0, 0
    for row in rows:
        result = row['result']
        raw = bytes.fromhex(result['raw_hex'])
        assert digest(raw) == result['sha256'] == row['sha256']
        assert len(raw) == row['bytes']
        check_integer(raw, tuple(result['versions']), 10000)
        assert result['count'] == len(result['versions'])
        count += result['count']
        extent += len(raw)
    assert (len(rows), count, extent) == (51, 82, 532)
    baseline('integer-native-reference.json', dict(carriers=len(rows), values=count, bytes=extent, actual_readers=2, explicit_limit=10000))


def native_unique() -> None:
    """Check every native key against retained adjacent words with explicit signed interpretation."""
    from cosworkspersist import read_unique_ids
    rows, count, extent = unique_tables(), 0, 0
    for row in rows:
        raw = bytes.fromhex(row['raw_hex'])
        assert digest(raw) == row['sha256']
        record = read_unique_ids(raw, max_entries=10000)
        assert (record.used_count, record.allocation_count) == (row['used'], row['allocation_count'])
        expected = tuple(signed_pair(pair) for pair in row['adjacent_word_pairs'])
        assert record.keys == expected
        assert tuple(struct.unpack('<' + 'I' * (len(raw[8:]) // 4), raw[8:])) == tuple(row['words_u32'])
        assert record.header + record.payload == raw and record.end == len(raw)
        count += len(record.keys)
        extent += len(raw)
    assert (len(rows), count, extent) == (51, 443, 3952)
    baseline('unique-native-reference.json', dict(carriers=len(rows), keys=count, bytes=extent, capacity_used_for_allocation=False))


def signed_pair(pair: list[int]) -> int:
    """Interpret all 64 bits without assuming a zero or positive high word."""
    assert len(pair) == 2
    bits = (pair[0] & 0xffffffff) | ((pair[1] & 0xffffffff) << 32)
    return bits if bits < 1 << 63 else bits - (1 << 64)


def owned_integers() -> None:
    """Keep empty, duplicate and signed-limit values independent of runtime release policy."""
    for values in ((), (0,), (-0x80000000, 0x7fffffff, -1, 0, -1)):
        raw = struct.pack('<i', len(values)) + b''.join(struct.pack('<i', value) for value in values)
        check_integer(raw, values, len(values))
    assert signed_pair([0, 0x80000000]) == -0x8000000000000000
    assert signed_pair([0xffffffff, 0x7fffffff]) == 0x7fffffffffffffff
    assert signed_pair([0xffffffff, 0xffffffff]) == -1


def owned_unique() -> None:
    """Prove metadata-only maximum capacity and complete signed high-word/duplicate key preservation."""
    from cosworkspersist import read_unique_ids
    for keys in ((), (-0x8000000000000000, 0x7fffffffffffffff, 1 << 40, -1, -1)):
        raw = struct.pack('<ii', len(keys), 0x7fffffff) + b''.join(struct.pack('<q', key) for key in keys)
        record = read_unique_ids(raw, max_entries=len(keys))
        assert record.keys == keys and record.allocation_count == 0x7fffffff
        assert record.header + record.payload == raw


def malformed() -> None:
    """Reject independently invalid prefixes, counts, capacities and trailing bytes."""
    from cosworkspersist import read_integer_array, read_unique_ids
    import cosworks_study_versions as newer
    integer = struct.pack('<iii', 2, -1, 0x7fffffff)
    for raw in [integer[:end] for end in range(len(integer))] + [integer + b'\0', struct.pack('<i', -1)]:
        with unittest.TestCase().assertRaises(ValueError):
            read_integer_array(raw, max_items=2)
        result = newer.parse(raw, max_items=2)
        assert not result['success'] and not result['complete']
    unique = struct.pack('<iiq', 1, 1, -1)
    invalid = [unique[:end] for end in range(len(unique))] + [unique + b'\0', struct.pack('<ii', -1, 0), struct.pack('<ii', 1, 0), struct.pack('<ii', 0, -1)]
    for raw in invalid:
        with unittest.TestCase().assertRaises(ValueError):
            read_unique_ids(raw, max_entries=1)


def resource_bounds() -> None:
    """Reject otherwise complete records under a smaller explicit caller budget."""
    from cosworkspersist import read_integer_array, read_unique_ids
    import cosworks_study_versions as newer
    raw = struct.pack('<iii', 2, -1, 0x7fffffff)
    with unittest.TestCase().assertRaises(ValueError):
        read_integer_array(raw, max_items=1)
    result = newer.parse(raw, max_items=1)
    assert not result['success'] and not result['complete']
    with unittest.TestCase().assertRaises(ValueError):
        read_unique_ids(struct.pack('<iiq', 1, 0x7fffffff, -1), max_entries=0)


def schema() -> None:
    """Retain complete framing clauses and current source inventory identities."""
    path = ROOT.parent / 'inventor-interface/docs/solidworks-file-format/specification.md'
    text = path.read_bytes().decode('utf-8').replace('\r\n', '\n')
    clauses = []
    for number in (267, 271, 272, 276, 444):
        start = text.index(f'### 14.{number} ')
        end = text.index('\n### ', start + 1)
        clauses.append(text[start:end].rstrip() + '\n')
    raw = '\n'.join(clauses).encode('utf-8')
    destination = FEATURE / 'schema-snapshot.md'
    if '--write' in sys.argv:
        destination.write_bytes(raw)
    assert destination.read_bytes() == raw
    baseline('schema-reference.json', dict(clauses=[267, 271, 272, 276, 444], bytes=len(raw), sha256=digest(raw)))
    names = ('cosworks-study-versions-native-evidence.json', 'cosworks-unique-ids-native-evidence.json', 'cosworks-unique-decoder-validation.json')
    baseline('native-source-reference.json', {name: digest((RESEARCH / name).read_bytes()) for name in names})


def native_cases() -> list[ScalarCase]:
    """Retain every current native carrier and all independently checked field edits."""
    cases = []
    for row in studies():
        raw = bytes.fromhex(row['result']['raw_hex'])
        assert digest(raw) == row['sha256']
        case = ScalarCase(0, 10000, raw)
        cases += [case, *edits(case)]
    for row in unique_tables():
        raw = bytes.fromhex(row['raw_hex'])
        assert digest(raw) == row['sha256']
        case = ScalarCase(1, 10000, raw)
        cases += [case, *edits(case)]
    return cases


def corpus() -> None:
    """Freeze owned cases and untracked native packets, then verify the complete Rust export."""
    from scalar_cases import owned_cases
    from scalar_projection import packet
    from owned_persistence import span
    owned_inputs, native_inputs = owned_cases(), native_cases()
    owned, native = packet(owned_inputs), packet(native_inputs)
    fixture = ROOT / 'tests/fixtures/simulation-persistent-scalars/owned.bin'
    if '--write' in sys.argv:
        fixture.parent.mkdir(parents=True, exist_ok=True)
        fixture.write_bytes(zlib.compress(owned, 9))
    assert zlib.decompress(fixture.read_bytes()) == owned
    (ROOT / 'target/simulation-persistent-scalars-native.bin').write_bytes(native)
    baseline('corpus-reference.json', dict(owned_cases=len(owned_inputs), native_cases=len(native_inputs), owned_bytes=len(owned), owned_sha256=digest(owned), native_bytes=len(native), native_sha256=digest(native), native_inputs_committed=False))
    if '--verify-rust' in sys.argv:
        actual = (ROOT / 'target/simulation-persistent-scalars-projections.bin').read_bytes()
        assert actual == span(owned) + span(native)
        baseline('rust-projection-reference.json', dict(bytes=len(actual), sha256=digest(actual)))


def run() -> bool:
    """Import the actual root and research modules only from the complete immutable copy."""
    with tempfile.TemporaryDirectory(prefix='bessemer-persistent-scalars-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin / 'research'))
        checks = (native_integers, native_unique, owned_integers, owned_unique, malformed, resource_bounds, schema, corpus)
        suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
        result = unittest.TextTestRunner(verbosity=2).run(suite)
        record_snapshot(pin, hashes, result, FEATURE, ('cosworkspersist', 'cosworks_study_versions'), ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
