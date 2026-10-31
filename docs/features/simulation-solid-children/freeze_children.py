"""Fresh direct-component replay and explicitly static-only folder vectors (#149)."""
from pathlib import Path
from typing import cast
import json
import struct
import sys
import tempfile
import unittest

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
for folder in ('simulation-binary-thermal', 'model-archives', 'simulation-solid-manager'):
    sys.path.insert(0, str(FEATURE.parent / folder))
sys.path.insert(0, str(ROOT / 'target/reference-python'))
import pefile
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot
from native_carriers import SOURCE, RESEARCH, digest, recover_sample, pinned_auxiliaries
from freeze_solid_manager import verify_image
from owned_children import owned_component_headers, child_versions, folder_fields, folder_projection_case
from child_projection import component_projection, component_history, folder_history
from child_adversarial import component_mutations, child_budgets


def baseline(name: str, value: object) -> None:
    """Only initialize complete evidence explicitly; e.g. baseline('native-reference.json', report)."""
    path = FEATURE / name
    raw = (json.dumps(value, indent=2) + '\n').encode()
    if '--write' in sys.argv: path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected unchanged full evidence'


def retained_report() -> dict[str, object]:
    """Preserve every original native component context and static bindings; e.g. retained_report()."""
    retained = json.loads((RESEARCH / 'cosworks-solid-component-native-evidence.json').read_bytes())
    assert retained['evidence'] == 'C-730' and not retained['target_executed']
    baseline('initial-reference.json', retained)
    return cast(dict[str, object], retained)


def input_case(version: int, raw: bytes, parsed: dict[str, object]) -> bytes:
    """Keep every full component body and accepted/failed span list; e.g. input_case(840013, raw, parsed)."""
    spans = cast(list[dict[str, int]], parsed['spans'])
    return (struct.pack('<III', version, int(cast(bool, parsed['success'])), len(raw)) + raw
            + struct.pack('<I', len(spans)) + b''.join(struct.pack('<II', span['offset'], span['end']) for span in spans))


def native_row(row: dict[str, object], streams: dict[str, bytes]) -> tuple[bytes, bytes, bytes]:
    """Match every complete original outcome after fresh envelope recovery; e.g. native_row(row, streams)."""
    from cosworks_solid_component_layout import parse
    raw = streams[cast(str, row['stream'])]
    assert len(raw) == row['bytes'] and digest(raw) == row['sha256']
    packet = bytearray()
    projection = bytearray()
    history = bytearray()
    for expected in cast(list[dict[str, object]], row['attempts']):
        version = cast(int, expected['version'])
        parsed = parse(raw, version)
        assert dict(version=version, **parsed) == expected
        packet.extend(input_case(version, raw, parsed))
        if parsed['success']:
            projection.extend(component_projection(version, raw))
            history.extend(component_history(version, raw))
    return bytes(packet), bytes(projection), bytes(history)


def replay_native(pin: Path) -> None:
    """Recover all 685 native component occurrences and 752 original arguments; e.g. replay_native(pin)."""
    rows = cast(list[dict[str, object]], retained_report()['rows'])
    packet = bytearray()
    projection = bytearray()
    history = bytearray()
    for document in sorted({cast(str, row['path']) for row in rows}):
        streams = recover_sample(document, pin)
        for row in (row for row in rows if row['path'] == document):
            inputs, projected, edited = native_row(row, streams)
            packet.extend(inputs)
            projection.extend(projected)
            history.extend(edited)
    attempts = [attempt for row in rows for attempt in cast(list[dict[str, object]], row['attempts'])]
    assert len(rows) == 685 and sum(cast(int, row['bytes']) for row in rows) == 33518
    assert len(attempts) == 752 and sum(cast(bool, attempt['success']) for attempt in attempts) == 699
    (ROOT / 'target/simulation-solid-component-native.bin').write_bytes(packet)
    (ROOT / 'target/simulation-solid-component-native-projection.bin').write_bytes(projection)
    (ROOT / 'target/simulation-solid-component-native-history.bin').write_bytes(history)
    baseline('native-reference.json', dict(streams=685, arguments=752, accepted=699, failed=53,
             bytes=len(packet), sha256=digest(packet), native_folders=0))


def save_owned(name: str, packet: bytes, count: int, scope: str) -> None:
    """Keep static folder vectors distinguishable from actual component-reader results; e.g. save_owned('component', packet, 48, 'actual Source')."""
    path = ROOT / f'tests/fixtures/simulation-solid-children/{name}.bin'
    if '--write' in sys.argv:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(packet)
    assert path.read_bytes() == packet
    baseline(f'owned-{name}-reference.json', dict(cases=count, bytes=len(packet), sha256=digest(packet), scope=scope))


def replay_owned_components() -> None:
    """Require complete actual-reader framing for every independent component literal; e.g. replay_owned_components()."""
    from cosworks_solid_component_layout import parse
    cases = owned_component_headers()
    packet = bytearray()
    for _, version, raw in cases:
        parsed = parse(raw, version)
        assert parsed['success'] and parsed['end'] == len(raw)
        assert parsed['reconstruction_sha256'] == digest(raw)
        packet.extend(input_case(version, raw, parsed))
    save_owned('component', bytes(packet), len(cases), 'actual unmodified component reader')
    save_owned('component-projection', b''.join(component_projection(version, raw) for _, version, raw in cases), len(cases), 'complete actual component reader fields')
    save_owned('component-history', b''.join(component_history(version, raw) for _, version, raw in cases), len(cases), 'complete actual component reader edited fields and ordered parents')


def verify_static_folder_vectors() -> None:
    """Validate literal frames and exact historical field presence without claiming a folder parser; e.g. verify_static_folder_vectors()."""
    from cosworkspersist import read_text
    packet = bytearray()
    for version in child_versions():
        for profile in range(4):
            fields = folder_fields(version, profile)
            names = [name for name, _, _ in fields]
            assert ('folder.worda0' in names) == (version >= 840014)
            assert ('folder.wordac' in names) == (version >= 870017)
            for _, tag, raw in fields:
                if tag == 3: assert read_text(raw).end == len(raw)
                else: assert tag == 0 and len(raw) == 4
            packet.extend(folder_projection_case(version, profile))
    save_owned('folder', bytes(packet), len(child_versions()) * 4, 'static C-730 clause/installed bindings plus literal vectors; no native folder reader replay')
    save_owned('folder-history', b''.join(folder_history(version, profile) for version in child_versions() for profile in range(4)), len(child_versions()) * 4, 'static literal edited fields and ordered parents; no folder reader replay')


def verify_component_mutations() -> None:
    """Keep every accepted/failed targeted framing input; e.g. verify_component_mutations()."""
    packet, count, accepted = component_mutations()
    save_owned('component-mutations', packet, count, f'actual component reader: {accepted} accepted, {count - accepted} failed')


def verify_child_budgets() -> None:
    """Keep independent complete-call and literal measures; e.g. verify_child_budgets()."""
    save_owned('budgets', child_budgets(), 96, 'actual component calls and separately static folder literal measures')


def verify_sources() -> None:
    """Bind all original source/decompilation bytes and the complete two-body clause; e.g. verify_sources()."""
    for original, expected in cast(dict[str, str], retained_report()['sources']).items():
        relative = original.replace('\\', '/').split('/solidworks/research/', 1)[1]
        assert digest((RESEARCH / relative).read_bytes()) == expected
    text = (SOURCE / 'docs/solidworks-file-format/specification.md').read_bytes().decode().replace('\r\n', '\n')
    start = text.index('### 14.354 ')
    raw = (text[start:text.index('\n### ', start + 1)].rstrip() + '\n').encode()
    path = FEATURE / 'schema-14.354.md'
    if '--write' in sys.argv: path.write_bytes(raw)
    assert path.read_bytes() == raw


def verify_folder_inventory_empty() -> None:
    """State only the retained payload denominator, never absence of empty storages; e.g. verify_folder_inventory_empty()."""
    inventory = json.loads((RESEARCH / 'third-party-storage-corpus.json').read_bytes())
    matches = [(sample['path'], stream['name']) for sample in inventory['samples']
               for stream in sample['nested_streams'] if '/SolidManager/Folders/' in stream['name']]
    assert not matches, matches
    baseline('folder-inventory-reference.json', dict(retained_payloads=0,
             scope='C-590 retained stream inventory only; empty storages and unsampled files are not excluded'))


def verify_bindings() -> None:
    """Check component and folder installed reader identities without executing either; e.g. verify_bindings()."""
    retained = retained_report()
    raw = Path('C:/Program Files/SOLIDWORKS Corp/SOLIDWORKS/Simulation/cosworks.dll').read_bytes()
    assert digest(raw) == retained['module_sha256']
    image = pefile.PE(data=raw, fast_load=True)
    try:
        verify_image(image, retained)
        base = image.OPTIONAL_HEADER.ImageBase
        assert struct.unpack('<Q', image.get_data(0x18205eab0 - base, 8))[0] == 0x1813b5db0
    finally:
        image.close()


def run_checks(pin: Path) -> unittest.TestResult:
    """Keep component native replay and folder static-vector checks as distinct tests; e.g. run_checks(pin)."""
    from functools import partial, update_wrapper
    import test_cosworks_solid_component
    replay = update_wrapper(partial(replay_native, pin), replay_native)
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in
                               (replay, replay_owned_components, verify_static_folder_vectors, verify_component_mutations, verify_child_budgets, verify_sources, verify_bindings, verify_folder_inventory_empty))
    suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(test_cosworks_solid_component))
    return unittest.TextTestRunner(verbosity=2).run(suite)


def run() -> bool:
    """Keep every actual component/text reader and container adapter on an immutable copy; e.g. run()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-solid-children-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        pinned_auxiliaries(pin, hashes)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin / 'research'))
        result = run_checks(pin)
        record_snapshot(pin, hashes, result, FEATURE, ('cosworks_solid_component_layout', 'cosworkspersist', 'cfb', 'swcontainer'), ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
