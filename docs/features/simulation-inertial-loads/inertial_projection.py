"""Independent typed expansion of complete actual inertial/excitation spans (#136)."""
import re
import struct
from typing import cast
from thermal_projection import FieldShape, repeated_shape, span_shape, projected_fields
from owned_inertial import LOAD_KINDS, owned_inertial_cases


def excitation_shape(label: str) -> list[FieldShape] | None:
    """Split the mixed header and late word/double/word group; e.g. excitation_shape('record0.excitation.five_signed_one_unsigned')."""
    prefix, suffix = label.rsplit('.', 1)
    if suffix == 'five_signed_one_unsigned':
        return repeated_shape(prefix + '.signed_header', 0, 4, 5) + [(prefix + '.unsigned_header', 1, 4)]
    if suffix == 'word_double_word':
        return [(prefix + '.' + name, tag, width) for name, tag, width in
                (('mixed_word0', 0, 4), ('mixed_double', 2, 8), ('mixed_word1', 0, 4))]
    return None


def inertial_shape(label: str, size: int) -> list[FieldShape]:
    """Use declared scalar groups and preserve all raw counted text; e.g. inertial_shape('record0.gravity.four_words', 16)."""
    if not any(f'.{prefix}.' in label for prefix in ('gravity', 'centrifugal', 'excitation')):
        return span_shape(label, size)
    if re.search(r'\.text\d+$', label):
        return [(re.sub(r'\.text(\d+)$', r'.text[\1]', label), 3, size)]
    mixed = excitation_shape(label)
    if mixed is not None: return mixed
    prefix, suffix = label.rsplit('.', 1)
    if suffix.endswith('_doubles'): return repeated_shape(label, 2, 8, size // 8)
    groups = {'four_words': 'words', 'seven_words': 'words', 'three_late_words': 'late_words',
              'two_late_words': 'late_words', 'three_words': 'three_words', 'three_tail_words': 'tail_words'}
    if suffix in groups: return repeated_shape(prefix + '.' + groups[suffix], 0, 4, size // 4)
    assert suffix == 'fourth_tail_word', label
    return [(label, 0, 4)]


def projection_case(kind: int, version: int, raw: bytes) -> bytes:
    """Validate complete actual source consumption before projecting every field; e.g. projection_case(0, 870022, raw)."""
    from cosworks_inertial_layout import parse as parse_inertial
    from cosworks_base_excitation_layout import parse as parse_excitation
    from cosworks_binary_thermal_transform_audit import archive_reader
    parser = parse_excitation if kind == 2 else parse_inertial
    parsed = parser(raw, version, LOAD_KINDS[kind], archive_reader=archive_reader)
    assert parsed['success'] and parsed['end'] == len(raw)
    fields = projected_fields(raw, cast(list[dict[str, object]], parsed['spans']), inertial_shape)
    return struct.pack('<III', kind, version, len(raw)) + raw + struct.pack('<I', len(fields)) + b''.join(fields)


def owned_projection_packet() -> bytes:
    """Preserve every complete owned field and gate context; e.g. owned_projection_packet()."""
    return b''.join(projection_case(kind, version, raw) for kind, version, raw in owned_inertial_cases())
