"""Complete edited-field and composed physical-occurrence history expectations (#138)."""
from owned_mass import owned_mass_cases
from mass_reference import actual_mass_parser
from mass_projection import mass_shape
from thermal_history_projection import history_projection_case
from thermal_nested_history import nested_history_case


def mass_history_case(version: int, raw: bytes, nested: bool) -> bytes:
    """Replay the complete source body and independently observed physical parents; e.g. mass_history_case(830036, raw, True)."""
    project = nested_history_case if nested else history_projection_case
    return project(0, version, raw, ('MassElement',), actual_mass_parser, mass_shape)


def owned_mass_history(nested: bool) -> bytes:
    """Keep every complete owned historical and selector context; e.g. owned_mass_history(True)."""
    return b''.join(mass_history_case(version, raw, nested) for _, version, raw in owned_mass_cases())
