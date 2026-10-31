"""Independent complete owned bodies and an explicit failed SurfaceBody occurrence (#154)."""
from solid_inventory_bindings import SolidInventoryPayload
from solid_inventory_replay import actual_solid_inventory
from native_carriers import digest
from owned_solid_manager import owned_solid_headers
from owned_children import owned_component_headers
from owned_body import owned_solid_bodies
from owned_sheet import owned_sheets
from owned_material import material_literal
from owned_surface import owned_surface_bodies
from owned_persist import owned_component_persist_ids


def owned_solid_payload(family: int, ordinal: int, suffix: str, version: int, raw: bytes) -> SolidInventoryPayload:
    """Keep synthetic occurrences separate from native identities; e.g. owned_solid_payload(0, 0, 'Property', 0, raw)."""
    payload = SolidInventoryPayload(document=f'owned/solid-inventory-{ordinal}.sldprt',
              stream='Cosworks/StudyManager/StudyNumber-0/SolidManager/' + suffix,
              family=family, size=len(raw), sha256=digest(raw), attempts=[])
    payload['attempts'] = [actual_solid_inventory(payload, raw, version)]
    return payload


def owned_solid_inventory() -> list[tuple[SolidInventoryPayload, bytes]]:
    """Combine low/populated owned profiles and a failed body without inventing native evidence; e.g. owned_solid_inventory()."""
    component = 'Solid Components/Component owned/'
    definitions = ((0, 'Property', owned_solid_headers()), (1, component + 'Property', owned_component_headers()),
                   (2, component + 'Solid Bodies/Body owned/Property', owned_solid_bodies()),
                   (3, component + 'Sheet Bodies/Sheet owned/Property', owned_sheets()),
                   (4, component + 'Solid Bodies/Body owned/Material', [(0, 891107, material_literal(891107, 1))]),
                   (5, component + 'Sheet Bodies/Sheet owned/SurfaceBody', owned_surface_bodies()),
                   (6, component + 'Component PersistID', owned_component_persist_ids()))
    cases: list[tuple[SolidInventoryPayload, bytes]] = []
    for family, suffix, profiles in definitions:
        for ordinal, (_, version, raw) in enumerate((profiles[0], profiles[-1])):
            payload = owned_solid_payload(family, ordinal, suffix, version, raw)
            assert payload['attempts'][0]['success']
            cases.append((payload, raw))
    payload, raw = cases[-4]
    version = int(payload['attempts'][0]['version'])
    incomplete = owned_solid_payload(5, 2, component + 'Sheet Bodies/Sheet owned/SurfaceBody', version, raw[:-1])
    assert not incomplete['attempts'][0]['success']
    return sorted([*cases, (incomplete, raw[:-1])], key=lambda case: (case[0]['document'], case[0]['stream']))
