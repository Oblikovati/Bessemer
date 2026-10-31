"""Independent primitive expansion of actual complete source spans (#134)."""
import re
import struct
from collections.abc import Callable
from typing import cast
from owned_thermal import THERMAL_KINDS, owned_thermal_cases
from thermal_malformed import is_text_span
from native_carriers import ThermalCarrier

FieldShape = tuple[str, int, int]


def repeated_shape(label: str, tag: int, width: int, count: int) -> list[FieldShape]:
    """Expand physical array ordinals; e.g. repeated_shape('base.words', 0, 4, 2)."""
    return [(f'{label}[{index}]', tag, width) for index in range(count)]


def scalar_shape(label: str) -> list[FieldShape]:
    """Split the documented mixed scalar block; e.g. scalar_shape('record0.base.scalar_block')."""
    return [(label + suffix, tag, width) for suffix, tag, width in
            (('.f8', 2, 8), ('.ec', 0, 4), ('.e0', 2, 8), ('.e8', 1, 4),
             ('.words[0]', 0, 4), ('.words[1]', 0, 4), ('.words[2]', 0, 4))]


def optional_shape(label: str) -> list[FieldShape]:
    """Preserve all five independent optional header words; e.g. optional_shape('record0.optional0.header')."""
    prefix = label.removesuffix('.header')
    return [(prefix + suffix, 0, 4) for suffix in
            ('.header[0]', '.header[1]', '.header_middle[0]', '.header_middle[1]', '.header[4]')]


def thermal_shape(label: str) -> list[FieldShape] | None:
    """Expand mixed derived tails from the full reference grammar; e.g. thermal_shape('record0.thermal.two_words_double')."""
    prefix, suffix = label.rsplit('.', 1)
    shapes = {
        'two_words_double': [('words[0]', 0, 4), ('words[1]', 0, 4), ('double', 2, 8)],
        'two_words_two_doubles': [('words[0]', 0, 4), ('words[1]', 0, 4), ('doubles[0]', 2, 8), ('doubles[1]', 2, 8)],
        'words_double_word': [('words[0]', 0, 4), ('words[1]', 0, 4), ('double0', 2, 8), ('word0', 0, 4)],
        'two_doubles_word': [('doubles[0]', 2, 8), ('doubles[1]', 2, 8), ('word1', 0, 4)],
    }
    if suffix not in shapes: return None
    return [(prefix + '.' + name, tag, width) for name, tag, width in shapes[suffix]]


def primitive_shape(label: str, size: int) -> list[FieldShape]:
    """Expand only declared source span groups; e.g. primitive_shape('record0.base.six_doubles', 48)."""
    if label.endswith('.base.scalar_block'): return scalar_shape(label)
    if '.optional' in label and label.endswith('.header'): return optional_shape(label)
    derived = thermal_shape(label)
    if derived is not None: return derived
    if re.search(r'\.table\d+\.pairs$', label):
        return [(f'{label}[{index}].{name}', tag, width) for index in range(15)
                for name, tag, width in (('double', 2, 8), ('word', 0, 4))]
    if label.endswith(('.six_doubles', '.nine_doubles', '.four_doubles')):
        return repeated_shape(label.replace('.four_doubles', '.doubles'), 2, 8, size // 8)
    if label.endswith('.pairs'):
        return repeated_shape(label, 0 if '.thermal.pairs' in label else 2, 4 if '.thermal.pairs' in label else 8, size // (4 if '.thermal.pairs' in label else 8))
    if label.endswith(('.tail', '.late_tail', '.legacy_header', '.id_header', '.header', '.two_integers', '.three_integers', '.two_late_words', '.four_words')):
        return repeated_shape(label.replace('.four_words', '.words'), 0, 4, size // 4)
    return [(label.replace('.thermal.optional_word', '.thermal.optional.integer').replace('.thermal.optional_present', '.thermal.optional.present'), 2 if label.endswith('.base.108') else 0, size)]


def span_shape(label: str, size: int) -> list[FieldShape]:
    """Keep text/blob/embedded boundaries atomic and expand primitives; e.g. span_shape('record0.base.text0', 4)."""
    if is_text_span(label):
        path = re.sub(r'(\.base\.(?:array_text|text)|\.thermal\.text)(\d+)$', r'\1[\2]', label)
        return [(path, 3, size)]
    if label.endswith('.blob'): return [(label, 4, size)]
    if label.endswith(('.archive0', '.archive1')): return [(label, 5, size)]
    return primitive_shape(label, size)


def projected_fields(raw: bytes, spans: list[dict[str, object]], shape: Callable[[str, int], list[FieldShape]] = span_shape) -> list[bytes]:
    """Encode every complete typed field with independent source offsets; e.g. projected_fields(raw, spans)."""
    fields: list[bytes] = []
    for span in spans:
        start, end = cast(int, span['offset']), cast(int, span['end'])
        for path, tag, width in shape(cast(str, span['label']), end - start):
            name = path.encode('utf-8')
            fields.append(struct.pack('<I', len(name)) + name + struct.pack('<III', tag, start, start + width) + raw[start:start + width])
            start += width
        assert start == end, (span, start)
    return fields


def projection_case(kind: int, version: int, raw: bytes) -> bytes:
    """Require complete actual callback acceptance before primitive projection; e.g. projection_case(0, 890501, raw)."""
    from cosworks_binary_thermal_transform_audit import parse_native, archive_reader
    parsed = parse_native(raw, version, THERMAL_KINDS[kind], archive_reader)
    assert parsed['success'] and parsed['end'] == len(raw)
    fields = projected_fields(raw, parsed['spans'])
    return struct.pack('<III', kind, version, len(raw)) + raw + struct.pack('<I', len(fields)) + b''.join(fields)


def owned_projection_packet() -> bytes:
    """Project every owned gate and selector context; e.g. owned_projection_packet()."""
    return b''.join(projection_case(kind, version, raw) for kind, version, raw in owned_thermal_cases())


def native_projection_packet(carriers: list[tuple[ThermalCarrier, bytes]]) -> bytes:
    """Retain each successful native candidate without inferring runtime dispatch; e.g. native_projection_packet(carriers)."""
    packet = bytearray()
    for row, raw in carriers:
        kind = THERMAL_KINDS.index(row['stream'].split('/')[-1])
        selected = next(attempt for attempt in row['attempts'] if attempt['success'])
        packet.extend(projection_case(kind, cast(int, selected['version']), raw))
    return bytes(packet)
