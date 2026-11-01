"""Independent signed32 branch matrices for the named rendering decisions (#166)."""
from itertools import product
import importlib
import struct
from dialog_evidence import FEATURE, dialog_baseline
from dialog_operation_packets import persist_dialog_packets


def layer0_case(word: int, present: bool) -> bytes:
    """Retain the exact raw control and explicit runtime presence beside the Source result; e.g. layer0_case(-1, True)."""
    result = importlib.import_module('cosworks_legend_render_gate').enters_body(word, plot_object_present=present)
    return struct.pack('<iII', word, int(present), int(result))


def tessellation_case(inputs: tuple[int, ...]) -> bytes:
    """Evaluate the actual copied six-input model without invoking native code; e.g. tessellation_case((0, 0, 0, 0, 53, 2))."""
    assert len(inputs) == 6, inputs
    result = importlib.import_module('cosworks_need_model_tess').need_model_tess(*inputs)
    assert result in (0, 1), result
    return struct.pack('<6iI', *inputs, result)


def signed_gate_values() -> list[int]:
    """Include every power boundary, both signed extremes and independent mixed words; e.g. signed_gate_values()."""
    values = {-2147483648, -1, 0, 1, 2, 2147483647}
    values.update(sign * ((1 << shift) + neighbor) for shift in range(1, 31) for sign in (-1, 1) for neighbor in (-1, 0, 1))
    values.update(struct.unpack('<i', struct.pack('<I', (index * 0x9e3779b9) & 0xffffffff))[0] for index in range(256))
    return sorted(values)


def verify_render_policy_packets() -> None:
    """Freeze signed gate neighbors and the complete declared tessellation control cross-product; e.g. verify_render_policy_packets()."""
    gates = [layer0_case(word, present) for word, present in product(signed_gate_values(), (False, True))]
    inputs = product((-2147483648, -1, 0, 1, 2147483647), (-1, 0, 2), (-1, 0, 1),
                     (-2147483648, -1, 0, 1, 2, 2147483647), (-2147483648, -1, 0, 3, 4, 5, 52, 53, 54, 64, 65, 66, 2147483647),
                     (-1, 0, 1, 2, 3, 2147483647))
    tess = [tessellation_case(words) for words in inputs]
    packets = {name: struct.pack('<II', 1, len(rows)) + b''.join(rows) for name, rows in
               (('legend-layer0-gate.bin', gates), ('model-tessellation.bin', tess))}
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', packets)
    dialog_baseline('render-policy-packets-reference.json', dict(gate_cases=len(gates), tessellation_cases=len(tess),
        additional_stored_byte_credit=0, packets=summaries))
