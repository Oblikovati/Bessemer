"""Independent owned seven-family bodies and asymmetric Extra controls (#160)."""
from typing import TypedDict
import struct
from owned_shell_property import shell_property_literal
from owned_material import material_literal
from owned_composite import flip_turn_case, composite_extra_case


class OwnedInventoryCase(TypedDict):
    family: int
    version: int
    raw: bytes
    outcomes: list[tuple[str, bool]]


def owned_shell_inventory() -> list[OwnedInventoryCase]:
    """Keep explicit reader failures beside complete writer and field controls; e.g. owned_shell_inventory()."""
    cases = [OwnedInventoryCase(family=0, version=800027, raw=struct.pack('<i', -2147483648), outcomes=[('reader', True)]),
             OwnedInventoryCase(family=1, version=890017, raw=struct.pack('<i', 0), outcomes=[('reader', True)])]
    for kind in (0, 1):
        body = shell_property_literal(kind, 840002, 0)
        cases.append(OwnedInventoryCase(family=kind + 2, version=840002, raw=body['raw'], outcomes=[('reader', True)]))
    cases.append(OwnedInventoryCase(family=4, version=830036, raw=material_literal(830036, 1), outcomes=[('reader', True)]))
    flip = flip_turn_case(840002, 'reader', 3, 1)
    cases.append(OwnedInventoryCase(family=5, version=840002, raw=flip['raw'], outcomes=[('reader', True), ('writer', True)]))
    for selector in (0, 1):
        extra = composite_extra_case(840002, 'writer', selector, 3, None)
        cases.append(OwnedInventoryCase(family=6, version=840002, raw=extra['raw'], outcomes=[('reader', selector == 1), ('writer', True)]))
    cases.append(OwnedInventoryCase(family=6, version=840030, raw=struct.pack('<i', 0), outcomes=[('reader', False), ('writer', False)]))
    return cases
