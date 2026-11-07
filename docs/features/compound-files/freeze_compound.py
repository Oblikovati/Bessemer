"""Freeze complete MS-CFB stream/metadata oracles before implementing the carrier (#106)."""
from pathlib import Path
import hashlib
import json
import struct
import sys
import tempfile
import unittest
import zlib

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
sys.path.insert(0, str(FEATURE.parent / 'model-archives'))
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot, SOURCE
from owned_compound import owned_cases, canonical_case
from allocation_observations import observations


def sha(raw: bytes) -> str:
    """Name every input and projection by its actual bytes."""
    return hashlib.sha256(raw).hexdigest()


def retain(name: str, raw: bytes) -> None:
    """Require an explicit baseline update for any changed expected result."""
    path = FEATURE / name
    if '--write' in sys.argv:
        path.write_bytes(raw)
        return
    assert path.read_bytes() == raw, f'{path}; expected unchanged reviewed compound-file oracle'


def report(name: str, facts: object) -> None:
    """Keep payloads out of the human-readable provenance report."""
    retain(name, (json.dumps(facts, indent=2) + '\n').encode())


def word(value: int) -> bytes:
    """Encode portable unsigned oracle sizes and identifiers."""
    return struct.pack('<I', value)


def span(raw: bytes) -> bytes:
    """Frame arbitrary bytes without assuming text or platform encoding."""
    return word(len(raw)) + raw


def parent_ids(document: object) -> dict[int, int]:
    """Use the actual reader's reconstructed direct-child relationships."""
    parents = {0: 0xffffffff}
    pending = [document.entries[0]]
    while pending:
        parent = pending.pop()
        for child in parent.children:
            assert child.sid not in parents, f'{child.sid}; expected one parent per directory entry'
            parents[child.sid] = parent.sid
            if child.type == 1:
                pending.append(child)
    return parents


def projection(document: object) -> tuple[bytes, dict[str, object]]:
    """Retain every active raw directory record and every complete logical stream."""
    directory = document._chain_read(document.dir_start)
    parents = parent_ids(document)
    entries = [entry for entry in document.entries if entry.type != 0]
    packets, streams = [], []
    for entry in entries:
        record = directory[entry.sid * 128:(entry.sid + 1) * 128]
        logical_size = entry.size & 0xffffffff if document.major == 3 else entry.size
        payload = document.read(entry)[:logical_size] if entry.type == 2 else b''
        assert entry.type != 2 or len(payload) == logical_size, f'{entry.path}; expected complete logical size {logical_size}'
        packets.append(word(entry.sid) + word(parents.get(entry.sid, 0xffffffff)) + record + span(payload))
        if entry.type == 2:
            streams.append(dict(id=entry.sid, path=entry.path, bytes=len(payload), sha256=sha(payload)))
    facts = dict(major=document.major, sector_bytes=document.ssize, directory_slots=len(document.entries),
                 root_color=document.entries[0].color,
                 active_entries=len(entries), orphan_entries=len(entries) - len(parents),
                 fat_sectors=document.n_fat, difat_sectors=document.n_difat, mini_fat_sectors=document.n_minifat,
                 streams=streams)
    facts['allocation_observations'] = observations(document)
    return word(document.major) + word(len(document.entries)) + word(len(entries)) + b''.join(packets), facts


def inspect(raw: bytes, temporary: Path) -> tuple[bytes, dict[str, object]]:
    """Run the unmodified frozen reader on an actual file, never seed parsed state."""
    from cfb import CFB
    temporary.write_bytes(raw)
    document = CFB(temporary)
    assert document.data == raw
    return projection(document)


def freeze_owned() -> None:
    """Check independently constructed bytes before retaining any Rust expected result."""
    packets, facts = [], []
    with tempfile.TemporaryDirectory(prefix='bessemer-owned-cfb-') as temporary:
        for label, raw, expected in owned_cases():
            oracle, record = inspect(raw, Path(temporary) / 'owned.cfb')
            actual = {item['path']: (item['bytes'], item['sha256']) for item in record['streams']}
            assert actual == {key: (len(value), sha(value)) for key, value in expected.items()}
            canonical = canonical_case(raw, expected)
            inspect(canonical, Path(temporary) / 'canonical.cfb')
            facts.append(dict(label=label, bytes=len(raw), sha256=sha(raw), oracle_sha256=sha(oracle), canonical_sha256=sha(canonical), **record))
            packets.append(span(raw) + span(oracle) + span(canonical))
    packet = zlib.compress(b'CFBO' + word(len(packets)) + b''.join(packets), 9)
    fixture = ROOT / 'tests/fixtures/compound-files/owned.bin'
    if '--write' in sys.argv:
        fixture.parent.mkdir(parents=True, exist_ok=True)
        fixture.write_bytes(packet)
    assert fixture.read_bytes() == packet
    report('owned-reference.json', dict(fixture_sha256=sha(packet), cases=facts))


def candidate_paths() -> list[Path]:
    """Use actual installed sample discovery plus all three retained Simulation inputs."""
    import projectinventory
    paths = {path for _, _, path in projectinventory.sample_paths() if path.is_file()}
    gen = json.loads((SOURCE / 'research/simulation-gen-native-evidence.json').read_text())
    paths.update(Path(record['path']) for record in gen['files'])
    selected = []
    for path in sorted(paths):
        with path.open('rb') as source:
            if source.read(8) == bytes.fromhex('d0cf11e0a1b11ae1'):
                selected.append(path)
    return selected


def freeze_native() -> None:
    """Inventory every discovered compound carrier without treating payloads as decoded."""
    packets, facts = [], []
    with tempfile.TemporaryDirectory(prefix='bessemer-native-cfb-') as temporary:
        for path in candidate_paths():
            raw = path.read_bytes()
            oracle, record = inspect(raw, Path(temporary) / 'native.cfb')
            facts.append(dict(path=str(path), bytes=len(raw), sha256=sha(raw), oracle_sha256=sha(oracle), **record))
            if '--export' in sys.argv:
                packets.append(span(raw) + span(oracle))
    report('native-reference.json', dict(files=len(facts), records=facts))
    if '--export' in sys.argv:
        packet = b'CFBN' + word(len(packets)) + b''.join(packets)
        (ROOT / 'target/compound-files-native.bin').write_bytes(packet)
        report('native-probe-reference.json', dict(bytes=len(packet), sha256=sha(packet)))


def pin_reader(pin: Path, hashes: dict[str, str]) -> None:
    """Add the separate actual CFB reader to the immutable source import boundary."""
    original = ROOT.parent / 'inventor-interface/reader/cfb.py'
    raw = original.read_bytes()
    hashes['cfb.py'] = sha(raw)
    (pin / 'cfb.py').write_bytes(raw)


def run() -> bool:
    """Pin all imports before inspecting owned or installed files."""
    with tempfile.TemporaryDirectory(prefix='bessemer-cfb-reference-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        pin_reader(pin, hashes)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        suite = unittest.TestSuite(unittest.FunctionTestCase(test) for test in (freeze_owned, freeze_native))
        result = unittest.TextTestRunner(verbosity=2).run(suite)
        record_snapshot(pin, hashes, result, FEATURE, ('cfb', 'projectinventory'), ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
