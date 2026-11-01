"""C-886 complete-body vector groups, retaining old absence and all raw RGBA components (#166)."""
from typing import cast
from collections import Counter
import struct
from native_carriers import digest
from dialog_evidence import FEATURE, dialog_baseline
from dialog_owned import dialog_owned_cases
from dialog_projection import actual_post_record
from dialog_operation_packets import persist_dialog_packets
from local_render_evidence import local_render_report
from style_preferences_projection import raw_style_native_contexts, raw_style_identity
from vector_style_evidence import vector_style_report

VECTOR_MEMBERS = (0x28c, 0x290, 0x294, 0x298)
VECTOR_PATHS = tuple(f'style.word_{member:x}' for member in VECTOR_MEMBERS) + tuple(f'style.color_29c.component_{index}' for index in range(4))


def vector_style_observation(family: int, version: int, raw: bytes) -> dict[str, object] | None:
    """Require complete EOF and an all-present or all-absent eight-word group; e.g. vector_style_observation(2, 700329, raw)."""
    parsed = actual_post_record(family, version, raw)
    assert parsed['success'] and parsed['end'] == len(raw), (family, version, parsed)
    spans = {span['label']: span for span in cast(list[dict[str, object]], parsed['spans'])}
    present = [path in spans for path in VECTOR_PATHS]
    assert all(present) or not any(present), (family, version, present)
    if not any(present): return None
    words = {hex(member): dict(raw_hex=spans[path]['raw_hex'],
        int32=struct.unpack('<i', bytes.fromhex(cast(str, spans[path]['raw_hex'])))[0], offset=spans[path]['offset'])
        for member, path in zip(VECTOR_MEMBERS, VECTOR_PATHS[:4], strict=True)}
    colors = [spans[path] for path in VECTOR_PATHS[4:]]
    start = cast(int, colors[0]['offset'])
    assert [span['offset'] for span in colors] == [start + 4 * index for index in range(4)]
    rgba = ''.join(cast(str, span['raw_hex']) for span in colors)
    assert len(bytes.fromhex(rgba)) == 16
    return dict(words=words, vector_rgba_raw_hex=rgba, color_offset=start)


def verify_vector_style_native() -> None:
    """Check every original identity, raw control, color component and exact offset from complete native bytes; e.g. verify_vector_style_native()."""
    rows = cast(list[dict[str, object]], vector_style_report()['rows'])
    original = cast(list[dict[str, object]], local_render_report()['rows'])
    identities = {raw_style_identity(row): 0 if row['kind'] == 'cwPostDlgParam' else 2 for row in original}
    available = {(family, digest(raw), version): vector_style_observation(family, version, raw)
        for family, version, raw in raw_style_native_contexts()}
    expected = Counter(raw_style_identity(row) for row in original if row['kind'] == 'cwPostDlgParam')
    assert Counter(raw_style_identity(row) for row in rows) == expected
    for row in rows:
        assert row['scope'] == 'explicit_stored_fields'
        observed = available[(identities[raw_style_identity(row)], row['sha256'], row['selected_version'])]
        assert observed == {key: row[key] for key in ('words', 'vector_rgba_raw_hex', 'color_offset')}, row
    report = vector_style_report()
    counts = {hex(member): dict(Counter(str(cast(dict[str, dict[str, object]], row['words'])[hex(member)]['int32']) for row in rows)) for member in VECTOR_MEMBERS}
    colors = dict(Counter(cast(str, row['vector_rgba_raw_hex']) for row in rows))
    assert counts == report['native_counts'] and colors == report['native_colors']
    dialog_baseline('vector-style-native-reference.json', dict(explicit_input_sets=224,
        native_counts=counts, native_colors=colors, additional_stored_byte_credit=0))


def vector_style_context(family: int, version: int, raw: bytes) -> bytes:
    """Preserve the complete source and distinguish absence from a zero-valued vector group; e.g. vector_style_context(2, 700329, raw)."""
    observed = vector_style_observation(family, version, raw)
    packet = struct.pack('<IIII', family, version, len(raw), int(observed is not None)) + raw
    if observed is None: return packet
    words = cast(dict[str, dict[str, object]], observed['words'])
    controls = b''.join(bytes.fromhex(cast(str, words[hex(member)]['raw_hex'])) for member in VECTOR_MEMBERS)
    return packet + controls + bytes.fromhex(cast(str, observed['vector_rgba_raw_hex']))


def verify_vector_style_projection() -> None:
    """Freeze every owned/native parent/style view, including all older missing groups; e.g. verify_vector_style_projection()."""
    owned = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    native = raw_style_native_contexts()
    assert len(owned) == 425
    packets = [struct.pack('<II', 1, len(cases)) + b''.join(vector_style_context(*case) for case in cases) for cases in (owned, native)]
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'vector-style-projection.bin': packets[0]})
    summaries.update(persist_dialog_packets(FEATURE.parents[2] / 'target', {'simulation-vector-style-projection-native.bin': packets[1]}))
    present = [sum(vector_style_observation(*case) is not None for case in cases) for cases in (owned, native)]
    assert present[1] == 224
    dialog_baseline('vector-style-projection-reference.json', dict(owned_contexts=425, native_contexts=276,
        owned_present=present[0], native_present=224, native_absent=52, additional_stored_byte_credit=0, packets=summaries))
