"""Regenerate complete property graphs and compare two isolated reconstruction processes."""
import sys
from generate_component_properties import generate
from verify_earlier_processes import verify_groups


if __name__=="__main__":
    groups = [(name,generate) for name in ["component-properties","component-property-holders-part","component-property-holders-assembly"]]
    verify_groups(sys.argv[1],groups,"component-property-verification.json",["archive_objects","modern_component_properties","modern_ui","modern_item_style"])
