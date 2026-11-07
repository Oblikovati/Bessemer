"""Independent constant-surface and plane-section archive literals for C-737 (#151)."""
import struct
from owned_components import signed_words, holder_words
from owned_embedded import literal_wrapper
from owned_chains import tagged_component
from owned_references import component_bodies, component_reference_body

INNER_VERSIONS = (4280, 4281, 4306, 4307, 7101, 7102, 14000, 19000, 19001)


def constant_surface_literal(version: int, presence: int) -> bytes:
    """Keep the exact 4281 exception and modern +A8 gate; e.g. constant_surface_literal(4281, 0)."""
    raw = literal_wrapper(version)
    if version > 7101: raw += struct.pack('<I', 0xffffffff)
    return raw + signed_words(presence)


def sheet_component_literal(version: int) -> bytes:
    """Encode the distinct complete 4281 continuation independently; e.g. sheet_component_literal(4281)."""
    if version >= 4307: return component_bodies(version)['moCompSolidBody_c']
    return holder_words(version) + bytes(2) + signed_words(2, -2147483648, 2147483647, -108, 104, -100)


def sheet_embedded_literals() -> list[bytes]:
    """Include supported boundaries, nonzero kernel stops and unrelated names; e.g. sheet_embedded_literals()."""
    cases: list[bytes] = []
    for version in INNER_VERSIONS:
        for presence in (-1, 0, 1):
            body = constant_surface_literal(version, presence)
            cases.append(struct.pack('<I', version) + tagged_component('moConstSurfRef_w', body))
        for name in ('moCompSurfaceBody_c', 'moCompPlaneSectionBody_c'):
            cases.append(struct.pack('<I', version) + tagged_component(name, sheet_component_literal(version)))
    for version in (4281, 14000):
        for name in ('moConstSurfRef_alias_w', 'moCompPlaneSectionBody_alias_c'):
            body = constant_surface_literal(version, 0)
            cases.append(struct.pack('<I', version) + tagged_component(name, body))
    for name in ('moSolidRef_w', 'moCompSolidBody_c', 'moCompHolder_c'):
        body = constant_surface_literal(4281, 0) if name == 'moSolidRef_w' else sheet_component_literal(4281)
        cases.append(struct.pack('<I', 4281) + tagged_component(name, body))
    for version in INNER_VERSIONS:
        body = component_reference_body(version) + signed_words(-100)
        cases.append(struct.pack('<I', version) + tagged_component('moCompRefSurface_c', body))
    return cases
