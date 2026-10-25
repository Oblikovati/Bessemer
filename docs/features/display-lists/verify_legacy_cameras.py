"""Regenerate frozen camera-boundary roots and reconstruct in two isolated processes."""
import sys
from generate_legacy_cameras import generate
from verify_earlier_processes import verify_groups


if __name__=="__main__":
    verify_groups(sys.argv[1],[("legacy-camera-roots",generate)],"legacy-camera-verification.json",["modern_root","modern_component","modern_item_text"])
