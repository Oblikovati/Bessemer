"""Retain independent packed-array inputs and actual reference results (#101)."""
from pathlib import Path
from collections.abc import Iterator
from types import ModuleType
import hashlib
import itertools
import json
import struct
import sys
import tempfile
import unittest

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
sys.path.insert(0, str(FEATURE.parent / 'model-archives'))
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot, SOURCE


def retained(name: str, value: object) -> None:
    """Compare reviewed results unless explicitly regenerating; e.g. retained('owned.json', [])."""
    raw = (json.dumps(value, indent=2) + '\n').encode()
    path = FEATURE / name
    if '--write' in sys.argv:
        path.write_bytes(raw)
        return
    assert path.read_bytes() == raw, f'{path}; expected unchanged reference baseline'


def vector_wire(mode: str, count: int, flags: tuple[int, ...]) -> bytes:
    """Construct from the documented field order, without using a reference writer."""
    header = mode.encode() + (bytes(flags) if len(flags) == 3 else b'')
    constants = [0x7f812345, 0x80000000, 0xff800000]
    header += b''.join(struct.pack('<I', constants[i]) for i, flag in enumerate(flags) if flag)
    if mode != 'a':
        header += b''.join(struct.pack('<II', 0xbf800000, 0x3f800000) for flag in flags if not flag)
    width = dict(a=4, b=2, c=1)[mode]
    length = count * flags.count(0) * width
    return header + bytes((i * 37 + 65) % 256 for i in range(length))


def vector_case(mode: str, count: int, flags: tuple[int, ...]) -> dict[str, object]:
    """Check complete actual framing and store its arithmetic projection separately."""
    import composerscene
    import composervectors
    raw = vector_wire(mode, count, flags)
    block = composerscene.read_vector(raw, count=count, dimensions=len(flags))
    assert block['start'] == 0 and block['end'] == len(raw)
    assert block['constant_axis_flags'] == flags and block['mode'] == mode
    projected = composervectors.reconstruct(block, version=7.9, fp_profile=composervectors.PROFILE)
    return dict(mode=mode, count=count, dimensions=len(flags), flags=flags, wire_hex=raw.hex(),
                constants=block['constant_patterns'], endpoints=block['endpoint_patterns'],
                packed_hex=block['packed_values']['raw_bytes'].hex(),
                nearest_even_gradual_patterns_hex=projected['patterns_le'].hex())


def owned_vectors() -> None:
    """Cover every canonical V3 flag combination and the flag-free V2 layout."""
    cases = []
    for mode, count in itertools.product('abc', (0, 2)):
        cases.append(vector_case(mode, count, (0, 0)))
        cases.extend(vector_case(mode, count, flags) for flags in itertools.product((0, 1), repeat=3))
    assert len(cases) == 54
    retained('owned-vectors.json', cases)


def owned_indices() -> None:
    """Pin domain-selected widths including zero and both unsigned transitions."""
    import composerscene
    cases = []
    for domain, count in itertools.product((0, 255, 256, 65535, 65536, 0xffffffff), (0, 3)):
        width = 1 if domain <= 255 else 2 if domain <= 65535 else 4
        values = (0, 1, (1 << (8 * width)) - 1)[:count]
        raw = b''.join(value.to_bytes(width, 'little') for value in values)
        block = composerscene.read_indices(raw, count=count, domain=domain)
        assert block['end'] == len(raw) and block['value_width'] == width
        reject_indices(raw, count, domain)
        cases.append(dict(domain=domain, count=count, width=width, values=values, wire_hex=raw.hex()))
    retained('owned-indices.json', cases)


def reject_indices(raw: bytes, count: int, domain: int) -> None:
    """Reject every incomplete index span and extensions even when the count is zero."""
    import composer
    import composerscene
    for candidate in [raw[:end] for end in range(len(raw))] + [raw + b'X']:
        try:
            composerscene.read_indices(candidate, count=count, domain=domain)
        except composer.ComposerError:
            continue
        raise AssertionError(f'{candidate.hex()}; expected exact {count} index values for {domain}')


def rejected_vectors() -> None:
    """Require exact EOF for every prefix, suffix and noncanonical mode/flag."""
    import composer
    import composerscene
    rejected = 0
    for mode in 'abc':
        raw = vector_wire(mode, 2, (1, 0, 1))
        candidates = [raw[:end] for end in range(len(raw))] + [raw + b'X']
        candidates += [b'x' + raw[1:], raw[:1] + b'\x02' + raw[2:]]
        for candidate in candidates:
            try:
                composerscene.read_vector(candidate, count=2, dimensions=3)
            except composer.ComposerError:
                rejected += 1
                continue
            raise AssertionError(f'{candidate.hex()}; expected invalid complete vector')
    retained('rejections.json', dict(rejected_vectors=rejected))


def pin_schema() -> None:
    """Retain the normative clauses by heading rather than unstable line numbers."""
    path = SOURCE.parent / 'docs/solidworks-file-format/specification.md'
    text = path.read_text(encoding='utf-8')
    spans = []
    for heading in ('#### 14.88.2 ', '### 14.97 '):
        start = text.index(heading)
        stop = text.index('\n' + ('#### ' if heading.startswith('####') else '### '), start)
        spans.append(text[start:stop].rstrip() + '\n')
    raw = '\n'.join(spans).encode()
    snapshot = FEATURE / 'schema-snapshot.md'
    if '--write' in sys.argv:
        snapshot.write_bytes(raw)
    assert snapshot.read_bytes() == raw, 'schema clauses changed; expected reviewed snapshot'
    schema_identity(path, raw)


def schema_identity(path: Path, raw: bytes) -> None:
    """Check the scoped clauses while retaining the original whole-document provenance."""
    digest = hashlib.sha256(path.read_bytes()).hexdigest()
    if '--write' not in sys.argv:
        digest = json.loads((FEATURE / 'schema-reference.json').read_bytes())['source_sha256']
    retained('schema-reference.json', dict(source_sha256=digest,
             snapshot_sha256=hashlib.sha256(raw).hexdigest(), issue=101, clauses=['14.88.2', '14.97']))


def array_blocks(value: object) -> Iterator[dict[str, object]]:
    """Visit only actual decoded array nodes, without interpreting opaque bytes."""
    if isinstance(value, dict):
        if 'packed_values' in value or 'domain_parameter' in value:
            yield value
            return
        for child in value.values():
            yield from array_blocks(child)
    elif isinstance(value, (tuple, list)):
        for child in value:
            yield from array_blocks(child)


def native_arrays() -> None:
    """Bind all installed arrays to complete reference scenes and original member hashes."""
    import composervectors
    records = []
    for row in json.loads((SOURCE / 'research/composer-project-corpus.json').read_bytes())['files']:
        encoded, decoded, scene = native_scene(row)
        records.append(native_record(row, encoded, decoded, scene, composervectors))
    assert len(records) == 38
    assert sum(record['vectors'] for record in records) == 4367
    retained('native-reference.json', records)


def native_scene(row: dict[str, str]) -> tuple[bytes, bytes, dict[str, object]]:
    """Require exact archive identity and complete geometry framing before collecting arrays."""
    import composer
    import composergeometry
    import composerscene
    raw = Path(row['path']).read_bytes()
    assert hashlib.sha256(raw).hexdigest() == row['sha256']
    document = composer.Document(raw)
    member = next(entry for entry in document.entries if entry.name.endswith('.smgGeom'))
    encoded = document.read(member)
    decoded = composergeometry.decode(encoded)['decoded']
    scene = composerscene.read_decoded(decoded, texture_import_enabled=True)
    assert scene['end'] == len(decoded)
    return encoded, decoded, scene


def native_record(row: dict[str, str], encoded: bytes, decoded: bytes, scene: dict[str, object], vectors: ModuleType) -> dict[str, object]:
    """Hash original wire arrays and separately declared arithmetic output in scene order."""
    wire_digest, projected_digest = hashlib.sha256(), hashlib.sha256()
    vector_count = index_count = wire_bytes = 0
    for block in array_blocks(scene['entries']):
        wire = decoded[block['start']:block['end']]
        wire_digest.update(struct.pack('<I', len(wire)) + wire)
        wire_bytes += len(wire)
        if 'packed_values' not in block:
            index_count += 1
            continue
        vector_count += 1
        projected = vectors.reconstruct(block, version=7.9, fp_profile=vectors.PROFILE)
        projected_digest.update(struct.pack('<I', len(projected['patterns_le'])) + projected['patterns_le'])
    return dict(source_path=row['path'], archive_sha256=row['sha256'],
                member_sha256=hashlib.sha256(encoded).hexdigest(),
                decoded_sha256=hashlib.sha256(decoded).hexdigest(),
                decoded_bytes=len(decoded), vectors=vector_count, indices=index_count,
                array_wire_bytes=wire_bytes, framed_arrays_sha256=wire_digest.hexdigest(),
                nearest_even_gradual_patterns_sha256=projected_digest.hexdigest())


def main() -> None:
    """Freeze imports before checking fixtures; e.g. python freeze_arrays.py --write."""
    with tempfile.TemporaryDirectory(prefix='bessemer-composer-arrays-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin))
        suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in
                                   (owned_vectors, owned_indices, rejected_vectors, pin_schema, native_arrays))
        result = unittest.TextTestRunner(verbosity=2).run(suite)
        assert result.wasSuccessful() and result.testsRun == 5 and not result.skipped
        record_snapshot(pin, hashes, result, FEATURE, ('composerscene', 'composervectors'), ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'owned-source-snapshot.json')


if __name__ == '__main__':
    main()
