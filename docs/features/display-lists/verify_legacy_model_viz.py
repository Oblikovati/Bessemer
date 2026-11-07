"""Regenerate complete early model and light archives and verify two isolated writers."""
import sys
from generate_legacy_model_viz import generate
from verify_earlier_processes import verify_groups


if __name__=="__main__":
    verify_groups(sys.argv[1],[("legacy-model-viz-roots",generate)],"legacy-model-viz-verification.json",["modern_root","modern_body_tess","modern_part_tess","modern_item_text","modern_material","modern_ui","modern_component","legacy_bucket_poly","modern_bucket","textured_planes","legacy_bucket_visibility","modern_lights"])
