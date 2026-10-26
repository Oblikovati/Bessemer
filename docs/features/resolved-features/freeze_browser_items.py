"""Bind final browser-item and section-folder layouts to owned primitives (#54)."""
from pathlib import Path
from types import ModuleType
import hashlib
import json
import struct
import sys
from freeze_manager_context import WORKSPACE, reference_snapshot, source_module
from primitive_inventory import observe_primitives, verify_primitive_coverage

CLASSES = ['moFavoriteItemData_c', 'moHistoryItemData_c', 'moFavoriteFeatItemData_c',
           'moHistoryFeatItemData_c', 'moLiveSectionFolder_c', 'moPlaneSectionFolder_c',
           'moConfigCommentsFolder_c']


def browser_object(name: str) -> bytes:
    """Build an independently specified final serializer; e.g. browser_object(CLASSES[0])."""
    if name not in CLASSES:
        raise ValueError(f'browser class {name!r}; expected one of {CLASSES!r}')
    encoded = name.encode('ascii')
    if name in CLASSES[:4]:
        body = struct.pack('<H i I i I B', 0, -7, 0xffffffff, -2147483648, 0x80000000, 0)
        return struct.pack('<3H', 0xffff, 1, len(encoded))+encoded+body+(bytes(2) if name in CLASSES[2:4] else b'')
    body = bytes(139)+struct.pack('<ii', -7, -2147483648)
    if name in CLASSES[4:6]:
        body += struct.pack('<i', 2147483647)
    return struct.pack('<3H', 0xffff, 1, len(encoded))+encoded+body


def freeze_browser_items(reference: Path, destination: Path) -> dict[str, object]:
    """Observe every final object through EOF; e.g. freeze_browser_items(source, temporary_root)."""
    before = reference_snapshot(reference)
    sys.path.insert(0, str(reference))
    source_module('modelfeatures', reference)
    objects, archive = source_module('mfcobjects', reference), source_module('swarchive', reference)
    destination.mkdir(parents=True, exist_ok=True)
    records = [freeze_browser_case(objects, archive, name, destination) for name in CLASSES]
    if before != reference_snapshot(reference):
        raise ValueError('reference changed; expected stable source hashes')
    return {'issue': 54, 'records': records, 'source_hashes': before,
            'generator_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}


def freeze_browser_case(objects: ModuleType, archive: ModuleType, name: str,
                        destination: Path) -> dict[str, object]:
    """Bind one final runtime table; e.g. freeze_browser_case(objects, archive, CLASSES[0], root)."""
    raw = browser_object(name)
    cursor = objects.ObjectArchive(raw, 19000)
    cursor.feature_context = 'ordinary_external_xml'
    with observe_primitives(archive.Archive, raw) as reads:
        parsed = cursor.object()
    if cursor.p != len(raw) or parsed['_class'] != name:
        raise ValueError(f'{name} at {cursor.p}; expected complete object ending at {len(raw)}')
    nonempty = [read for read in reads if read.end > read.start]
    verify_primitive_coverage(raw, nonempty)
    rows = ''.join(f'{read.start}\t{read.end}\t{read.kind}\t{read.raw_hex}\n' for read in nonempty).encode()
    (destination / f'{name}.bin').write_bytes(raw)
    (destination / f'{name}-primitives.tsv').write_bytes(rows)
    return {'class': name, 'bytes': len(raw), 'reads': len(nonempty),
            'sha256': hashlib.sha256(raw).hexdigest(), 'oracle_sha256': hashlib.sha256(rows).hexdigest()}


if __name__ == '__main__':
    report = freeze_browser_items(WORKSPACE.parent / 'inventor-interface/solidworks',
                                 WORKSPACE / 'tests/fixtures/resolved-features')
    Path(__file__).with_name('browser-items-reference.json').write_bytes((json.dumps(report, indent=2)+'\n').encode())
    print(report['records'])
