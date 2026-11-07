"""Bind all five reference-accepted embedded kernel frames and field-value oracles."""
from pathlib import Path
import hashlib
import json
import struct
from freeze_views import REFERENCE, WORKSPACE
from freeze_blank_part import reference_snapshot
import kernelarchive
import xtbin
from kernel_field_oracle import neutral_field_values
from mfcobjects import ObjectArchive
from swcontainer import Document


def node_rows(parsed: dict[str, object]) -> bytes:
    rows: list[str] = []
    for index, (code, identity, values, _) in enumerate(parsed['nodes']):
        prefix = f'records[{index}]'
        rows += [f'{prefix}.type=i:{code}', f'{prefix}.index=i:{identity}']
        fields = parsed['layouts'][code][1]
        if len({field.name for field in fields}) != len(fields):
            raise ValueError(f'type {code}: duplicate field names; expected an unambiguous independent value oracle')
        for ordinal, field in enumerate(fields):
            label = f'{prefix}.fields[{ordinal}]'
            rows += [f'{label}.name=t:{field.name.encode("latin1").hex()}', f'{label}.kind=t:{field.type}']
            rows += [f'{label}.values[{slot}]={value}' for slot, value in enumerate(neutral_field_values(field.type, values[field.name]))]
    return ('\n'.join(rows)+'\n').encode()


def freeze(row: dict[str, object], version: int) -> tuple[bytes, dict[str, object]]:
    document = Document(Path(row['path']), table={})
    raw = document.read(row['stream'])[row['start']:row['end']]
    digest = hashlib.sha256(raw[6:]).hexdigest()
    if row['format'] != 'framed_compressed_parasolid' or digest != row['sha256']:
        raise ValueError(f'kernel {row["path"]}: {row["format"]}, {digest}; expected the frozen framed payload {row["sha256"]}')
    archive = ObjectArchive(raw, version)
    archive.feature_context = 'ordinary_external_xml'
    parsed = kernelarchive.body(archive)['decoded']['parasolid']
    if archive.p != len(raw): raise ValueError(f'kernel end {archive.p}; expected {len(raw)}')
    oracle = node_rows(parsed)
    packet = struct.pack('<III', len(raw), row['node_count'], len(oracle))+raw+oracle
    return packet, {'bytes': len(raw), 'records': row['node_count'], 'fields': len(oracle.splitlines()), 'sha256': hashlib.sha256(raw).hexdigest()}


def prepare() -> None:
    before = reference_snapshot(REFERENCE)
    source = json.loads((WORKSPACE/'target/current-part-model-reference-corpus.json').read_text())
    versions = {(row['path'], row['stream']): row['version'] for row in source['after_details']}
    frames = [freeze(row, versions[(row['path'], row['stream'])]) for row in source['kernel_frames']]
    if len(frames) != 5 or before != reference_snapshot(REFERENCE):
        raise ValueError(f'kernel frames {len(frames)}; expected five under unchanged reference sources')
    packet = b'BKR1'+struct.pack('<I', len(frames))+b''.join(raw for raw, _ in frames)
    (WORKSPACE/'target/model-native-kernel-frames.bin').write_bytes(packet)
    schema = Path(xtbin.PSCHEMA)/'sch_13006.s_t'
    report = {'frames': [record for _, record in frames], 'packet_sha256': hashlib.sha256(packet).hexdigest(),
              'schema_sha256': hashlib.sha256(schema.read_bytes()).hexdigest(), 'source_hashes': before,
              'generator_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    Path(__file__).with_name('native-kernel-frame-reference.json').write_text(json.dumps(report)+'\n')
    print({'frames': len(frames), 'records': sum(row['records'] for _, row in frames), 'fields': sum(row['fields'] for _, row in frames), 'sha256': report['packet_sha256']})


if __name__ == '__main__': prepare()
