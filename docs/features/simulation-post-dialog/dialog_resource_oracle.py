"""Independent outer and embedded resource denominators from complete Source observations (#166)."""
import struct
from typing import cast
from dialog_evidence import FEATURE, dialog_baseline
from dialog_edit_oracle import DialogEditField, owned_edit_contexts
from dialog_projection import dialog_embedded_fields
from dialog_operation_packets import persist_dialog_packets


def source_embedded_metrics(raw: bytes) -> tuple[int, int, int]:
    """Use actual helper class counts and observed primitive calls, without Rust counters; e.g. source_embedded_metrics(raw)."""
    from cosworks_embedded_const_surface import archive_reader
    end, details = archive_reader(raw, 0)
    assert end == len(raw), (end, len(raw))
    projected = dialog_embedded_fields(raw)
    fields = struct.unpack_from('<I', projected, 4 + len(raw))[0]
    objects = sum(cast(dict[str, int], details['classes']).values())
    return len(raw), fields, objects


def source_outer_metrics(fields: list[DialogEditField]) -> tuple[int, int]:
    """Charge explicit variable counts and character/opaque-byte units while keeping BSTR bytes in their own axis; e.g. source_outer_metrics(fields)."""
    entries, maximum_text = 0, 0
    for field in fields:
        if field.tag == 0 and field.path.endswith('.count'): entries += struct.unpack('<i', field.wire)[0]
        if field.tag == 6:
            count = struct.unpack_from('<i', field.wire)[0]
            entries += count
            maximum_text = max(maximum_text, count * 2)
        if field.tag == 10: entries += struct.unpack_from('<i', field.wire)[0]
        if field.tag == 3: maximum_text = max(maximum_text, struct.unpack_from('<I', field.wire)[0])
    return entries, maximum_text


def source_resource_row(raw: bytes, fields: list[DialogEditField], cache: dict[bytes, tuple[int, int, int]]) -> tuple[int, ...]:
    """Retain per-root maxima separately from aggregate nested coverage; e.g. source_resource_row(raw, fields, cache)."""
    entries, maximum_text = source_outer_metrics(fields)
    nested = []
    for field in fields:
        if field.tag != 5: continue
        if field.wire not in cache: cache[field.wire] = source_embedded_metrics(field.wire)
        nested.append(cache[field.wire])
    maxima = tuple(max((item[index] for item in nested), default=0) for index in range(3))
    return (len(raw), len(fields), entries, maximum_text, *maxima, len(nested), sum(item[1] for item in nested))


def verify_dialog_resources() -> None:
    """Freeze all225 complete independent resource rows beside their unchanged owned context order; e.g. verify_dialog_resources()."""
    contexts = owned_edit_contexts()
    cache: dict[bytes, tuple[int, int, int]] = {}
    rows = [source_resource_row(raw, fields, cache) for _, raw, fields in contexts]
    packet = struct.pack('<II', 1, len(rows)) + b''.join(struct.pack('<10I', version, *metrics) for (version, _, _), metrics in zip(contexts, rows, strict=True))
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'resources.bin': packet})
    dialog_baseline('resource-reference.json', dict(contexts=len(rows), distinct_embedded_roots=len(cache),
        columns=['version', 'bytes', 'fields', 'entries', 'text_payload_bytes', 'embedded_bytes', 'embedded_fields_max', 'embedded_new_objects_max', 'embedded_roots', 'embedded_fields_total'],
        packets=summaries))
