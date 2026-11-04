"""Freeze original Source before importing any upstream parameter readers (#202)."""
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


def run_parameter_reference(pin: Path, destination: Path) -> unittest.TestResult:
    """Install the immutable import guard before reader-dependent helpers; e.g. run_parameter_reference(pin,out)."""
    sys.meta_path.insert(0, FrozenSourceImports(pin))
    sys.path.insert(0, str(FEATURE))
    sys.path.insert(0, str(ROOT / 'docs/features/trim-edge-identifiers'))
    os.environ['BESSEMER_PARAMETER_PIN'] = str(pin)
    os.environ['BESSEMER_PARAMETER_FIXTURES'] = str(destination)
    import freeze_trim_edges
    suite = unittest.defaultTestLoader.discover(str(FEATURE), pattern='test_owned_reference.py')
    return unittest.TextTestRunner(verbosity=2).run(suite)


def retain_parameter_snapshot(raw: bytes, prefix: str = 'owned-source-snapshot') -> None:
    """Keep the complete original snapshot with a bounded index; e.g. retain_parameter_snapshot(raw)."""
    snapshot = json.loads(raw)
    compressed = gzip.compress(raw, mtime=0)
    assert gzip.decompress(compressed) == raw, 'expected lossless complete Source snapshot'
    (FEATURE / f'{prefix}.json.gz').write_bytes(compressed)
    index = {key: snapshot[key] for key in ('source_python_files', 'source_origins', 'tests_run', 'successful', 'snapshot_identity')}
    index.update(original_snapshot_gzip=f'{prefix}.json.gz',
                 original_snapshot_sha256=hashlib.sha256(raw).hexdigest(), gzip_sha256=hashlib.sha256(compressed).hexdigest())
    (FEATURE / f'{prefix}.json').write_bytes((json.dumps(index, indent=2) + '\n').encode())


def verify_parameter_regeneration(generated: Path, expected: Path | None = None) -> None:
    """Compare every reviewed fixture byte without updating goldens; e.g. verify_parameter_regeneration(out)."""
    expected = expected or ROOT / 'tests/fixtures/stored-parameters'
    if not expected.exists():
        return
    names = sorted(path.name for path in expected.iterdir() if path.is_file())
    assert names == sorted(path.name for path in generated.iterdir() if path.is_file()), 'expected identical parameter fixture sets'
    for name in names:
        assert (expected / name).read_bytes() == (generated / name).read_bytes(), f'{name}; expected unchanged owned parameter bytes'


def main() -> int:
    """Retain evidence only after original-reader acceptance and guard checks; e.g. main()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-parameters-', dir=ROOT / 'target') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        evidence = ROOT / 'target/stored-parameter-source-evidence'
        evidence.mkdir(exist_ok=True)
        generated = evidence / 'fixtures'
        result = run_parameter_reference(pin, generated)
        record_snapshot(pin, hashes, result, evidence, ('modelnode', 'dimensions', 'attributes'),
                        ('swcontainer', 'resolvedfeatures', 'modelprefix', 'mfcobjects'))
        if not result.wasSuccessful():
            return 1
        verify_parameter_regeneration(generated)
        retain_parameter_snapshot((evidence / 'fixture-reference-snapshot.json').read_bytes())
        return 0


if __name__ == '__main__':
    raise SystemExit(main())
