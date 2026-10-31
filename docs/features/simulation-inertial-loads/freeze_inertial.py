"""Fresh complete native and owned replay using copied actual source readers (#136)."""
from pathlib import Path
from types import ModuleType
from typing import cast
from functools import partial, update_wrapper
import importlib
import json
import struct
import sys
import tempfile
import unittest

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
sys.path.insert(0, str(FEATURE.parent / 'simulation-binary-thermal'))
sys.path.insert(0, str(FEATURE.parent / 'model-archives'))
sys.path.insert(0, str(ROOT / 'target/reference-python'))
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot
from native_carriers import ThermalCarrier, SOURCE, RESEARCH, digest, recover_sample, pinned_auxiliaries
from owned_inertial import LOAD_KINDS, owned_inertial_cases


def baseline(name: str, value: object) -> None:
    """Create only explicitly requested initial feature baselines; e.g. baseline('native-reference.json', value)."""
    path = FEATURE / name
    raw = (json.dumps(value, indent=2) + '\n').encode('utf-8')
    if '--write' in sys.argv: path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected unchanged complete reference baseline'


def reader_module(kind: str) -> ModuleType:
    """Select the actual declared reader, without rewriting it; e.g. reader_module('Gravity')."""
    name = 'cosworks_base_excitation_audit' if kind == 'BaseExcitation' else 'cosworks_inertial_audit'
    return importlib.import_module(name)


def parse_complete(raw: bytes, version: int, kind: str, full_registry: bool) -> dict[str, object]:
    """Call the complete original parser with an explicit actual callback; e.g. parse_complete(raw, 870022, 'Gravity', True)."""
    reader = reader_module(kind)
    callback = importlib.import_module('cosworks_binary_thermal_transform_audit').archive_reader if full_registry else reader.archive_reader
    return cast(dict[str, object], reader.parse(raw, version, kind, archive_reader=callback))


def retained_rows() -> list[ThermalCarrier]:
    """Keep every original retained candidate and identity; e.g. retained_rows()."""
    rows: list[ThermalCarrier] = []
    for name, evidence in (('cosworks-inertial-native-evidence.json', 'C-714'),
                           ('cosworks-base-excitation-native-evidence.json', 'C-717')):
        report = json.loads((RESEARCH / name).read_bytes())
        assert report['evidence'] == evidence and not report['target_executed']
        rows.extend(cast(list[ThermalCarrier], report['rows']))
    assert len(rows) == 18 and sum(row['bytes'] for row in rows) == 17928
    return rows


def case_packet(kind: int, version: int, raw: bytes, result: dict[str, object]) -> bytes:
    """Preserve all original bytes and every actual source span; e.g. case_packet(0, 870022, raw, result)."""
    spans = cast(list[dict[str, int]], result['spans'])
    packet = struct.pack('<IIII', kind, version, int(cast(bool, result['success'])), len(raw)) + raw
    packet += struct.pack('<I', len(spans))
    return packet + b''.join(struct.pack('<II', span['offset'], span['end']) for span in spans)


def verify_native_row(row: ThermalCarrier, raw: bytes) -> bytes:
    """Compare full retained outcomes with both original and full additive callbacks; e.g. verify_native_row(row, raw)."""
    assert len(raw) == row['bytes'] and digest(raw) == row['sha256']
    kind = row['stream'].split('/')[-1]
    packet = bytearray()
    for expected in row['attempts']:
        version = cast(int, expected['version'])
        actual = parse_complete(raw, version, kind, False)
        assert dict(version=version, **actual) == expected, (row['path'], row['stream'], version)
        assert parse_complete(raw, version, kind, True) == actual
        packet.extend(case_packet(LOAD_KINDS.index(kind), version, raw, actual))
    return bytes(packet)


def replay_native(pin: Path) -> None:
    """Recover every complete native image with actual approved adapters and hash checks; e.g. replay_native(pin)."""
    rows = retained_rows()
    packet = bytearray()
    for path in sorted({row['path'] for row in rows}):
        streams = recover_sample(path, pin)
        for row in (row for row in rows if row['path'] == path):
            packet.extend(verify_native_row(row, streams[row['stream']]))
    (ROOT / 'target/simulation-inertial-native-cases.bin').write_bytes(packet)
    baseline('native-reference.json', dict(streams=18, bytes=17928, attempts=18, accepted=18,
             packet_bytes=len(packet), packet_sha256=digest(packet), callbacks='Original retained and full actual C-716 additive registry'))


def replay_owned() -> None:
    """Replay complete independently constructed historical gates; e.g. replay_owned()."""
    packet = bytearray()
    cases = owned_inertial_cases()
    for kind, version, raw in cases:
        result = parse_complete(raw, version, LOAD_KINDS[kind], True)
        assert result['success'] and result['end'] == len(raw)
        assert len(cast(list[object], result['complete_records'])) == 2
        assert len(cast(list[object], result['archive_records'])) == (4 if version >= 840002 else 0)
        if kind == 2:
            records = cast(list[dict[str, object]], result['complete_records'])
            assert all(record['header'] == [2, -1, 3, 1, 0, 0xffffffff] for record in records)
        packet.extend(case_packet(kind, version, raw, result))
    path = ROOT / 'tests/fixtures/simulation-inertial-loads/owned.bin'
    if '--write' in sys.argv:
        path.parent.mkdir(exist_ok=True)
        path.write_bytes(packet)
    assert path.read_bytes() == packet
    baseline('owned-reference.json', dict(cases=len(cases), packet_bytes=len(packet), packet_sha256=digest(packet)))


def verify_clauses() -> None:
    """Require the initial complete clauses and module identities to remain visible; e.g. verify_clauses()."""
    reference = json.loads((FEATURE / 'initial-reference.json').read_bytes())
    text = (SOURCE / 'docs/solidworks-file-format/specification.md').read_bytes().decode('utf-8').replace('\r\n', '\n')
    for name, expected in reference['schema_sha256'].items():
        section = name.removeprefix('schema-').removesuffix('.md')
        start = text.index(f'### {section} ')
        end = text.find('\n### ', start + 1)
        raw = (text[start:end if end >= 0 else len(text)].rstrip() + '\n').encode('utf-8')
        assert (FEATURE / name).read_bytes() == raw and digest(raw) == expected
    for name, expected in reference['reader_module_sha256'].items():
        assert digest((RESEARCH / name).read_bytes()) == expected, name


def run() -> bool:
    """Execute only copied actual sources and retain their origins; e.g. run()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-inertial-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        pinned_auxiliaries(pin, hashes)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin / 'research'))
        replay = update_wrapper(partial(replay_native, pin), replay_native)
        suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in (replay, replay_owned, verify_clauses))
        result = unittest.TextTestRunner(verbosity=2).run(suite)
        readers = ('cosworks_inertial_audit', 'cosworks_base_excitation_audit', 'cosworks_inertial_layout', 'cosworks_base_excitation_layout', 'cosworks_binary_thermal_transform_audit', 'cfb', 'swcontainer')
        record_snapshot(pin, hashes, result, FEATURE, readers, ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
