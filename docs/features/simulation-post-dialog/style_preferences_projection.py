"""Complete C-885 raw-word observations retain conditional absence and initializer counterexamples (#166)."""
from collections import Counter
from typing import cast
import struct
from native_carriers import digest
from dialog_evidence import FEATURE, dialog_baseline
from dialog_owned import dialog_owned_cases
from dialog_projection import actual_post_record
from dialog_operation_packets import accepted_dialog_native_cases, persist_dialog_packets
from local_render_evidence import local_render_report
from style_preferences_evidence import style_preferences_report, RAW_STYLE_INITIAL


def raw_style_observations(family: int, version: int, raw: bytes) -> dict[str, dict[str, object]]:
    """Derive only present canonical raw words after complete EOF; e.g. raw_style_observations(0, 890620, raw)."""
    parsed = actual_post_record(family, version, raw)
    assert parsed['success'] and parsed['end'] == len(raw), (family, version, parsed)
    spans = {span['label']: span for span in cast(list[dict[str, object]], parsed['spans'])}
    observed: dict[str, dict[str, object]] = {}
    for member, initial in RAW_STYLE_INITIAL.items():
        span = spans.get(f'style.word_{member:x}')
        if span is None: continue
        wire = bytes.fromhex(cast(str, span['raw_hex']))
        assert len(wire) == 4, (member, wire)
        word = struct.unpack('<I', wire)[0]
        observed[hex(member)] = dict(offset=span['offset'], raw_hex=wire.hex(), uint32=word, equals_initializer=word == initial)
    return observed


def raw_style_native_contexts() -> list[tuple[int, int, bytes]]:
    """Keep every explicit accepted parent/style context from the complete operation packet; e.g. raw_style_native_contexts()."""
    packet = (FEATURE.parents[2] / 'target/simulation-post-dialog-native.bin').read_bytes()
    contexts = [case for case in accepted_dialog_native_cases(packet) if case[0] in (0, 2)]
    assert len(contexts) == 276, len(contexts)
    return contexts


def raw_style_identity(row: dict[str, object]) -> tuple[object, ...]:
    """Preserve caller identity beside content identity and version; e.g. raw_style_identity(row)."""
    return tuple(row[key] for key in ('path', 'stream', 'sha256', 'selected_version'))


def verify_style_preferences_native() -> None:
    """Replay all276 original identities and each present raw value, offset and initializer comparison; e.g. verify_style_preferences_native()."""
    report = style_preferences_report()
    rows = cast(list[dict[str, object]], report['rows'])
    identities = {raw_style_identity(row): 0 if row['kind'] == 'cwPostDlgParam' else 2
        for row in cast(list[dict[str, object]], local_render_report()['rows'])}
    assert Counter(raw_style_identity(row) for row in rows) == Counter(identities.keys())
    available = {(family, digest(raw), version): raw_style_observations(family, version, raw)
        for family, version, raw in raw_style_native_contexts()}
    for row in rows:
        key = identities[raw_style_identity(row)], row['sha256'], row['selected_version']
        assert available[key] == row['values'], row
    counts, matches = raw_style_native_counts(rows)
    assert counts == report['native_counts'] and matches == report['initializer_match_counts']
    dialog_baseline('raw-preferences-native-reference.json', dict(explicit_input_sets=276,
        native_counts=counts, initializer_match_counts=matches, additional_stored_byte_credit=0))


def raw_style_native_counts(rows: list[dict[str, object]]) -> tuple[dict[str, dict[str, int]], dict[str, int]]:
    """Retain smaller presence denominators and stored counterexamples to initialization; e.g. raw_style_native_counts(rows)."""
    counts: dict[str, Counter[str]] = {hex(member): Counter() for member in RAW_STYLE_INITIAL}
    matches: Counter[str] = Counter()
    for row in rows:
        for member, observation in cast(dict[str, dict[str, object]], row['values']).items():
            counts[member][str(observation['uint32'])] += 1
            matches[member] += observation['equals_initializer'] is True
    return {member: dict(values) for member, values in counts.items()}, dict(matches)


def raw_style_projection_context(family: int, version: int, raw: bytes) -> bytes:
    """Frame complete bytes and twelve independent presence/raw pairs; e.g. raw_style_projection_context(2, 700005, raw)."""
    observed = raw_style_observations(family, version, raw)
    packet = bytearray(struct.pack('<III', family, version, len(raw)) + raw)
    for member in RAW_STYLE_INITIAL:
        entry = observed.get(hex(member))
        packet.extend(struct.pack('<II', int(entry is not None), 0 if entry is None else cast(int, entry['uint32'])))
    return bytes(packet)


def verify_style_preferences_projection() -> None:
    """Freeze complete owned/native views with absent fields left absent; e.g. verify_style_preferences_projection()."""
    owned = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    native = raw_style_native_contexts()
    assert len(owned) == 425, len(owned)
    packets = [struct.pack('<II', 1, len(cases)) + b''.join(raw_style_projection_context(*case) for case in cases)
        for cases in (owned, native)]
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'raw-preferences-projection.bin': packets[0]})
    summaries.update(persist_dialog_packets(FEATURE.parents[2] / 'target', {'simulation-raw-preferences-projection-native.bin': packets[1]}))
    dialog_baseline('raw-preferences-projection-reference.json', dict(owned_contexts=425, native_contexts=276,
        members=[hex(member) for member in RAW_STYLE_INITIAL], additional_stored_byte_credit=0, packets=summaries))
