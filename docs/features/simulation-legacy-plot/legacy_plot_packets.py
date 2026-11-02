"""Complete atomic typed projections from independent frozen historical helper spans (#174)."""
from typing import cast
import importlib
import json
import struct
from native_carriers import digest
from dialog_operation_packets import persist_dialog_packets
from legacy_plot_evidence import FEATURE, legacy_plot_baseline, legacy_plot_reports
from legacy_plot_owned import owned_legacy_cases


def legacy_field_parts(span: dict[str, object], next_span: dict[str, object] | None) -> tuple[str, int, int, int]:
    """Combine only counted character frames and preserve every other source span; e.g. legacy_field_parts(span, next_span)."""
    label = cast(str, span['label'])
    start, end = cast(int, span['offset']), cast(int, span['end'])
    if label.endswith('.character_count'):
        assert next_span is not None and next_span['label'] == label.replace('.character_count', '.raw_characters')
        return 'legacy_plot.' + label.split('.')[0].replace('text', 'text_'), 6, start, cast(int, next_span['end'])
    if label.startswith('legacy_bulk'):
        return 'legacy_plot.' + label.replace('legacy_bulk', 'bulk_'), 4, start, end
    if label.startswith('member'):
        tag = {4: 7, 8: 9}.get(end - start, 4)
        return 'legacy_plot.' + label.replace('member', 'member_'), tag, start, end
    assert label in ('word568', 'word56c'), f'{label}; expected historical fixed word or atomic text'
    return 'legacy_plot.' + label.replace('word', 'word_'), 7, start, end


def legacy_plot_projection(version: int, raw: bytes) -> bytes:
    """Project exact names, kinds, complete offsets and bytes from the frozen reader; e.g. legacy_plot_projection(700018, raw)."""
    parsed = importlib.import_module('cosworks_legacy_plot_record').parse(raw, version)
    assert parsed['success'], (version, parsed)
    spans = cast(list[dict[str, object]], parsed['spans'])
    fields: list[bytes] = []
    for index, span in enumerate(spans):
        if cast(str, span['label']).endswith('.raw_characters'): continue
        following = spans[index + 1] if index + 1 < len(spans) else None
        path, tag, start, end = legacy_field_parts(span, following)
        name = path.encode()
        fields.append(struct.pack('<I', len(name)) + name + struct.pack('<III', tag, start, end) + raw[start:end])
    return struct.pack('<II', version, len(raw)) + raw + struct.pack('<I', len(fields)) + b''.join(fields)


def native_legacy_projections() -> bytes:
    """Project all96 successful original arguments, preserving other outcomes in the full native packet; e.g. native_legacy_projections()."""
    packet = (FEATURE.parents[2] / 'target/simulation-legacy-plot-native.bin').read_bytes()
    offset = 0
    projected: list[bytes] = []
    while offset < len(packet):
        version, accepted, _, _, size = struct.unpack_from('<5I', packet, offset)
        offset += 20
        raw = packet[offset:offset + size]
        assert len(raw) == size, (offset, size)
        offset += size
        if accepted: projected.append(legacy_plot_projection(version, raw))
    assert offset == len(packet) and len(projected) == 96, (offset, len(projected))
    return b''.join(projected)


def native_legacy_wide_packet() -> bytes:
    """Retain all240 original C908 raw buffers and bounded-prefix expectations; e.g. native_legacy_wide_packet()."""
    _, report = legacy_plot_reports()
    native = json.loads((FEATURE / 'native-reference.json').read_bytes())
    payloads = {(row['path'], row['stream']): bytes.fromhex(row['wire_hex']) for row in native['rows']}
    packet = bytearray()
    for row in cast(list[dict[str, object]], report['rows']):
        raw = payloads[row['path'], row['stream']]
        views = cast(list[dict[str, object]], row['views'])
        packet.extend(struct.pack('<4I', cast(int, row['version']), int(cast(bool, row['named_stream_dispatch_enabled'])), len(raw), len(views)))
        packet.extend(raw)
        for view in views:
            packet.extend(struct.pack('<6I', *(cast(int, view[key]) for key in
                ('member', 'offset', 'end', 'code_unit_limit', 'prefix_code_unit_count', 'terminator_present'))))
            packet.extend(bytes.fromhex(cast(str, view['raw_hex'])))
    return bytes(packet)


def verify_legacy_plot_operation_packets() -> None:
    """Freeze every owned complete expected field and all native helper-success projections; e.g. verify_legacy_plot_operation_packets()."""
    owned = b''.join(legacy_plot_projection(version, raw) for version, _, raw in owned_legacy_cases())
    folder = FEATURE.parents[2] / 'tests/fixtures/simulation-legacy-plot'
    summaries = persist_dialog_packets(folder, {'projection.bin': owned})
    native = native_legacy_projections()
    (FEATURE.parents[2] / 'target/simulation-legacy-plot-native-projection.bin').write_bytes(native)
    summaries['native-projection'] = dict(bytes=len(native), sha256=digest(native), cases=96)
    wide = native_legacy_wide_packet()
    (FEATURE.parents[2] / 'target/simulation-legacy-plot-native-wide.bin').write_bytes(wide)
    summaries['native-wide'] = dict(bytes=len(wide), sha256=digest(wide), cases=48, views=240)
    legacy_plot_baseline('operation-packets-reference.json', summaries)
