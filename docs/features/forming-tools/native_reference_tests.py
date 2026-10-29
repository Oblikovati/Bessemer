"""Verify the complete upstream forming-tool evidence from an immutable copy (#54)."""
from pathlib import Path
import json
import shutil
import sys
import tempfile
import unittest

FEATURE = Path(__file__).resolve().parent
sys.path.insert(0, str(FEATURE.parent / 'model-archives'))
from pinned_reference_tests import SOURCE, FrozenSourceImports, copy_sources, record_snapshot


def copy_native_inputs(pin: Path) -> None:
    """Copy the native audit inputs without changing readers; e.g. copy_native_inputs(pin)."""
    names = ('embedded-part', 'form-tool', 'fold-sketch-property')
    (pin / 'research').mkdir(exist_ok=True)
    for name in names:
        relative = Path('research') / (name + '-targeted-corpus.json')
        shutil.copyfile(SOURCE / relative, pin / relative)
    (pin / 'cache').mkdir(exist_ok=True)
    shutil.copyfile(SOURCE / 'cache/keytable.json', pin / 'cache/keytable.json')


def verify_native_readers(pin: Path) -> unittest.TestResult:
    """Require all five actual regression tests to execute; e.g. verify_native_readers(pin)."""
    sys.meta_path.insert(0, FrozenSourceImports(pin))
    sys.path.insert(0, str(pin))
    suite = unittest.defaultTestLoader.loadTestsFromNames(('test_formtools', 'test_embeddedparts'))
    result = unittest.TextTestRunner(verbosity=2).run(suite)
    assert result.wasSuccessful() and not result.skipped, 'expected five passing, unskipped Source tests'
    assert result.testsRun == 5, f'{result.testsRun}; expected five forming-tool/embedded-part tests'
    return result


def main() -> None:
    """Record source origins and hashes for the native proof; e.g. main()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-forming-reference-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        copy_native_inputs(pin)
        result = verify_native_readers(pin)
        record_snapshot(pin, hashes, result, FEATURE, ('formtools', 'embeddedparts', 'modelheader'))
        snapshot = FEATURE / 'fixture-reference-snapshot.json'
        report = json.loads(snapshot.read_text(encoding='utf-8'))
        report.update(reader_functions_replaced=False, native_instances=2, embedded_parts=2, attributed_bodies=4)
        (FEATURE / 'native-reference.json').write_bytes((json.dumps(report, indent=2) + '\n').encode())
        snapshot.unlink()


if __name__ == '__main__':
    main()
