"""Retain complete native assembly records and actual reference primitive inventories (#99)."""
from pathlib import Path
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
from verify_reference_corpus import reference_cases
from freeze_owned_assembly import freeze_bytes

SELECTED = (0, 7, 49, 58, 71, 136, 137, 156, 174)


def native_record(index: int, kind: str, row: dict[str, object]) -> tuple[bytes, dict[str, object]]:
    """Keep the real root, header, shared map and complete stream without reader replacements."""
    from swcontainer import Document
    from swarchive import Archive
    import modelprefix
    document = Document(Path(row['path']), table={})
    raw = document.read(row['stream'])
    assert hashlib.sha256(raw).hexdigest() == row['stream_sha256']
    with observe_primitives(Archive, raw) as reads:
        root, cursor = modelprefix.read(document, row['stream'], root_class=kind, assembly_resolved_stage='assembly_tail')
    assert cursor.p == len(raw) and root['_class'] == kind
    verify_primitive_coverage(raw, reads)
    header = document.read(row['stream'] + '-ModelHeader')
    oracle = ''.join(f'{r.start}\t{r.end}\t{r.kind}\t{r.raw_hex}\t{r.source_file}:{r.source_function}:{r.source_line}\n' for r in reads).encode()
    packet = struct.pack('<IBIII', cursor.version, kind == 'moRouteAssembly_c', len(header), len(raw), len(cursor.map))
    packet += header + raw + struct.pack('<I', len(oracle)) + oracle
    facts = dict(index=index, bytes=len(raw), version=cursor.version, root_class=kind, map_slots=len(cursor.map),
                 primitive_reads=len(reads), stream_sha256=row['stream_sha256'], header_sha256=hashlib.sha256(header).hexdigest(),
                 fresh_classes=dict(sorted(cursor.classes.items())))
    return packet, facts


def freeze_native() -> None:
    """Cover references, patterns, cameras, exploded profiles, relations, toolbox and routing."""
    records = reference_cases()
    packets, facts = [], []
    for index in SELECTED:
        kind, row = records[index]
        packet, record = native_record(index, kind, row)
        packets.append(packet)
        facts.append(record)
    raw = b'AFIX' + struct.pack('<I', len(packets)) + b''.join(packets)
    freeze_bytes(ROOT / 'tests/fixtures/assembly-configurations/native-records.bin', raw)
    freeze_bytes(FEATURE / 'native-record-reference.json', (json.dumps(facts, indent=2) + '\n').encode())


def main() -> None:
    """Require immutable serializers and record all imported module origins."""
    with tempfile.TemporaryDirectory(prefix='bessemer-native-assembly-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin))
        result = unittest.TextTestRunner(verbosity=2).run(unittest.FunctionTestCase(freeze_native))
        assert result.wasSuccessful() and result.testsRun == 1 and not result.skipped
        record_snapshot(pin, hashes, result, FEATURE,
                        ('assemblyreferences', 'assemblyresolvedtail', 'assemblyprofiles', 'camerafeatures'),
                        ('swcontainer', 'modelprefix', 'mfcobjects'))
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'native-record-source-snapshot.json')


if __name__ == '__main__':
    main()
