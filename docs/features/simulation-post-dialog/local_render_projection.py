"""Every original native local predicate input and complete owned/native parent/style projection (#166)."""
from collections import Counter
from typing import cast
import struct
from native_carriers import digest
from dialog_evidence import FEATURE, dialog_baseline
from dialog_field_evidence import dialog_field_reports
from dialog_owned import dialog_owned_cases
from dialog_projection import actual_post_record
from dialog_operation_packets import accepted_dialog_native_cases, persist_dialog_packets
from local_render_evidence import local_render_report
from local_render_packets import local_render_case, local_render_packet

PREDICATE_PATHS = ('legend.word_10', 'legend.word_14', 'legend.word_ac', 'legend.word_b0')


def local_render_spans(family: int, version: int, raw: bytes) -> dict[str, dict[str, object]]:
    """Require full EOF before deriving a conditional stored-input view; e.g. local_render_spans(2, 890702, raw)."""
    parsed = actual_post_record(family, version, raw)
    assert parsed['success'] and parsed['end'] == len(raw), (family, version, parsed)
    return {cast(str, span['label']): span for span in cast(list[dict[str, object]], parsed['spans'])}


def local_render_span_case(spans: dict[str, dict[str, object]]) -> bytes | None:
    """Keep old missing loaded fields unavailable rather than insert migration defaults; e.g. local_render_span_case(spans)."""
    if not all(path in spans for path in PREDICATE_PATHS): return None
    words = [bytes.fromhex(cast(str, spans[path]['raw_hex'])) for path in PREDICATE_PATHS]
    assert all(len(raw) == 4 for raw in words), words
    counts = [struct.unpack('<i', raw)[0] for raw in words[:2]]
    positions = [struct.unpack('<I', raw)[0] for raw in words[2:]]
    return local_render_case(*counts, *positions)


def native_local_render_cases() -> list[tuple[int, int, bytes]]:
    """Retain all explicit accepted parent/style helper contexts, without raw legacy migration guesses; e.g. native_local_render_cases()."""
    packet = (FEATURE.parents[2] / 'target/simulation-post-dialog-native.bin').read_bytes()
    cases = [case for case in accepted_dialog_native_cases(packet) if case[0] in (0, 2)]
    assert len(cases) == 276, len(cases)
    return cases


def verify_local_render_native() -> None:
    """Check all276 original identities, wire offsets, raw words and conditional results from complete bytes; e.g. verify_local_render_native()."""
    report = local_render_report()
    rows = cast(list[dict[str, object]], report['rows'])
    source = cast(list[dict[str, object]], dialog_field_reports()[2]['rows'])
    identities = Counter((row['path'], row['stream'], row['sha256'], attempt['version']) for row in source
        for attempt in cast(list[dict[str, object]], row['attempts']) if attempt['success'])
    original = Counter((row['path'], row['stream'], row['sha256'], row['selected_version']) for row in rows)
    assert original == identities, (len(original), len(identities))
    available = {(family, digest(raw), version): local_render_spans(family, version, raw)
                 for family, version, raw in native_local_render_cases()}
    cases = [verify_original_local_render(row, available) for row in rows]
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'target',
        {'simulation-local-render-native.bin': local_render_packet(cases)})
    dialog_baseline('local-render-native-reference.json', dict(original_explicit_input_sets=276,
        modeled_true=sum(struct.unpack_from('<I', row, 16)[0] for row in cases),
        additional_stored_byte_credit=0, packets=summaries))


def verify_original_local_render(row: dict[str, object], available: dict[tuple[int, str, int], dict[str, dict[str, object]]]) -> bytes:
    """Match the exact original spans before computing any local predicate; e.g. verify_original_local_render(row, available)."""
    family = 0 if row['kind'] == 'cwPostDlgParam' else 2
    assert row['kind'] in ('cwPostDlgParam', 'DspPostStyle'), row['kind']
    spans = available[(family, cast(str, row['sha256']), cast(int, row['selected_version']))]
    expected = cast(dict[str, dict[str, object]], row['inputs'])
    assert tuple(expected) == PREDICATE_PATHS, tuple(expected)
    for path in PREDICATE_PATHS:
        assert (spans[path]['offset'], spans[path]['raw_hex']) == (expected[path]['offset'], expected[path]['raw_hex']), (row, path)
    case = local_render_span_case(spans)
    assert case is not None and bool(struct.unpack_from('<I', case, 16)[0]) == row['modeled_result'], row
    return case


def local_render_context(family: int, version: int, raw: bytes) -> bytes:
    """Retain complete input bytes and distinguish missing old state from a completed false result; e.g. local_render_context(2, 700005, raw)."""
    case = local_render_span_case(local_render_spans(family, version, raw))
    return struct.pack('<IIII', family, version, len(raw), int(case is not None)) + raw + (b'' if case is None else case)


def verify_local_render_projection() -> None:
    """Freeze every complete owned/native context, including raw legacy states outside this explicit-input model; e.g. verify_local_render_projection()."""
    owned = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    native = native_local_render_cases()
    assert len(owned) == 425, len(owned)
    packets = [local_render_packet([local_render_context(*case) for case in cases]) for cases in (owned, native)]
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'local-render-projection.bin': packets[0]})
    summaries.update(persist_dialog_packets(FEATURE.parents[2] / 'target', {'simulation-local-render-projection-native.bin': packets[1]}))
    dialog_baseline('local-render-projection-reference.json', dict(owned_contexts=425, native_contexts=276,
        additional_stored_byte_credit=0, packets=summaries))
