"""Independent numeric unit tuples with explicit missing-context outcomes (#166)."""
from itertools import product
from typing import TypeAlias
import importlib
import struct
from dialog_evidence import FEATURE, dialog_baseline
from dialog_operation_packets import persist_dialog_packets
from render_policy_packets import signed_gate_values

UnitArguments: TypeAlias = tuple[int, int, int, int | None, int | None, int | None]


def unit_result(arguments: UnitArguments) -> tuple[int, int]:
    """Keep numeric fallback codes separate from the two required-context failures; e.g. unit_result((1, 1, 0, None, None, None))."""
    helper = importlib.import_module('cosworks_component_unit_type')
    try:
        return 0, helper.unit_type(*arguments)
    except helper.MissingUnitContext as error:
        assert str(error) in ('global57b68 required', 'loaded BE8/BEC context required'), str(error)
        return (1 if str(error) == 'global57b68 required' else 2), 0


def unit_case(arguments: UnitArguments) -> bytes:
    """Encode exact optional-word presence and the independent numeric/error result; e.g. unit_case((0, 1, 0, None, 0, None))."""
    analysis, plot, component, *context = arguments
    mask = sum(1 << index for index, word in enumerate(context) if word is not None)
    status, code = unit_result(arguments)
    return struct.pack('<3iI3iIi', analysis, plot, component, mask,
        *(0 if word is None else word for word in context), status, code)


def unit_argument_matrix() -> list[UnitArguments]:
    """Cross all dispatch boundaries and context short-circuits, then every signed context neighbor; e.g. unit_argument_matrix()."""
    analyses = [-2147483648, -1, *range(13), 2147483647]
    plots = sorted(set(range(24)) | set(range(52, 60)) | set(range(64, 72)) | {-2147483648, -1, 2147483647})
    components = [-2147483648, -1, *range(32), 2147483647]
    contexts = ((None, None, None), (None, 0, None), (0, None, 0), (2, None, 1),
                (None, 1, 2), (2, -1, 1), (2, 1, 2), (3, 1, -1))
    rows = {(a, p, c, *context) for a, p, c, context in product(analyses, plots, components, contexts)}
    optional = (None, -2147483648, -1, 0, 1, 2, 3, 2147483647)
    rows.update((8, p, c, be8, bec, None) for p, c, be8, bec in product((1, 2, 19, 20),
        (-2147483648, -1, 0, 3, 4, 6, 7, 8, 10, 11, 14, 15, 2147483647), optional, optional))
    rows.update((a, 1, c, None, None, flag) for a, c, flag in product((1, 2), range(8), (None, *signed_gate_values())))
    return sorted(rows, key=lambda row: tuple((word is not None, 0 if word is None else word) for word in row))


def unit_packet(rows: list[bytes]) -> bytes:
    """Retain every independent tuple in a stable versioned packet; e.g. unit_packet(rows)."""
    return struct.pack('<II', 1, len(rows)) + b''.join(rows)


def verify_unit_packets() -> None:
    """Freeze the full declared boundary/context matrix with distinct known-code and missing-context results; e.g. verify_unit_packets()."""
    arguments = unit_argument_matrix()
    cases = [unit_case(row) for row in arguments]
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'unit-type.bin': unit_packet(cases)})
    counts = {status: sum(struct.unpack_from('<I', row, 28)[0] == status for row in cases) for status in range(3)}
    assert all(count > 0 for count in counts.values()), counts
    dialog_baseline('unit-packets-reference.json', dict(cases=len(cases), outcome_counts=counts,
        additional_stored_byte_credit=0, packets=summaries))
