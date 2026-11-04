"""Freeze complete Source before multi-parameter fixture acceptance (#202)."""
import os
from pathlib import Path
import sys
import tempfile
import unittest
from freeze_owned_reference import (ROOT, FEATURE, FrozenSourceImports, copy_sources,
                                    record_snapshot, retain_parameter_snapshot,
                                    verify_parameter_regeneration)


def run_multiple_reference(pin: Path, destination: Path) -> unittest.TestResult:
    """Install the immutable guard before any upstream import; e.g. run_multiple_reference(pin,out)."""
    sys.meta_path.insert(0, FrozenSourceImports(pin))
    sys.path.insert(0, str(ROOT / 'docs/features/resolved-root-graph'))
    sys.path.insert(0, str(ROOT / 'docs/features/trim-edge-identifiers'))
    os.environ['BESSEMER_PARAMETER_FIXTURES'] = str(destination)
    import freeze_trim_edges
    suite = unittest.defaultTestLoader.discover(str(FEATURE), pattern='test_multiple_reference.py')
    return unittest.TextTestRunner(verbosity=2, failfast=True).run(suite)


def main() -> int:
    """Require full Source guards and exact fixture regeneration; e.g. main()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-parameters-multiple-', dir=ROOT / 'target') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        evidence = ROOT / 'target/parameter-multiple-source-evidence'
        evidence.mkdir(exist_ok=True)
        generated = evidence / 'fixtures'
        result = run_multiple_reference(pin, generated)
        record_snapshot(pin, hashes, result, evidence, ('modelnode', 'dimensions', 'attributes'),
                        ('swcontainer', 'modelprefix', 'resolvedfeatures', 'mfcobjects', 'swarchive'))
        if not result.wasSuccessful():
            return 1
        verify_parameter_regeneration(generated, ROOT / 'tests/fixtures/stored-parameters/multiple')
        retain_parameter_snapshot((evidence / 'fixture-reference-snapshot.json').read_bytes(), 'multiple-source-snapshot')
        return 0


if __name__ == '__main__':
    raise SystemExit(main())
