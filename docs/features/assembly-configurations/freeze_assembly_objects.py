"""Independent assembly object graphs for documented branches absent from native representatives."""
from pathlib import Path
from functools import partial
import hashlib
import json
import struct
import sys
import tempfile
import unittest

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
sys.path.insert(0, str(FEATURE.parent / 'model-archives'))
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot
from primitive_inventory import observe_primitives, verify_primitive_coverage
from freeze_owned_assembly import freeze_bytes, wide_text
from freeze_string_maps import object_tag


def faceted_bytes(kind: int) -> bytes:
    """Preserve all five faceted variants and their original high flag bits."""
    raw = struct.pack('<I', kind | 0x80000030)
    if kind in (1, 2):
        raw += struct.pack('<i6d', 2, 1, 2, 3, 4, 5, 6)
        if kind == 2:
            raw += struct.pack('<i', -7)
    elif kind in (3, 4):
        raw += struct.pack('<6d', 1, 2, 3, 4, 5, 6)
    return raw + wide_text('Ref Ω')


def camera_bytes(kind: int) -> bytes:
    """Keep fresh attachment identity and repeat its actual MFC slot twice."""
    point = struct.pack('<6diH6dB', *range(6), -1, 0, *range(6, 12), 255)
    point += struct.pack('<ddHHI', 12, 13, 0, 65535, 0xfedcba98)
    first = object_tag('moParametricPoint_c') + point
    repeated = struct.pack('<H', 4)
    raw = struct.pack('<6i3di', -1, 2, -3, 4, -5, 6, 7, 8, 9, -10)
    raw += first + struct.pack('<di3di', 11, -12, 13, 14, 15, -16)
    raw += repeated + struct.pack('<d3diiH', 17, 18, 19, 20, -21, 22, 0)
    raw += faceted_bytes(kind) + struct.pack('<i5diidi', -23, 24, 25, 26, 27, 28, -29, 30, 31, -32)
    raw += repeated + struct.pack('<dd', 33, 34)
    return object_tag('moCameraFeature_c') + bytes(143) + raw


def object_graphs() -> list[bytes]:
    """Select complete real serializers and fresh nested unit components."""
    from freeze_views import transform
    result = []
    for index, flags in enumerate((0, 16, 32, 64)):
        first, second = transform(index % 2 == 0, flags), transform(index % 2 != 0, flags)
        unit = object_tag('moUnitComponent_c') + bytes(3)
        result.append(object_tag('moMirrorAssemblyData_c') + bytes(2) + first + second + struct.pack('<6d', *range(6)))
        component = first + unit
        result.append(object_tag('moCompXform_c') + component)
        result.append(object_tag('moRefXform_c') + component + second)
    result += [object_tag('moCirPatternItemId_c') + struct.pack('<iiBQ', -2147483648, 2147483647, 255, bits)
               for bits in (0x8000000000000000, 0x7ff800000000002a)]
    result += [camera_bytes(kind) for kind in range(5)]
    result.append(object_tag('moExplId_c') + wide_text('Exploded Ω') + struct.pack('<i', -2147483648))
    return result


def reference_graph(raw: bytes) -> tuple[bytes, dict[str, object]]:
    """Observe actual scoped serializers without preloading identities or replacing readers."""
    import assemblyresolvedtail as explosion
    import assemblyreferences as references
    import camerafeatures as camera
    import modelcomponents
    import partresolved
    from swarchive import Archive
    from mfcobjects import ObjectArchive
    cursor = ObjectArchive(raw, 19000)
    cursor.feature_context, cursor.model_root_class = 'ordinary_external_xml', 'moAssembly_c'
    cursor.readers = dict(cursor.readers, moMirrorAssemblyData_c=explosion.mirror,
                         moCompXform_c=explosion.component_transform, moRefXform_c=explosion.reference_transform,
                         moCirPatternItemId_c=references.circular_pattern_item, moCameraFeature_c=camera.read,
                         moParametricPoint_c=camera.parametric_point, moExplId_c=partresolved.exploded_id,
                         moUnitComponent_c=partial(modelcomponents.unit_component, owner_classes=('moAssembly_c', 'moRouteAssembly_c')))
    with observe_primitives(Archive, raw) as reads:
        root = cursor.object()
    assert cursor.p == len(raw)
    verify_primitive_coverage(raw, reads)
    oracle = ''.join(f'{r.start}\t{r.end}\t{r.kind}\t{r.raw_hex}\t{r.source_file}:{r.source_function}:{r.source_line}\n' for r in reads).encode()
    facts = dict(root_class=root['_class'], bytes=len(raw), map_slots=len(cursor.map), primitive_reads=len(reads),
                 sha256=hashlib.sha256(raw).hexdigest())
    return struct.pack('<III', len(raw), len(oracle), len(cursor.map)) + raw + oracle, facts


def freeze_objects() -> None:
    """Require complete primitive partitions for all twenty independent graphs."""
    records = [reference_graph(raw) for raw in object_graphs()]
    assert len(records) == 20
    raw = b'ASOB' + struct.pack('<I', len(records)) + b''.join(packet for packet, _ in records)
    freeze_bytes(ROOT / 'tests/fixtures/assembly-configurations/assembly-objects.bin', raw)
    freeze_bytes(FEATURE / 'assembly-object-reference.json', (json.dumps([facts for _, facts in records], indent=2) + '\n').encode())


def main() -> None:
    """Pin reference sources before importing transform helpers or scoped factories."""
    with tempfile.TemporaryDirectory(prefix='bessemer-assembly-objects-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin))
        result = unittest.TextTestRunner(verbosity=2).run(unittest.FunctionTestCase(freeze_objects))
        assert result.wasSuccessful() and result.testsRun == 1 and not result.skipped
        record_snapshot(pin, hashes, result, FEATURE, ('assemblyresolvedtail', 'assemblyreferences', 'camerafeatures'),
                        ('mfcobjects', 'dl_ui'))
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'assembly-object-source-snapshot.json')


if __name__ == '__main__':
    main()
