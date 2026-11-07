"""Freeze the full original native primary-value oracle before corpus edit acceptance (#202)."""
from pathlib import Path
import os
import sys
import tempfile
import unittest
from freeze_owned_reference import (ROOT, FEATURE, FrozenSourceImports, copy_sources, record_snapshot,
                                    retain_parameter_snapshot, verify_parameter_regeneration)


def run_all_primary_reference(pin: Path, destination: Path) -> unittest.TestResult:
    """Load only immutable original readers after guarding imports; e.g. run_all_primary_reference(pin,out)."""
    sys.meta_path.insert(0, FrozenSourceImports(pin))
    sys.path.insert(0, str(ROOT / 'docs/features/resolved-root-graph'))
    sys.path.insert(0, str(ROOT / 'docs/features/trim-edge-identifiers'))
    os.environ['BESSEMER_PARAMETER_FIXTURES'] = str(destination)
    import freeze_trim_edges
    suite = unittest.defaultTestLoader.discover(str(FEATURE), pattern='test_all_primary_reference.py')
    return unittest.TextTestRunner(verbosity=2, failfast=True).run(suite)


def main() -> int:
    """Retain the full census only after original-method and integrity checks; e.g. main()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-all-native-primaries-', dir=ROOT / 'target') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        evidence = ROOT / 'target/all-native-parameter-primary-source-evidence'
        evidence.mkdir(exist_ok=True)
        generated = evidence / 'fixtures'
        result = run_all_primary_reference(pin, generated)
        record_snapshot(pin, hashes, result, evidence, ('modelnode', 'partlightweight', 'dimensions', 'attributes'),
                        ('swcontainer', 'modelprefix', 'resolvedfeatures', 'mfcobjects', 'swarchive'))
        if not result.wasSuccessful():
            return 1
        verify_parameter_regeneration(generated, ROOT / 'tests/fixtures/stored-parameters/native-primary-corpus')
        retain_parameter_snapshot((evidence / 'fixture-reference-snapshot.json').read_bytes(), 'all-primary-source-snapshot')
        return 0


if __name__ == '__main__':
    raise SystemExit(main())
