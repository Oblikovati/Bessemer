"""Regenerate complete component-mode fixtures and force writes in two isolated processes."""
import sys
from generate_legacy_modes import generate
from verify_earlier_processes import verify_groups


if __name__=="__main__":
    verify_groups(sys.argv[1],[("legacy-mode-roots",generate)],"legacy-mode-verification.json",["modern_root","modern_component"])
