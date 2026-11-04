"""Admit and reproduce positive inherited parameter fixtures with immutable Source (#202)."""
from pathlib import Path
import argparse
import os
import sys
import tempfile
import unittest
from freeze_owned_reference import ROOT, FEATURE, FrozenSourceImports, copy_sources, record_snapshot, retain_parameter_snapshot, verify_parameter_regeneration


def run_inherited_reference(pin: Path, destination: Path, actual_exports: bool = False) -> unittest.TestResult:
    """Install the guard before importing any reader-dependent helpers; e.g. run_inherited_reference(pin,out)."""
    sys.meta_path.insert(0, FrozenSourceImports(pin))
    sys.path.insert(0, str(ROOT / 'docs/features/resolved-root-graph'))
    sys.path.insert(0, str(ROOT / 'docs/features/trim-edge-identifiers'))
    os.environ['BESSEMER_PARAMETER_PIN'] = str(pin)
    os.environ['BESSEMER_PARAMETER_FIXTURES'] = str(destination)
    import freeze_trim_edges
    pattern = 'test_inherited_exports.py' if actual_exports else 'test_inherited_reference.py'
    suite = unittest.defaultTestLoader.discover(str(FEATURE), pattern=pattern)
    return unittest.TextTestRunner(verbosity=2).run(suite)


def verify_inherited_regeneration(generated: Path) -> None:
    """Preserve every already-reviewed inherited fixture; e.g. verify_inherited_regeneration(out)."""
    verify_parameter_regeneration(generated, ROOT / 'tests/fixtures/stored-parameters/inherited')


def main() -> int:
    """Require original-reader acceptance, integrity and exact regeneration; e.g. main()."""
    actual_exports = inherited_export_option()
    with tempfile.TemporaryDirectory(prefix='bessemer-inherited-parameter-', dir=ROOT / 'target') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        evidence = ROOT / 'target' / ('inherited-parameter-export-evidence' if actual_exports else 'inherited-parameter-source-evidence')
        evidence.mkdir(exist_ok=True)
        generated = evidence / 'fixtures'
        result = run_inherited_reference(pin, generated, actual_exports)
        record_snapshot(pin, hashes, result, evidence, ('modelnode', 'dimensions', 'attributes'),
                        ('swcontainer', 'modelprefix', 'resolvedfeatures', 'mfcobjects', 'swarchive'))
        if not result.wasSuccessful():
            return 1
        if not actual_exports:
            verify_inherited_regeneration(generated)
        prefix = 'inherited-actual-source-snapshot' if actual_exports else 'inherited-source-snapshot'
        retain_parameter_snapshot((evidence / 'fixture-reference-snapshot.json').read_bytes(), prefix)
        return 0


def inherited_export_option() -> bool:
    """Separate wire-baseline regeneration from actual result comparison; e.g. inherited_export_option()."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--actual-exports', action='store_true')
    return bool(parser.parse_args().actual_exports)


if __name__ == '__main__':
    raise SystemExit(main())
