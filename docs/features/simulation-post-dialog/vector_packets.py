"""Independent complete branch-neighbor matrices and raw vector provenance packets (#166)."""
from itertools import product
from typing import cast
import importlib
import struct
from dialog_evidence import FEATURE, dialog_baseline
from dialog_operation_packets import persist_dialog_packets
from render_policy_packets import signed_gate_values
from vector_evidence import vector_reports

SOURCES = {'DGXPOST_fixed': 0, 'table': 1, 'constant_or_default': 2}


def vector_result_packet(view: dict[str, object]) -> bytes:
    """Keep origin, selected table even on fallback, and all four raw output words; e.g. vector_result_packet(view)."""
    rva = view['table_rva']
    output = bytes.fromhex(cast(str, view['output_raw_hex']))
    assert len(output) == 16, view
    return struct.pack('<III', SOURCES[cast(str, view['source'])], int(rva is not None), 0 if rva is None else int(cast(str, rva), 16)) + output


def provider_case(inputs: tuple[int, int, int, int]) -> bytes:
    """Use the actual copied provider model as the independent result oracle; e.g. provider_case((0, 1, 0, 0))."""
    view = importlib.import_module('cosworks_component_vector_policy').component_vector(*inputs)
    return struct.pack('<4i', *inputs) + vector_result_packet(view)


def composed_case(inputs: tuple[int, int, int, int], selector: int, present: bool) -> bytes:
    """Retain explicit loaded CString state and local-branch precedence; e.g. composed_case((0, 1, 0, 0), 5, True)."""
    view = importlib.import_module('cosworks_component_vector_policy').vector_view(*inputs, selector, loaded_text_nonempty=present)
    return struct.pack('<5iI', *inputs, selector, int(present)) + vector_result_packet(view)


def provider_arguments() -> list[tuple[int, int, int, int]]:
    """Cross all dispatch families, aliases, integer extremes and every table boundary; e.g. provider_arguments()."""
    analysis = sorted(set(range(13)) | {-2147483648, -1, 2147483647})
    plots = sorted(set(range(24)) | {-2147483648, -1, 56, 57, 58, 2147483647})
    components = [-2147483648, -2, -1, *range(33), 2147483647]
    return list(product(analysis, plots, components, (-2147483648, -1, 0, 1, 2, 2147483647)))


def composed_arguments() -> list[tuple[tuple[int, int, int, int], int, bool]]:
    """Check every signed selector neighbor with representative table/fallback and arbitrary raw provider inputs; e.g. composed_arguments()."""
    providers = ((0, 1, 0, 0), (-2147483648, 2147483647, -1, -1), (5, 2, 13, 0), (8, 19, -1, -2147483648))
    return list(product(providers, signed_gate_values(), (False, True)))


def verify_vector_packets() -> None:
    """Freeze the provider and composed matrices and require reachability of every original static entry; e.g. verify_vector_packets()."""
    arguments = provider_arguments()
    providers = [provider_case(inputs) for inputs in arguments]
    covered = {(struct.unpack_from('<I', row, 24)[0], inputs[2]) for inputs, row in zip(arguments, providers, strict=True)
               if struct.unpack_from('<I', row, 16)[0] == 1}
    tables = cast(list[dict[str, object]], vector_reports()[1]['tables'])
    expected = {(int(cast(str, table['rva']), 16), index) for table in tables for index in range(cast(int, table['count']))}
    assert covered == expected and len(covered) == 262, (len(covered), len(expected))
    composed = [composed_case(*args) for args in composed_arguments()]
    packets = {name: struct.pack('<II', 1, len(rows)) + b''.join(rows) for name, rows in
        (('component-vector.bin', providers), ('vector-direction.bin', composed))}
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', packets)
    dialog_baseline('vector-packets-reference.json', dict(provider_cases=len(providers), composed_cases=len(composed),
        static_entries_covered=len(covered), additional_stored_byte_credit=0, packets=summaries))
