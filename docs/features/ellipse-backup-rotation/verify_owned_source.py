"""Run ellipse fixture checks against immutable original Source readers (#192)."""
from pathlib import Path
import sys

FEATURE = Path(__file__).resolve().parent
sys.path.insert(0, str(FEATURE.parent/'model-archives'))
from pinned_reference_tests import run

if __name__ == '__main__':
    passed = run(FEATURE, 'freeze_owned_ellipse', ('sketchprefix', 'modelprefix', 'configurationmanager'))
    raise SystemExit(0 if passed else 1)
