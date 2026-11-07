"""Regenerate complete width-count gate fixtures and force writes in two isolated processes."""
import sys
from generate_legacy_widths import generate
from verify_earlier_processes import verify_groups


if __name__=="__main__":
    verify_groups(sys.argv[1],[("legacy-width-roots",generate)],"legacy-width-verification.json",["modern_root","modern_item_text"])
