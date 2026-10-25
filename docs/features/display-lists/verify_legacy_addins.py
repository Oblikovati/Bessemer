"""Regenerate complete body/add-in gate fixtures and force writes in two isolated processes."""
import sys
from generate_legacy_addins import generate
from verify_earlier_processes import verify_groups


if __name__=="__main__":
    verify_groups(sys.argv[1],[("legacy-addin-roots",generate)],"legacy-addin-verification.json",["modern_root","modern_material","modern_body_tess"])
