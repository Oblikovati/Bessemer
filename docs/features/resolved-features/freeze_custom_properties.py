"""Freeze independently specified complete custom-property records (#54)."""
from pathlib import Path
import hashlib
import json
import struct
import sys
from freeze_manager_context import WORKSPACE, reference_snapshot, source_module
from primitive_inventory import PrimitiveRead, observe_primitives, verify_primitive_coverage


def short_unicode(value: str) -> bytes:
    """Encode this fixture's short CString; e.g. short_unicode('材料')."""
    encoded = value.encode('utf-16le')
    units = len(encoded)//2
    if units >= 255:
        raise ValueError(f'CString length {units}; expected fewer than 255 fixture code units')
    return b'\xff\xfe\xff'+bytes([units])+encoded


def property_record(value: str, expression: str = '材料') -> bytes:
    """Construct one complete direct property without an outer tag; e.g. property_record('独立 Ω')."""
    first = b''.join(short_unicode(text) for text in ['材料', '', expression])
    first += struct.pack('<iI', -2147483648, 0xffffffff)+short_unicode(value)
    return first+struct.pack('<3i', -1, 0, 2147483647)


def property_bytes(value: str) -> bytes:
    """Preserve duplicates and raw extremes; e.g. property_bytes('独立 Ω')."""
    first = property_record(value)
    second = b''.join(short_unicode(text) for text in ['材料', 'same', ''])
    second += struct.pack('<iI', 2147483647, 0x80000000)+short_unicode('unchanged')
    return struct.pack('<i', 2)+first+second+struct.pack('<3i', 7, -8, 9)


def freeze_properties(reference: Path, destination: Path) -> dict[str, object]:
    """Require a full independent parse; e.g. freeze_properties(source, temporary_root)."""
    before = reference_snapshot(reference)
    sys.path.insert(0, str(reference))
    objects = source_module('mfcobjects', reference)
    archive = source_module('swarchive', reference)
    weldments = source_module('weldments', reference)
    raw, edited = property_bytes('original'), property_bytes('独立 Ω')
    cursor = objects.ObjectArchive(raw, 19000)
    cursor.feature_context = 'ordinary_external_xml'
    with observe_primitives(archive.Archive, raw) as reads:
        parsed = weldments.property_list(cursor)
    if cursor.p != len(raw) or parsed['properties'][0]['string_20'] != 'original':
        raise ValueError(f'property list at {cursor.p}; expected full original record and EOF {len(raw)}')
    verify_primitive_coverage(raw, reads)
    check = objects.ObjectArchive(edited, 19000)
    check.feature_context = 'ordinary_external_xml'
    if weldments.property_list(check)['properties'][0]['string_20'] != '独立 Ω' or check.p != len(edited):
        raise ValueError('edited property mismatch; expected independent UTF-16 replacement and EOF')
    if reference_snapshot(reference) != before:
        raise ValueError('reference changed; expected unchanged source hashes')
    return write_properties(destination, raw, edited, reads, before)


def write_properties(destination: Path, raw: bytes, edited: bytes, reads: list[PrimitiveRead],
                     sources: dict[str, str]) -> dict[str, object]:
    """Persist owned expected bytes; e.g. write_properties(root, raw, edited, reads, hashes)."""
    rows = ''.join(f'{read.start}\t{read.end}\t{read.kind}\t{read.raw_hex}\n' for read in reads if read.end > read.start).encode()
    destination.mkdir(parents=True, exist_ok=True)
    for name, payload in [('custom-properties.bin', raw), ('custom-properties-edited.bin', edited),
                          ('custom-properties-primitives.tsv', rows)]:
        (destination / name).write_bytes(payload)
    return {'issue': 54, 'bytes': len(raw), 'primitive_reads': len(rows.splitlines()),
            'raw_sha256': hashlib.sha256(raw).hexdigest(), 'edited_sha256': hashlib.sha256(edited).hexdigest(),
            'oracle_sha256': hashlib.sha256(rows).hexdigest(), 'source_hashes': sources,
            'generator_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}


if __name__ == '__main__':
    report = freeze_properties(WORKSPACE.parent / 'inventor-interface/solidworks',
                               WORKSPACE / 'tests/fixtures/resolved-features')
    Path(__file__).with_name('custom-properties-reference.json').write_bytes((json.dumps(report, indent=2)+'\n').encode())
    print({key: value for key, value in report.items() if key != 'source_hashes'})
