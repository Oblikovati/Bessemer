"""Freeze independently observed complete reference-axis objects (#54)."""
from pathlib import Path
import hashlib
import json
import struct
import sys
from types import ModuleType

WORKSPACE = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(WORKSPACE/'docs/features/model-archives'))
from freeze_blank_part import reference_snapshot, source_module
from primitive_inventory import observe_primitives, verify_primitive_coverage
sys.path.insert(0, str(WORKSPACE/'docs/features/display-lists'))
from generate_shaded_vectors import ObjectBuilder

KINDS = ('moRefAxisData_c', 'moSurfaceAxisData_c', 'moFacePtAxisData_c',
         'moPlanePtAxisData_c', 'moPlaneInterAxisData_c', 'moLineAxisData_c', 'moConFaceAxisData_c', 'moTwoPtsAxisData_c')
BASE = struct.pack('<6dQQ6d', 1, 2, 3, 4, 5, 6, 0x8000000000000000,
                   0x7ff8000000000042, 0, 0, 1, 7, 8, 9)
CASES = KINDS+('temporary-line',)


def axis_body(builder: ObjectBuilder, kind: str, axes: ModuleType) -> bytes:
    """Build the entire axis with nullable suffixes; e.g. axis_body(builder, KINDS[0], reference)."""
    suffixes = getattr(axes, 'SUFFIXES')
    if kind == 'temporary-line':
        return bytes(279)+builder.object('moLineAxisData_c', lambda state:
            BASE+state.object('moTempAxisRef_w', lambda _: bytes(179)))
    suffix_count = 2 if kind == 'moTwoPtsAxisData_c' else len(suffixes.get(kind, ()))
    payload = BASE+bytes(2*suffix_count)
    return bytes(279)+builder.object(kind, lambda _: payload)


def freeze(reference: Path, destination: Path) -> dict[str, object]:
    """Require stable independent reads; e.g. freeze(source, owned_fixture_root)."""
    before = reference_snapshot(reference)
    sys.path.insert(0, str(reference))
    destination.mkdir(parents=True, exist_ok=True)
    cases = [freeze_case(reference, destination, kind) for kind in CASES]
    if before != reference_snapshot(reference):
        raise ValueError('source changed; expected stable reference reader hashes')
    return dict(cases=cases, source_hashes=before,
                generator_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest())


def freeze_case(reference: Path, destination: Path, kind: str) -> dict[str, object]:
    """Observe one complete object; e.g. freeze_case(source, output, kind)."""
    objects, archive, axes, bodies = (source_module(name, reference) for name in
                                     ('mfcobjects', 'swarchive', 'referenceaxes', 'bodyfeatures'))
    wire = ObjectBuilder(1).object('moRefAxis_c', lambda builder: axis_body(builder, kind, axes))
    cursor = getattr(objects, 'ObjectArchive')(wire, 19000)
    cursor.feature_context = 'ordinary_external_xml'
    getattr(bodies, 'install_scoped')(cursor)
    cursor.readers['moTwoPtsAxisData_c'] = source_module('geometricfrontier', reference).two_points
    cursor.readers['moTempAxisRef_w'] = source_module('sketchprefix', reference).temporary_axis_reference
    with observe_primitives(getattr(archive, 'Archive'), wire) as reads:
        parsed = cursor.object()
    expected = 'moLineAxisData_c' if kind == 'temporary-line' else kind
    if cursor.p != len(wire) or parsed['data']['_class'] != expected:
        raise ValueError(f'{kind}: end {cursor.p}; expected EOF {len(wire)} and matching data class')
    reads = [read for read in reads if read.end > read.start]
    verify_primitive_coverage(wire, reads)
    rows = ''.join(f'{read.start}\t{read.end}\t{read.kind}\t{read.raw_hex}\n' for read in reads).encode()
    (destination/(kind+'.bin')).write_bytes(wire)
    (destination/(kind+'.tsv')).write_bytes(rows)
    return dict(kind=kind, bytes=len(wire), reads=len(reads),
                sha256=hashlib.sha256(wire).hexdigest(), oracle_sha256=hashlib.sha256(rows).hexdigest())


if __name__ == '__main__':
    report = freeze(WORKSPACE.parent/'inventor-interface/solidworks', WORKSPACE/'tests/fixtures/reference-axes')
    (Path(__file__).parent/'reference.json').write_text(json.dumps(report, indent=2)+'\n', encoding='utf-8')
