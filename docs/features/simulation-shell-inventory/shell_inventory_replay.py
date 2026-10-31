"""Fresh dispatch preserving each original ShellManager argument and boundary (#160)."""
from typing import cast
from shell_inventory_bindings import ShellInventoryPayload
from material_reference import actual_material_parser, retained_outcome


def actual_shell_inventory(payload: ShellInventoryPayload, raw: bytes, attempt: dict[str, object]) -> dict[str, object]:
    """Use the original profile without a success-based fallback; e.g. actual_shell_inventory(payload, raw, attempt)."""
    from cosworks_shell_manager_layout import parse as manager
    from cosworks_shell_property_layout import parse as shell
    from cosworks_composite_layout import parse as composite
    from cosworks_embedded_const_surface import archive_reader
    family, version = payload['family'], cast(int, attempt['version'])
    if family < 2:
        return dict(version=version, **manager(raw, version, 'Property' if family == 0 else 'ShellManagerGroup'))
    if family < 4:
        return dict(version=version, **shell(raw, version, 'Shell', archive_reader=archive_reader))
    if family == 4: return retained_outcome(version, actual_material_parser(raw, version))
    profile = cast(str, attempt['wire_profile'])
    assert family in (5, 6) and profile in ('reader', 'writer'), (family, profile)
    parsed = composite(raw, version, 'CompositeFlipTurn' if family == 5 else 'CompositeExtra',
                       wire_profile=profile, archive_reader=archive_reader)
    return dict(version=version, wire_profile=profile, **parsed)
