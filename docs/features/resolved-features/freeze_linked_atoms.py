"""Bind independent recursive configuration-map facts to owned bytes (#54)."""
from pathlib import Path
import hashlib
import json
import struct
import sys

from freeze_manager_context import WORKSPACE, reference_snapshot, source_module
from primitive_inventory import PrimitiveRead, observe_primitives, verify_primitive_coverage


def recursive_atom_bytes() -> bytes:
    """Build a specified graph independently; e.g. recursive_atom_bytes()."""
    name = b'moLinkedAtomIdNode_c'
    empty = struct.pack('<i2H4I', 9, 0, 0, 0, 0, 0, 0)
    body = struct.pack('<iHI2IH', -7, 0xffff, 2, 0xffffffff, 0x80000000, 0)
    body += struct.pack('<IiHiH', 2, -2147483648, 2, 7, 0x8001) + empty
    body += struct.pack('<IiHIiHIiH', 1, 2147483647, 3, 1, 0, 0, 1, -1, 2)
    first = struct.pack('<3H', 0xffff, 1, len(name)) + name + body
    return struct.pack('<Ii', 3, -2147483648) + first + struct.pack('<iHiH', -2147483648, 2, 2147483647, 3)


def freeze_atoms(reference: Path, destination: Path) -> dict[str, object]:
    """Require full independent framing; e.g. freeze_atoms(source, temporary_root)."""
    before = reference_snapshot(reference)
    sys.path.insert(0, str(reference))
    objects = source_module('mfcobjects', reference)
    archive = source_module('swarchive', reference)
    configurations = source_module('partconfigurations', reference)
    raw = recursive_atom_bytes()
    cursor = objects.ObjectArchive(raw, 19000)
    cursor.feature_context = 'ordinary_external_xml'
    cursor.readers = {'moLinkedAtomIdNode_c': configurations.linked_atom}
    with observe_primitives(archive.Archive, raw) as reads:
        graph = configurations.linked_atom_map(cursor)
    require_graph(graph, cursor.p, len(raw))
    nonempty = [read for read in reads if read.end > read.start]
    verify_primitive_coverage(raw, nonempty)
    if reference_snapshot(reference) != before:
        raise ValueError('reference changed; expected unchanged source hashes')
    return write_atoms(destination, raw, nonempty, before)


def require_graph(graph: dict[str, object], position: int, size: int) -> None:
    """Verify graph identities, not just bytes; e.g. require_graph(graph, end, size)."""
    entries = graph['entries']
    first, second = entries[0]['object'], entries[2]['object']
    valid = (position == size and len(entries) == 3 and first is entries[1]['object']
             and first['map_f0']['entries'][0]['object'] is first
             and first['map_f0']['entries'][1]['object'] is second
             and first['map_100']['entries'][0]['object'] is second
             and first['map_110']['entries'][0]['object'] is None
             and first['map_120']['entries'][0]['object'] is first
             and first['dwords_90']['items'] == [4294967295, 2147483648])
    if not valid:
        raise ValueError(f'linked graph at {position}; expected two aliased nodes and EOF {size}')


def write_atoms(destination: Path, raw: bytes, reads: list[PrimitiveRead], sources: dict[str, str]) -> dict[str, object]:
    """Freeze a byte partition; e.g. write_atoms(root, raw, observed_reads, hashes)."""
    rows = ''.join(f'{read.start}\t{read.end}\t{read.kind}\t{read.raw_hex}\n' for read in reads).encode()
    destination.mkdir(parents=True, exist_ok=True)
    (destination / 'linked-atoms.bin').write_bytes(raw)
    (destination / 'linked-atoms-primitives.tsv').write_bytes(rows)
    return {'issue': 54, 'bytes': len(raw), 'primitive_reads': len(reads),
            'raw_sha256': hashlib.sha256(raw).hexdigest(),
            'oracle_sha256': hashlib.sha256(rows).hexdigest(), 'source_hashes': sources,
            'generator_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}


if __name__ == '__main__':
    report = freeze_atoms(WORKSPACE.parent / 'inventor-interface/solidworks',
                          WORKSPACE / 'tests/fixtures/resolved-features')
    Path(__file__).with_name('linked-atoms-reference.json').write_bytes((json.dumps(report, indent=2)+'\n').encode())
    print({key: value for key, value in report.items() if key != 'source_hashes'})
