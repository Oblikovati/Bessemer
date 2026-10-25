"""Regenerate complete pre-object inline archives and force writes in two isolated processes."""
import sys
from generate_legacy_rendering import generate
from verify_earlier_processes import verify_groups


if __name__=="__main__":
    verify_groups(sys.argv[1],[("legacy-rendering-roots",generate)],"legacy-rendering-verification.json",["modern_root","modern_body_tess","modern_part_tess","modern_item_text","modern_material"])
