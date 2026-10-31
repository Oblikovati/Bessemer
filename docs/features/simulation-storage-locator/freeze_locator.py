"""Freeze the complete actual logical-name locator and independent code-unit slices (#126)."""
from pathlib import Path
import json
import sys
import tempfile
import unittest
import zlib

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
sys.path.insert(0, str(FEATURE.parent / 'simulation-images'))
import freeze_images as shared
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot
from owned_locator import valid_cases, invalid_cases, units
span, words, sha = shared.span, shared.words, shared.sha


def report(name: str, value: object) -> None:
    """Require explicit review before replacing a reference baseline."""
    path = FEATURE / name
    raw = (json.dumps(value, indent=2) + '\n').encode()
    if '--write' in sys.argv:
        path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected unchanged reviewed baseline'


def freeze_cases() -> None:
    """Compare explicit literal parents/outputs with actual unchanged UTF-16 CString-equivalent slices."""
    import simulationstorage as reader
    cases = valid_cases()
    packet = bytearray(b'SLOC' + words([len(cases)]))
    for original, storage, stream in cases:
        actual = reader.locator(original.decode('utf-16-le', 'surrogatepass'))
        assert units(actual['storage_name']) == storage and units(actual['stream_name']) == stream
        assert (actual['storage_mode'], actual['storage_format'], actual['stream_mode']) == (16, 0, 16)
        assert not actual['payload_decoded']
        packet += span(original) + span(storage) + span(stream) + words([16, 0, 16, 0])
    write_fixture(bytes(packet))
    report('lookup-reference.json', dict(cases=len(cases), oracle_sha256=sha(bytes(packet)), actual_com_or_filesystem_calls=False))


def write_fixture(packet: bytes) -> None:
    """Ship only synthetic code-unit inputs and independent exact component goldens."""
    path = ROOT / 'tests/fixtures/simulation-storage-locator/locators.bin'
    compressed = zlib.compress(packet, 9)
    if '--write' in sys.argv:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(compressed)
    assert path.read_bytes() == compressed


def freeze_guards() -> None:
    """Keep no-dot and embedded-NUL inputs outside the reproduced lexical profile."""
    import simulationstorage as reader
    for raw in invalid_cases():
        with unittest.TestCase().assertRaises(ValueError):
            reader.locator(raw.decode('utf-16-le', 'surrogatepass'))
    with unittest.TestCase().assertRaises(TypeError):
        reader.locator(b'study.GEN')
    report('guard-reference.json', dict(excluded_utf16_shapes=len(invalid_cases()), narrow_argument_rejected=True))


def schema() -> None:
    """Retain the full last-dot clause without importing later unrelated payload grammars."""
    text = (ROOT.parent / 'inventor-interface/docs/solidworks-file-format/specification.md').read_text(encoding='utf-8').replace('\r\n', '\n')
    raw = (text[text.index('### 14.100 '):text.index('### 14.101 ')].rstrip() + '\n').encode()
    path = FEATURE / 'schema-snapshot.md'
    if '--write' in sys.argv:
        path.write_bytes(raw)
    assert path.read_bytes() == raw
    report('schema-reference.json', dict(issue=126, clauses=['14.100'], sha256=sha(raw)))


def verify_rust() -> None:
    """Compare complete independently emitted UTF-16 locator parents, components and modes."""
    if '--verify-rust' not in sys.argv:
        return
    expected = b'SLOV' + words([len(valid_cases())])
    for original, storage, stream in valid_cases():
        expected += span(original) + span(storage) + span(stream) + words([16, 0, 16, 0])
    actual = (ROOT / 'target/simulation-storage-locator-projections.bin').read_bytes()
    assert actual == expected
    report('rust-projection-reference.json', dict(bytes=len(actual), sha256=sha(actual), cases=len(valid_cases()), inputs_are_owned_synthetic=True))


def run() -> bool:
    """Copy/hash complete actual modules before import and retain the exact reader origin."""
    with tempfile.TemporaryDirectory(prefix='bessemer-simulation-locator-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        suite = unittest.TestSuite(unittest.FunctionTestCase(test) for test in (freeze_cases, freeze_guards, schema, verify_rust))
        result = unittest.TextTestRunner(verbosity=2).run(suite)
        record_snapshot(pin, hashes, result, FEATURE, ('simulationstorage',), ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
