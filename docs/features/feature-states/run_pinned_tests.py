"""Run complete feature-states fixture checks with immutable actual reference readers (#54)."""
from pathlib import Path
import sys

FEATURE_DIRECTORY=Path(__file__).resolve().parent
sys.path.insert(0,str(FEATURE_DIRECTORY.parent/'model-archives'))
from pinned_reference_tests import run

if __name__=='__main__':sys.exit(0 if run(FEATURE_DIRECTORY,'freeze_feature_states',('flatpatterns',)) else 1)
