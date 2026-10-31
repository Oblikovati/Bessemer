"""Exhaustive independent Mesh Property truncation and trailing-byte outcomes (#163)."""
from property_evidence import property_baseline
from property_native import actual_property
from owned_property import owned_properties


def verify_property_extents() -> None:
    """Check every proper physical prefix, without sampling or version substitution; e.g. verify_property_extents()."""
    cases = owned_properties()
    checked = 0
    for case in cases:
        raw = case['raw']
        for end in range(len(raw)):
            assert not actual_property(raw[:end], case['version'])['success'], (case['version'], case['profile'], end)
            checked += 1
        assert not actual_property(raw + b'\0', case['version'])['success'], (case['version'], case['profile'])
    property_baseline('extent-reference.json', dict(contexts=len(cases), proper_prefixes=checked,
                      accepted_prefixes=0, trailing_byte_rejections=len(cases)))
