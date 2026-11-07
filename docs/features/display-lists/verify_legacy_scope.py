"""Regenerate complete pre-object inline archives and force writes in two isolated processes."""
import sys
from generate_legacy_scope import generate
from verify_earlier_processes import verify_groups


if __name__=="__main__":
    verify_groups(sys.argv[1],[("legacy-scope-roots",generate)],"legacy-scope-verification.json",["modern_root","modern_body_tess","modern_part_tess","modern_item_text","modern_material","modern_ui","modern_component","legacy_bucket_poly","modern_bucket","textured_planes","legacy_bucket_visibility"])
