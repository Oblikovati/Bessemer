"""Compact raw-bit expectations derived from the complete frozen parent-view observations (#166)."""
import json
import struct
import sys
from typing import cast
from native_carriers import digest
from dialog_evidence import FEATURE, dialog_baseline


def raw_pattern(view: dict[str, object], name: str) -> int:
    """Read a frozen four-byte wire pattern without floating-point conversion; e.g. raw_pattern(view, 'selected_scale')."""
    value = cast(dict[str, object], view[name])
    raw = bytes.fromhex(cast(str, value['raw_hex']))
    assert len(raw) == 4, (name, raw.hex())
    return struct.unpack('<I', raw)[0]


def parent_view_words(row: dict[str, object]) -> tuple[int, ...]:
    """Retain state, all original patterns, selections and the precise scale origin; e.g. parent_view_words(row)."""
    interval = cast(dict[str, object], row['range'])
    state = cast(dict[str, int], interval['state'])
    values = cast(dict[str, object], interval['range_view'])
    scale = cast(dict[str, object], cast(dict[str, object], row['scale'])['view'])
    range_names = ('baseline_minimum', 'baseline_maximum', 'user_minimum', 'user_maximum', 'selected_minimum', 'selected_maximum')
    scale_names = ('scale_528', 'scale_52c', 'selected_scale')
    origin = {'stored_528': 0, 'stored_52c': 1, 'constant_one': 2}
    return (cast(int, row['version']), state['stored_combination'] & 0xffffffff,
        cast(int, interval['reencoded_combination_view']), state['user_minimum_flag'], state['user_maximum_flag'],
        *(raw_pattern(values, name) for name in range_names), cast(int, scale['selector_raw_integer']) & 0xffffffff,
        *(raw_pattern(scale, name) for name in scale_names), origin[cast(str, scale['selected_source'])])


def verify_parent_view_packet() -> None:
    """Serialize the already independently replayed owned views, with input provenance; e.g. verify_parent_view_packet()."""
    source = (FEATURE / 'owned-parent-views-reference.json').read_bytes()
    rows = cast(list[dict[str, object]], json.loads(source))
    assert len(rows) == 225, len(rows)
    packet = struct.pack('<II', 1, len(rows)) + b''.join(struct.pack('<16I', *parent_view_words(row)) for row in rows)
    target = FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog/parent-views.bin'
    if '--write' in sys.argv: target.write_bytes(packet)
    assert target.read_bytes() == packet, str(target)
    dialog_baseline('parent-view-packet-reference.json', dict(contexts=len(rows), source_sha256=digest(source),
        packet_bytes=len(packet), packet_sha256=digest(packet)))
