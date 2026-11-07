"""Complete unmodified single-body parser boundary shared by mass Property oracles (#138)."""
from typing import cast


def actual_mass_parser(raw: bytes, version: int, kind: str = 'MassElement') -> dict[str, object]:
    """Use the complete actual callback with an explicit body kind; e.g. actual_mass_parser(raw, 830036)."""
    from cosworks_mass_element_layout import parse
    from cosworks_binary_thermal_transform_audit import archive_reader
    return cast(dict[str, object], parse(raw, version, kind, archive_reader=archive_reader))
