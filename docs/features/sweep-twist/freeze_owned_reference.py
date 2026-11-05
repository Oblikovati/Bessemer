"""Pin original sources before running complete sweep-twist oracle tests (#211)."""
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[3]
FEATURE = Path(__file__).resolve().parent
for folder in ('model-archives', 'stored-parameters', 'trim-edge-identifiers', 'sweeps'):
    sys.path.insert(0, str(ROOT / 'docs/features' / folder))
from pinned_reference_tests import run
from freeze_owned_reference import retain_parameter_snapshot
import os


def main() -> int:
    """Retain the complete imported-source provenance; e.g. main()."""
    os.environ['BESSEMER_SWEEP_TWIST_FIXTURES'] = str(ROOT / 'target/sweep-twist-owned-bare-control')
    passed = run(FEATURE, 'twist_wire_fixtures', ('modelcomponents', 'sweeps'),
                 ('mfcobjects', 'swarchive', 'swcontainer', 'modelprefix', 'resolvedfeatures'))
    snapshot = FEATURE / 'fixture-reference-snapshot.json'
    retain_parameter_snapshot(snapshot.read_bytes(), 'owned-source-snapshot', FEATURE)
    snapshot.unlink()
    return 0 if passed else 1


if __name__ == '__main__':
    raise SystemExit(main())
