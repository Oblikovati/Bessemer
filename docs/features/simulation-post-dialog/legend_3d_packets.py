"""C852 local integer/constructor expectations and complete count/precision parent packets (#166)."""
from collections import Counter
from typing import cast
import struct
from dialog_evidence import FEATURE, dialog_baseline
from dialog_owned import dialog_owned_cases
from dialog_operation_packets import persist_dialog_packets
from style_preferences_projection import raw_style_native_contexts
from legend_3d_projection import legend_3d_observation
from legend_3d_cases import COUNTS_3D, PRECISIONS_3D, Case3D, operation_3d_cases, expected_3d_words, case_3d_packet, signed_3d_word


def verify_3d_operation_packets() -> None:
    """Freeze26784 independent local requests and every missing-fact ordering; e.g. verify_3d_operation_packets()."""
    cases = list(operation_3d_cases())
    assert len(cases) == 26784
    statuses = Counter(str(expected_3d_words(case)[0]) for case in cases)
    assert set(statuses) == {'1', '2', '3', '10', '11', '12', '20', '21', '22'}, statuses
    packet = struct.pack('<II', 1, len(cases)) + b''.join(map(case_3d_packet, cases))
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'legend-3d-operations.bin': packet})
    dialog_baseline('legend-3d-operation-packets-reference.json', dict(cases=len(cases), status_counts=dict(statuses), packets=summaries,
        oracle='Independent modulo32/unbounded saturation arithmetic, exact plain-argument branches and original frozen nearest-even constructor',
        stored_contour_or_color_identity_inferred=False, renderer_executed=False))


def parent_3d_cases(family: int, version: int, raw: bytes) -> list[Case3D]:
    """Supply sixteen explicit hypothetical phase contexts beside actual count/precision fields; e.g. parent_3d_cases(2, 700006, raw)."""
    fields = {field['label']: cast(int, field['signed_value']) for field in legend_3d_observation(family, version, raw)}
    entry, precision = fields.get('legend.word_14'), fields.get('legend.word_bc')
    body_facts = ((None, 2), (entry, 2), (0, 0), (-1, 2), (2147483647, 2), (12, -2147483648), (12, None), (None, None))
    cases: list[Case3D] = []
    for later, argument in body_facts:
        mask = int(entry is not None) | (int(later is not None) << 1) | (int(argument is not None) << 2)
        cases.append((0, mask, entry or 0, later or 0, argument or 0))
    backgrounds = ((None, precision, 0), (0, precision, None), (0, precision, 0), (0xffffffff, precision, 1),
        (0xff000000, precision, 128), (0x12345678, 2147483647, 2), (0xff8000, -2147483648, 255), (1, None, 1))
    for packed, actual_precision, lighting in backgrounds:
        mask = int(packed is not None) | (int(actual_precision is not None) << 1) | (int(lighting is not None) << 2)
        cases.append((1, mask, packed or 0, actual_precision or 0, lighting or 0))
    return cases


def projection_3d_context(context: tuple[int, int, bytes]) -> bytes:
    """Retain the full parent and all sixteen complete independent local outcomes; e.g. projection_3d_context(context)."""
    family, version, raw = context
    cases = parent_3d_cases(family, version, raw)
    assert len(cases) == 16
    return struct.pack('<III', family, version, len(raw)) + raw + struct.pack('<I', len(cases)) + b''.join(map(case_3d_packet, cases))


def pattern_3d_contexts(owned: list[tuple[int, int, bytes]]) -> list[tuple[int, int, bytes]]:
    """Mutate only actual count/precision words across452 complete standalone parents; e.g. pattern_3d_contexts(owned)."""
    family, version, raw = next(context for context in owned if context[:2] == (2, 700006))
    fields = {field['label']: cast(int, field['offset']) for field in legend_3d_observation(family, version, raw)}
    pairs = [(count, precision) for count in COUNTS_3D for precision in PRECISIONS_3D]
    seed = 0x8523d
    for _ in range(256):
        seed = (1664525 * seed + 1013904223) % (1 << 32); count = signed_3d_word(seed)
        seed = (1664525 * seed + 1013904223) % (1 << 32); pairs.append((count, signed_3d_word(seed)))
    contexts: list[tuple[int, int, bytes]] = []
    for count, precision in pairs:
        mutated = bytearray(raw)
        for label, value in (('legend.word_14', count), ('legend.word_bc', precision)):
            start = fields[label]; mutated[start:start + 4] = struct.pack('<i', value)
        contexts.append((family, version, bytes(mutated)))
    return contexts


def verify_3d_projection_packets() -> None:
    """Freeze425 owned,452 complete scalar mutants and276 native parents without phase inference; e.g. verify_3d_projection_packets()."""
    owned = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    native = raw_style_native_contexts()
    assert (len(owned), len(native)) == (425, 276)
    mutants = pattern_3d_contexts(owned); assert len(mutants) == 452
    packets = [struct.pack('<II', 1, len(contexts)) + b''.join(map(projection_3d_context, contexts)) for contexts in (owned + mutants, native)]
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'legend-3d-projection.bin': packets[0]})
    summaries.update(persist_dialog_packets(FEATURE.parents[2] / 'target', {'simulation-legend-3d-projection-native.bin': packets[1]}))
    dialog_baseline('legend-3d-projection-packets-reference.json', dict(owned_contexts=425, native_contexts=276,
        complete_count_precision_mutants=452, hypothetical_contexts_per_parent=16, packets=summaries,
        unchanged_later_count_inferred=False, packed_color_from_file_inferred=False, native_target_executed=False))
