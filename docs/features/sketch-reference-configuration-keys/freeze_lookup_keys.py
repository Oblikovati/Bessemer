"""Freeze Source imports before the inherited transform/context helpers (#188)."""
from pathlib import Path
import sys
FEATURE = Path(__file__).resolve().parent
for name in ('model-archives', 'resolved-features', 'dimension-stored-state'):
    sys.path.insert(0, str(FEATURE.parent / name))
from pinned_reference_tests import run

if __name__ == '__main__':
    raise SystemExit(0 if run(FEATURE, 'lookup_key_owned', ('modelreferences', 'sketchprefix')) else 1)
