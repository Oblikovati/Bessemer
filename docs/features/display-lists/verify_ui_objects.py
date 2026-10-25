"""Reconstruct independently regenerated complete UI-object roots in two processes."""
import sys
from generate_ui_objects import generate
from verify_earlier_processes import verify_groups


if __name__=="__main__":
    verify_groups(sys.argv[1],[("line-styles",generate),("layer-maps",generate)],"ui-object-verification.json",["archive_objects","modern_line_style","modern_layers"])
