"""Bug #46 concrete extension-indicator cycles/aliases and alternate help-array class."""
import hashlib
import json
from freeze_sketch_tail import freeze, WORKSPACE
from sketch_tail_fixture_bytes import WireMap, ordinary
from sketch_dimension_fixture_bytes import tagged
from freeze_views import object_tag
from freeze_blank_part import reference_snapshot
from freeze_views import REFERENCE


def generate():
    before = reference_snapshot(REFERENCE)
    records = []
    for version in [0x27cc, 19000]:
        archive_map = WireMap()
        root = archive_map.new('sgSketch', lambda: ordinary(archive_map, version, False, indicator=2))
        records.append(freeze(0, version, root))
        logical = tagged('sgLogG2Dim', version, True)
        logical = logical.replace(object_tag('su_CObArray'), object_tag('suObArray'), 1)
        records.append(freeze(0, version, logical))
    packet = b''.join(raw for raw, _ in records)
    if before != reference_snapshot(REFERENCE): raise ValueError('reference changed; expected stable source hashes')
    (WORKSPACE/'tests/fixtures/model-archives/sketch-role-regressions.bin').write_bytes(packet)
    report = {'issue': 46, 'cases': len(records), 'fields': sum(count for _, count in records),
        'sha256': hashlib.sha256(packet).hexdigest()}
    (WORKSPACE/'docs/features/model-archives/sketch-role-regressions.json').write_bytes((json.dumps(report)+'\n').encode())
    print(report)


if __name__ == '__main__': generate()
