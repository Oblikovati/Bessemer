"""Complete inherited-sheet fields and ordered history from actual C-737 calls (#151)."""
from typing import cast
from thermal_projection import FieldShape, projected_fields, repeated_shape
from body_projection import body_shape, body_projection
from body_history import body_history
from embedded_field_projection import embedded_field_case


def sheet_shape(label: str, size: int) -> list[FieldShape]:
    """Keep the inherited three-word block distinct from solid-body fields; e.g. sheet_shape('words48_50', 12)."""
    if label == 'words48_50':
        assert size == 12
        return repeated_shape(label, 0, 4, 3)
    return body_shape(label, size)


def sheet_fields(version: int, raw: bytes) -> list[bytes]:
    """Require complete actual inherited framing before field projection; e.g. sheet_fields(840002, raw)."""
    from cosworks_sheet_property_layout import parse
    from cosworks_embedded_const_surface import archive_reader
    parsed = parse(raw, version, archive_reader=archive_reader)
    assert parsed['success'] and parsed['end'] == len(raw)
    return projected_fields(raw, cast(list[dict[str, object]], parsed['spans']), sheet_shape)


def sheet_embedded_fields(raw: bytes) -> bytes:
    """Observe the exact constant-surface/plane-section callback; e.g. sheet_embedded_fields(raw)."""
    from cosworks_embedded_const_surface import archive_reader
    return embedded_field_case(raw, archive_reader)


def sheet_projection(version: int, raw: bytes, parsed: dict[str, object]) -> bytes:
    """Retain all independent raw values and physical field spans; e.g. sheet_projection(840002, raw, parsed)."""
    return body_projection(version, raw, parsed, sheet_shape)


def sheet_history(version: int, raw: bytes) -> bytes:
    """Reparse an isolated first persist-ID word edit and every nested parent; e.g. sheet_history(840002, raw)."""
    return body_history(version, raw, sheet_fields, sheet_embedded_fields)
