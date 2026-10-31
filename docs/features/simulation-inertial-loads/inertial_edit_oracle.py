"""Complete independent edited images replayed by actual inertial readers (#136)."""
from typing import cast
from owned_inertial import LOAD_KINDS, owned_inertial_cases
from native_carriers import ThermalCarrier
from thermal_edit_oracle import scalar_edit_case


def actual_inertial_parser(raw: bytes, version: int, kind: str) -> dict[str, object]:
    """Call the complete actual parser with the full actual callback; e.g. actual_inertial_parser(raw, 870022, 'Gravity')."""
    from cosworks_inertial_layout import parse as inertial
    from cosworks_base_excitation_layout import parse as excitation
    from cosworks_binary_thermal_transform_audit import archive_reader
    parser = excitation if kind == 'BaseExcitation' else inertial
    return cast(dict[str, object], parser(raw, version, kind, archive_reader=archive_reader))


def owned_inertial_edits() -> bytes:
    """Freeze full edited images for all owned contexts; e.g. owned_inertial_edits()."""
    return b''.join(scalar_edit_case(kind, version, raw, LOAD_KINDS, actual_inertial_parser)
                    for kind, version, raw in owned_inertial_cases())


def native_inertial_edits(row: ThermalCarrier, raw: bytes) -> bytes:
    """Preserve every original native version context; e.g. native_inertial_edits(row, raw)."""
    kind = LOAD_KINDS.index(row['stream'].split('/')[-1])
    return b''.join(scalar_edit_case(kind, cast(int, attempt['version']), raw, LOAD_KINDS, actual_inertial_parser)
                    for attempt in row['attempts'])
