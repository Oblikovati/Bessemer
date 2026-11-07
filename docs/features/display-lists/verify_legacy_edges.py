"""Regenerate complete temporary-edge fixtures and force writes in two isolated processes."""
import sys
from generate_legacy_edges import generate
from verify_earlier_processes import verify_groups


if __name__=="__main__":
    verify_groups(sys.argv[1],[("legacy-edge-roots",generate)],"legacy-edge-verification.json",["modern_root","modern_bucket","legacy_bucket_visibility"])
