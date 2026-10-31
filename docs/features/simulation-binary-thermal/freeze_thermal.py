"""Replay complete actual C-716 callbacks from immutable source modules (#134)."""
from pathlib import Path
import json
import sys
import tempfile
import unittest
from functools import partial, update_wrapper
from typing import cast

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
sys.path.insert(0, str(FEATURE.parent / 'model-archives'))
sys.path.insert(0, str(ROOT / 'target/reference-python'))
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot
from native_carriers import recover_carriers, pinned_auxiliaries, digest, RESEARCH, SOURCE
from embedded_reference import verify_owned_embedded, verify_embedded_rejections


def baseline(name: str, value: object) -> None:
    """Create only initial feature evidence explicitly; e.g. baseline('native-reference.json', {})."""
    path = FEATURE / name
    raw = (json.dumps(value, indent=2) + '\n').encode('utf-8')
    if '--write' in sys.argv:
        path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected unchanged complete baseline'


def replay_candidates(pin: Path) -> None:
    """Compare all 75 complete retained attempts, including failed alternatives; e.g. replay_candidates(pin)."""
    from cosworks_binary_thermal_transform_audit import parse_native, archive_reader
    carriers = recover_carriers(pin)
    counts = {'success': 0, 'failed': 0, 'archives': 0, 'transforms': 0}
    for row, raw in carriers:
        for expected in row['attempts']:
            version = cast(int, expected['version'])
            actual = dict(version=version, **parse_native(raw, version, row['stream'].split('/')[-1], archive_reader))
            assert actual == expected, f"{row['path']}:{row['stream']}:{version}; expected complete retained outcome"
            counts['success' if actual['success'] else 'failed'] += 1
            counts['archives'] += len(actual['archive_records']) if actual['success'] else 0
            counts['transforms'] += len(actual['transform_wrappers']) if actual['success'] else 0
    assert counts == dict(success=68, failed=7, archives=242, transforms=19)
    payload = b''.join(len(raw).to_bytes(4, 'little') + raw for _, raw in carriers)
    (ROOT / 'target/simulation-binary-thermal-native.bin').write_bytes(payload)
    baseline('native-reference.json', dict(**counts, streams=len(carriers), bytes=267820, packet_bytes=len(payload), packet_sha256=digest(payload)))


def source_evidence() -> None:
    """Pin both retained inventories independently of replay; e.g. source_evidence()."""
    names = ('cosworks-binary-thermal-transform-native-evidence.json', 'third-party-storage-corpus.json')
    baseline('native-source-reference.json', {name: digest((RESEARCH / name).read_bytes()) for name in names})
    names = ('reader/cfb.py', 'solidworks/cache/keytable.json')
    baseline('auxiliary-source-reference.json', {name: digest((SOURCE / name).read_bytes()) for name in names})


def owned_archives() -> None:
    """Freeze independent complete populated wrapper images and source boundaries; e.g. owned_archives()."""
    baseline('owned-embedded-reference.json', verify_owned_embedded())


def schema_evidence() -> None:
    """Freeze complete relevant clauses without importing live code; e.g. schema_evidence()."""
    specification = SOURCE / 'docs/solidworks-file-format/specification.md'
    text = specification.read_bytes().decode('utf-8').replace('\r\n', '\n')
    numbers = (282, 292, 293, 296, 298, 299, 301, 304, 305, 308, 309, 310, 311, 312, 313, 315, 316, 319, 320, 339, 340)
    clauses: dict[str, str] = {}
    for section in ('11.6', *(f'14.{number}' for number in numbers)):
        start = text.index(f'### {section} ')
        end = text.find('\n### ', start + 1)
        raw = (text[start:end if end >= 0 else len(text)].rstrip() + '\n').encode('utf-8')
        path = FEATURE / f'schema-{section}.md'
        if '--write' in sys.argv:
            path.write_bytes(raw)
        assert path.read_bytes() == raw, f'{path}; expected unchanged complete clause'
        clauses[section] = digest(raw)
    baseline('schema-reference.json', clauses)


def run() -> bool:
    """Run actual unmodified modules from a consistent source copy; e.g. run()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-binary-thermal-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        pinned_auxiliaries(pin, hashes)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin / 'research'))
        replay = update_wrapper(partial(replay_candidates, pin), replay_candidates)
        checks = (replay, source_evidence, owned_archives, verify_embedded_rejections, schema_evidence)
        suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
        result = unittest.TextTestRunner(verbosity=2).run(suite)
        readers = ('cosworks_binary_thermal_transform_audit', 'cosworks_binary_thermal_layout', 'cosworks_force_optional_audit', 'cfb', 'swcontainer')
        record_snapshot(pin, hashes, result, FEATURE, readers, ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
