"""Complete primitive Results projections from exact original reader spans (#164)."""
import struct
from typing import cast
from thermal_projection import FieldShape, projected_fields, repeated_shape
from results_native import actual_results


def results_shape(label: str, size: int) -> list[FieldShape]:
    """Expand only the documented plot word pair; e.g. results_shape('temporary_word_and_member28', 8)."""
    if label == 'temporary_word_and_member28':
        assert size == 8, (label, size)
        return repeated_shape(label, 0, 4, 2)
    if label.startswith('text') and label != 'text_list.count': return [(label, 3, size)]
    assert size == 4, (label, size)
    return [(label, 1 if label in ('member70', 'unsigneda8') else 0, 4)]


def results_fields(level: int, version: int, raw: bytes) -> list[bytes]:
    """Retain every accepted primitive and raw text frame; e.g. results_fields(0, 890605, raw)."""
    parsed = actual_results(raw, version, level)
    assert parsed['success'] and parsed['end'] == len(raw), (level, version, parsed)
    return projected_fields(raw, cast(list[dict[str, object]], parsed['spans']), results_shape)


def results_projection(level: int, version: int, raw: bytes) -> bytes:
    """Project every physical field after complete actual reference acceptance; e.g. results_projection(0, 890605, raw)."""
    fields = results_fields(level, version, raw)
    return struct.pack('<III', level, version, len(raw)) + raw + struct.pack('<I', len(fields)) + b''.join(fields)
