"""Freeze independent complete detail-folder fixtures and numeric primitive partitions (#54)."""
from pathlib import Path
import hashlib
import json
import struct
import sys

WORKSPACE = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(WORKSPACE/'docs/features/model-archives'))
from freeze_blank_part import reference_snapshot, source_module
from primitive_inventory import PrimitiveRead, observe_primitives, verify_primitive_coverage
sys.path.insert(0, str(WORKSPACE/'docs/features/display-lists'))
from bucket_vector_wire import bucket
from generate_shaded_vectors import ObjectBuilder
from generate_item_vectors import payloads, record


def feature_body(state: ObjectBuilder, name: str | None) -> bytes:
    """Use the verified common feature without a model-feature word; e.g. feature_body(state, None)."""
    if name is None:
        return bytes(139)
    encoded = name.encode('utf-16-le')
    units = len(encoded)//2
    if units >= 255:
        raise ValueError(f'name length {units}; expected fewer than 255 UTF-16 units in this fixture')
    return state.object('moNodeName_c', lambda _: b'\xff\xfe\xff'+bytes([units])+encoded)+bytes(137)


def folder_body(state: ObjectBuilder, mode: str, name: str | None) -> bytes:
    """Keep the root's reserved identity across direct lists; e.g. folder_body(state, 'absent', None)."""
    identity = state.next_index-1
    raw = feature_body(state, name)+struct.pack('<HI2H', 65535, 2, identity, identity)+bytes(34)
    raw += b'\0'+struct.pack('<4dB', 1, 2, 3, 1, 0)+bytes(2)+bytes(28)
    raw += struct.pack('<i', 0 if mode == 'absent' else -7)
    if mode != 'absent':
        raw += bucket(state, identity, mode in ('bucket-2d', 'bucket-populated'), mode != 'bucket-populated')
    return raw+struct.pack('<3Hi', 0, 1, identity, -71)


def numeric_fragments(reads: list[PrimitiveRead]) -> list[PrimitiveRead]:
    """Expand only the reference polygon coordinate reads; e.g. numeric_fragments(observations)."""
    result: list[PrimitiveRead] = []
    for read in reads:
        coordinates = read.kind == 'raw' and read.source_file == 'dl_ui.py' and read.source_function == 'read_gcBucketPoly_c'
        if not coordinates:
            result.append(read)
            continue
        if (read.end-read.start) % 8:
            raise ValueError(f'coordinate span {read.start}:{read.end}; expected complete f64 elements at version 19000')
        for start in range(read.start, read.end, 8):
            offset = (start-read.start)*2
            result.append(PrimitiveRead(start, start+8, 'd', read.raw_hex[offset:offset+16], read.source_file, read.source_function, read.source_line))
    return result


def freeze_case(reference: Path, destination: Path, label: str, mode: str, name: str | None) -> dict[str, object]:
    """Bind a complete object and independent reads; e.g. freeze_case(source, output, 'absent', 'absent', None)."""
    objects, archive = source_module('mfcobjects', reference), source_module('swarchive', reference)
    folders = source_module('detailfolder', reference)
    wire = ObjectBuilder(1).object('moDetailFolder_c', lambda state: folder_body(state, mode, name))
    cursor = objects.ObjectArchive(wire, 19000)
    cursor.feature_context = 'ordinary_external_xml'
    cursor.readers = dict(cursor.readers, moDetailFolder_c=folders.read)
    with observe_primitives(archive.Archive, wire) as observed:
        cursor.object()
    if cursor.p != len(wire):
        raise ValueError(f'{label} end {cursor.p}; expected EOF {len(wire)}')
    reads = numeric_fragments([read for read in observed if read.end > read.start])
    verify_primitive_coverage(wire, reads)
    oracle = ''.join(f'{read.start}\t{read.end}\t{read.kind}\t{read.raw_hex}\n' for read in reads).encode()
    (destination/(label+'.bin')).write_bytes(wire)
    (destination/(label+'-primitives.tsv')).write_bytes(oracle)
    return dict(name=label, bytes=len(wire), reads=len(reads), sha256=hashlib.sha256(wire).hexdigest(), oracle_sha256=hashlib.sha256(oracle).hexdigest())


def freeze(reference: Path, destination: Path) -> dict[str, object]:
    """Reject source drift across all cases; e.g. freeze(source, fixture_directory)."""
    before = reference_snapshot(reference)
    sys.path.insert(0, str(reference))
    cases = [(name, name, None) for name in ('absent', 'bucket-3d', 'bucket-2d', 'bucket-populated')]
    cases += [('named', 'absent', 'Detail Ω'), ('named-edited', 'absent', 'Detail Changed Ω')]
    rows = [freeze_case(reference, destination, *case) for case in cases]
    rows.append(freeze_items(reference, destination))
    if before != reference_snapshot(reference):
        raise ValueError('reference changed during fixture generation; expected stable source hashes')
    builders = WORKSPACE/'docs/features/display-lists'
    return dict(scope='Independent complete object framing and declared numeric arrays; not native geometry acceptance.', source_hashes=before,
                builder_hashes={path.name: hashlib.sha256(path.read_bytes()).hexdigest() for path in sorted(builders.glob('*.py'))}, cases=rows)


def freeze_items(reference: Path, destination: Path) -> dict[str, object]:
    """Observe complete direct items across all payload families; e.g. freeze_items(source, output)."""
    objects, archive, ui = (source_module(name, reference) for name in ('mfcobjects', 'swarchive', 'dl_ui'))
    variants = payloads(True)
    wire = struct.pack('<H', len(variants))+b''.join(record(kind, value) for kind, value in variants.items())
    cursor = objects.ObjectArchive(wire, 19000)
    with observe_primitives(archive.Archive, wire) as reads:
        ui.read_suDynList_uoDisplayItem_c(cursor)
    if cursor.p != len(wire):
        raise ValueError(f'items end {cursor.p}; expected EOF {len(wire)}')
    reads = [read for read in reads if read.end > read.start]
    verify_primitive_coverage(wire, reads)
    oracle = ''.join(f'{read.start}\t{read.end}\t{read.kind}\t{read.raw_hex}\n' for read in reads).encode()
    (destination/'items.bin').write_bytes(wire)
    (destination/'items-primitives.tsv').write_bytes(oracle)
    return dict(name='items', bytes=len(wire), variants=len(variants), reads=len(reads), sha256=hashlib.sha256(wire).hexdigest(), oracle_sha256=hashlib.sha256(oracle).hexdigest())


if __name__ == '__main__':
    source = WORKSPACE.parent/'inventor-interface/solidworks'
    report = freeze(source, WORKSPACE/'tests/fixtures/detail-folders')
    (Path(__file__).parent/'reference.json').write_text(json.dumps(report, indent=2)+'\n', encoding='utf-8')
