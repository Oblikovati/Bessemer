"""Freeze original Source and label the local sweep-twist grammar extension (#211)."""
from pathlib import Path
import os
import sys

ROOT = Path(__file__).resolve().parents[3]
FEATURE = Path(__file__).resolve().parent
sys.path.insert(0, str(ROOT / 'docs/features/model-archives'))
sys.path.insert(0, str(ROOT / 'docs/features/stored-parameters'))
sys.path.insert(0, str(ROOT / 'docs/features/trim-edge-identifiers'))
from pinned_reference_tests import run
from freeze_owned_reference import retain_parameter_snapshot, verify_parameter_regeneration


def main() -> int:
    """Pin original code before importing any reader; e.g. main()."""
    generated = ROOT / 'target/sweep-twist-source-evidence/bare-fixtures'
    os.environ['BESSEMER_SWEEP_TWIST_FIXTURES'] = str(generated)
    passed = run(FEATURE, 'twist_wire_fixtures', ('modelcomponents',), ('mfcobjects', 'swarchive'))
    snapshot = FEATURE / 'fixture-reference-snapshot.json'
    retain_parameter_snapshot(snapshot.read_bytes(), 'bare-source-snapshot', FEATURE)
    snapshot.unlink()
    if not passed:
        return 1
    verify_parameter_regeneration(generated, ROOT / 'tests/fixtures/sweep-twist')
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
