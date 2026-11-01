"""Complete independent dialog primitive fields and literal edit/history packets (#166)."""
import importlib
import struct
from typing import cast
from thermal_projection import projected_fields, FieldShape
from thermal_history_projection import outer_history
from thermal_nested_history import nested_copy_rows
from embedded_field_projection import embedded_field_case
from dialog_atomic_spans import dialog_atomic_spans
from dialog_projection_shapes import dialog_span_shapes


def actual_post_record(family: int, version: int, raw: bytes) -> dict[str, object]:
    """Dispatch only explicit complete-parent/helper families to their pinned readers; e.g. actual_post_record(0, 890620, raw)."""
    modules = ('cosworks_post_dialog_layout', 'cosworks_post_info_layout', 'cosworks_post_style_layout')
    assert type(family) is int and 0 <= family < 3, f'{family}; expected complete dialog/PostInfo/PostStyle family 0..2'
    return cast(dict[str, object], importlib.import_module(modules[family]).parse(raw, version))


def dialog_fields(family: int, version: int, raw: bytes) -> list[bytes]:
    """Project all complete typed fields after actual reader acceptance; e.g. dialog_fields(0, 890620, raw)."""
    parsed = actual_post_record(family, version, raw)
    assert parsed['success'] and parsed['end'] == len(raw), (family, version, parsed)
    spans = dialog_atomic_spans(parsed)
    labels = [cast(str, span['label']) for span in spans]
    assert len(labels) == len(set(labels)), 'expected unique physical role paths'
    tags = {cast(str, span['label']): cast(int, span['atomic_tag']) for span in spans if 'atomic_tag' in span}
    def shape(label: str, size: int) -> list[FieldShape]:
        return [(label, tags[label], size)] if label in tags else dialog_span_shapes(label, size)
    return projected_fields(raw, spans, shape)


def dialog_projection(family: int, version: int, raw: bytes) -> bytes:
    """Retain every path/type/value/source offset and complete input context; e.g. dialog_projection(0, 890620, raw)."""
    fields = dialog_fields(family, version, raw)
    return struct.pack('<III', family, version, len(raw)) + raw + struct.pack('<I', len(fields)) + b''.join(fields)


def dialog_embedded_fields(raw: bytes) -> bytes:
    """Use the exact scoped dialog reference archive callback; e.g. dialog_embedded_fields(raw)."""
    from cosworks_embedded_const_surface import archive_reader
    return embedded_field_case(raw, archive_reader)


def dialog_history(family: int, version: int, raw: bytes) -> bytes:
    """Reparse a literal first-word replacement and every ordered outer/nested parent; e.g. dialog_history(0, 890620, raw)."""
    replacement = struct.unpack_from('<i', raw)[0] ^ 32
    edited = struct.pack('<i', replacement) + raw[4:]
    fields = dialog_fields(family, version, edited)
    path_length = struct.unpack_from('<I', fields[0])[0]
    path = fields[0][4:4 + path_length].decode('utf-8')
    return (struct.pack('<IIiII', family, version, replacement, 1, len(raw)) + raw + edited
            + struct.pack('<I', len(fields)) + b''.join(fields) + outer_history(fields, True, path)
            + nested_copy_rows(fields, dialog_embedded_fields))
