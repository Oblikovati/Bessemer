"""Assemble independent clause 11.9 vectors and reference semantic rows."""
from pathlib import Path
import struct
import biography
import swcontainer

ROOT = Path(__file__).resolve().parents[3]
DESTINATION = ROOT / 'tests/fixtures/biography'


def cstring(text, wide=False):
    encoded = text.encode('utf-16le' if wide else 'latin1')
    count = len(encoded) // 2 if wide else len(encoded)
    assert count < 255
    return (b'\xff\xfe\xff' if wide else b'') + bytes([count]) + encoded


def snapshot(record_index, phase):
    output = struct.pack('<QI', 0xfedcba9800000000 + record_index * 2 + phase, 0x80000000 + phase)
    for group in range(6):
        path = f'C:\\phase-{phase}\\group-{group}\\'
        if record_index > 0:
            path = '*' if group % 2 == 0 or record_index == 2 else ''
        output += cstring(path, wide=group % 2 == 0)
        output += struct.pack('<iQ', -group - record_index, 0xffffffffffffffff - phase * 6 - group - record_index)
    return output


def record(index):
    output = struct.pack('<Iii6I', 0xffffffff - index, -2147483648 + index, -1 - index,
                         0xffffffff, 0, 0x80000000, 7, 8, 9)
    output += cstring(('SP\xe9', '*', '*')[index], wide=index == 1)
    output += struct.pack('<H', 0xffff - index)
    return output + snapshot(index, 0) + snapshot(index, 1)


def archive(records, schema=0):
    name = b'moBiography_c'
    return struct.pack('<3H', 0xffff, schema, len(name)) + name + struct.pack('<i', len(records)) + b''.join(records)


def projection(records):
    scalar_names = ('file_version', 'date_code', 'unknown_10', 'processor_architecture', 'processor_count',
                    'os_platform_id', 'os_major', 'os_minor', 'os_build')
    rows = []
    for item in records:
        fields = [str(item[name]) for name in scalar_names]
        fields += [item['os_csd_version'].encode('utf-8').hex(), str(item['language_id'])]
        for snapshot in item['start_end']:
            fields += [str(snapshot['filetime']), str(snapshot['available_page_file_low32'])]
            for group in snapshot['groups']:
                fields += [group['path'].encode('utf-8').hex(), str(group['drive_type']), str(group['available_disk_bytes'])]
        rows.append(','.join(fields))
    return '|'.join(rows)


def generate():
    cases = {'empty': archive([], 65535), 'mixed': archive([record(index) for index in range(3)], 17)}
    for kind, suffix in [('part', 'PRT'), ('assembly', 'ASM'), ('drawing', 'DRW')]:
        document = swcontainer.Document(ROOT / f'tests/fixtures/blank-{kind}.SLD{suffix}')
        cases['blank-' + kind] = document.read('_MO_VERSION_19000/Biography')
    expected = []
    for name, payload in cases.items():
        (DESTINATION / (name + '.bin')).write_bytes(payload)
        records = biography.read(payload)['records']
        expected.append('\t'.join((name, projection(records), projection(biography.expanded_records(records)))))
    (DESTINATION / 'expected.tsv').write_text('\n'.join(expected) + '\n', encoding='utf-8', newline='\n')
    print(f'Generated and reference-verified {len(cases)} biography vectors')


if __name__ == '__main__':
    generate()
