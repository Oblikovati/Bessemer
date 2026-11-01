"""C857 complete parent contexts retain wire words beside derived-only zero events (#166)."""
from collections import Counter
from typing import cast
import struct
from dialog_evidence import FEATURE, dialog_baseline
from dialog_owned import dialog_owned_cases
from dialog_operation_packets import persist_dialog_packets
from style_preferences_projection import raw_style_native_contexts
from legend_resets_evidence import LEGEND_RESET_LABELS
from legend_resets_projection import legend_resets_observation
from legend_resets_cases import reset_operation_versions, reset_case_packet


def verify_reset_operation_packets() -> None:
    """Freeze4137 versions across all scope/zero/empty intervals and exact edge neighborhoods; e.g. verify_reset_operation_packets()."""
    versions = list(reset_operation_versions()); assert len(versions) == 4137
    rows = list(map(reset_case_packet, versions))
    statuses = Counter(str(struct.unpack_from('<I', row, 4)[0]) for row in rows)
    assert set(statuses) == {'1', '2', '10'}, statuses
    counts = Counter(str(struct.unpack_from('<I', row, 8)[0]) for row in rows)
    assert set(counts) == {'0', '2', '3'}, counts
    packet = struct.pack('<II', 1, len(rows)) + b''.join(rows)
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'legend-resets-operations.bin': packet})
    dialog_baseline('legend-resets-operation-packets-reference.json', dict(cases=len(rows), status_counts=dict(statuses), assignment_counts=dict(counts),
        packets=summaries, oracle='Frozen exact reset model with explicit native member order and distinct unsupported scope',
        wire_offsets_assigned=False, constructor_defaults_inferred=False))


def stored_reset_words(view: dict[str, object]) -> tuple[int, ...]:
    """Keep all three optional actual wire patterns, including unknown signed values; e.g. stored_reset_words(view)."""
    fields = {field['label']: field for field in cast(list[dict[str, object]], view['fields'])}
    mask = sum(1 << index for index, label in enumerate(LEGEND_RESET_LABELS) if label in fields)
    words = [cast(int, fields[label]['signed_value']) & 0xffffffff if label in fields else 0 for label in LEGEND_RESET_LABELS]
    return (mask, *words)


def projection_reset_context(context: tuple[int, int, bytes]) -> bytes:
    """Retain full parent bytes and actual wire presence before version-only assignment projection; e.g. projection_reset_context(context)."""
    family, version, raw = context
    view = legend_resets_observation(*context)
    prefix = struct.pack('<III', family, version, len(raw)) + raw
    return prefix + struct.pack('<4I', *stored_reset_words(view)) + reset_case_packet(version)


def reset_wire_mutants(owned: list[tuple[int, int, bytes]]) -> list[tuple[int, int, bytes]]:
    """Change all three actual later-profile words without inventing reset/default semantics; e.g. reset_wire_mutants(owned)."""
    family, version, raw = next(context for context in owned if context[0] == 2 and context[1] >= 840010)
    fields = cast(list[dict[str, object]], legend_resets_observation(family, version, raw)['fields'])
    offsets = {cast(str, field['label']): cast(int, field['offset']) for field in fields}
    patterns = [(0, 0, 0), (0xffffffff, 0x80000000, 0x7fffffff), (1, 2, 3), (0x7f800001, 0x80000001, 0xffffffff)]
    seed = 0x857c0
    for _ in range(128):
        words: list[int] = []
        for _ in range(3):
            seed = (1664525 * seed + 1013904223) % (1 << 32); words.append(seed)
        patterns.append(tuple(words))
    contexts: list[tuple[int, int, bytes]] = []
    for words in patterns:
        mutated = bytearray(raw)
        for label, word in zip(LEGEND_RESET_LABELS, words, strict=True):
            offset = offsets[label]; mutated[offset:offset + 4] = struct.pack('<I', word)
        contexts.append((family, version, bytes(mutated)))
    return contexts


def verify_reset_projection_packets() -> None:
    """Freeze425 owned,132 complete later-word mutants and276 native parents without byte mutation; e.g. verify_reset_projection_packets()."""
    owned = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    native = raw_style_native_contexts(); assert (len(owned), len(native)) == (425, 276)
    mutants = reset_wire_mutants(owned); assert len(mutants) == 132
    packets = [struct.pack('<II', 1, len(contexts)) + b''.join(map(projection_reset_context, contexts)) for contexts in (owned + mutants, native)]
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'legend-resets-projection.bin': packets[0]})
    summaries.update(persist_dialog_packets(FEATURE.parents[2] / 'target', {'simulation-legend-resets-projection-native.bin': packets[1]}))
    dialog_baseline('legend-resets-projection-packets-reference.json', dict(owned_contexts=425, complete_later_wire_mutants=132,
        native_contexts=276, packets=summaries, reset_events_independent_of_stored_values=True,
        unsupported_raw_class_as_empty=False, zero_assignments_as_stored_fields=False))
