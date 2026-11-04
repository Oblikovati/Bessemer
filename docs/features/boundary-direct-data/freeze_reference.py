"""Run complete direct-boundary oracles under immutable Source imports (#205)."""
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
    """Retain complete evidence and require exact reviewed fixture bytes; e.g. main()."""
    generated = ROOT / 'target/boundary-direct-data-source-evidence/fixtures'
    os.environ['BESSEMER_BOUNDARY_FIXTURES'] = str(generated)
    passed = run(FEATURE, 'boundary_data_fixtures', ('modelboundary',),
                 ('swcontainer', 'modelprefix', 'resolvedfeatures', 'mfcobjects', 'swarchive'))
    snapshot = FEATURE / 'fixture-reference-snapshot.json'
    retain_parameter_snapshot(snapshot.read_bytes(), 'source-snapshot', FEATURE)
    snapshot.unlink()
    if not passed:
        return 1
    verify_parameter_regeneration(generated, ROOT / 'tests/fixtures/boundary-direct-data')
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
