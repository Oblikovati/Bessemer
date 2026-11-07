"""C860 operation cases and complete original A8 parent projections (#166)."""
from collections import Counter
import struct
from dialog_evidence import FEATURE, dialog_baseline
from dialog_owned import dialog_owned_cases
from dialog_operation_packets import persist_dialog_packets
from style_preferences_projection import raw_style_native_contexts
from legend_inverse_projection import inverse_observation
from legend_inverse_cases import inverse_operation_cases, inverse_case_packet, INVERSE_WORD_EDGES


def verify_inverse_operation_packets() -> None:
    """Freeze all scalar boundaries and generated getter/setter cases with exact unknown/skip distinctions; e.g. verify_inverse_operation_packets()."""
    cases = list(inverse_operation_cases()); assert len(cases) == 12678
    rows = list(map(inverse_case_packet, cases))
    statuses = Counter(str(struct.unpack_from('<I', row, 20)[0]) for row in rows)
    assert set(statuses) == {'1', '2', '10', '11', '12'}, statuses
    routes = Counter(str(struct.unpack_from('<I', row, 24)[0]) for row in rows)
    assert set(routes) == {'0', '1', '2', '3'}, routes
    packet = struct.pack('<II', 1, len(rows)) + b''.join(rows)
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'legend-inverse-operations.bin': packet})
    dialog_baseline('legend-inverse-operation-packets-reference.json', dict(cases=len(rows), status_counts=dict(statuses),
        route_counts=dict(routes), packets=summaries, oracle='Original frozen zero_test plus independent unsigned subtraction/bounded BT mask',
        universal_setter_policy=False, archived_a8_normalized=False))


def inverse_wire_mutants(owned: list[tuple[int, int, bytes]]) -> list[tuple[int, int, bytes]]:
    """Change one complete owned parent's actual A8 word across boundaries and generated patterns; e.g. inverse_wire_mutants(owned)."""
    family, version, raw = next(context for context in owned if context[0] == 2 and context[1] >= 700006)
    observed = inverse_observation(family, version, raw); assert observed is not None
    offset = int(observed['offset']); patterns = list(INVERSE_WORD_EDGES); seed = 0x860
    for _ in range(128):
        seed = (1664525 * seed + 1013904223) % (1 << 32); patterns.append(seed)
    contexts: list[tuple[int, int, bytes]] = []
    for word in patterns:
        mutated = bytearray(raw); mutated[offset:offset + 4] = struct.pack('<I', word & 0xffffffff)
        contexts.append((family, version, bytes(mutated)))
    return contexts


def inverse_projection_parent(context: tuple[int, int, bytes]) -> bytes:
    """Retain complete bytes and original optional A8 before one getter projection; e.g. inverse_projection_parent(context)."""
    family, version, raw = context; observed = inverse_observation(*context)
    word = 0 if observed is None else int(observed['stored_a8'])
    case = (0, 0, 0, int(observed is not None), word)
    return struct.pack('<III', family, version, len(raw)) + raw + inverse_case_packet(case)


def verify_inverse_projection_packets() -> None:
    """Freeze425 owned,140 complete A8 mutants and276 native parents, preserving wire absence; e.g. verify_inverse_projection_packets()."""
    owned = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    native = raw_style_native_contexts(); assert (len(owned), len(native)) == (425, 276)
    mutants = inverse_wire_mutants(owned); assert len(mutants) == 140
    packets = [struct.pack('<II', 1, len(contexts)) + b''.join(map(inverse_projection_parent, contexts)) for contexts in (owned + mutants, native)]
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'legend-inverse-projection.bin': packets[0]})
    summaries.update(persist_dialog_packets(FEATURE.parents[2] / 'target', {'simulation-legend-inverse-projection-native.bin': packets[1]}))
    dialog_baseline('legend-inverse-projection-packets-reference.json', dict(owned_contexts=425, complete_a8_mutants=140,
        native_contexts=276, packets=summaries, absent_words_defaulted=False, original_wire_words_preserved=True))
