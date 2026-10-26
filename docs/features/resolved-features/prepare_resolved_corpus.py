"""Bind complete context and features for the explicit 906-case parity gate (#54)."""
from pathlib import Path
from typing import cast
import hashlib
import json
import struct
import sys
from freeze_manager_context import WORKSPACE, reference_snapshot, source_module


def bind_case(reference: Path, case: dict[str, object], index: int) -> tuple[bytes, dict[str, object]]:
    """Check the model input hash; e.g. bind_case(source, manifest_record, 0)."""
    sys.path.insert(0, str(reference))
    container = source_module('swcontainer', reference)
    path, name = Path(cast(str, case['path'])), cast(str, case['stream'])
    document = container.Document(path, table={})
    parts = [f'{index}: {path}::{name}'.encode(), document.read(name+'-ModelHeader'),
             document.read(name), document.read('Contents/CMgr'), document.read(name+'-ResolvedFeatures')]
    digest = hashlib.sha256(parts[2]).hexdigest()
    if digest != case['stream_sha256']:
        raise ValueError(f'{path}::{name} hash {digest}; expected {case["stream_sha256"]}')
    prefix = struct.pack('<6I', cast(int, case['version']), *(len(part) for part in parts))
    row = {'index': index, 'path': str(path), 'configuration': name,
           'sha256': [hashlib.sha256(part).hexdigest() for part in parts[1:]]}
    return prefix+b''.join(parts), row


def prepare(reference: Path, manifest: Path, destination: Path) -> dict[str, object]:
    """Freeze all selected cases without filtering failures; e.g. prepare(source, manifest, target)."""
    before = reference_snapshot(reference)
    cases = json.loads(manifest.read_bytes())['records']
    if len(cases) != 906:
        raise ValueError(f'case count {len(cases)}; expected the frozen 906-case configuration corpus')
    records = [bind_case(reference, case, index) for index, case in enumerate(cases)]
    if reference_snapshot(reference) != before:
        raise ValueError('reference sources changed; expected a stable input reader')
    packet = b'BRF1'+struct.pack('<I', len(records))+b''.join(raw for raw, _ in records)
    destination.mkdir(parents=True, exist_ok=True)
    (destination / 'resolved-native-probe.bin').write_bytes(packet)
    report = {'cases': len(records), 'bytes': len(packet), 'sha256': hashlib.sha256(packet).hexdigest(),
              'manifest_sha256': hashlib.sha256(manifest.read_bytes()).hexdigest(),
              'source_hashes': before, 'records': [row for _, row in records]}
    (destination / 'resolved-native-inputs.json').write_bytes((json.dumps(report, indent=2)+'\n').encode())
    return report


if __name__ == '__main__':
    report = prepare(WORKSPACE.parent / 'inventor-interface/solidworks',
                     WORKSPACE / 'target/model-native-corpus-manifest.json', WORKSPACE / 'target')
    print({key: value for key, value in report.items() if key not in ['source_hashes', 'records']})
