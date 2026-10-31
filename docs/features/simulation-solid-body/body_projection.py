"""Complete independent primitive expansion of actual solid-body spans (#150)."""
import struct
from collections.abc import Callable
from typing import cast
from thermal_projection import FieldShape, projected_fields, repeated_shape


def body_shape(label: str, size: int) -> list[FieldShape]:
    """Expand only the documented physical primitive groups; e.g. body_shape('words90_94', 8)."""
    if label.startswith('text') or label.startswith('list128.') and label != 'list128.count': return [(label, 3, size)]
    if label == 'persistid.blob': return [(label, 4, size)]
    if label.startswith('persistid.archive'): return [(label, 5, size)]
    widths = {'persistid.words': 2, 'words70_7c': 4, 'words90_94': 2,
              'wordsa8_b0': 3, 'helper20.two_words': 2, 'words160_168': 3, 'words1f0_1f4': 2}
    if label in widths:
        assert size == widths[label] * 4
        return repeated_shape(label, 0, 4, widths[label])
    if label == 'wordsbc_c8_doublesd0_f0':
        assert size == 56
        return repeated_shape('wordsbc_c8', 0, 4, 4) + repeated_shape('doublesd0_f0', 2, 8, 5)
    assert label.startswith('word') or label in ('persistid.blob_count', 'list128.count'), label
    assert size == 4, (label, size)
    return [(label, 0, 4)]


def body_projection(version: int, raw: bytes, parsed: dict[str, object], shape: Callable[[str, int], list[FieldShape]] = body_shape) -> bytes:
    """Retain every exact path/tag/raw value/offset from actual reader calls; e.g. body_projection(version, raw, parsed)."""
    assert parsed['success'] and parsed['end'] == len(raw)
    fields = projected_fields(raw, cast(list[dict[str, object]], parsed['spans']), shape)
    return struct.pack('<II', version, len(raw)) + raw + struct.pack('<I', len(fields)) + b''.join(fields)
