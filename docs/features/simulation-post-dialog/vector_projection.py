"""Complete parent projections and original C-878/C-880 native identities with explicit text-state assumptions (#166)."""
from collections import Counter
from typing import cast
import importlib
import struct
from native_carriers import digest
from dialog_evidence import FEATURE, dialog_baseline, dialog_reports
from dialog_owned import dialog_owned_cases
from dialog_operation_packets import persist_dialog_packets
from offset_fringe_projection import parent_decision_spans, native_parent_decisions
from vector_evidence import vector_reports
from vector_packets import provider_case, composed_case


def parent_vector_inputs(spans: dict[str, dict[str, object]]) -> tuple[tuple[int, int, int, int], int]:
    """Extract raw arguments in exact native order without inferring loaded CString presence; e.g. parent_vector_inputs(spans)."""
    header = bytes.fromhex(cast(str, spans['fourteen_words0c_28_30_3c_518_51c']['raw_hex']))
    selector = bytes.fromhex(cast(str, spans['three_words568_570']['raw_hex']))
    assert len(header) == 56 and len(selector) == 12, (len(header), len(selector))
    words = struct.unpack('<14i', header)
    return (words[0], words[1], words[3], words[2]), struct.unpack_from('<i', selector, 8)[0]


def original_vector_identities(reports: list[dict[str, object]]) -> None:
    """Require every file, stream, hash and selected complete version in both original native reports; e.g. original_vector_identities(reports)."""
    current = cast(list[dict[str, object]], dialog_reports()[4]['rows'])
    identities = Counter((row['path'], row['stream'], row['sha256'], attempt['version']) for row in current
        for attempt in cast(list[dict[str, object]], row['attempts']) if attempt.get('complete'))
    for report in reports:
        original = Counter((row['path'], row['stream'], row['sha256'], row['selected_version'])
            for row in cast(list[dict[str, object]], report['rows']))
        assert original == identities, report['evidence']


def verify_vector_native() -> None:
    """Replay all original native inputs, text bytes and branch/provenance outputs from exact complete streams; e.g. verify_vector_native()."""
    local, _, policy = vector_reports()
    original_vector_identities([local, policy])
    available = {(digest(raw), version): parent_decision_spans(version, raw) for version, raw in native_parent_decisions()}
    local_rows = cast(list[dict[str, object]], local['rows'])
    for row in local_rows: verify_original_local_vector(row, available)
    provider_rows, composed_rows = [], []
    for row in cast(list[dict[str, object]], policy['rows']):
        provider, composed = verify_original_vector_policy(row, available)
        provider_rows.append(provider)
        composed_rows.append(composed)
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'target', {
        'simulation-component-vector-native.bin': struct.pack('<II', 1, len(provider_rows)) + b''.join(provider_rows),
        'simulation-vector-direction-native.bin': struct.pack('<II', 1, len(composed_rows)) + b''.join(composed_rows)})
    dialog_baseline('vector-native-reference.json', dict(native_complete_contexts=224, native_complete_bytes=496979,
        original_local_records=len(local_rows), original_composed_records=len(composed_rows), additional_stored_byte_credit=0, packets=summaries))


def verify_original_local_vector(row: dict[str, object], available: dict[tuple[str, int], dict[str, dict[str, object]]]) -> None:
    """Retain exact selector/text offsets and verify the ordinary-wire-text assumption separately; e.g. verify_original_local_vector(row, available)."""
    spans = available[(cast(str, row['sha256']), cast(int, row['selected_version']))]
    _, selector = parent_vector_inputs(spans)
    text = bytes.fromhex(cast(str, spans['text578.utf16']['raw_hex']))
    units = struct.unpack('<' + 'H' * (len(text) // 2), text)
    assert 0 not in units, units
    assert (selector, text.hex(), list(units)) == (row['word570'], row['text_raw_hex'], row['text_code_units']), row
    assert cast(int, spans['three_words568_570']['offset']) + 8 == row['selector_wire_offset'], row
    assert spans['text578.utf16']['offset'] == row['text_wire_offset'], row
    assert bool(units) == row['wire_text_presence_assumption'], row
    model = importlib.import_module('cosworks_vector_direction_branch')
    assert model.select_branch(selector, loaded_text_nonempty=bool(units)) == row['conditional_branch_view'], row


def verify_original_vector_policy(row: dict[str, object], available: dict[tuple[str, int], dict[str, dict[str, object]]]) -> tuple[bytes, bytes]:
    """Match every original argument and all four raw output words, including selected table on fallback; e.g. verify_original_vector_policy(row, available)."""
    spans = available[(cast(str, row['sha256']), cast(int, row['selected_version']))]
    provider, selector = parent_vector_inputs(spans)
    expected = cast(dict[str, int], row['inputs'])
    assert (*provider, selector) == tuple(expected[key] for key in ('analysis', 'plot_type', 'component', 'fourth_word', 'word570')), row
    text = bytes.fromhex(cast(str, spans['text578.utf16']['raw_hex']))
    units = struct.unpack('<' + 'H' * (len(text) // 2), text)
    assert 0 not in units and text.hex() == row['wire_text_raw_hex'], row
    assert bool(units) == row['wire_text_presence_assumption'], row
    helper = importlib.import_module('cosworks_component_vector_policy')
    assert helper.vector_view(*provider, selector, loaded_text_nonempty=bool(units)) == row['view'], row
    return provider_case(provider), composed_case(provider, selector, bool(units))


def vector_parent_projection(version: int, raw: bytes) -> bytes:
    """Retain complete wire input and both explicit loaded-text choices without guessing from wire bytes; e.g. vector_parent_projection(890620, raw)."""
    provider, selector = parent_vector_inputs(parent_decision_spans(version, raw))
    results = composed_case(provider, selector, False) + composed_case(provider, selector, True)
    return struct.pack('<II', version, len(raw)) + raw + results


def verify_vector_projection() -> None:
    """Freeze every complete owned/native parent under both caller-supplied CString states; e.g. verify_vector_projection()."""
    owned = [(version, raw) for family, version, _, raw in dialog_owned_cases() if family == 0]
    native = native_parent_decisions()
    assert len(owned) == 225, len(owned)
    packets = [struct.pack('<II', 1, len(cases)) + b''.join(vector_parent_projection(*case) for case in cases) for cases in (owned, native)]
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'vector-parent-projection.bin': packets[0]})
    summaries.update(persist_dialog_packets(FEATURE.parents[2] / 'target', {'simulation-vector-parent-projection-native.bin': packets[1]}))
    dialog_baseline('vector-projection-reference.json', dict(owned_contexts=225, native_contexts=224,
        explicit_presence_choices=2, additional_stored_byte_credit=0, packets=summaries))
