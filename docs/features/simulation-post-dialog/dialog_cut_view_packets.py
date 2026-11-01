"""Complete synthetic PostInfo slot expectations from frozen independent field observations (#166)."""
import json
import struct
import sys
from typing import cast
from native_carriers import digest
from dialog_evidence import FEATURE, dialog_baseline


def cut_slot_annotations(record: dict[str, object]) -> bytes:
    """Encode known names separately from preserved raw enum words; e.g. cut_slot_annotations(record)."""
    names = {'cylinder': 0, 'plane': 1, 'sphere': 2, 'iso': 3, None: 4}
    tokens = {'Yon': 0, 'Hither': 1, 'Both': 2, None: 3}
    negated = cast(dict[str, object], record['negated_D_view'])
    pattern = int.from_bytes(bytes.fromhex(cast(str, negated['view_raw_hex'])), 'little')
    return struct.pack('<III', names[cast(str | None, record['primitive_name'])],
        tokens[cast(str | None, record['cut_direction_display_token'])], pattern)


def cut_view_context(row: dict[str, object]) -> bytes:
    """Retain the complete fixed block and independent named/alias outcomes; e.g. cut_view_context(row)."""
    fields = cast(dict[str, object], row['fields'])
    raw = bytes.fromhex(cast(str, fields['raw_hex']))
    records = cast(list[dict[str, object]], fields['records'])
    assert len(raw) == 460 and len(records) == 12, (len(raw), len(records))
    contour = cast(dict[str, int], fields['contour_type'])
    return (raw + struct.pack('<i', contour['preference_integer_view'])
        + b''.join(cut_slot_annotations(record) for record in records))


def verify_cut_view_packet() -> None:
    """Freeze all five independent bit rotations and sixty physical slots; e.g. verify_cut_view_packet()."""
    source = (FEATURE / 'owned-postinfo-fields-reference.json').read_bytes()
    reference = cast(dict[str, object], json.loads(source))
    rows = cast(list[dict[str, object]], reference['rows'])
    assert len(rows) == 5, len(rows)
    packet = struct.pack('<II', 1, len(rows)) + b''.join(cut_view_context(row) for row in rows)
    target = FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog/cut-views.bin'
    if '--write' in sys.argv: target.write_bytes(packet)
    assert target.read_bytes() == packet, str(target)
    dialog_baseline('cut-view-packet-reference.json', dict(contexts=5, slots=60, source_sha256=digest(source),
        packet_bytes=len(packet), packet_sha256=digest(packet)))
