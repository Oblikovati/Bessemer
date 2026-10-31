"""Complete standalone persist-ID fields and ordered edit history from actual C-738 calls (#152)."""
from typing import cast
from thermal_projection import projected_fields
from body_projection import body_shape, body_projection
from body_history import body_history
from sheet_projection import sheet_embedded_fields


def persist_id_fields(version: int, raw: bytes) -> list[bytes]:
    """Require complete actual standalone framing before projecting; e.g. persist_id_fields(840002, raw)."""
    from cosworks_component_persistid_layout import parse
    from cosworks_embedded_const_surface import archive_reader
    parsed = parse(raw, version, archive_reader=archive_reader)
    assert parsed['success'] and parsed['end'] == len(raw)
    return projected_fields(raw, cast(list[dict[str, object]], parsed['spans']), body_shape)


def persist_id_projection(version: int, raw: bytes, parsed: dict[str, object]) -> bytes:
    """Retain every raw field/tag/offset without inferring blob semantics; e.g. persist_id_projection(840002, raw, parsed)."""
    return body_projection(version, raw, parsed)


def persist_id_history(version: int, raw: bytes) -> bytes:
    """Reparse the first-word edit and derive every ordered outer/nested parent; e.g. persist_id_history(840002, raw)."""
    return body_history(version, raw, persist_id_fields, sheet_embedded_fields)
