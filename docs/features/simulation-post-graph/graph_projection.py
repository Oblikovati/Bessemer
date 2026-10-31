"""Complete graph primitive and atomic character-text reference projections (#165)."""
import importlib
import struct
from typing import cast
from thermal_projection import FieldShape, repeated_shape, projected_fields
from thermal_history_projection import outer_history
from graph_evidence import READER


def graph_span_shape(label: str, size: int) -> list[FieldShape]:
    """Preserve unassigned raw64 patterns while expanding documented word groups; e.g. graph_span_shape('twelve_words08_34', 48)."""
    counts = {'twelve_words08_34': 12, 'three_words3c_44': 3, 'six_words4c_60': 6,
              'eighteen_words64_a8': 18, 'words158_160': 3}
    if label in counts:
        assert size == counts[label] * 4, (label, size)
        return repeated_shape(label, 0, 4, counts[label])
    if label == 'sixteen_raw64_d8_150': return repeated_shape(label, 4, 8, 16)
    if label == 'word16c_two_raw64_words180_184':
        return [(label + '.word16c', 0, 4)] + repeated_shape(label + '.raw64', 4, 8, 2) + repeated_shape(label + '.words180_184', 0, 4, 2)
    if label == 'six_words198_1ac_raw64_1b0':
        return repeated_shape(label + '.words', 0, 4, 6) + [(label + '.raw64_1b0', 4, 8)]
    if label.startswith('text'): return [(label, 6, size)]
    assert label.startswith('word') and size == 4, (label, size)
    return [(label, 0, 4)]


def graph_atomic_spans(spans: list[dict[str, object]]) -> list[dict[str, object]]:
    """Merge each count and exact payload into one editable text field; e.g. graph_atomic_spans(spans)."""
    output: list[dict[str, object]] = []
    index = 0
    while index < len(spans):
        span = spans[index]
        label = cast(str, span['label'])
        if not label.endswith('.character_count'):
            output.append(span)
            index += 1
            continue
        payload = spans[index + 1]
        assert payload['label'] == label.removesuffix('.character_count') + '.utf16' and payload['offset'] == span['end']
        output.append(dict(label=label.removesuffix('.character_count'), offset=span['offset'], end=payload['end']))
        index += 2
    return output


def graph_fields(version: int, raw: bytes) -> list[bytes]:
    """Require full actual source acceptance before complete typed projection; e.g. graph_fields(890110, raw)."""
    parsed = importlib.import_module(READER).parse(raw, version)
    assert parsed['success'] and parsed['end'] == len(raw), (version, parsed)
    return projected_fields(raw, graph_atomic_spans(parsed['spans']), graph_span_shape)


def graph_projection(version: int, raw: bytes) -> bytes:
    """Retain every typed value, path and physical boundary; e.g. graph_projection(890110, raw)."""
    fields = graph_fields(version, raw)
    return struct.pack('<II', version, len(raw)) + raw + struct.pack('<I', len(fields)) + b''.join(fields)


def graph_history(version: int, raw: bytes) -> bytes:
    """Reparse a literal first-word edit and every ordered copy/derived parent; e.g. graph_history(890110, raw)."""
    replacement = struct.unpack_from('<i', raw)[0] ^ 32
    edited = struct.pack('<i', replacement) + raw[4:]
    fields = graph_fields(version, edited)
    return (struct.pack('<IIiII', 0, version, replacement, 1, len(raw)) + raw + edited
            + struct.pack('<I', len(fields)) + b''.join(fields)
            + outer_history(fields, True, 'twelve_words08_34[0]') + struct.pack('<I', 0))
