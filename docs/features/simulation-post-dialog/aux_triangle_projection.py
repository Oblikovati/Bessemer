"""Full C-888 versioned control/color groups and every original native gate input (#166)."""
from collections import Counter
from typing import cast
import importlib
import struct
from native_carriers import digest
from dialog_evidence import FEATURE, dialog_baseline
from dialog_owned import dialog_owned_cases
from dialog_projection import actual_post_record
from dialog_operation_packets import persist_dialog_packets
from local_render_evidence import local_render_report
from style_preferences_projection import raw_style_native_contexts, raw_style_identity
from aux_triangle_evidence import aux_triangle_report

AUX_PATHS = ('style.word_204', 'style.word_208') + tuple(f'style.color_20c.component_{index}' for index in range(4))


def aux_triangle_observation(family: int, version: int, raw: bytes) -> dict[str, object] | None:
    """Require complete EOF and the actual all-or-none840008 field group; e.g. aux_triangle_observation(2, 840007, raw)."""
    parsed = actual_post_record(family, version, raw)
    assert parsed['success'] and parsed['end'] == len(raw), (family, version, parsed)
    spans = {span['label']: span for span in cast(list[dict[str, object]], parsed['spans'])}
    present = [path in spans for path in AUX_PATHS]
    assert all(present) or not any(present), (family, version, present)
    if not any(present): return None
    words = [bytes.fromhex(cast(str, spans[path]['raw_hex'])) for path in AUX_PATHS]
    assert all(len(word) == 4 for word in words)
    helper = importlib.import_module('cosworks_aux_triangle_gate')
    view = helper.request(struct.unpack('<i', words[0])[0], struct.unpack('<i', words[1])[0], words[5], masked_exceptions=True)
    return dict(enable_raw_hex=words[0].hex(), selector_raw_hex=words[1].hex(),
        rgba_raw_hex=b''.join(words[2:]).hex(), conditional_view=view)


def verify_aux_triangle_native() -> None:
    """Replay all160 exact original identities and all absent native contexts without defaults; e.g. verify_aux_triangle_native()."""
    report = aux_triangle_report()
    rows = cast(list[dict[str, object]], report['rows'])
    original = cast(list[dict[str, object]], local_render_report()['rows'])
    families = {raw_style_identity(row): 0 if row['kind'] == 'cwPostDlgParam' else 2 for row in original}
    available = {(family, digest(raw), version): aux_triangle_observation(family, version, raw)
        for family, version, raw in raw_style_native_contexts()}
    expected = Counter(raw_style_identity(row) for row in original
        if available[(families[raw_style_identity(row)], row['sha256'], row['selected_version'])] is not None)
    assert Counter(raw_style_identity(row) for row in rows) == expected
    for row in rows:
        assert row['scope'] == 'explicit_stored_inputs_only'
        assert available[(families[raw_style_identity(row)], row['sha256'], row['selected_version'])] == {
            name: row[name] for name in ('enable_raw_hex', 'selector_raw_hex', 'rgba_raw_hex', 'conditional_view')}, row
    counts = dict(Counter(str(cast(dict[str, object], row['conditional_view'])['request_helper']) for row in rows))
    assert counts == report['request_counts']
    dialog_baseline('aux-triangle-native-reference.json', dict(original_input_sets=160, native_contexts=276,
        native_missing=116, request_counts=counts, additional_stored_byte_credit=0))


def aux_triangle_context(family: int, version: int, raw: bytes) -> bytes:
    """Frame complete bodies and conditional input views without conflating absence with false; e.g. aux_triangle_context(0, 840008, raw)."""
    observed = aux_triangle_observation(family, version, raw)
    packet = struct.pack('<IIII', family, version, len(raw), int(observed is not None)) + raw
    if observed is None: return packet
    inputs = b''.join(bytes.fromhex(cast(str, observed[name])) for name in ('enable_raw_hex', 'selector_raw_hex', 'rgba_raw_hex'))
    view = cast(dict[str, bool], observed['conditional_view'])
    return packet + inputs + struct.pack('<II', int(view['request_helper']), int(view['per_group_rgb']))


def verify_aux_triangle_projection() -> None:
    """Retain every owned/native parent/style context across the exact version boundary; e.g. verify_aux_triangle_projection()."""
    owned = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    native = raw_style_native_contexts()
    assert len(owned) == 425
    packets = [struct.pack('<II', 1, len(cases)) + b''.join(aux_triangle_context(*case) for case in cases) for cases in (owned, native)]
    present = [sum(aux_triangle_observation(*case) is not None for case in cases) for cases in (owned, native)]
    assert present[1] == 160
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'aux-triangle-projection.bin': packets[0]})
    summaries.update(persist_dialog_packets(FEATURE.parents[2] / 'target', {'simulation-aux-triangle-projection-native.bin': packets[1]}))
    dialog_baseline('aux-triangle-projection-reference.json', dict(owned_contexts=425, owned_present=present[0],
        native_contexts=276, native_present=160, additional_stored_byte_credit=0, packets=summaries))
