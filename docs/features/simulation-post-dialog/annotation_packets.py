"""Reviewed zero/nonzero truth classes cover annotation and material-only gates (#166)."""
from itertools import product
import struct
from render_policy_packets import signed_gate_values
from dialog_evidence import FEATURE, dialog_baseline
from dialog_operation_packets import persist_dialog_packets

ANNOTATION_TRUTH = {(0, 0): (0, 0, 0), (1, 0): (1, 0, 1), (0, 1): (0, 1, 1), (1, 1): (1, 1, 1)}
MATERIAL_TRUE = {(1, 1, 1), (1, 1, 2), (1, 1, 3)}


def annotation_pairs() -> list[tuple[int, int]]:
    """Exercise every signed boundary in either control against all basic sign classes; e.g. annotation_pairs()."""
    others = (-2147483648, -1, 0, 1, 2147483647)
    pairs = {(word, other) for word, other in product(signed_gate_values(), others)}
    return sorted(pairs | {(right, left) for left, right in pairs})


def annotation_policy_case(minimum: int, maximum: int, render: int, plot: bool) -> bytes:
    """Use independently reviewed truth tables rather than the Rust predicate expressions; e.g. annotation_policy_case(-1, 0, 1, True)."""
    classes = {0: 0}
    minimum_flag, maximum_flag, values_flag = ANNOTATION_TRUTH[(classes.get(minimum, 1), classes.get(maximum, 1))]
    mask = minimum_flag | maximum_flag << 1
    material = int((classes.get(render, 1), int(plot), mask) in MATERIAL_TRUE)
    return struct.pack('<iiiIIIII', minimum, maximum, render, int(plot), minimum_flag, maximum_flag, values_flag, material)


def verify_annotation_packets() -> None:
    """Freeze all truth classes, including the minimum-only case absent from native observations; e.g. verify_annotation_packets()."""
    pairs = annotation_pairs()
    render_values = (-2147483648, -1, 0, 1, 2147483647)
    rows = [annotation_policy_case(minimum, maximum, render, plot)
        for (minimum, maximum), render, plot in product(pairs, render_values, (False, True))]
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog',
        {'annotation-policy.bin': struct.pack('<II', 1, len(rows)) + b''.join(rows)})
    dialog_baseline('annotation-policy-reference.json', dict(control_pairs=len(pairs), render_arguments=5,
        plot_presence_states=2, cases=len(rows), oracle='Four annotation truth classes and three material-allowed classes bound to exact native guards',
        excluded='Extrema computation, coordinate selection and actual rendering; the two glyph helper calls are outside the material gate',
        additional_stored_byte_credit=0, packets=summaries))
