"""Actual copied-source adapters for every retained SolidManager family (#154)."""
from typing import cast
from solid_inventory_bindings import SolidInventoryPayload
from freeze_body import actual_body
from freeze_sheet import actual_sheet
from freeze_persist import actual_persist_id
from material_reference import actual_material_parser, retained_outcome


def actual_solid_inventory(payload: SolidInventoryPayload, raw: bytes, version: int) -> dict[str, object]:
    """Use an explicit persisted family and original version; e.g. actual_solid_inventory(payload, raw, 840002)."""
    from cosworks_solid_manager_layout import parse as manager
    from cosworks_solid_component_layout import parse as component
    from cosworks_surface_body_layout import parse as surface
    family = payload['family']
    if family == 4: return retained_outcome(version, actual_material_parser(raw, version))
    readers = {0: manager, 1: component, 2: actual_body, 3: actual_sheet, 5: surface, 6: actual_persist_id}
    assert family in readers, f'{family}; expected an explicit supported SolidManager family'
    return dict(version=version, **cast(dict[str, object], readers[family](raw, version)))
