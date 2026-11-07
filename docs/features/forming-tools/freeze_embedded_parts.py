"""Verify an inline part composition with the real forming-tool reader (#54)."""
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
from embedded_part_bytes import tool_cases


def observe_tool(pin: Path, name: str, raw: bytes, write: bool) -> dict[str, object]:
    """Require real section ends and complete primitive coverage; e.g. observe_tool(pin,name,raw,False)."""
    from swarchive import Archive
    cursor = object_cursor(pin, raw, 19000, None)
    prior = cursor.readers
    with observe_primitives(Archive, raw) as reads:
        tool = cursor.object()
    assert cursor.p == len(raw) and cursor.readers is prior
    if name == 'paired-tools':
        assert tool['items'][0]['part_08'] is tool['items'][1]['part_08']
        tool = tool['items'][0]
    part = tool['part_08']
    if part is not None:
        assert part['_class'] == 'moPart_c' and part['stage'] == 'complete'
        assert part['end'] == part['resolved']['end'] and part['lightweight_end'] < part['end']
    spans = [row for row in reads if row.end > row.start]
    verify_primitive_coverage(raw, spans)
    oracle = ''.join(f'{r.start}\t{r.end}\t{r.kind}\t{r.raw_hex}\n' for r in spans).encode()
    destination = ROOT / 'tests/fixtures/forming-tools'
    for suffix, payload in [('.bin', raw), ('.tsv', oracle)]:
        path = destination / (name + suffix)
        if write:
            path.write_bytes(payload)
        else:
            assert path.read_bytes() == payload, f'{path.name}; expected identical independent composition'
    facts = dict(name=name, bytes=len(raw), reads=len(spans), slots=len(cursor.map),
                 sha256=hashlib.sha256(raw).hexdigest(), oracle_sha256=hashlib.sha256(oracle).hexdigest())
    if part is not None:
        facts.update(part_start=part['start'], part_end=part['end'],
                     lightweight_size=part['lightweight_size'], resolved_size=part['resolved']['size'])
    return facts


def freeze_tool(pin: Path, write: bool) -> None:
    """Regenerate all actual shared-map cases and the independent edit; e.g. freeze_tool(pin,False)."""
    facts = [observe_tool(pin, name, raw, write) for name, raw in tool_cases(pin)]
    (FEATURE / 'embedded-reference.json').write_bytes((json.dumps(facts, indent=2) + '\n').encode())
    rejected = verify_tool_boundaries(pin)
    (FEATURE / 'embedded-boundary-reference.json').write_bytes((json.dumps(rejected, indent=2) + '\n').encode())


def verify_tool_boundaries(pin: Path) -> list[dict[str, object]]:
    """Require actual Source rejection and reader restoration for both sections; e.g. verify_tool_boundaries(pin)."""
    from swarchive import ArchiveError
    raw = tool_cases(pin)[0][1]
    result = []
    for offset, size in [(32, 4856), (4892, 21103)]:
        for count in [-1, 0, size - 1, size + 1, 2**31 - 1]:
            malformed = raw[:offset] + struct.pack('<i', count) + raw[offset + 4:]
            cursor = object_cursor(pin, malformed, 19000, None)
            prior = cursor.readers
            try:
                cursor.object()
            except ArchiveError as error:
                result.append(dict(offset=offset, count=count, cause=str(error)))
            else:
                raise AssertionError(f'count {count} at {offset}; expected explicit section failure')
            assert cursor.readers is prior, 'expected the actual forming-tool factory to restore readers'
    return result


def main() -> None:
    """Reproduce the composed fixture from immutable readers; e.g. main()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-embedded-parts-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin))
        replay = update_wrapper(partial(freeze_tool, pin, '--write' in sys.argv), freeze_tool)
        result = unittest.TextTestRunner(verbosity=2).run(unittest.FunctionTestCase(replay))
        assert result.wasSuccessful() and result.testsRun == 1 and not result.skipped
        record_snapshot(pin, hashes, result, FEATURE, ('formtools', 'embeddedparts', 'modelheader'),
                        ('swcontainer', 'modelprefix', 'mfcobjects'))
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'embedded-source-snapshot.json')


if __name__ == '__main__':
    main()
