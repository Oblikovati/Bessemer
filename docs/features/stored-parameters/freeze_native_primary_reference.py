"""Freeze independent original-reader spans for the first complete native parameter edits (#202)."""
from pathlib import Path
import os
import sys
import tempfile
import unittest
from freeze_owned_reference import (ROOT, FEATURE, FrozenSourceImports, copy_sources, record_snapshot,
                                    retain_parameter_snapshot, verify_parameter_regeneration)


def run_native_primary_reference(pin: Path, destination: Path) -> unittest.TestResult:
    """Guard upstream imports before loading the independent observer; e.g. run_native_primary_reference(pin,out)."""
    sys.meta_path.insert(0, FrozenSourceImports(pin))
    sys.path.insert(0, str(ROOT / 'docs/features/resolved-root-graph'))
    sys.path.insert(0, str(ROOT / 'docs/features/trim-edge-identifiers'))
    os.environ['BESSEMER_PARAMETER_FIXTURES'] = str(destination)
    import freeze_trim_edges
    suite = unittest.defaultTestLoader.discover(str(FEATURE), pattern='test_native_primary_reference.py')
    return unittest.TextTestRunner(verbosity=2).run(suite)


def main() -> int:
    """Require complete original parses, independent edits and integrity guards; e.g. main()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-native-parameter-primary-', dir=ROOT / 'target') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        evidence = ROOT / 'target/native-parameter-primary-source-evidence'
        evidence.mkdir(exist_ok=True)
        generated = evidence / 'fixtures'
        result = run_native_primary_reference(pin, generated)
        record_snapshot(pin, hashes, result, evidence, ('modelnode', 'partlightweight'),
                        ('swcontainer', 'modelprefix', 'resolvedfeatures', 'mfcobjects', 'swarchive'))
        if not result.wasSuccessful():
            return 1
        verify_parameter_regeneration(generated, ROOT / 'tests/fixtures/stored-parameters/native-first')
        retain_parameter_snapshot((evidence / 'fixture-reference-snapshot.json').read_bytes(), 'native-primary-source-snapshot')
        return 0


if __name__ == '__main__':
    raise SystemExit(main())
