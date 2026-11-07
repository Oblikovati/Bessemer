"""Regenerate complete line-visibility fixtures and force writes in two isolated processes."""
import sys
from generate_legacy_visibility import generate
from verify_earlier_processes import verify_groups


if __name__=="__main__":
    verify_groups(sys.argv[1],[("legacy-visibility-roots",generate)],"legacy-visibility-verification.json",["modern_root","modern_bucket_poly"])
