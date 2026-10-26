"""Freeze planar surfaces with inline bodies and mixed-width keeper maps (#54)."""
from pathlib import Path
import hashlib
import json
import struct
import sys
from freeze_manager_context import WORKSPACE, reference_snapshot, source_module
from primitive_inventory import observe_primitives, verify_primitive_coverage
import body_feature_fixture_bytes as body
from sketch_tail_fixture_bytes import WireMap
from kernel_geometry_fixture import frame


def keeper_payload(identity: int) -> bytes:
    """Keep the final word count as a plain u16; e.g. keeper_payload(1)."""
    raw = struct.pack('<HI2HHIH', 0xffff, 2, identity, identity, 0, 0xfedcba98, 1)
    return raw+struct.pack('<HHH2I', 1, identity, 2, 0x80000000, 0xffffffff)


def full_planar_body(state: WireMap, tail: int = -77) -> bytes:
    """Use a real derived keeper and inline attribute; e.g. full_planar_body(state)."""
    header = body.header(state, 19000, False)
    header = header[:-2]+struct.pack('<H', 1)+state.new('moPlanarSurfIdKeeper_c', lambda: keeper_payload(2))
    geometry = header+body.atom_fields(state, False, 2)+struct.pack('<3dIiHi', -0.0, 1.25, -2.5, 0xffffffff, -1, 0, -7)
    attribute_id = state.slot+2
    attribute = state.new('moAttribute_c', lambda: bytes(137)+struct.pack('<4Hi', 0, 0, 0, 0, -123))
    surface = struct.pack('<iiBH', 9, 1, 255, 1)+attribute+struct.pack('<iBi2i', -1, 255, 2, -7, 0x12345678)
    return geometry+surface+struct.pack('<HHi', 1, attribute_id, tail)


def planar_object(tail: int = -77) -> bytes:
    """Bind a complete ordinary planar class; e.g. planar_object()."""
    state = WireMap()
    return state.new('moPlanarSurface_c', lambda: full_planar_body(state, tail))


def surface_payload(version: int, serialized: bool) -> bytes:
    """Retain nonzero backend markers without guessing external bodies; e.g. surface_payload(19000, True)."""
    if serialized:
        return struct.pack('<3i', 9, 0, 1)+frame()+struct.pack('<3Hi2i', 2, 1, 1, 2, -8, 9)
    raw = struct.pack('<iiBHHiB', 9, 1, 255, 1, 1, -1, 255)
    return raw+(struct.pack('<i2i', 2, -7, 0x12345678) if version >= 0x2bcd else b'')


def freeze_planar_case(reference: Path, destination: Path, stem: str, raw: bytes, version: int, mode: str) -> dict[str, object]:
    """Require complete independent framing; e.g. freeze_planar_case(source, root, stem, raw, 19000, 'object')."""
    objects, archive = source_module('mfcobjects', reference), source_module('swarchive', reference)
    surfaces, attributes = source_module('planarsurfaces', reference), source_module('attributes', reference)
    cursor = objects.ObjectArchive(raw, version)
    cursor.feature_context = 'ordinary_external_xml'
    cursor.model_root_class = 'moPart_c'
    cursor.readers = dict(cursor.readers, moPlanarSurface_c=surfaces.read, moPlanarSurfIdKeeper_c=surfaces.keeper,
                          moAttribute_c=attributes.read)
    if mode != 'object': cursor.map.append({'_class': 'moCompEdge_c' if mode == 'keeper' else 'moAttribute_c'})
    with observe_primitives(archive.Archive, raw) as reads:
        cursor.object() if mode == 'object' else getattr(surfaces, mode)(cursor)
    if cursor.p != len(raw): raise ValueError(f'{stem} at {cursor.p}; expected EOF {len(raw)}')
    nonempty = [read for read in reads if read.end > read.start]
    verify_primitive_coverage(raw, nonempty)
    rows = ''.join(f'{read.start}\t{read.end}\t{read.kind}\t{read.raw_hex}\n' for read in nonempty).encode()
    (destination/f'{stem}.bin').write_bytes(raw)
    (destination/f'{stem}-primitives.tsv').write_bytes(rows)
    return {'stem': stem, 'version': version, 'bytes': len(raw), 'reads': len(nonempty),
            'sha256': hashlib.sha256(raw).hexdigest(), 'oracle_sha256': hashlib.sha256(rows).hexdigest()}


def freeze_surfaces(reference: Path, destination: Path) -> dict[str, object]:
    """Bind suppression and word-vector gates; e.g. freeze_surfaces(source, temporary_root)."""
    before = reference_snapshot(reference)
    sys.path.insert(0, str(reference))
    destination.mkdir(parents=True, exist_ok=True)
    cases = [('planar-object', planar_object(), 19000, 'object'),
             ('planar-keeper', keeper_payload(1), 19000, 'keeper'),
             ('planar-surface-old', surface_payload(0x2bcc, False), 0x2bcc, 'surface_payload'),
             ('planar-surface-current', surface_payload(0x2bcd, False), 0x2bcd, 'surface_payload'),
             ('planar-surface-kernel', surface_payload(19000, True), 19000, 'surface_payload'),
             ('planar-object-edited', planar_object(-770), 19000, 'object')]
    records = [freeze_planar_case(reference, destination, *case) for case in cases]
    if before != reference_snapshot(reference): raise ValueError('reference changed; expected stable sources')
    builders = ['body_feature_fixture_bytes.py', 'sketch_tail_fixture_bytes.py', 'freeze_environment.py',
                'freeze_node_units.py', 'owned_name_edit.py', 'kernel_geometry_fixture.py']
    return {'issue': 54, 'records': records, 'source_hashes': before,
            'wire_builder_hashes': {name: hashlib.sha256((WORKSPACE/'docs/features/model-archives'/name).read_bytes()).hexdigest() for name in builders},
            'generator_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}


if __name__ == '__main__':
    report = freeze_surfaces(WORKSPACE.parent/'inventor-interface/solidworks', WORKSPACE/'tests/fixtures/resolved-features')
    Path(__file__).with_name('planar-surfaces-reference.json').write_bytes((json.dumps(report, indent=2)+'\n').encode())
    print(report['records'])
