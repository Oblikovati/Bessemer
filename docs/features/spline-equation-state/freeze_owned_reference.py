"""Reproduce owned equation fixtures through an immutable Source copy (#199)."""
from pathlib import Path
import gzip
import hashlib
import json
import os
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[3]
FEATURE = Path(__file__).resolve().parent
sys.path.insert(0, str(ROOT / 'docs/features/model-archives'))
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot


def run_owned_reference(pin: Path, generated: Path) -> unittest.TestResult:
    """Observe original Source without writing goldens; e.g. run_owned_reference(pin, temporary)."""
    sys.meta_path.insert(0, FrozenSourceImports(pin))
    sys.path.insert(0, str(FEATURE))
    sys.path.insert(0, str(ROOT / 'docs/features/trim-edge-identifiers'))
    os.environ['BESSEMER_EQUATION_PIN'] = str(pin)
    os.environ['BESSEMER_EQUATION_FIXTURE_ROOT'] = str(generated)
    import freeze_trim_edges
    suite = unittest.defaultTestLoader.discover(str(FEATURE), pattern='test_owned_reference.py')
    return unittest.TextTestRunner(verbosity=2).run(suite)


def verify_owned_fixture_regeneration(generated: Path) -> None:
    """Require every checked-in byte unchanged; e.g. verify_owned_fixture_regeneration(temporary)."""
    expected = ROOT / 'tests/fixtures/spline-equation-state'
    names = sorted(path.relative_to(expected).as_posix() for path in expected.rglob('*') if path.is_file())
    assert names == sorted(path.relative_to(generated).as_posix() for path in generated.rglob('*') if path.is_file()), 'expected identical fixture file sets'
    for name in names:
        assert (expected / name).read_bytes() == (generated / name).read_bytes(), f'{name}; expected unchanged reviewed baseline'


def retain_compact_source_snapshot(raw: bytes) -> None:
    """Retain every original snapshot byte with a small index; e.g. retain_compact_source_snapshot(raw)."""
    snapshot = json.loads(raw)
    compressed = gzip.compress(raw, mtime=0)
    (FEATURE / 'owned-source-snapshot.json.gz').write_bytes(compressed)
    imported = {name: snapshot['source_hashes'][str(Path(origin).relative_to(Path(os.environ['BESSEMER_EQUATION_PIN']))).replace('\\', '/')]
                for name, origin in snapshot['source_origins'].items()}
    index = {key: snapshot[key] for key in ('source_python_files', 'source_origins', 'tests_run', 'successful', 'snapshot_identity')}
    index.update(original_snapshot_gzip='owned-source-snapshot.json.gz', original_snapshot_sha256=hashlib.sha256(raw).hexdigest(),
                 gzip_sha256=hashlib.sha256(compressed).hexdigest(), imported_source_hashes=imported)
    assert gzip.decompress(compressed) == raw, 'expected lossless complete snapshot retention'
    (FEATURE / 'owned-source-snapshot.json').write_bytes((json.dumps(index, indent=2) + '\n').encode())


def main() -> int:
    """Check the frozen reader, all owned bytes and import guard; e.g. main()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-equation-state-', dir=ROOT / 'target') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        generated = pin / 'owned-fixtures'
        result = run_owned_reference(pin, generated)
        destination = ROOT / 'target/equation-state-source-evidence'
        destination.mkdir(exist_ok=True)
        record_snapshot(pin, hashes, result, destination, ('sketchprefix',), ('swcontainer', 'resolvedfeatures', 'modelprefix', 'mfcobjects'))
        verify_owned_fixture_regeneration(generated)
        retain_compact_source_snapshot((destination / 'fixture-reference-snapshot.json').read_bytes())
        return 0 if result.wasSuccessful() else 1


if __name__ == '__main__':
    raise SystemExit(main())
