"""Freeze independent direct-keeper chamfer records and inherited body bytes (#54)."""
from pathlib import Path
import hashlib
import json
import struct
import sys
from freeze_manager_context import WORKSPACE, reference_snapshot, source_module
from primitive_inventory import observe_primitives, verify_primitive_coverage

CLASSES = ['moEntityIdRep_c', 'moCompEdge_c', 'suObList', 'moDisplayDistanceDim_c',
           'moDisplayAngularDim_c', 'moCompFace_c', 'moCompVertex_c', 'moRefWrapper_c']


def populated_payload() -> bytes:
    """Specify the native field order independently; e.g. populated_payload()."""
    raw = struct.pack('<i5H', 2, 1, 1, 2, 2, 2)
    raw += struct.pack('<6H3B3Hi', 3, 3, 3, 4, 5, 4, 255, 254, 253, 2, 6, 6, -123)
    raw += struct.pack('<HI3I', 65535, 3, 0, 0x80000000, 0xffffffff)
    return raw+struct.pack('<7HBiHB', 4, 4, 4, 2, 2, 2, 7, 252, -9, 8, 251)


def chamfer_object() -> bytes:
    """Reuse the established independent inherited body builder; e.g. chamfer_object()."""
    import body_feature_fixture_bytes as body
    from sketch_tail_fixture_bytes import WireMap
    state = WireMap()
    return state.new('Chamfer_c', lambda: body.header(state, 19000, False)
        +body.atom_fields(state, False, 2)+body.body_suffix(False)+bytes(53))


def observe_chamfer(reference: Path, raw: bytes, mode: str) -> tuple[bytes, int]:
    """Verify complete direct or tagged records; e.g. observe_chamfer(source, raw, 'payload')."""
    objects, archive = source_module('mfcobjects', reference), source_module('swarchive', reference)
    chamfers = source_module('chamfers', reference)
    cursor = objects.ObjectArchive(raw, 19000)
    cursor.feature_context = 'ordinary_external_xml'
    cursor.model_root_class = 'moPart_c'
    cursor.readers = dict(cursor.readers, Chamfer_c=chamfers.read)
    if mode == 'payload': cursor.map.extend({'_class': name} for name in CLASSES)
    with observe_primitives(archive.Archive, raw) as reads:
        parsed = cursor.object() if mode == 'object' else chamfers.payload(cursor)
    if cursor.p != len(raw):
        raise ValueError(f'chamfer {mode} at {cursor.p}; expected EOF {len(raw)}')
    if mode == 'payload' and parsed['pointer_880'] is not parsed['pointer_890']:
        raise ValueError('chamfer list aliases differ; expected one shared object')
    nonempty = [read for read in reads if read.end > read.start]
    verify_primitive_coverage(raw, nonempty)
    rows = ''.join(f'{read.start}\t{read.end}\t{read.kind}\t{read.raw_hex}\n' for read in nonempty).encode()
    return rows, len(nonempty)


def freeze_chamfers(reference: Path, destination: Path) -> dict[str, object]:
    """Require stable independent sources; e.g. freeze_chamfers(source, temporary_root)."""
    before = reference_snapshot(reference)
    sys.path.insert(0, str(reference))
    raw = populated_payload()
    edited = raw.replace(struct.pack('<i', -123), struct.pack('<i', 123), 1)
    cases = [('chamfer-payload', raw, 'payload'), ('chamfer-payload-edited', edited, 'payload'),
             ('chamfer-object', chamfer_object(), 'object')]
    destination.mkdir(parents=True, exist_ok=True)
    report = []
    for stem, wire, mode in cases:
        rows, count = observe_chamfer(reference, wire, mode)
        (destination/f'{stem}.bin').write_bytes(wire)
        (destination/f'{stem}-primitives.tsv').write_bytes(rows)
        report.append({'stem': stem, 'bytes': len(wire), 'reads': count,
            'sha256': hashlib.sha256(wire).hexdigest(), 'oracle_sha256': hashlib.sha256(rows).hexdigest()})
    if before != reference_snapshot(reference): raise ValueError('reference changed; expected stable sources')
    return {'issue': 54, 'records': report, 'source_hashes': before,
            'body_builder_sha256': hashlib.sha256((WORKSPACE/'docs/features/model-archives/body_feature_fixture_bytes.py').read_bytes()).hexdigest(),
            'generator_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}


if __name__ == '__main__':
    report = freeze_chamfers(WORKSPACE.parent/'inventor-interface/solidworks', WORKSPACE/'tests/fixtures/resolved-features')
    Path(__file__).with_name('chamfers-reference.json').write_bytes((json.dumps(report, indent=2)+'\n').encode())
    print(report['records'])
