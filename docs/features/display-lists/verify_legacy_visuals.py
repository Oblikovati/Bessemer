"""Regenerate complete appearance-gate fixtures and force writes in two isolated processes."""
import sys
from generate_legacy_visuals import generate
from verify_earlier_processes import verify_groups


if __name__=="__main__":
    verify_groups(sys.argv[1],[("legacy-visual-roots",generate)],"legacy-visual-verification.json",["modern_root","modern_material"])
