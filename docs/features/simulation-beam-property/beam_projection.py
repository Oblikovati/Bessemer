"""Independent complete typed beam expansion from actual Source spans (#142)."""
import struct
from typing import cast
from thermal_projection import FieldShape, projected_fields, repeated_shape, span_shape
from thermal_history_projection import outer_history
from thermal_nested_history import nested_copy_rows


def beam_shape(label: str, size: int) -> list[FieldShape]:
    """Preserve exact beam scalar widths and mixed unsigned fields; e.g. beam_shape(label, 20)."""
    if '.beam.' not in label: return span_shape(label, size)
    prefix, suffix = label.rsplit('.', 1)
    if suffix == 'four_words_two_shorts':
        return repeated_shape(prefix + '.four_words', 0, 4, 4) + [(prefix + '.shorte2', 6, 2), (prefix + '.shorte0', 6, 2)]
    if suffix == 'five_words':
        return [(prefix + '.' + name, tag, 4) for name, tag in
                (('word08', 0), ('word0c', 0), ('word10', 1), ('word14', 0), ('word18', 1))]
    if suffix in ('four_doubles', 'three_doubles', 'points'):
        assert size % 8 == 0, (label, size)
        return repeated_shape(label, 2, 8, size // 8)
    if suffix == 'two_longs': return repeated_shape(label, 0, 4, 2)
    if suffix in ('textd0', 'textc8'): return [(label, 3, size)]
    assert suffix in ('point_count', 'link_word', 'word80', 'worda8') and size == 4, (label, size)
    return [(label, 0, 4)]


def beam_parser(raw: bytes, version: int) -> dict[str, object]:
    """Use the unmodified complete parser with explicit archive dependencies; e.g. beam_parser(raw, 850036)."""
    from cosworks_beam_property_layout import parse
    from cosworks_binary_thermal_transform_audit import archive_reader
    return cast(dict[str, object], parse(raw, version, 'Beam', archive_reader=archive_reader))


def beam_fields(raw: bytes, parsed: dict[str, object]) -> list[bytes]:
    """Expand all physical raw values only after complete acceptance; e.g. beam_fields(raw, parsed)."""
    assert parsed['success'] and parsed['end'] == len(raw)
    return projected_fields(raw, cast(list[dict[str, object]], parsed['spans']), beam_shape)


def projection_case(version: int, raw: bytes, parsed: dict[str, object]) -> bytes:
    """Export full independently typed fields and offsets; e.g. projection_case(850036, raw, parsed)."""
    fields = beam_fields(raw, parsed)
    return struct.pack('<II', version, len(raw)) + raw + struct.pack('<I', len(fields)) + b''.join(fields)


def history_case(version: int, raw: bytes) -> bytes:
    """Independently edit the signed short and derive all ordered outer parents; e.g. history_case(850036, raw)."""
    parsed = beam_parser(raw, version)
    span = next(span for span in cast(list[dict[str, object]], parsed['spans']) if span['label'] == 'record0.beam.four_words_two_shorts')
    start = cast(int, span['offset']) + 16
    old = struct.unpack_from('<h', raw, start)[0]
    replacement = old ^ 32
    expected = raw[:start] + struct.pack('<h', replacement) + raw[start + 2:]
    after = beam_parser(expected, version)
    fields = beam_fields(expected, after)
    return (struct.pack('<IiI', version, replacement, len(raw)) + raw + expected
            + struct.pack('<I', len(fields)) + b''.join(fields)
            + outer_history(fields, True, 'record0.beam.shorte2'))


def nested_history_case(version: int, raw: bytes) -> bytes:
    """Include every independently observed embedded copy occurrence; e.g. nested_history_case(850036, raw)."""
    fields = beam_fields(raw, beam_parser(raw, version))
    return history_case(version, raw) + nested_copy_rows(fields)
