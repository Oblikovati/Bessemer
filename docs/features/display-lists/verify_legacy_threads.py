"""Regenerate complete thread-pattern gate fixtures and force writes in two isolated processes."""
import sys
from generate_legacy_threads import generate
from verify_earlier_processes import verify_groups


if __name__=="__main__":
    verify_groups(sys.argv[1],[("legacy-thread-roots",generate)],"legacy-thread-verification.json",["modern_root","modern_bucket","legacy_bucket_poly"])
