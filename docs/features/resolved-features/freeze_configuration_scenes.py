"""Reuse owned display wire builders with ordinary-scene pointer policy (#54)."""
from pathlib import Path
import hashlib
import json
import struct
import sys
from freeze_manager_context import WORKSPACE, reference_snapshot, source_module
from primitive_inventory import observe_primitives, verify_primitive_coverage

sys.path.insert(0, str(WORKSPACE / 'docs/features/display-lists'))
from generate_scene_vectors import ObjectBuilder, cstring, integers, node_list, pw_scene, wall, floor


def scene_object(name: str, empty: bool = False, exposure: int = 0x401c000000000000) -> bytes:
    """Build only mapped concept helpers; e.g. scene_object('swCubicalScene_c')."""
    builder = ObjectBuilder(1)
    return builder.object(name, lambda state: scene_body(state, empty, exposure))


def scene_body(builder: ObjectBuilder, empty: bool, exposure: int) -> bytes:
    """Keep existing typed scene data/walls/floor bytes; e.g. scene_body(builder, False, bits)."""
    name_id = builder.next_index + 1
    raw = builder.object('moNodeName_c', lambda _: cstring('scene 鋼', True))
    raw += integers(-1, 2, -3, 4)+cstring('tip λ', True)+integers(0 if empty else -17)
    if not empty:
        raw += node_list([struct.pack('<H', name_id)], -71, True)
    raw += bytes(2)+node_list([] if empty else [struct.pack('<H', name_id), struct.pack('<H', 2)], -72, True)
    raw += builder.object('pwSceneData_c', lambda state: pw_scene(state, empty))
    raw += integers(-2 if empty else 3)
    raw += b'' if empty else b''.join(wall(kind, 0) for kind in [1, 2, 3])
    return raw+floor(name_id, empty, exposure)


def owned_scene_map() -> bytes:
    """Retain duplicate keys and root aliases; e.g. owned_scene_map()."""
    raw = struct.pack('<i', 2)+scene_object('swCubicalScene_c')+struct.pack('<iHi', -2147483648, 2, 2147483647)
    return raw+struct.pack('<iiHiHiH', 3, 7, 2, 7, 2, -1, 0)


def freeze_scene_case(reference: Path, destination: Path, stem: str, raw: bytes, mapped: bool) -> dict[str, object]:
    """Observe the separate ordinary scene policy; e.g. freeze_scene_case(source, root, stem, raw, False)."""
    objects, archive = source_module('mfcobjects', reference), source_module('swarchive', reference)
    scenes, maps = source_module('configurationscenes', reference), source_module('configurationscenemaps', reference)
    cursor = objects.ObjectArchive(raw, 19000)
    cursor.feature_context = 'ordinary_external_xml'
    scenes.install(cursor)
    with observe_primitives(archive.Archive, raw) as reads:
        parsed = maps.owned_map(cursor) if mapped else cursor.object()
    if cursor.p != len(raw):
        raise ValueError(f'{stem} cursor {cursor.p}; expected complete scene EOF {len(raw)}')
    if mapped and parsed['objects'][0]['scene'] is not parsed['associations'][1]['scene']:
        raise ValueError(f'{stem} aliases differ; expected identical loaded scene objects')
    # The reference's separate Archive preflight re-reads the PW prefix before its real decoder.
    probes = [read for read in reads if read.source_file == 'configurationscenes.py' and read.source_function == 'scene_data']
    nonempty = [read for read in reads if read.end > read.start and read not in probes]
    verify_primitive_coverage(raw, nonempty)
    rows = ''.join(f'{read.start}\t{read.end}\t{read.kind}\t{read.raw_hex}\n' for read in nonempty).encode()
    (destination / f'{stem}.bin').write_bytes(raw)
    (destination / f'{stem}-primitives.tsv').write_bytes(rows)
    return {'stem': stem, 'bytes': len(raw), 'reads': len(nonempty), 'reference_preflight_reads_excluded': len(probes),
            'sha256': hashlib.sha256(raw).hexdigest(), 'oracle_sha256': hashlib.sha256(rows).hexdigest()}


def freeze_scenes(reference: Path, destination: Path) -> dict[str, object]:
    """Require unchanged independent readers and wire builders; e.g. freeze_scenes(source, temporary_root)."""
    before = reference_snapshot(reference)
    sys.path.insert(0, str(reference))
    destination.mkdir(parents=True, exist_ok=True)
    cases = [('configuration-scene-cube', scene_object('swCubicalScene_c'), False),
             ('configuration-scene-sphere', scene_object('swSphericalScene_c'), False),
             ('configuration-scene-empty', scene_object('swCubicalScene_c', True), False),
             ('configuration-scene-edited', scene_object('swCubicalScene_c', exposure=0x4037000000000000), False),
             ('configuration-scene-map', owned_scene_map(), True)]
    rows = [freeze_scene_case(reference, destination, *case) for case in cases]
    if before != reference_snapshot(reference):
        raise ValueError('reference changed; expected stable source hashes')
    builders = ['generate_scene_vectors.py', 'generate_shaded_vectors.py', 'generate_named_vectors.py',
                'generate_transform_vectors.py', 'generate_component_vectors.py']
    return {'issue': 54, 'cases': rows, 'source_hashes': before,
            'wire_builder_hashes': {name: hashlib.sha256((WORKSPACE/'docs/features/display-lists'/name).read_bytes()).hexdigest() for name in builders},
            'generator_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}


if __name__ == '__main__':
    report = freeze_scenes(WORKSPACE.parent / 'inventor-interface/solidworks', WORKSPACE / 'tests/fixtures/resolved-features')
    Path(__file__).with_name('configuration-scenes-reference.json').write_bytes((json.dumps(report, indent=2)+'\n').encode())
    print(report['cases'])
