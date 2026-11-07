"""Frozen complete older roots through two isolated reconstruction processes."""
import sys
from generate_pre_facet_roots import generate
from verify_earlier_processes import verify_groups


if __name__=="__main__":
    verify_groups(sys.argv[1],[("pre-facet-roots",generate)],"pre-facet-verification.json",["modern_root","modern_assembly","modern_item_style","modern_item_geometry","modern_body_tess","archive_layout"])
