"""Regenerate the complete early scene roots and compare two isolated Rust processes."""
import sys
from generate_earliest_scene_roots import generate
from verify_earlier_processes import verify_groups


if __name__=="__main__":
    groups = [("earliest-scene-"+kind,generate) for kind in ["part","assembly"]]
    verify_groups(sys.argv[1],groups,"earliest-scene-verification.json",["modern_root","modern_part_tess","modern_pw_scene"])
