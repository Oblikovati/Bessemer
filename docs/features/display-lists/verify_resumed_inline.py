"""Regenerate complete pre-object inline archives and force writes in two isolated processes."""
import sys
from generate_resumed_inline import generate
from verify_earlier_processes import verify_groups


if __name__=="__main__":
    verify_groups(sys.argv[1],[("resumed-inline-roots",generate)],"resumed-inline-verification.json",["modern_root","modern_body_tess","modern_part_tess"])
