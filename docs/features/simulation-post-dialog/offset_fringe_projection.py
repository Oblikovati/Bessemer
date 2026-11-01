"""Complete native and owned dialog decisions, including genuine missing runtime fields (#166)."""
from collections import Counter
from typing import cast
import importlib
import struct
from native_carriers import digest
from dialog_evidence import FEATURE, dialog_baseline, dialog_reports
from dialog_owned import dialog_owned_cases
from dialog_projection import actual_post_record
from dialog_operation_packets import accepted_dialog_native_cases, persist_dialog_packets
from offset_fringe_evidence import offset_fringe_reports
from offset_fringe_packets import offset_case, fringe_case, decision_packet


def parent_decision_spans(version: int, raw: bytes) -> dict[str, dict[str, object]]:
    """Require exact full EOF before extracting named controls; e.g. parent_decision_spans(890620, raw)."""
    parsed = actual_post_record(0, version, raw)
    assert parsed['success'] and parsed['end'] == len(raw), (version, parsed)
    return {cast(str, span['label']): span for span in cast(list[dict[str, object]], parsed['spans'])}


def offset_fringe_inputs(spans: dict[str, dict[str, object]]) -> tuple[int, int, int | None]:
    """Preserve signed bits and actual word48 absence without choosing a runtime default; e.g. offset_fringe_inputs(spans)."""
    header = bytes.fromhex(cast(str, spans['fourteen_words0c_28_30_3c_518_51c']['raw_hex']))
    assert len(header) == 56, len(header)
    mode = struct.unpack('<i', bytes.fromhex(cast(str, spans['style.word_1b8']['raw_hex'])))[0]
    word14 = struct.unpack_from('<i', header, 8)[0]
    stored48 = spans.get('moving_objects.word_48')
    word48 = None if stored48 is None else struct.unpack('<i', bytes.fromhex(cast(str, stored48['raw_hex'])))[0]
    return mode, word14, word48


def native_parent_decisions() -> list[tuple[int, bytes]]:
    """Retain all complete native interpretations, without adding failed/legacy fallback arguments; e.g. native_parent_decisions()."""
    packet = (FEATURE.parents[2] / 'target/simulation-post-dialog-native.bin').read_bytes()
    cases = [(version, raw) for family, version, raw in accepted_dialog_native_cases(packet) if family == 0]
    assert len(cases) == 224 and sum(len(raw) for _, raw in cases) == 496979, (len(cases), sum(len(raw) for _, raw in cases))
    return cases


def verify_offset_fringe_native() -> None:
    """Match every original native identity and every raw control/output against complete input bytes; e.g. verify_offset_fringe_native()."""
    offsets, fringes = offset_fringe_reports()
    current = cast(list[dict[str, object]], dialog_reports()[4]['rows'])
    identities = Counter((row['path'], row['stream'], row['sha256'], attempt['version']) for row in current
        for attempt in cast(list[dict[str, object]], row['attempts']) if attempt.get('complete'))
    assert Counter((row['path'], row['stream'], row['sha256'], row['selected_version'])
        for row in cast(list[dict[str, object]], offsets['rows'])) == identities
    available = {(digest(raw), version): parent_decision_spans(version, raw) for version, raw in native_parent_decisions()}
    offset_rows = [verify_original_offset(row, available) for row in cast(list[dict[str, object]], offsets['rows'])]
    fringe_rows = [verify_original_fringe(row, available) for row in cast(list[dict[str, object]], fringes['rows'])]
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'target', {
        'simulation-offset-request-native.bin': decision_packet(offset_rows),
        'simulation-fringe-option-native.bin': decision_packet(fringe_rows)})
    dialog_baseline('offset-fringe-native-reference.json', dict(native_complete_contexts=224, native_complete_bytes=496979,
        offset_cases=len(offset_rows), fringe_cases=len(fringe_rows), missing_fringe_states=16, additional_stored_byte_credit=0, packets=summaries))


def verify_original_offset(row: dict[str, object], available: dict[tuple[str, int], dict[str, dict[str, object]]]) -> bytes:
    """Check original field spans, exact signed controls and independently computed result; e.g. verify_original_offset(row, available)."""
    spans = available[(cast(str, row['sha256']), cast(int, row['selected_version']))]
    mode, _, word48 = offset_fringe_inputs(spans)
    assert (mode, word48) == (row['style_1b8'], row['word48']), row
    style = spans['style.word_1b8']
    assert (style['offset'], style['raw_hex']) == (row['style_wire_offset'], row['style_raw_hex']), row
    word = spans.get('moving_objects.word_48')
    assert (None if word is None else word['offset']) == row['word48_wire_offset'], row
    assert (None if word is None else word['raw_hex']) == row['word48_raw_hex'], row
    encoded = offset_case(mode, word48)
    assert struct.unpack_from('<I', encoded, 12)[0] == row['modeled_need_offset'], row
    return encoded


def verify_original_fringe(row: dict[str, object], available: dict[tuple[str, int], dict[str, dict[str, object]]]) -> bytes:
    """Retain original local mutation and actual-mode pre/post offset decisions; e.g. verify_original_fringe(row, available)."""
    spans = available[(cast(str, row['sha256']), cast(int, row['selected_version']))]
    mode, word14, word48 = offset_fringe_inputs(spans)
    assert word48 is not None and word14 == row['word14'], row
    assert spans['moving_objects.word_48']['offset'] == row['word48_wire_offset'], row
    assert spans['moving_objects.word_48']['raw_hex'] == row['word48_raw_hex'], row
    assert cast(int, spans['fourteen_words0c_28_30_3c_518_51c']['offset']) + 8 == row['word14_wire_offset'], row
    view = importlib.import_module('cosworks_fringe_option').fringe_option(word14, word48)
    assert view == row['fringe_view'], row
    need = importlib.import_module('cosworks_need_offset').need_offset
    assert (need(mode, word48), need(mode, view['runtime_word_48_after'])) == (row['modeled_need_offset_before'], row['modeled_need_offset_after']), row
    return fringe_case(word14, word48)


def offset_fringe_context(version: int, raw: bytes) -> bytes:
    """Keep complete stored bytes separate from the modeled post-call word48 and call-order outcomes; e.g. offset_fringe_context(890620, raw)."""
    mode, word14, word48 = offset_fringe_inputs(parent_decision_spans(version, raw))
    offset = offset_case(mode, word48)
    if word48 is None: return struct.pack('<II', version, len(raw)) + raw + offset + struct.pack('<I', 0)
    fringe = fringe_case(word14, word48)
    after = struct.unpack_from('<i', fringe, 8)[0]
    need = importlib.import_module('cosworks_need_offset').need_offset
    order = struct.pack('<II', need(mode, word48), need(mode, after))
    return struct.pack('<II', version, len(raw)) + raw + offset + struct.pack('<I', 1) + fringe + order


def verify_offset_fringe_projection() -> None:
    """Project every complete owned/native parent, preserving all older missing-state cases; e.g. verify_offset_fringe_projection()."""
    owned = [(version, raw) for family, version, _, raw in dialog_owned_cases() if family == 0]
    native = native_parent_decisions()
    assert len(owned) == 225, len(owned)
    packets = [decision_packet([offset_fringe_context(*case) for case in cases]) for cases in (owned, native)]
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'offset-fringe-projection.bin': packets[0]})
    summaries.update(persist_dialog_packets(FEATURE.parents[2] / 'target', {'simulation-offset-fringe-projection-native.bin': packets[1]}))
    dialog_baseline('offset-fringe-projection-reference.json', dict(owned_contexts=225, native_contexts=224,
        additional_stored_byte_credit=0, packets=summaries))
