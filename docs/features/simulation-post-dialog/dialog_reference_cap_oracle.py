"""Whole-record reference count boundaries observed through the pinned parent parser (#166)."""
import struct
from dialog_cap_oracle import cap_observation, collection_extent
from dialog_edit_oracle import DialogEditField, owned_edit_contexts
from dialog_evidence import dialog_baseline


def reference_first_record(raw: bytes, fields: list[DialogEditField], ordinal: int) -> bytes:
    """Reuse one independently accepted full record, including its scoped archive roots; e.g. reference_first_record(raw, fields, 3)."""
    prefix = fields[ordinal].path.removesuffix('.count') + '.record[0].'
    first = fields[ordinal + 1]
    assert first.path.startswith(prefix), (first.path, prefix)
    end = collection_extent(fields, ordinal, prefix)
    return raw[first.start:end]


def reference_cap_rows(version: int, raw: bytes, fields: list[DialogEditField], ordinal: int) -> list[dict[str, object]]:
    """Keep the original suffix after replacing all existing records with repeated complete records; e.g. reference_cap_rows(version, raw, fields, 3)."""
    target = fields[ordinal]
    record = reference_first_record(raw, fields, ordinal)
    prefix = target.path.removesuffix('.count') + '.record['
    end = collection_extent(fields, ordinal, prefix)
    rows: list[dict[str, object]] = []
    for count in (-1, 10000, 10001):
        wire = struct.pack('<i', count) + record * max(count, 0)
        edited = raw[:target.start] + wire + raw[end:]
        rows.append(cap_observation(version, edited, target.path, count, count == 10000))
    return rows


def verify_reference_caps() -> None:
    """Replay every reached populated reference role without replacing it with a shorter surrogate; e.g. verify_reference_caps()."""
    seen: set[str] = set()
    rows: list[dict[str, object]] = []
    for version, raw, fields in owned_edit_contexts():
        for ordinal, field in enumerate(fields):
            if 'reference' not in field.path or not field.path.endswith('.count'): continue
            if field.path in seen or struct.unpack('<i', field.wire)[0] == 0: continue
            seen.add(field.path)
            rows.extend(reference_cap_rows(version, raw, fields, ordinal))
    assert len(seen) == 9 and len(rows) == 27, (seen, len(rows))
    dialog_baseline('reference-cap-reference.json', dict(paths=len(seen), controls=rows))
