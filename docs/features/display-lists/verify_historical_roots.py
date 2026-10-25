"""Replay complete historical roots in two isolated reconstruction processes."""
import sys
from generate_historical_roots import generate
from generate_historical_buckets import generate as bucket
from verify_earlier_processes import verify_groups


if __name__=="__main__":
    groups = [("historical-roots-"+kind,generate) for kind in ["part","assembly"]]
    groups += [("historical-buckets-"+dimension,bucket) for dimension in ["3d","2d"]]
    verify_groups(sys.argv[1],groups,"historical-root-verification.json",["modern_root","modern_scene","modern_walls","modern_pw_scene","modern_component","modern_item_style","modern_bucket"])
