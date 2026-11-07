"""Run owned caller tests through the shared immutable Source harness (#184)."""
from pathlib import Path
import sys

FEATURE = Path(__file__).resolve().parent
for relative in ('model-archives', 'resolved-features', 'dimension-stored-state'):
    sys.path.insert(0, str(FEATURE.parent / relative))
sys.path.insert(0, str(FEATURE.parents[2] / 'target/reference-python'))
from pinned_reference_tests import run

if __name__ == '__main__':
    raise SystemExit(0 if run(FEATURE, 'owned_intersection_match', ('sketchintersections', 'sketchtail')) else 1)
