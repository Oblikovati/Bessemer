"""Freeze independently specified per-body chooser graphs and scalar records (#54)."""
from pathlib import Path
from types import ModuleType
import hashlib
import json
import struct
import sys
from freeze_manager_context import WORKSPACE, reference_snapshot, source_module
from primitive_inventory import observe_primitives, verify_primitive_coverage


def chooser_body(tail: int = -7) -> bytes:
    """Build direct lists with self aliases; e.g. chooser_body()."""
    arrays = struct.pack('<3HHIHH', 2, 2, 0, 0xffff, 1, 2, 0)
    return arrays+struct.pack('<iiHI2Ii', -2147483648, 2147483647, 0xffff,
                              2, 0xffffffff, 0x80000000, tail)


def tagged_chooser(name: str, tail: int = -7) -> bytes:
    """Bind an explicit runtime class; e.g. tagged_chooser('moPerBodyChooserData_c')."""
    encoded = name.encode('ascii')
    prefix = struct.pack('<3H', 0xffff, 1, len(encoded))+encoded
    if name == 'moBBoxCenterData_c':
        return prefix+struct.pack('<i4d', -2147483648, -0.0, 1.25, -2.5, 7.0)
    suffix = b''
    if name == 'moPerBodyChooserDataWithFileName_c':
        value = '部品 Ω'.encode('utf-16le')
        suffix = b'\xff\xfe\xff'+bytes([len(value)//2])+value+struct.pack('<Hi', 0, -9)
    return prefix+chooser_body(tail)+suffix


def freeze_choosers(reference: Path, destination: Path) -> dict[str, object]:
    """Require complete reference object reads; e.g. freeze_choosers(source, temporary_root)."""
    before = reference_snapshot(reference)
    sys.path.insert(0, str(reference))
    objects, archive = source_module('mfcobjects', reference), source_module('swarchive', reference)
    bodies = source_module('bodyfeatures', reference)
    cases = [('body-chooser', 'moPerBodyChooserData_c'),
             ('named-body-chooser', 'moPerBodyChooserDataWithFileName_c'),
             ('body-bounding-center', 'moBBoxCenterData_c')]
    rows = [freeze_case(objects, archive, bodies, name, tagged_chooser(name), destination, stem) for stem, name in cases]
    edited = tagged_chooser('moPerBodyChooserData_c', 2147483647)
    freeze_case(objects, archive, bodies, cases[0][1], edited, destination, 'body-chooser-edited')
    if before != reference_snapshot(reference):
        raise ValueError('reference changed; expected stable source hashes')
    return {'issue': 54, 'cases': rows, 'source_hashes': before,
            'generator_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}


def freeze_case(objects: ModuleType, archive: ModuleType, bodies: ModuleType, name: str, raw: bytes,
                destination: Path, stem: str) -> dict[str, object]:
    """Observe a complete class and graph; e.g. freeze_case(modules, name, raw, root, stem)."""
    cursor = objects.ObjectArchive(raw, 19000)
    cursor.feature_context = 'ordinary_external_xml'
    cursor.readers = {'moPerBodyChooserData_c': bodies.per_body_chooser_data,
        'moPerBodyChooserDataWithFileName_c': bodies.per_body_chooser_data_with_file_name,
        'moBBoxCenterData_c': bodies.bbox_center_data}
    with observe_primitives(archive.Archive, raw) as reads:
        parsed = cursor.object()
    if cursor.p != len(raw) or parsed['_class'] != name:
        raise ValueError(f'{name} at {cursor.p}; expected complete {len(raw)}-byte object')
    if name != 'moBBoxCenterData_c' and (parsed['array_08'][0] is not parsed or parsed['array_28'][0] is not parsed):
        raise ValueError(f'{name} identity mismatch; expected both direct arrays to alias the root')
    nonempty = [read for read in reads if read.end > read.start]
    verify_primitive_coverage(raw, nonempty)
    oracle = ''.join(f'{read.start}\t{read.end}\t{read.kind}\t{read.raw_hex}\n' for read in nonempty).encode()
    destination.mkdir(parents=True, exist_ok=True)
    (destination / f'{stem}.bin').write_bytes(raw)
    (destination / f'{stem}-primitives.tsv').write_bytes(oracle)
    return {'class': name, 'stem': stem, 'bytes': len(raw), 'primitive_reads': len(nonempty),
            'raw_sha256': hashlib.sha256(raw).hexdigest(), 'oracle_sha256': hashlib.sha256(oracle).hexdigest()}


if __name__ == '__main__':
    report = freeze_choosers(WORKSPACE.parent / 'inventor-interface/solidworks',
                            WORKSPACE / 'tests/fixtures/resolved-features')
    Path(__file__).with_name('body-choosers-reference.json').write_bytes((json.dumps(report, indent=2)+'\n').encode())
    print(report['cases'])
