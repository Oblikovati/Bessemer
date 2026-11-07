"""Bind the reference-accepted native corpus to explicit Rust acceptance inputs."""
from pathlib import Path
import hashlib
import json
import struct
import sys
from freeze_blank_part import reference_snapshot

ROOT = Path(__file__).resolve().parents[3]
REFERENCE = ROOT.parent/'inventor-interface/solidworks'
sys.path.insert(0, str(REFERENCE))
from swcontainer import Document


def record(case: dict[str, object], index: int) -> tuple[bytes, dict[str, object]]:
    path, name = Path(case['path']), case['stream']
    document = Document(path, table={})
    stream = document.read(name)
    digest = hashlib.sha256(stream).hexdigest()
    if digest != case['stream_sha256']:
        raise ValueError(f'{path}: stream {name} hash {digest}; expected {case["stream_sha256"]}')
    header = document.read(name+'-ModelHeader')
    label = f'{index}: {path}::{name}'.encode('utf-8')
    prefix = struct.pack('<4I', len(label), case['version'], len(header), len(stream))
    manifest = {'index': index, 'path': str(path), 'stream': name,
                'version': case['version'], 'stream_sha256': digest,
                'header_sha256': hashlib.sha256(header).hexdigest()}
    return prefix+label+header+stream, manifest


def prepare() -> None:
    before = reference_snapshot(REFERENCE)
    reference = ROOT/'target/current-part-model-reference-corpus.json'
    cases = json.loads(reference.read_text())['after_details']
    if len(cases) != 906:
        raise ValueError(f'case count {len(cases)}; expected the frozen 906-case corpus')
    records = [record(case, index) for index, case in enumerate(cases)]
    if before != reference_snapshot(REFERENCE):
        raise ValueError('reference changed; expected stable container-reader sources')
    packet = b'BMC1'+struct.pack('<I', len(records))+b''.join(raw for raw, _ in records)
    (ROOT/'target/model-native-corpus.bin').write_bytes(packet)
    report = {'cases': len(records), 'bytes': len(packet),
              'sha256': hashlib.sha256(packet).hexdigest(),
              'reference_report_sha256': hashlib.sha256(reference.read_bytes()).hexdigest(),
              'source_hashes': before, 'records': [row for _, row in records]}
    (ROOT/'target/model-native-corpus-manifest.json').write_text(json.dumps(report)+'\n')
    print({key: value for key, value in report.items() if key not in ('source_hashes', 'records')})


if __name__ == '__main__':
    prepare()
