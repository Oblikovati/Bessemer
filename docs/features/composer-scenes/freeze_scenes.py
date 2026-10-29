"""Retain complete Composer geometry-stream evidence before the whole-stream codec (#103)."""
from pathlib import Path
import gzip
import hashlib
import itertools
import json
import struct
import sys
import tempfile
import unittest
from types import ModuleType
from typing import Protocol

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
sys.path.insert(0, str(FEATURE.parent / 'model-archives'))
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot, SOURCE


class GeometryReader(Protocol):
    """Typed boundary shared by the actual gzip and decoded scene readers."""
    def __call__(self, raw: bytes, *, texture_import_enabled: bool) -> dict[str, object]: ...


def retain(path: Path, raw: bytes) -> None:
    """Require reviewed baseline replacement, e.g. retain(path, expected_wire)."""
    if '--write' in sys.argv:
        path.write_bytes(raw)
        return
    assert path.read_bytes() == raw, f'{path}; expected reviewed complete geometry baseline'


def retained_report(name: str, report: object) -> None:
    """Store stable facts without losing surrogate code units, e.g. retained_report('owned.json', [])."""
    retain(FEATURE / name, (json.dumps(report, indent=2) + '\n').encode())


def text_wire(value: str, terminator: int = 10) -> bytes:
    """Write the documented UTF-16LE code units and explicit terminator independently."""
    return value.encode('utf-16-le', errors='surrogatepass') + struct.pack('<H', terminator)


def words(*values: int) -> bytes:
    """Write independent little-endian scalar fixtures, e.g. words(1, 2)."""
    return struct.pack('<' + 'I' * len(values), *values)


def information_prefix(info: bytes = b'', source: str = 'source') -> bytes:
    """Keep logical info size distinct from physical string terminators."""
    logical = len(info) - 4 if info else 0
    return struct.pack('<d', 7.9) + text_wire(source, 0) + words(0xfedcba98, logical) + info


def full_information() -> bytes:
    """Cover all known info cells, both terminators, a surrogate and sized future bytes."""
    body = struct.pack('<6Q', *range(6)) + b'\xff\x02' + words(0x80000000)
    body += text_wire('A\ud800', 0) + text_wire('B', 10)
    return body + words(0x7fc12345) + struct.pack('<Q', 0xffffffffffffffff) + b'\xfeXYZ'


def face_wire(geometry: bytes, body: bytes = b'', first_name: str = 'parent') -> bytes:
    """Assemble common attributes directly from documented field order, independent of readers."""
    return b'v' + text_wire(first_name) + text_wire('second', 0) + words(*range(11)) + words(len(body)) + body + geometry


def material_body(name: str) -> bytes:
    """Supply complete texture columns plus an explicitly sized extension."""
    return words(1) + text_wire(name) + words(*range(16)) + b'\x02\xff' + words(17) + b'\x03\xfe' + b'future'


def mixed_scene(geode_name: str = 'geode', texture_name: str = 'texture') -> bytes:
    """Construct two LODs and a top-level face with raw matrix presence and bounded user data."""
    import test_composerscene as builders
    import test_composerattributes as user_builders
    user = user_builders.user_body(2) + b'user-future'
    geode = b'u\xff' + words(*range(16)) + text_wire(geode_name)
    geode += words(len(user)) + user + words(1) + face_wire(builders.geometry('b'), material_body(texture_name)) + words(0)
    return information_prefix(full_information()) + geode + face_wire(builders.geometry('f'))


def owned_cases() -> list[tuple[str, bool, bytes]]:
    """Exercise every drawable and presence branch before native fixture comparison."""
    import test_composerscene as builders
    cases = [('empty', True, information_prefix())]
    for kind, normal, textures in itertools.product('abcdef', (False, True), (False, True)):
        geometry = builders.geometry(kind, normal, textures)
        cases.append((f'drawable-{kind}-{int(normal)}-{int(textures)}', True, information_prefix() + face_wire(geometry)))
    skipped = builders.geometry('c', textures=False)[:-4] + words(1)
    cases.append(('texture-context-disabled', False, information_prefix() + face_wire(skipped)))
    cases += material_cases(builders)
    cases.extend([('mixed', True, mixed_scene()), ('mixed-name-edited', True, mixed_scene('Bessemer Ω')),
                  ('mixed-material-edited', True, mixed_scene(texture_name='Bessemer texture Ω'))])
    return cases


def material_cases(builders: ModuleType) -> list[tuple[str, bool, bytes]]:
    """Distinguish absent, empty, short and unsupported sized material bodies."""
    bodies = [b'', words(0), words(5) + b'unknown', words(2), words(2) + text_wire('first'),
              words(1) + text_wire('first') + words(*range(16)) + b'\x02']
    return [(f'material-{index}', True, information_prefix() + face_wire(builders.geometry('d'), body))
            for index, body in enumerate(bodies)]


def stable_projection(value: object) -> object:
    """Retain all decoded fields while binding large packed byte spans by exact hashes."""
    if isinstance(value, bytes):
        return dict(bytes=len(value), sha256=hashlib.sha256(value).hexdigest())
    if isinstance(value, dict):
        return {key: stable_projection(child) for key, child in value.items()}
    if isinstance(value, (tuple, list)):
        return [stable_projection(child) for child in value]
    return value


def verify_scene(encoded: bytes, decoded: bytes, enabled: bool) -> dict[str, object]:
    """Require independent gzip contents plus actual complete scene EOF and projection."""
    import composergeometry
    import composerscene
    assert gzip.decompress(encoded) == decoded
    assert composergeometry.decode(encoded)['decoded'] == decoded
    scene = composerscene.read_decoded(decoded, texture_import_enabled=enabled)
    assert scene['end'] == len(decoded) and scene['complete_scene_framing']
    assert not scene['complete_document_schema']
    return dict(encoded_bytes=len(encoded), decoded_bytes=len(decoded),
                encoded_sha256=hashlib.sha256(encoded).hexdigest(), decoded_sha256=hashlib.sha256(decoded).hexdigest(),
                counts=scene['counts'], projection=stable_projection(scene))


def scene_packet(encoded: bytes, decoded: bytes, enabled: bool) -> bytes:
    """Keep independent compressed/decoded oracles and explicit texture-import context together."""
    return struct.pack('<BII', int(enabled), len(encoded), len(decoded)) + encoded + decoded


def freeze_owned() -> None:
    """Retain complete inputs, independent name/material-edit goldens and all reference fields."""
    records, packets = [], []
    for name, enabled, decoded in owned_cases():
        encoded = gzip.compress(decoded, mtime=0)
        records.append(dict(name=name, texture_import_enabled=enabled, **verify_scene(encoded, decoded, enabled)))
        packets.append(scene_packet(encoded, decoded, enabled))
    retain(ROOT / 'tests/fixtures/composer-scenes/scenes.bin', b'SCNE' + words(len(packets)) + b''.join(packets))
    retained_report('owned-reference.json', records)


def freeze_failures() -> None:
    """Reject all compressed prefixes/extensions while respecting valid decoded record boundaries."""
    import composer
    import composerscene
    rejected = 0
    for _, enabled, decoded in owned_cases():
        encoded = gzip.compress(decoded, mtime=0)
        for end in range(len(encoded)):
            rejected += expect_failure(composerscene.read, encoded[:end], enabled, composer.ComposerError)
        rejected += expect_failure(composerscene.read_decoded, decoded + b'X', enabled, composer.ComposerError)
    wrong_version = struct.pack('<d', 7.8) + information_prefix()[8:]
    rejected += expect_failure(composerscene.read_decoded, wrong_version, True, composer.ComposerError)
    retained_report('rejections.json', dict(compressed_prefixes_and_invalid_extensions=rejected,
                    decoded_record_boundary_prefixes='valid shorter streams; not classified as failures'))


def expect_failure(reader: GeometryReader, raw: bytes, enabled: bool, exception: type[Exception]) -> int:
    """Require the actual reader to reject one complete unsupported/invalid input."""
    try:
        reader(raw, texture_import_enabled=enabled)
    except exception:
        return 1
    raise AssertionError(f'{raw.hex()}; expected complete geometry-stream rejection')


def pin_schema() -> None:
    """Pin the complete applicable clauses without binding unrelated upstream prose changes."""
    source = SOURCE.parent / 'docs/solidworks-file-format/specification.md'
    text = source.read_text(encoding='utf-8')
    sections = []
    for number in ('14.7', '14.87', '14.88'):
        start = text.index('### ' + number + ' ')
        end = text.index('\n### ', start)
        sections.append(text[start:end].rstrip() + '\n')
    raw = '\n'.join(sections).encode()
    retain(FEATURE / 'schema-snapshot.md', raw)
    original = hashlib.sha256(source.read_bytes()).hexdigest()
    if '--write' not in sys.argv:
        original = json.loads((FEATURE / 'schema-reference.json').read_bytes())['source_sha256']
    retained_report('schema-reference.json', dict(clauses=['14.7', '14.87', '14.88'], issue=103,
                    source_sha256=original, snapshot_sha256=hashlib.sha256(raw).hexdigest()))


def freeze_native() -> None:
    """Require all complete native streams and bind every original archive/member identity."""
    import composer
    records, packets = [], []
    catalog = json.loads((SOURCE / 'research/composer-project-corpus.json').read_bytes())
    for row in catalog['files']:
        raw = Path(row['path']).read_bytes()
        assert hashlib.sha256(raw).hexdigest() == row['sha256']
        document = composer.Document(raw)
        member = next(entry for entry in document.entries if entry.name.endswith('.smgGeom'))
        encoded = document.read(member)
        decoded = gzip.decompress(encoded)
        facts = verify_scene(encoded, decoded, True)
        records.append(native_summary(row, member.name, facts))
        if '--export' in sys.argv:
            packets.append(scene_packet(encoded, decoded, True))
    retain_native(records, packets)


def native_summary(row: dict[str, object], member: str, facts: dict[str, object]) -> dict[str, object]:
    """Bind full projection hashes without committing large native byte payloads."""
    projection = (json.dumps(facts.pop('projection'), sort_keys=True) + '\n').encode()
    return dict(source_path=row['path'], archive_sha256=row['sha256'], member=member,
                projection_sha256=hashlib.sha256(projection).hexdigest(), **facts)


def retain_native(records: list[dict[str, object]], packets: list[bytes]) -> None:
    """Require documented native totals and retain the complete local Rust probe identity."""
    counts = {}
    for row in records:
        for key, count in row['counts'].items():
            counts[key] = counts.get(key, 0) + count
    total = sum(row['decoded_bytes'] for row in records)
    assert (len(records), total, counts['geodes']) == (38, 45280926, 1787)
    assert {key: count for key, count in counts.items() if key.startswith('geometry:')} == {
        'geometry:a': 138, 'geometry:b': 2, 'geometry:c': 3, 'geometry:d': 1903, 'geometry:e': 10}
    retained_report('native-reference.json', dict(files=len(records), decoded_bytes=total, counts=counts, records=records))
    if packets:
        raw = b'SCNE' + words(len(packets)) + b''.join(packets)
        (ROOT / 'target/composer-scenes-native.bin').write_bytes(raw)
        retained_report('native-probe-reference.json', dict(bytes=len(raw), sha256=hashlib.sha256(raw).hexdigest()))


def main() -> None:
    """Pin actual reader and builder modules before any source imports or fixture work."""
    with tempfile.TemporaryDirectory(prefix='bessemer-composer-scenes-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin))
        suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in (freeze_owned, freeze_native, freeze_failures, pin_schema))
        result = unittest.TextTestRunner(verbosity=2).run(suite)
        assert result.wasSuccessful() and result.testsRun == 4 and not result.skipped
        record_snapshot(pin, hashes, result, FEATURE, ('composer', 'composergeometry', 'composerscene'), ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'owned-source-snapshot.json')


if __name__ == '__main__':
    main()
