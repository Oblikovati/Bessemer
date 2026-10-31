"""Independent typed primitive positions derived from actual complete reader spans (#133)."""
from dataclasses import dataclass
from cosworkspersist import ParameterSpan, ParameterSets


@dataclass(frozen=True)
class ParameterField:
    role: tuple[int, int, int]
    offset: int
    kind: int
    raw: bytes


def words(span: ParameterSpan, tag: int, record: int | None = None) -> list[ParameterField]:
    """Split complete table/header/value spans at their documented fixed primitive width."""
    width = 8 if tag == 10 else 4
    kind = 2 if tag == 10 else 0
    assert len(span.payload) % width == 0
    return [ParameterField((tag, index if record is None else record, 0 if record is None else index), span.offset + index * width, kind, span.payload[index * width:index * width + width]) for index in range(len(span.payload) // width)]


def parent_fields(span: ParameterSpan) -> list[ParameterField]:
    """Assign no business meanings to the five header words or parent gates."""
    if span.label == 'header':
        assert len(span.payload) == 20
        return words(span, 0)
    if span.label == 'table':
        return words(span, 4)
    tag = {'gate700013': 1, 'gate700018': 2, 'record_count': 3, 'tail': 11, 'gate820015': 12, 'gate830009': 13}[span.label]
    assert len(span.payload) == 4
    return [ParameterField((tag, 0, 0), span.offset, 0, span.payload)]


def record_fields(span: ParameterSpan) -> list[ParameterField]:
    """Keep signed/unsigned distinction and raw double bits without numeric conversion."""
    prefix, label = span.label.split('.', 1)
    index = int(prefix.removeprefix('record'))
    if label == 'integers':
        assert len(span.payload) == 12
        return [ParameterField((6, index, 0), span.offset, 0, span.payload[:4]), *[ParameterField((7, index, component), span.offset + 4 + component * 4, 1, span.payload[4 + component * 4:8 + component * 4]) for component in range(2)]]
    if label == 'values':
        return words(span, 10, index)
    tag, kind = {'text': (5, 3), 'long': (8, 0), 'double': (9, 2)}[label]
    return [ParameterField((tag, index, 0), span.offset, kind, span.payload)]


def fields(record: ParameterSets) -> list[ParameterField]:
    """Flatten every complete physical primitive/text in original order."""
    result = []
    for span in record.spans:
        result.extend(record_fields(span) if span.label.startswith('record') and '.' in span.label else parent_fields(span))
    assert b''.join(field.raw for field in result) == b''.join(span.payload for span in record.spans)
    return result


def structural(field: ParameterField, version: int) -> bool:
    """Count edits are distinct from full reconstruction; legacy header zero stays ordinary."""
    return field.role[0] == 3 or (field.role == (0, 0, 0) and version >= 810026)
