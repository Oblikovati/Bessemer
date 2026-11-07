"""Freeze complete independently specified sensor records and precision gates (#54)."""
from pathlib import Path
import hashlib
import json
import struct
import sys
from freeze_manager_context import WORKSPACE, reference_snapshot, source_module
from primitive_inventory import observe_primitives, verify_primitive_coverage

VERSIONS = [0x27cc, 0x27e3, 0x27e4, 0x3764, 0x3765, 0x3b15, 0x3b16, 19000]


def observe_sensor(reference: Path, raw: bytes, version: int, mode: str) -> tuple[bytes, int]:
    """Require complete independent reads; e.g. observe_sensor(source, raw, 19000, 'data')."""
    objects, archive = source_module('mfcobjects', reference), source_module('swarchive', reference)
    sensors = source_module('sensors', reference)
    cursor = objects.ObjectArchive(raw, version)
    cursor.feature_context = 'ordinary_external_xml'
    cursor.readers = dict(cursor.readers, moSensor_c=sensors.read)
    if mode == 'payload':
        cursor.map.extend([{'_class': 'moRefWrapper_c'}, {'_class': 'moDispObjRef_c'}])
    with observe_primitives(archive.Archive, raw) as reads:
        parsed = cursor.object() if mode == 'object' else getattr(sensors, mode)(cursor)
    if cursor.p != len(raw):
        raise ValueError(f'sensor {mode} at {cursor.p}; expected complete EOF {len(raw)}')
    nonempty = [read for read in reads if read.end > read.start]
    verify_primitive_coverage(raw, nonempty)
    rows = ''.join(f'{read.start}\t{read.end}\t{read.kind}\t{read.raw_hex}\n' for read in nonempty).encode()
    if mode == 'payload' and parsed['references_570'][0] is not parsed['references_570'][1]:
        raise ValueError('sensor vector aliases differ; expected one shared wrapper')
    return rows, len(nonempty)


def sensor_object(raw: bytes) -> bytes:
    """Use the independently pinned ordinary model-feature footprint; e.g. sensor_object(data)."""
    name = b'moSensor_c'
    return struct.pack('<3H', 0xffff, 1, len(name))+name+bytes(143)+raw+bytes(12)


def freeze_sensors(reference: Path, destination: Path) -> dict[str, object]:
    """Bind precision gates and repeated members; e.g. freeze_sensors(source, temporary_root)."""
    before = reference_snapshot(reference)
    sys.path.insert(0, str(reference))
    fixture = source_module('test_sensors', reference)
    destination.mkdir(parents=True, exist_ok=True)
    records = []
    for version in VERSIONS:
        for kind, count in [(2, 2), (0x47, 2), (0x47, 0), (0x47, -1)]:
            raw = fixture.data_wire(version, kind, count)
            rows, reads = observe_sensor(reference, raw, version, 'data')
            records.append(struct.pack('<3I', version, len(raw), len(rows))+raw+rows)
    packet = struct.pack('<I', len(records))+b''.join(records)
    (destination/'sensor-data-cases.bin').write_bytes(packet)
    objects = write_sensor_objects(reference, destination, fixture.data_wire(19000, 0x47))
    if before != reference_snapshot(reference):
        raise ValueError('reference changed; expected stable independent reader/fixture sources')
    return {'issue': 54, 'data_cases': len(records), 'data_packet_sha256': hashlib.sha256(packet).hexdigest(),
            'objects': objects, 'source_hashes': before, 'generator_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}


def write_sensor_objects(reference: Path, destination: Path, raw: bytes) -> list[dict[str, object]]:
    """Pin an independent repeated-member edit; e.g. write_sensor_objects(source, root, raw)."""
    repeated = struct.pack('<i', 202)
    if raw.count(repeated) != 1:
        raise ValueError(f'sensor repeated 202 matches {raw.count(repeated)}; expected exactly one')
    edited = raw.replace(repeated, struct.pack('<i', -202), 1)
    cases = [('sensor-object', sensor_object(raw), 'object'),
             ('sensor-object-edited', sensor_object(edited), 'object'),
             ('sensor-payload', raw+struct.pack('<II2HIH', 0, 2, 1, 1, 1, 2), 'payload')]
    report = []
    for stem, wire, mode in cases:
        rows, reads = observe_sensor(reference, wire, 19000, mode)
        (destination/f'{stem}.bin').write_bytes(wire)
        (destination/f'{stem}-primitives.tsv').write_bytes(rows)
        report.append({'stem': stem, 'bytes': len(wire), 'reads': reads,
                       'sha256': hashlib.sha256(wire).hexdigest(), 'oracle_sha256': hashlib.sha256(rows).hexdigest()})
    return report


if __name__ == '__main__':
    report = freeze_sensors(WORKSPACE.parent/'inventor-interface/solidworks', WORKSPACE/'tests/fixtures/resolved-features')
    Path(__file__).with_name('sensors-reference.json').write_bytes((json.dumps(report, indent=2)+'\n').encode())
    print({'data_cases': report['data_cases'], 'objects': report['objects']})
