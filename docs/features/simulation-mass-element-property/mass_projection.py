"""Independent typed expansion of every actual single-body mass Property span (#138)."""
import re
import struct
from typing import cast
from thermal_projection import FieldShape, projected_fields, repeated_shape, span_shape


def mass_shape(label: str, size: int) -> list[FieldShape]:
    """Expand only declared scalar/text groups in physical order; e.g. mass_shape('record0.mass.three_words', 12)."""
    if '.mass.' not in label:
        return span_shape(label, size)
    if re.search(r'\.(?:header_text|text)\d+$', label):
        return [(re.sub(r'(\.(?:header_text|text))(\d+)$', r'\1[\2]', label), 3, size)]
    groups = {'three_words': (0, 4, 3), 'three_flags': (0, 4, 3),
              'six_doubles': (2, 8, 6), 'six_words': (0, 4, 6),
              'sixteen_doubles': (2, 8, 16), 'six_tail_words': (0, 4, 6)}
    suffix = label.rsplit('.', 1)[1]
    if suffix in groups:
        tag, width, count = groups[suffix]
        assert size == width * count, (label, size)
        return repeated_shape(label, tag, width, count)
    if suffix == 'pairs':
        assert size % 8 == 0, (label, size)
        return repeated_shape(label, 0, 4, size // 4)
    assert suffix in ('selector', 'pair_count', 'word410') and size == 4, (label, size)
    return [(label, 0, 4)]


def mass_projection_case(version: int, raw: bytes, parsed: dict[str, object]) -> bytes:
    """Require complete actual-reader consumption before projecting raw values; e.g. mass_projection_case(830036, raw, parsed)."""
    assert parsed['success'] and parsed['end'] == len(raw)
    fields = projected_fields(raw, cast(list[dict[str, object]], parsed['spans']), mass_shape)
    return struct.pack('<II', version, len(raw)) + raw + struct.pack('<I', len(fields)) + b''.join(fields)
