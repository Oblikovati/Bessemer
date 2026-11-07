"""Independent raw-preserving offset and local fringe transition cases (#166)."""
from itertools import product
import importlib
import struct
from dialog_evidence import FEATURE, dialog_baseline
from dialog_operation_packets import persist_dialog_packets
from render_policy_packets import signed_gate_values


def offset_case(mode: int, word: int | None) -> bytes:
    """Keep decisive absent-input successes separate from genuinely missing required state; e.g. offset_case(1, None)."""
    try:
        outcome = importlib.import_module('cosworks_need_offset').need_offset(mode, word)
        assert outcome in (0, 1), outcome
    except ValueError as error:
        assert word is None and str(error) == 'loaded dialog48 required for this branch', (mode, word, error)
        outcome = 2
    return struct.pack('<iIiI', mode, int(word is not None), 0 if word is None else word, outcome)


def fringe_case(word14: int, word48: int) -> bytes:
    """Preserve both stored inputs, the local mutation and three conditional call-order comparisons; e.g. fringe_case(1, 1)."""
    view = importlib.import_module('cosworks_fringe_option').fringe_option(word14, word48)
    assert view['stored_word_48'] == word48, view
    need = importlib.import_module('cosworks_need_offset').need_offset
    order = [need(mode, word) for mode in (0, 1, 2) for word in (word48, view['runtime_word_48_after'])]
    return struct.pack('<4i6I', word14, word48, view['runtime_word_48_after'], view['fringe_option_result'], *order)


def owned_offset_pairs() -> list[tuple[int, int | None]]:
    """Exercise every signed neighbor in both arguments and every decisive/missing-state branch; e.g. owned_offset_pairs()."""
    values = signed_gate_values()
    critical = (-2147483648, -1, 0, 1, 2, 3, 4, 5, 6, 2147483647)
    pairs = set(product(values, (None, *critical))) | set(product(critical, values))
    return sorted(pairs, key=lambda pair: (pair[0], pair[1] is not None, 0 if pair[1] is None else pair[1]))


def owned_fringe_pairs() -> list[tuple[int, int]]:
    """Cover all return/mutation branches without restricting noncanonical signed words; e.g. owned_fringe_pairs()."""
    values = signed_gate_values()
    critical = (-2147483648, -1, 0, 1, 2, 3, 4, 5, 6, 2147483647)
    return sorted(set(product(values, critical)) | set(product(critical, values)))


def decision_packet(rows: list[bytes]) -> bytes:
    """Frame every independent result in deterministic input order; e.g. decision_packet(rows)."""
    return struct.pack('<II', 1, len(rows)) + b''.join(rows)


def verify_offset_fringe_packets() -> None:
    """Freeze both signed-neighbor matrices and their exact unchanged/runtime outputs; e.g. verify_offset_fringe_packets()."""
    offsets, fringes = owned_offset_pairs(), owned_fringe_pairs()
    packets = {'offset-request.bin': decision_packet([offset_case(*pair) for pair in offsets]),
               'fringe-option.bin': decision_packet([fringe_case(*pair) for pair in fringes])}
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', packets)
    dialog_baseline('offset-fringe-packets-reference.json', dict(offset_cases=len(offsets), fringe_cases=len(fringes),
        additional_stored_byte_credit=0, packets=summaries))
