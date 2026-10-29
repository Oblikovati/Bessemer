"""Bind the shared-map header codec to actual Source primitive reads (#54)."""
from pathlib import Path
from functools import partial, update_wrapper
import hashlib
import json
import re
import struct
import sys
import tempfile
import unittest

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
sys.path.insert(0, str(FEATURE.parent / 'model-archives'))
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot
from primitive_inventory import observe_primitives, verify_primitive_coverage


def header_inputs() -> list[tuple[str, int, bytes]]:
    """Reuse committed independent vectors; e.g. header_inputs()."""
    source = (ROOT / 'tests/support/header_cases.rs').read_text()
    pattern = r'"([^"]+)",\s*(\d+),\s*include_bytes!\("[^"\n]+"\)'
    cases = [(name, int(version)) for name, version in re.findall(pattern, source)]
    result = [(name, version, (ROOT / f'tests/fixtures/headers/{name}.bin').read_bytes())
              for name, version in cases if version > 0x27cb]
    result.append(('blank-part', 19000, (ROOT / 'tests/fixtures/headers/blank-part.bin').read_bytes()))
    sys.path.insert(0, str(FEATURE.parent / 'header-archives'))
    from generate_fixtures import header
    older = header(19000, 0xfa6)
    assert older.count(struct.pack('<i', -24)) == 1
    result.extend([('older-external', 19000, older),
                   ('older-external-edited', 19000, older.replace(struct.pack('<i', -24), struct.pack('<i', 123456)))])
    count_position = older.index(b'su_CStringArray') + len(b'su_CStringArray')
    assert older[count_position:count_position + 2] == struct.pack('<H', 2)
    extended = older[:count_position] + struct.pack('<HI', 0xffff, 2) + older[count_position + 2:]
    result.append(('escaped-users-count', 19000, extended))
    assert len(result) > 15, f'{len(result)}; expected modern boundary vectors and native header'
    return result


def observe_header(name: str, version: int, raw: bytes) -> tuple[bytes, dict[str, object]]:
    """Observe the actual tagged graph without preloading objects; e.g. observe_header(name,v,raw)."""
    import modelheader
    from mfcobjects import ObjectArchive
    from swarchive import Archive
    cursor = ObjectArchive(raw, version)
    with observe_primitives(Archive, raw) as reads:
        parsed = cursor.object('moHeader_c')
    assert parsed['_class'] == 'moHeader_c' and cursor.p == len(raw)
    spans = [row for row in reads if row.end > row.start]
    verify_primitive_coverage(raw, spans)
    oracle = ''.join(f'{r.start}\t{r.end}\t{r.kind}\t{r.raw_hex}\n' for r in spans).encode()
    frame = struct.pack('<IIII', version, len(raw), len(oracle), len(cursor.map)) + raw + oracle
    return frame, dict(name=name, version=version, bytes=len(raw), reads=len(spans), slots=len(cursor.map),
                       sha256=hashlib.sha256(raw).hexdigest(), oracle_sha256=hashlib.sha256(oracle).hexdigest())


def freeze_headers(pin: Path, write: bool) -> None:
    """Check byte-identical oracles unless writing explicitly; e.g. freeze_headers(pin,False)."""
    sys.meta_path.insert(0, FrozenSourceImports(pin))
    sys.path.insert(0, str(pin))
    observations = [observe_header(*case) for case in header_inputs()]
    packet = b'IH01' + struct.pack('<I', len(observations)) + b''.join(frame for frame, _ in observations)
    path = ROOT / 'tests/fixtures/forming-tools/inline-headers.bin'
    if write:
        path.parent.mkdir(exist_ok=True)
        path.write_bytes(packet)
    else:
        assert path.read_bytes() == packet, 'inline header packet changed; expected identical Source replay'
    for name, _, raw in header_inputs():
        if not name.startswith('older-external'):
            continue
        golden = path.parent / (name + '.bin')
        if write:
            golden.write_bytes(raw)
        else:
            assert golden.read_bytes() == raw, f'{name}; expected unchanged independent edit golden'
    facts = dict(cases=[fact for _, fact in observations], packet_sha256=hashlib.sha256(packet).hexdigest())
    (FEATURE / 'inline-header-reference.json').write_bytes((json.dumps(facts, indent=2) + '\n').encode())


def main() -> None:
    """Record immutable reader origins and exact fixture regeneration; e.g. main()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-inline-headers-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        replay = update_wrapper(partial(freeze_headers, pin, '--write' in sys.argv), freeze_headers)
        case = unittest.FunctionTestCase(replay)
        result = unittest.TextTestRunner(verbosity=2).run(case)
        assert result.wasSuccessful() and result.testsRun == 1 and not result.skipped
        record_snapshot(pin, hashes, result, FEATURE, ('modelheader', 'mfcobjects', 'swarchive'), ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'inline-header-source-snapshot.json')


if __name__ == '__main__':
    main()
