"""Frozen complete pane/line/symbol roots under two isolated reconstruction processes."""
import sys
from generate_legacy_panes import generate
from verify_earlier_processes import verify_groups


if __name__=="__main__":
    verify_groups(sys.argv[1],[("legacy-pane-roots",generate)],"legacy-pane-verification.json",["modern_root","modern_ui","modern_item_geometry","modern_item_text","modern_item_style","modern_assembly"])
