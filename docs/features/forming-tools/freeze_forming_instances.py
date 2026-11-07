"""Bind complete instances, keepers, suppressed bodies and transforms to actual Source (#54)."""
from functools import partial, update_wrapper
from pathlib import Path
import hashlib
import json
import struct
import sys
import tempfile
import unittest

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
sys.path.insert(0, str(FEATURE.parent / 'model-archives'))
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot
from primitive_inventory import observe_primitives, verify_primitive_coverage
from object_fixture_observer import object_cursor


def instance_cases(pin: Path) -> list[tuple[str, int, int, bytes, int]]:
    """Build actual graphs without imported or fake dependency objects; e.g. instance_cases(pin)."""
    from forming_instance_bytes import instance_object, representation_object
    result = []
    for version in (11000, 18000, 19000):
        for style in range(6):
            raw, slots = instance_object(version, style, pin)
            result.append((f'instance-{version}-{style}', version, style, raw, slots))
        for wrapped in (False, True):
            raw, slots = representation_object(wrapped)
            result.append((f'representation-{version}-{int(wrapped)}', version, 7 + int(wrapped), raw, slots))
    for name, style, word in [('embedded-instance', 6, -13), ('edited-instance', 1, 123456)]:
        raw, slots = instance_object(19000, style, pin, word)
        result.append((name, 19000, style, raw, slots))
    return result


def observe_case(pin: Path, case: tuple[str, int, int, bytes, int]) -> tuple[bytes, dict[str, object]]:
    """Require exact EOF, map census and primitive partition; e.g. observe_case(pin,case)."""
    from swarchive import Archive
    name, version, style, raw, slots = case
    cursor = object_cursor(pin, raw, version, None)
    with observe_primitives(Archive, raw) as reads:
        parsed = cursor.object()
    assert cursor.p == len(raw) and len(cursor.map) == slots, (name, cursor.p, len(raw), len(cursor.map), slots)
    if parsed['_class'] == 'moFormToolInstance_c':
        assert parsed['payload']['raw_9d8'] == (123456 if name == 'edited-instance' else -13)
    spans = [row for row in reads if row.end > row.start]
    verify_primitive_coverage(raw, spans)
    oracle = ''.join(f'{r.start}\t{r.end}\t{r.kind}\t{r.raw_hex}\n' for r in spans).encode()
    frame = struct.pack('<IBIII', version, style, len(raw), len(oracle), slots) + raw + oracle
    return frame, dict(name=name, version=version, style=style, bytes=len(raw), slots=slots, reads=len(spans),
                       sha256=hashlib.sha256(raw).hexdigest(), oracle_sha256=hashlib.sha256(oracle).hexdigest())


def freeze_instances(pin: Path, write: bool) -> None:
    """Regenerate the complete packet and independent word golden; e.g. freeze_instances(pin,False)."""
    cases = instance_cases(pin)
    observations = [observe_case(pin, case) for case in cases]
    outputs = {'instances.bin': b'FI01' + struct.pack('<I', len(cases)) + b''.join(frame for frame, _ in observations)}
    outputs.update({name + '.bin': raw for name, _, _, raw, _ in cases if name in ('instance-19000-1', 'edited-instance', 'embedded-instance')})
    for name, raw in outputs.items():
        path = ROOT / 'tests/fixtures/forming-tools' / name
        if write:
            path.write_bytes(raw)
        else:
            assert path.read_bytes() == raw, f'{name}; expected unchanged independent graph bytes'
    report = dict(cases=[facts for _, facts in observations], packet_sha256=hashlib.sha256(outputs['instances.bin']).hexdigest())
    (FEATURE / 'instance-reference.json').write_bytes((json.dumps(report, indent=2) + '\n').encode())
    freeze_owned_instances(pin, write)


def freeze_owned_instances(pin: Path, write: bool) -> None:
    """Verify public feature streams with real saved configuration/manager context."""
    from freeze_trim_edges import owned, ObjectFixtureFamily
    from forming_instance_bytes import owned_instance
    family = ObjectFixtureFamily(build=partial(owned_instance, reference=pin))
    destination = ROOT / 'tests/fixtures/forming-tools'
    with tempfile.TemporaryDirectory(prefix='bessemer-owned-forming-') as temporary:
        generated = Path(temporary)
        facts = [owned(pin, generated, kind, 1, edited, name, family)
                 for kind, edited, name in [(0, False, 'owned-instance'), (0, True, 'owned-instance-edited'),
                                            (1, False, 'owned-embedded'), (1, True, 'owned-embedded-edited')]]
        for path in generated.iterdir():
            target = destination / path.name
            if write:
                target.write_bytes(path.read_bytes())
            else:
                assert target.read_bytes() == path.read_bytes(), f'{path.name}; expected unchanged owned Source replay'
    (FEATURE / 'owned-instance-reference.json').write_bytes((json.dumps(facts, indent=2) + '\n').encode())


def main() -> None:
    """Record the immutable Source replay and imported module origins; e.g. main()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-forming-instances-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin))
        replay = update_wrapper(partial(freeze_instances, pin, '--write' in sys.argv), freeze_instances)
        result = unittest.TextTestRunner(verbosity=2).run(unittest.FunctionTestCase(replay))
        assert result.wasSuccessful() and result.testsRun == 1 and not result.skipped
        record_snapshot(pin, hashes, result, FEATURE, ('formtools', 'embeddedparts', 'modelheader'), ('swcontainer', 'modelprefix', 'mfcobjects'))
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'instance-source-snapshot.json')


if __name__ == '__main__':
    main()
