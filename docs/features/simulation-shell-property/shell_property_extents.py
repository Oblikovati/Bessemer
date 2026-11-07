"""All shorter complete-context shell extents and trailing bytes remain explicit failures (#159)."""
from owned_shell_property import OwnedShellProperty, owned_shell_properties
from shell_property_malformed import actual_shell_case
from shell_property_evidence import shell_baseline


def verify_shell_case_extents(case: OwnedShellProperty) -> int:
    """Check every prefix under the original explicit context; e.g. verify_shell_case_extents(case)."""
    raw = case['raw']
    for end in range(len(raw)):
        parsed = actual_shell_case(case, raw[:end])
        assert not parsed['success'], (case['kind'], case['version'], end, parsed)
    assert not actual_shell_case(case, raw + b'\x00')['success']
    return len(raw)


def verify_shell_extents() -> None:
    """Retain the full denominator rather than sampling truncation offsets; e.g. verify_shell_extents()."""
    cases = owned_shell_properties()
    prefixes = sum(verify_shell_case_extents(case) for case in cases)
    shell_baseline('extent-reference.json', dict(contexts=len(cases), proper_prefixes=prefixes, accepted_prefixes=0, rejected_trailing_cases=len(cases)))
