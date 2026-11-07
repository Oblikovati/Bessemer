"""C856 complete wire parents and derived-only migration operation packets (#166)."""
from collections import Counter
from typing import cast
import struct
from dialog_evidence import FEATURE, dialog_baseline
from dialog_owned import dialog_owned_cases
from dialog_operation_packets import persist_dialog_packets
from style_preferences_projection import raw_style_native_contexts
from hidden_migration_projection import hidden_migration_observation
from hidden_migration_cases import HiddenCase, HIDDEN_WORDS, hidden_operation_cases, hidden_case_packet


def verify_hidden_operation_packets() -> None:
    """Freeze4336 independent scoped/store/missing-word outcomes; e.g. verify_hidden_operation_packets()."""
    cases = list(hidden_operation_cases()); assert len(cases) == 4336
    rows = list(map(hidden_case_packet, cases))
    statuses = Counter(str(struct.unpack_from('<I', row, 12)[0]) for row in rows)
    assert set(statuses) == {'1', '2', '10', '11'}, statuses
    packet = struct.pack('<II', 1, len(cases)) + b''.join(rows)
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'hidden-migration-operations.bin': packet})
    dialog_baseline('hidden-migration-operation-packets-reference.json', dict(cases=len(cases), status_counts=dict(statuses), packets=summaries,
        oracle='Frozen migration model plus independent little-endian byte stores and unsigned decrement gate',
        wire_bytes_rewritten=False, out_of_scope_load_paths_inferred=False))


def stored_hidden_word(context: tuple[int, int, bytes]) -> int | None:
    """Read only the original optional1F0 while retaining the full observed parent; e.g. stored_hidden_word(context)."""
    fields = cast(list[dict[str, object]], hidden_migration_observation(*context)['fields'])
    return next((cast(int, field['signed_value']) for field in fields if field['label'] == 'style.word_1f0'), None)


def parent_hidden_cases(version: int, stored: int | None) -> list[HiddenCase]:
    """Keep actual version scope with eight explicit original/unknown/hypothetical words; e.g. parent_hidden_cases(700006, 0)."""
    return [(version, int(stored is not None), stored or 0), (version, 0, 0)] + [(version, 1, word) for word in (0, 1, 2, 3, 4, -2147483648)]


def projection_hidden_context(context: tuple[int, int, bytes]) -> bytes:
    """Retain complete bytes and original-word presence before hypothetical migrations; e.g. projection_hidden_context(context)."""
    family, version, raw = context
    stored = stored_hidden_word(context)
    cases = parent_hidden_cases(version, stored); assert len(cases) == 8
    prefix = struct.pack('<III', family, version, len(raw)) + raw
    return prefix + struct.pack('<III', int(stored is not None), (stored or 0) & 0xffffffff, len(cases)) + b''.join(map(hidden_case_packet, cases))


def hidden_mutant_contexts(owned: list[tuple[int, int, bytes]]) -> list[tuple[int, int, bytes]]:
    """Change only the actual stored1F0 across140 complete migration-range parents; e.g. hidden_mutant_contexts(owned)."""
    family, version, raw = next(context for context in owned if context[:2] == (2, 700006))
    fields = cast(list[dict[str, object]], hidden_migration_observation(family, version, raw)['fields'])
    offset = cast(int, next(field['offset'] for field in fields if field['label'] == 'style.word_1f0'))
    values = list(HIDDEN_WORDS); seed = 0x8561f0
    for _ in range(128):
        seed = (1664525 * seed + 1013904223) % (1 << 32); values.append(seed - (1 << 32) if seed >= 1 << 31 else seed)
    contexts: list[tuple[int, int, bytes]] = []
    for word in values:
        mutated = bytearray(raw); mutated[offset:offset + 4] = struct.pack('<i', word)
        contexts.append((family, version, bytes(mutated)))
    return contexts


def verify_hidden_projection_packets() -> None:
    """Freeze425 owned,140 complete1F0 mutants and276 native parents with unchanged original scope; e.g. verify_hidden_projection_packets()."""
    owned = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    native = raw_style_native_contexts(); assert (len(owned), len(native)) == (425, 276)
    mutants = hidden_mutant_contexts(owned); assert len(mutants) == 140
    packets = [struct.pack('<II', 1, len(contexts)) + b''.join(map(projection_hidden_context, contexts)) for contexts in (owned + mutants, native)]
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'hidden-migration-projection.bin': packets[0]})
    summaries.update(persist_dialog_packets(FEATURE.parents[2] / 'target', {'simulation-hidden-migration-projection-native.bin': packets[1]}))
    dialog_baseline('hidden-migration-projection-packets-reference.json', dict(owned_contexts=425, complete_word_1f0_mutants=140,
        native_contexts=276, hypothetical_contexts_per_parent=8, packets=summaries,
        out_of_scope_load_paths_inferred=False, source_wire_bytes_rewritten=False))
