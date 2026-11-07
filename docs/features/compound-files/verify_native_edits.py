"""Reopen every isolated native edit through the actual frozen CFB reader (#106)."""
from pathlib import Path
import json
import struct
import sys
import tempfile
import unittest

import freeze_compound as frozen
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]


def word(raw: bytes, cursor: int) -> tuple[int, int]:
    """Read one bounded bundle count without accepting an incomplete integer."""
    assert cursor + 4 <= len(raw)
    return struct.unpack_from('<I', raw, cursor)[0], cursor + 4


def span(raw: bytes, cursor: int) -> tuple[bytes, int]:
    """Require the entire declared file or payload slice."""
    count, cursor = word(raw, cursor)
    assert cursor + count <= len(raw)
    return raw[cursor:cursor + count], cursor + count


def records(oracle: bytes) -> dict[int, tuple[int, bytes, bytes]]:
    """Retain every metadata byte except the explicitly rebuilt allocation/size fields."""
    _, cursor = word(oracle, 0)
    _, cursor = word(oracle, cursor)
    count, cursor = word(oracle, cursor)
    entries = {}
    for _ in range(count):
        identifier, cursor = word(oracle, cursor)
        parent, cursor = word(oracle, cursor)
        record = bytearray(oracle[cursor:cursor + 128])
        cursor += 128
        if record[66] in (2, 5):
            record[116:128] = bytes(12)
        payload, cursor = span(oracle, cursor)
        entries[identifier] = parent, bytes(record), payload
    assert cursor == len(oracle)
    return entries


def original_inputs(temporary: Path) -> list[dict[int, tuple[int, bytes, bytes]]]:
    """Bind each original projection to the retained file and complete oracle hashes."""
    native = json.loads((FEATURE / 'native-reference.json').read_text())['records']
    originals = []
    for record in native:
        raw = Path(record['path']).read_bytes()
        assert frozen.sha(raw) == record['sha256']
        oracle, _ = frozen.inspect(raw, temporary)
        assert frozen.sha(oracle) == record['oracle_sha256']
        originals.append(records(oracle))
    return originals


def expected_replacement(original: bytes) -> bytes:
    """Construct the expected single stream edit independently of the Rust allocator."""
    return bytes([original[0] ^ 1]) + original[1:] if original else b'BessemerCFB'


def verify_edit(raw: bytes, original: dict[int, tuple[int, bytes, bytes]], identifier: int, temporary: Path) -> dict[str, object]:
    """Reopen all streams and metadata; the chosen payload is the only logical change."""
    oracle, facts = frozen.inspect(raw, temporary)
    expected = dict(original)
    parent, record, payload = expected[identifier]
    assert record[66] == 2
    expected[identifier] = parent, record, expected_replacement(payload)
    assert records(oracle) == expected
    assert struct.unpack_from('<I', raw, 52)[0] == 0
    checks = facts['allocation_observations']
    assert all(checks[key] == 0 for key in ('normal_duplicate_claims', 'mini_duplicate_claims', 'stream_extent_mismatches', 'unowned_allocated_sectors'))
    return dict(entry=identifier, bytes=len(raw), sha256=frozen.sha(raw), oracle_sha256=frozen.sha(oracle), streams=len(facts['streams']))


def verify_bundle() -> None:
    """Require exactly one isolated edit of every installed user stream."""
    raw = (ROOT / 'target/compound-files-native-edits.bin').read_bytes()
    assert raw[:4] == b'CFBE'
    count, cursor = word(raw, 4)
    assert count == 528
    seen, edits = set(), []
    with tempfile.TemporaryDirectory(prefix='bessemer-cfb-edits-') as directory:
        temporary = Path(directory) / 'reference.cfb'
        originals = original_inputs(temporary)
        for _ in range(count):
            source, cursor = word(raw, cursor)
            identifier, cursor = word(raw, cursor)
            encoded, cursor = span(raw, cursor)
            assert (source, identifier) not in seen
            seen.add((source, identifier))
            edits.append(dict(source=source, **verify_edit(encoded, originals[source], identifier, temporary)))
    finish_bundle(raw, cursor, seen, originals, edits)


def finish_bundle(raw: bytes, cursor: int, seen: set[tuple[int, int]], originals: list[dict], edits: list[dict]) -> None:
    """Prove full corpus coverage rather than accepting a successful partial prefix."""
    expected = {(source, identifier) for source, entries in enumerate(originals)
                for identifier, (_, record, _) in entries.items() if record[66] == 2}
    assert cursor == len(raw) and seen == expected
    frozen.report('native-edit-reference.json', dict(files=len(originals), edits=len(edits),
                                                    bundle_bytes=len(raw), bundle_sha256=frozen.sha(raw), records=edits))


def run() -> bool:
    """Freeze import origins before loading any actual reference reader."""
    with tempfile.TemporaryDirectory(prefix='bessemer-cfb-edits-source-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        frozen.pin_reader(pin, hashes)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        result = unittest.TextTestRunner(verbosity=2).run(unittest.FunctionTestCase(verify_bundle))
        record_snapshot(pin, hashes, result, FEATURE, ('cfb',), ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'native-edit-source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
