"""Frozen unsigned-version roots through two isolated complete reconstruction processes."""
from generate_unsigned_versions import generate
from verify_earlier_processes import verify_groups
import sys
from pathlib import Path
from verify_unsigned_gates import NAMES,comparisons


if __name__=="__main__":
    for name in NAMES:comparisons(Path("../inventor-interface/solidworks")/(name+".py"))
    verify_groups(sys.argv[1],[("unsigned-versions",generate)],"unsigned-verification.json",["modern_root","modern_material","modern_body_tess","configurations","modern_bucket","archive_layout"])
