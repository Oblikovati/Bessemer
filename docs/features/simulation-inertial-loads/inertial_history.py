"""Complete edited fields and physical occurrence histories from actual readers (#136)."""
from typing import cast
from owned_inertial import LOAD_KINDS, owned_inertial_cases
from inertial_edit_oracle import actual_inertial_parser
from inertial_projection import inertial_shape
from native_carriers import ThermalCarrier
from thermal_history_projection import history_projection_case
from thermal_nested_history import nested_history_case


def inertial_history_case(kind: int, version: int, raw: bytes, nested: bool) -> bytes:
    """Replay the complete actual parser and every source occurrence; e.g. inertial_history_case(0, 870022, raw, True)."""
    project = nested_history_case if nested else history_projection_case
    return project(kind, version, raw, LOAD_KINDS, actual_inertial_parser, inertial_shape)


def owned_inertial_history(nested: bool) -> bytes:
    """Retain every complete historical owned context; e.g. owned_inertial_history(True)."""
    return b''.join(inertial_history_case(kind, version, raw, nested)
                    for kind, version, raw in owned_inertial_cases())


def native_inertial_history(row: ThermalCarrier, raw: bytes, nested: bool) -> bytes:
    """Preserve every retained native attempt in report order; e.g. native_inertial_history(row, raw, True)."""
    kind = LOAD_KINDS.index(row['stream'].split('/')[-1])
    return b''.join(inertial_history_case(kind, cast(int, attempt['version']), raw, nested)
                    for attempt in row['attempts'])
