"""Regenerate graph-relocated roots and verify two isolated writers."""
import sys
from generate_legacy_zero import generate
from verify_earlier_processes import verify_groups


if __name__=="__main__":
    verify_groups(sys.argv[1],[("legacy-zero-roots",generate)],"legacy-zero-verification.json",["modern_items","../../examples/support/item_trace","../tessellation/groups","../tessellation/arrays","../tessellation/facets","modern_root","modern_assembly","modern_body_tess","modern_part_tess","modern_item_text","modern_material","modern_ui","modern_component","legacy_bucket_poly","modern_bucket","textured_planes","legacy_bucket_visibility","modern_lights"])
