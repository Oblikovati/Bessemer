"""Exact original/current PostInfo and RGBA views over recovered complete native inputs (#166)."""
from typing import cast
import importlib
import struct
from native_carriers import digest
from solid_inventory_bindings import complete_identity_match
from dialog_evidence import FEATURE, dialog_reports, dialog_baseline
from dialog_field_evidence import dialog_field_reports


def original_postinfo_projection(current: dict[str, object]) -> dict[str, object]:
    """Remove only the three explicitly added C-842/C-843 annotations; e.g. original_postinfo_projection(current)."""
    projected = {key: value for key, value in current.items() if key != 'contour_type'}
    projected['records'] = [{key: value for key, value in row.items() if key not in ('primitive_name', 'cut_direction_display_token')}
                            for row in cast(list[dict[str, object]], current['records'])]
    return projected


def verify_postinfo_field_views() -> None:
    """Require all sixty complete C-843 views and every original C-841 numeric/raw field unchanged; e.g. verify_postinfo_field_views()."""
    original, current = dialog_field_reports()[:2]
    old_rows = cast(list[dict[str, object]], original['rows'])
    rows = cast(list[dict[str, object]], current['rows'])
    expected = [row for row in cast(list[dict[str, object]], dialog_reports()[5]['rows']) if row['kind'] == 'DspPostInfo']
    complete_identity_match(identity_rows(expected), identity_rows(rows))
    complete_identity_match(identity_rows(old_rows), identity_rows(rows))
    old = {(row['path'], row['stream']): row for row in old_rows}
    decode = importlib.import_module('cosworks_post_info_fields').decode_block
    for row in rows:
        raw = bytes.fromhex(cast(str, cast(dict[str, object], row['fields'])['raw_hex']))
        assert len(raw) == row['bytes'] and digest(raw) == row['sha256'], row['stream']
        actual = decode(raw)
        assert actual == row['fields'], row['stream']
        assert original_postinfo_projection(actual) == old[row['path'], row['stream']]['fields'], row['stream']
    assert len(rows) == 60 and sum(len(cast(dict[str, object], row['fields'])['records']) for row in rows) == 720


def identity_rows(rows: list[dict[str, object]]) -> list[dict[str, object]]:
    """Retain complete native identities before view comparisons; e.g. identity_rows(rows)."""
    return [{key: row[key] for key in ('path', 'stream', 'bytes', 'sha256')} for row in rows]


def dialog_native_payloads() -> dict[str, bytes]:
    """Read full inputs freshly recovered by the original checkpoint replay, including failed helpers; e.g. dialog_native_payloads()."""
    packet = (FEATURE.parents[2] / 'target/simulation-post-dialog-native.bin').read_bytes()
    offset = 0
    payloads: dict[str, bytes] = {}
    while offset < len(packet):
        _, _, _, _, size = struct.unpack_from('<IIIII', packet, offset)
        offset += 20
        raw = packet[offset:offset + size]
        assert len(raw) == size, (offset, size, len(packet))
        offset += size
        identity = digest(raw)
        if identity in payloads: assert payloads[identity] == raw, identity
        payloads[identity] = raw
    assert offset == len(packet), (offset, len(packet))
    return payloads


def rgba_attempt(raw: bytes, version: int, kind: str) -> dict[str, object]:
    """Reproduce exact C-844 scoped reader results and complete RGBA component views; e.g. rgba_attempt(raw, 890620, 'cwPostDlgParam')."""
    module = 'cosworks_post_dialog_style_prefix' if kind == 'cwPostDlgParam' else 'cosworks_post_style_layout'
    try: result = importlib.import_module(module).parse(raw, version)
    except ValueError as error: result = dict(success=False, end=0, error=str(error))
    success = result.get('success', result.get('prefix_success', False))
    colors = importlib.import_module('cosworks_post_color_fields').colors_from_spans(result['spans']) if success else []
    return dict(version=version, success=success, named_stream_dispatch_enabled=version >= 700335 if kind == 'cwPostDlgParam' else version < 700335,
                prefix_only=kind == 'cwPostDlgParam', result=result, colors=colors)


def verify_rgba_native_views() -> None:
    """Reparse every original color-view candidate against complete independently recovered payloads; e.g. verify_rgba_native_views()."""
    report = dialog_field_reports()[2]
    payloads = dialog_native_payloads()
    original = dialog_reports()
    identities = cast(list[dict[str, object]], original[4]['rows']) + [row for row in cast(list[dict[str, object]], original[5]['rows']) if row['kind'] == 'DspPostStyle']
    rows = cast(list[dict[str, object]], report['rows'])
    complete_identity_match(identity_rows(identities), identity_rows(rows))
    count = 0
    for row in rows:
        raw = payloads[cast(str, row['sha256'])]
        assert len(raw) == row['bytes'], row['stream']
        for expected in cast(list[dict[str, object]], row['attempts']):
            assert rgba_attempt(raw, cast(int, expected['version']), cast(str, row['kind'])) == expected, (row['stream'], expected['version'])
            count += 1
    dialog_baseline('rgba-view-reference.json', dict(streams=len(rows), arguments=count,
        original_summary=report['summary'], complete_values_spans_and_failure_traces_preserved=True))
