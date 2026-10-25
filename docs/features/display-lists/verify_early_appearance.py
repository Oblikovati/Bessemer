"""Regenerate and rebuild complete early appearance roots in two processes."""
import sys
from generate_early_appearance_roots import generate as full_root
from generate_early_appearance_buckets import generate as bucket_root
from verify_earlier_processes import verify_groups


if __name__=="__main__":
    groups = [("early-appearance-roots-"+kind,full_root) for kind in ["part","assembly"]]
    groups += [("early-appearance-buckets-"+dimension,bucket_root) for dimension in ["3d","2d"]]
    verify_groups(sys.argv[1],groups,"early-appearance-verification.json",["modern_root","modern_material","modern_body_tess","modern_concept","modern_item_text","modern_item_geometry","modern_bucket"])
