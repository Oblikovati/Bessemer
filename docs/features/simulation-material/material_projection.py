"""Independent physical scalar/text expansion of every complete actual Material source span (#140)."""
from typing import cast
import struct
from thermal_projection import FieldShape, projected_fields, repeated_shape


def mixed_shape(label: str) -> list[FieldShape] | None:
    """Expand the two declared mixed groups without treating them as byte blobs; e.g. mixed_shape('curve_group0.item0.mixed_scalars')."""
    if label == 'double78_words80_84_88':
        return [('double78', 2, 8)] + repeated_shape('words80_84_88', 0, 4, 3)
    if label.endswith('.mixed_scalars'):
        prefix = label.removesuffix('.mixed_scalars')
        return [(prefix + '.' + name, tag, width) for name, tag, width in
                (('word90', 0, 4), ('double88', 2, 8), ('word94', 0, 4),
                 ('doublesb0_b8[0]', 2, 8), ('doublesb0_b8[1]', 2, 8))]
    return None


def material_text_label(label: str) -> bool:
    """Identify only the actual Material persisted text spans; e.g. material_text_label('property0.name')."""
    return label.startswith('text') or label.endswith(('.name', '.auxiliary_text', '.curve_name', '.text10'))


def material_shape(label: str, size: int) -> list[FieldShape]:
    """Assign only declared physical widths/types and preserve raw text frames; e.g. material_shape('header_words34_38_3c', 12)."""
    if material_text_label(label):
        return [(label, 3, size)]
    mixed = mixed_shape(label)
    if mixed is not None: return mixed
    if label.endswith('.pairs'):
        assert size % 16 == 0, (label, size)
        return repeated_shape(label, 2, 8, size // 8)
    if label.endswith('.nine_doubles'):
        assert size == 72, (label, size)
        return repeated_shape(label, 2, 8, 9)
    return word_or_scalar_shape(label, size)


def word_or_scalar_shape(label: str, size: int) -> list[FieldShape]:
    """Retain complete direct word arrays and individual raw doubles; e.g. word_or_scalar_shape('property0.scalar', 8)."""
    groups = {'header_words34_38_3c': 3, 'words3900_3904': 2, 'words70_74': 2,
              'words3958_3988_398c_3990': 4, 'words8c_90': 2,
              'four_words': 4, 'seven_words': 7, 'three_words': 3}
    count = groups.get(label.rsplit('.', 1)[-1])
    if count is not None:
        assert size == count * 4, (label, size)
        return repeated_shape(label, 0, 4, count)
    tag = 2 if label.endswith('.scalar') or label in ('double78', 'double98') else 0
    assert size == (8 if tag == 2 else 4), (label, size)
    return [(label, tag, size)]


def material_projection_case(raw: bytes, version: int, parsed: dict[str, object]) -> bytes:
    """Require complete actual consumption before freezing every path/value/offset; e.g. material_projection_case(raw, 830036, parsed)."""
    assert parsed['success'] and parsed['end'] == len(raw)
    fields = projected_fields(raw, cast(list[dict[str, object]], parsed['spans']), material_shape)
    return struct.pack('<II', version, len(raw)) + raw + struct.pack('<I', len(fields)) + b''.join(fields)
