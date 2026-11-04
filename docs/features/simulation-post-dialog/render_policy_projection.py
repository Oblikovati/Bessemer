"""Complete stored projections and original C-872/C-873 native identity replay (#166)."""
from typing import cast
import importlib
import struct
from dialog_evidence import FEATURE, dialog_baseline
from dialog_field_evidence import dialog_field_reports
from dialog_owned import dialog_owned_cases
from dialog_projection import actual_post_record
from dialog_operation_packets import accepted_dialog_native_cases, persist_dialog_packets
from render_policy_evidence import render_policy_reports
from render_policy_packets import layer0_case, tessellation_case


def tessellation_span_inputs(spans: list[dict[str, object]]) -> tuple[int, ...] | None:
    """Extract six explicit controls only when the hidden option is actually stored; e.g. tessellation_span_inputs(spans)."""
    fields = {span['label']: span for span in spans}
    if 'style.word_1ec' not in fields: return None
    header = bytes.fromhex(cast(str, spans[0]['raw_hex']))
    assert len(header) == 56, len(header)
    words = struct.unpack('<14i', header)
    style = [struct.unpack('<i', bytes.fromhex(cast(str, fields[f'style.word_{offset:x}']['raw_hex'])))[0] for offset in (0x21c, 0x1ec, 0x1f0)]
    return style[0], style[1], words[12], style[2], words[1], words[3]


def original_render_attempts(report: dict[str, object]) -> list[tuple[dict[str, object], dict[str, object]]]:
    """Match every original file/stream/hash/version identity without selecting a new fitting version; e.g. original_render_attempts(report)."""
    rows = cast(list[dict[str, object]], dialog_field_reports()[2]['rows'])
    current = {(row['path'], row['stream'], row['sha256']): row for row in rows}
    matched: list[tuple[dict[str, object], dict[str, object]]] = []
    for original in cast(list[dict[str, object]], report['rows']):
        row = current[(original['path'], original['stream'], original['sha256'])]
        attempts = cast(list[dict[str, object]], row['attempts'])
        selected = [attempt for attempt in attempts if attempt['success'] and attempt['version'] == original['selected_version']]
        assert len(selected) == 1, (original, selected)
        matched.append((original, cast(dict[str, object], selected[0]['result'])))
    return matched


def verify_render_policy_native() -> None:
    """Replay all 276 original gate states and 160 original tessellation states with exact stored inputs; e.g. verify_render_policy_native()."""
    gate_report, tess_report = render_policy_reports()
    gates = original_gate_packets(gate_report)
    tess: list[bytes] = []
    for original, parsed in original_render_attempts(tess_report):
        values = tessellation_span_inputs(cast(list[dict[str, object]], parsed['spans']))
        expected = cast(dict[str, int], original['explicit_loaded_inputs'])
        assert values == tuple(expected[key] for key in ('style_21c', 'hidden_option', 'dialog_518', 'style_1f0', 'plot_type', 'word_18')), original
        packet = tessellation_case(cast(tuple[int, ...], values))
        assert struct.unpack_from('<I', packet, 24)[0] == original['modeled_need_model_tess'], original
        tess.append(packet)
    packets = {name: struct.pack('<II', 1, len(rows)) + b''.join(rows) for name, rows in
               (('simulation-layer0-gate-native.bin', gates), ('simulation-model-tessellation-native.bin', tess))}
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'target', packets)
    dialog_baseline('render-policy-native-reference.json', dict(original_gate_states=276, gate_cases=len(gates),
        original_tessellation_states=len(tess), additional_stored_byte_credit=0, packets=summaries))


def original_gate_packets(report: dict[str, object]) -> list[bytes]:
    """Check every recorded gate span, signed word and both explicit runtime pointer cases; e.g. original_gate_packets(report)."""
    packets: list[bytes] = []
    for original, parsed in original_render_attempts(report):
        spans = cast(list[dict[str, object]], parsed['spans'])
        span = next(span for span in spans if span['label'] == 'legend.word_a8')
        assert (span['offset'], span['end'], span['raw_hex']) == (original['offset'], original['end'], original['raw_hex']), original
        word = struct.unpack('<i', bytes.fromhex(cast(str, span['raw_hex'])))[0]
        assert word == original['stored_a8'], original
        for present, key in ((False, 'enters_if_plot_absent'), (True, 'enters_if_plot_present')):
            packet = layer0_case(word, present)
            assert bool(struct.unpack_from('<I', packet, 8)[0]) == original[key], original
            packets.append(packet)
    assert len(packets) == 552, len(packets)
    return packets


def render_context_projection(family: int, version: int, raw: bytes) -> bytes:
    """Retain complete inputs, raw gate controls and explicit availability of tessellation inputs; e.g. render_context_projection(0, 890620, raw)."""
    parsed = actual_post_record(family, version, raw)
    assert parsed['success'] and parsed['end'] == len(raw), (family, version, parsed)
    spans = cast(list[dict[str, object]], parsed['spans'])
    if version < 700006:
        legacy = importlib.import_module('cosworks_post_v7_composed_view').compose(raw, version)
        span = next(span for span in legacy['spans'] if span['current_object_member'] == '0xa8')
    else: span = next(span for span in spans if span['label'] == 'legend.word_a8')
    word = struct.unpack('<i', bytes.fromhex(cast(str, span['raw_hex'])))[0]
    gate = layer0_case(word, False) + layer0_case(word, True)
    inputs = tessellation_span_inputs(spans) if family == 0 else None
    flag = int(inputs is not None) if family == 0 else 2
    tess = struct.pack('<I', flag) + (b'' if inputs is None else tessellation_case(inputs))
    return struct.pack('<III', family, version, len(raw)) + raw + gate + tess


def verify_render_policy_projection() -> None:
    """Preserve every owned/native parent and style context, including older missing runtime fields; e.g. verify_render_policy_projection()."""
    owned = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    original = (FEATURE.parents[2] / 'target/simulation-post-dialog-native.bin').read_bytes()
    native = [case for case in accepted_dialog_native_cases(original) if case[0] in (0, 2)]
    assert (len(owned), len(native)) == (425, 276), (len(owned), len(native))
    encoded = [struct.pack('<II', 1, len(cases)) + b''.join(render_context_projection(*case) for case in cases) for cases in (owned, native)]
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'render-policy-projection.bin': encoded[0]})
    summaries.update(persist_dialog_packets(FEATURE.parents[2] / 'target', {'simulation-render-policy-projection-native.bin': encoded[1]}))
    dialog_baseline('render-policy-projection-reference.json', dict(owned_contexts=425, native_contexts=276,
        additional_stored_byte_credit=0, packets=summaries))
