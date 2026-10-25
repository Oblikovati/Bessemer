"""Regenerate complete region-word gate fixtures and force writes in two isolated processes."""
import sys
from generate_legacy_regions import generate
from verify_earlier_processes import verify_groups


if __name__=="__main__":
    verify_groups(sys.argv[1],[("legacy-region-roots",generate)],"legacy-region-verification.json",["modern_root","legacy_bucket_poly"])
