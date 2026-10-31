"""Fresh complete mass-element Property reference evidence, without target execution (#138)."""
from pathlib import Path
from typing import cast
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
import pefile
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot
from native_carriers import SOURCE, RESEARCH, digest, recover_sample, pinned_auxiliaries
from owned_mass import owned_mass_cases


def baseline(name: str, value: object) -> None:
    """Only initialize evidence on explicit request; e.g. baseline('owned-reference.json', report)."""
    path = FEATURE / name
    raw = (json.dumps(value, indent=2) + '\n').encode('utf-8')
    if '--write' in sys.argv: path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected unchanged complete reference baseline'


def actual_mass_parser(raw: bytes, version: int) -> dict[str, object]:
    """Use the unchanged full actual callback; e.g. actual_mass_parser(raw, 830036)."""
    from cosworks_mass_element_layout import parse
    from cosworks_binary_thermal_transform_audit import archive_reader
    return cast(dict[str, object], parse(raw, version, 'MassElement', archive_reader=archive_reader))


def case_packet(version: int, raw: bytes, parsed: dict[str, object]) -> bytes:
    """Retain the full body and each original span; e.g. case_packet(830036, raw, parsed)."""
    spans = cast(list[dict[str, int]], parsed['spans'])
    packet = struct.pack('<III', version, int(cast(bool, parsed['success'])), len(raw)) + raw
    return packet + struct.pack('<I', len(spans)) + b''.join(
        struct.pack('<II', span['offset'], span['end']) for span in spans)


def native_report() -> dict[str, object]:
    """Keep complete original identities and outcomes, without duplicating native payloads; e.g. native_report()."""
    report = json.loads((RESEARCH / 'cosworks-mass-element-native-evidence.json').read_bytes())
    assert report['evidence'] == 'C-722' and not report['target_executed']
    for group in ('rows', 'undecoded_material'):
        for row in report[group]: row.pop('raw_hex')
    return cast(dict[str, object], report)


def verify_native_row(row: dict[str, object], streams: dict[str, bytes]) -> bytes:
    """Check fresh hashes and every original retained outcome; e.g. verify_native_row(row, streams)."""
    raw = streams[cast(str, row['stream'])]
    assert len(raw) == row['bytes'] and digest(raw) == row['sha256']
    packet = bytearray()
    for expected in cast(list[dict[str, object]], row['attempts']):
        version = cast(int, expected['version'])
        parsed = actual_mass_parser(raw, version)
        assert dict(version=version, **parsed) == expected
        packet.extend(case_packet(version, raw, parsed))
    return bytes(packet)


def replay_native(pin: Path) -> None:
    """Recover both Property streams and verify the separate Material identities; e.g. replay_native(pin)."""
    report = native_report()
    baseline('retained-reference.json', report)
    rows = cast(list[dict[str, object]], report['rows'])
    material = cast(list[dict[str, object]], report['undecoded_material'])
    packet = bytearray()
    for path in sorted({cast(str, row['path']) for row in rows}):
        streams = recover_sample(path, pin)
        for row in (row for row in rows if row['path'] == path):
            packet.extend(verify_native_row(row, streams))
        for row in (row for row in material if row['path'] == path):
            raw = streams[cast(str, row['stream'])]
            assert len(raw) == row['bytes'] and digest(raw) == row['sha256']
    assert len(rows) == 2 and sum(cast(int, row['bytes']) for row in rows) == 2276
    (ROOT / 'target/simulation-mass-element-native.bin').write_bytes(packet)
    baseline('native-reference.json', dict(cases=2, bytes=len(packet), sha256=digest(packet), material_decoded=False))


def replay_owned() -> None:
    """Require every complete literal body and exact selector branch; e.g. replay_owned()."""
    packet = bytearray()
    cases = owned_mass_cases()
    for selector, version, raw in cases:
        parsed = actual_mass_parser(raw, version)
        assert parsed['success'] and parsed['end'] == len(raw)
        records = cast(list[dict[str, object]], parsed['complete_records'])
        assert len(records) == 1
        assert records[0]['selector'] == (selector if version >= 890204 else None)
        assert records[0]['pair_count'] == (3 if version >= 890204 and selector == 1 else None)
        assert not any(span['label'] == 'collection.count' for span in cast(list[dict[str, object]], parsed['spans']))
        packet.extend(case_packet(version, raw, parsed))
    path = ROOT / 'tests/fixtures/simulation-mass-element-property/owned.bin'
    if '--write' in sys.argv: path.parent.mkdir(exist_ok=True); path.write_bytes(packet)
    assert path.read_bytes() == packet
    baseline('owned-reference.json', dict(cases=len(cases), bytes=len(packet), sha256=digest(packet)))


def verify_schema() -> None:
    """Pin the entire relevant clause and actual reader hashes; e.g. verify_schema()."""
    text = (SOURCE / 'docs/solidworks-file-format/specification.md').read_bytes().decode('utf-8').replace('\r\n', '\n')
    start = text.index('### 14.346 ')
    raw = (text[start:text.index('\n### ', start + 1)].rstrip() + '\n').encode('utf-8')
    path = FEATURE / 'schema-14.346.md'
    if '--write' in sys.argv: path.write_bytes(raw)
    assert path.read_bytes() == raw
    readers = ('cosworks_mass_element_layout.py', 'cosworks_mass_element_audit.py', 'cosworks_binary_thermal_transform_audit.py')
    baseline('schema-reference.json', dict(clause_sha256=digest(raw), readers={name: digest((RESEARCH / name).read_bytes()) for name in readers}))


def verify_static_bindings() -> None:
    """Compare installed module bytes with the retained binding evidence; e.g. verify_static_bindings()."""
    report = native_report()
    module = Path('C:/Program Files/SOLIDWORKS Corp/SOLIDWORKS/Simulation/cosworks.dll').read_bytes()
    assert digest(module) == report['module_sha256']
    image = pefile.PE(data=module, fast_load=True)
    try:
        verify_image_bindings(image, report)
    finally:
        image.close()


def verify_image_bindings(image: pefile.PE, report: dict[str, object]) -> None:
    """Check every function span, slot, literal and version constant; e.g. verify_image_bindings(image, report)."""
    base = image.OPTIONAL_HEADER.ImageBase
    for function in cast(list[dict[str, str]], report['functions']):
        start, end = int(function['address'], 16), int(function['end'], 16)
        assert digest(image.get_data(start - base, end - start)) == function['raw_sha256']
    slot = cast(dict[str, str], report['serializer_slot'])
    actual = struct.unpack('<Q', image.get_data(int(slot['address'], 16) - base, 8))[0]
    assert actual == int(slot['target'], 16)
    for group in ('literals', 'version_constants'):
        for binding in cast(list[dict[str, object]], report[group]):
            expected = bytes.fromhex(cast(str, binding['raw_hex']))
            assert image.get_data(int(cast(str, binding['address']), 16) - base, len(expected)) == expected


def run() -> bool:
    """Execute complete readers only from an immutable source copy; e.g. run()."""
    from functools import partial, update_wrapper
    with tempfile.TemporaryDirectory(prefix='bessemer-mass-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        pinned_auxiliaries(pin, hashes)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin / 'research'))
        replay = update_wrapper(partial(replay_native, pin), replay_native)
        suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in (replay, replay_owned, verify_schema, verify_static_bindings))
        result = unittest.TextTestRunner(verbosity=2).run(suite)
        readers = ('cosworks_mass_element_layout', 'cosworks_binary_thermal_transform_audit', 'cfb', 'swcontainer')
        record_snapshot(pin, hashes, result, FEATURE, readers, ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
