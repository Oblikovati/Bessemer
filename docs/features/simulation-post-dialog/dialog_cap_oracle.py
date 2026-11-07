"""Complete literal boundary controls parsed by the pinned parent reader (#166)."""
import struct
from dialog_evidence import dialog_baseline
from dialog_edit_oracle import DialogEditField, owned_edit_contexts
from dialog_projection import actual_post_record
from native_carriers import digest


COLLECTIONS = (
    ('sensors.count', 'sensor[', 8),
    ('sensor[0].points.count', 'sensor[0].point[', 44),
    ('dialog.da4.count', 'dialog.da8_integer_array[', 4),
    ('dialog.string_list_dd0.count', 'dialog.string_list[', 4),
    ('equation.variables.count', 'equation.variable[', 20),
)


def cap_observation(version: int, raw: bytes, path: str, count: int, expected: bool) -> dict[str, object]:
    """Retain full-input identity and actual complete-parser outcome; e.g. cap_observation(890620, raw, path, 10000, True)."""
    parsed = actual_post_record(0, version, raw)
    accepted = bool(parsed['success'])
    assert accepted == expected, (version, path, count, parsed)
    if accepted: assert parsed['end'] == len(raw), (path, parsed['end'], len(raw))
    return dict(version=version, path=path, count=count, bytes=len(raw), sha256=digest(raw), accepted=accepted)


def collection_extent(fields: list[DialogEditField], ordinal: int, prefix: str) -> int:
    """Replace existing records as well as the count, avoiding empty-fixture assumptions; e.g. collection_extent(fields, 4, 'sensor[')."""
    end = fields[ordinal].end
    for field in fields[ordinal + 1:]:
        if not field.path.startswith(prefix): break
        end = field.end
    return end


def collection_cap_rows(contexts: list[tuple[int, bytes, list[DialogEditField]]]) -> list[dict[str, object]]:
    """Use literal zero-valued complete records with each imported count boundary; e.g. collection_cap_rows(contexts)."""
    rows: list[dict[str, object]] = []
    for path, prefix, width in COLLECTIONS:
        version, raw, fields, ordinal = next((v, r, f, i) for v, r, f in contexts for i, field in enumerate(f) if field.path == path)
        target = fields[ordinal]
        end = collection_extent(fields, ordinal, prefix)
        for count in (-1, 10000, 10001):
            wire = struct.pack('<i', count) + bytes(max(count, 0) * width)
            edited = raw[:target.start] + wire + raw[end:]
            rows.append(cap_observation(version, edited, path, count, count == 10000))
    return rows


def byte_cap_rows(contexts: list[tuple[int, bytes, list[DialogEditField]]]) -> list[dict[str, object]]:
    """Keep opaque reference bytes and old list bytes under their distinct literal caps; e.g. byte_cap_rows(contexts)."""
    rows: list[dict[str, object]] = []
    seen: set[str] = set()
    for version, raw, fields in contexts:
        for field in fields:
            if field.tag != 10 or field.path in seen: continue
            seen.add(field.path)
            cap = 0x100000 if field.path.startswith('dialog.string_list[') else 1000000
            for count in (-1, cap, cap + 1):
                wire = struct.pack('<i', count) + b'\xff' * max(count, 0)
                edited = raw[:field.start] + wire + raw[field.end:]
                rows.append(cap_observation(version, edited, field.path, count, count == cap))
    assert seen and any(path.startswith('dialog.string_list[') for path in seen), seen
    return rows


def verify_dialog_caps() -> None:
    """Prove boundary outcomes using fresh pinned complete Source parsing; e.g. verify_dialog_caps()."""
    contexts = owned_edit_contexts()
    rows = collection_cap_rows(contexts) + byte_cap_rows(contexts)
    dialog_baseline('cap-reference.json', dict(contexts=len(contexts), controls=rows))
