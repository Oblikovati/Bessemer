"""Independent primitive expansion of every complete Mesh Property source span (#163)."""
from typing import cast
import re
import struct
from thermal_projection import FieldShape, projected_fields, repeated_shape
from shell_base_projection import shell_base_shape

INITIAL_GROUPS: dict[str, tuple[tuple[str, int, int], ...]] = {
    'mesh.initial_three_words_three_doubles_word284': (('word594', 0, 4), ('word598', 0, 4), ('word5dc', 0, 4),
        ('double5a0', 2, 8), ('double5a8', 2, 8), ('double5d0', 2, 8), ('word284', 0, 4)),
    'mesh.words708_70c': (('word708', 0, 4), ('word70c', 0, 4)),
    'mesh.four_doubles710_728': tuple((f'double{member:x}', 2, 8) for member in (0x710, 0x718, 0x720, 0x728)),
    'mesh.doubles5b0_5b8': (('double5b0', 2, 8), ('double5b8', 2, 8)),
    'mesh.words730_734': (('word730', 0, 4), ('word734', 0, 4)),
    'mesh.doubles560_568': (('double560', 2, 8), ('double568', 2, 8)),
    'mesh.words89c_8a0': (('word89c', 0, 4), ('word8a0', 0, 4)),
}
SINGLE_WORDS = ('mesh.word37c', 'mesh.word188', 'mesh.word5e0', 'mesh.word378', 'mesh.word280',
                'mesh.word738', 'mesh.word9b0', 'mesh.temporary_word', 'mesh.word2ac', 'mesh.word73c', 'mesh.duration_count')


def property_span_shape(label: str, size: int) -> list[FieldShape]:
    """Accept only documented groups, with exact fixed widths; e.g. property_span_shape('mesh.words708_70c', 8)."""
    if label in INITIAL_GROUPS:
        shape = [('mesh.' + name, tag, width) for name, tag, width in INITIAL_GROUPS[label]]
        assert sum(width for _, _, width in shape) == size, (label, size)
        return shape
    if label.startswith('mesh.settings.'):
        return [('mesh.settings' + path.removeprefix('property'), tag, width)
                for path, tag, width in shell_base_shape(label.removeprefix('mesh.settings.'), size)]
    if re.fullmatch(r'mesh\.text(?:5f[08]|600_[0-3])', label):
        path = label if '600_' not in label else 'mesh.text' + ('600', '608', '610', '618')[int(label[-1])]
        return [(path, 3, size)]
    if re.fullmatch(r'mesh\.text_list\.\d+', label): return [('mesh.text_list[' + label.rsplit('.', 1)[1] + ']', 3, size)]
    return property_collection_shape(label, size)


def property_collection_shape(label: str, size: int) -> list[FieldShape]:
    """Expand all scalar arrays and fixed record headers without interpreting raw children; e.g. property_collection_shape('mesh.double_array', 16)."""
    if property_count_shape(label):
        assert size == 4, (label, size)
        return [(label, 0, 4)]
    if re.fullmatch(r'mesh\.(?:record\d+\.(?:five_words|four_words1c_28|words14_18)|array_object\d+\.five_words)', label):
        widths = {'.five_words': 20, '.four_words1c_28': 16, '.words14_18': 8}
        assert size == next(width for suffix, width in widths.items() if label.endswith(suffix)), (label, size)
        return repeated_shape(label, 0, 4, size // 4)
    if re.fullmatch(r'mesh\.map\d+\.key_three_words_three_doubles_word', label): return property_key_header_shape(label, size)
    if re.fullmatch(r'mesh\.map\d+\.children', label):
        assert size % 40 == 0, (label, size)
        return repeated_shape(label, 4, 40, size // 40)
    arrays = {'mesh.duration_words': (0, 4), 'mesh.double_array': (2, 8), 'mesh.newest_set.keys': (0, 4)}
    if label in arrays: return property_array_shape(label, size, *arrays[label])
    assert re.fullmatch(r'mesh\.array_object\d+\.array[01]', label), f'{label}/{size}; expected a documented Mesh array'
    return property_array_shape(label, size, 0, 4)


def property_count_shape(label: str) -> bool:
    """Keep scalar/count roles closed to the declared stream grammar; e.g. property_count_shape('mesh.direct_records.count')."""
    if label in SINGLE_WORDS: return True
    pattern = (r'mesh\.(?:(?:text_list|direct_records|array_objects|map|double_array|newest_set)\.count|'
               r'record\d+\.extension(?:2c|38)\.(?:mode|count)|array_object\d+\.count[01]|map\d+\.child_count)')
    return re.fullmatch(pattern, label) is not None


def property_array_shape(label: str, size: int, tag: int, width: int) -> list[FieldShape]:
    """Require an integral number of complete physical entries; e.g. property_array_shape('mesh.double_array', 16, 2, 8)."""
    assert size % width == 0, f'{label}/{size}; expected complete {width}-byte array entries'
    return repeated_shape(label, tag, width, size // width)


def property_key_header_shape(label: str, size: int) -> list[FieldShape]:
    """Retain the older 44-byte keyed parent header separately from newest keys; e.g. property_key_header_shape(label, 44)."""
    assert size == 44, f'{label}/{size}; expected a complete 44-byte keyed parent header'
    prefix = label.removesuffix('.key_three_words_three_doubles_word')
    return [(prefix + '.key', 0, 4)] + repeated_shape(prefix + '.words', 0, 4, 3) + repeated_shape(prefix + '.doubles', 2, 8, 3) + [(prefix + '.word', 0, 4)]


def property_nested_shape(label: str, size: int) -> list[FieldShape]:
    """Keep unassigned eight-byte fields as fixed raw bytes rather than inferred doubles; e.g. property_nested_shape('fixed_eight_byte_fields', 264)."""
    fixed = {'fixed_eight_byte_fields': 264, 'eleven_raw64_tail': 88, 'two_raw64_tail': 16}
    if label in fixed:
        assert size == fixed[label], (label, size)
        return repeated_shape(label, 4, 8, size // 8)
    if re.fullmatch(r'array[01]\.nine_raw64', label):
        assert size == 72, (label, size)
        return repeated_shape(label, 4, 8, 9)
    if re.fullmatch(r'array[01]\.raw64_values', label): return property_array_shape(label, size, 4, 8)
    assert label == 'word00' or re.fullmatch(r'array[01]\.(word|count)', label), (label, size)
    assert size == 4, (label, size)
    return [(label, 0, 4)]


def nested_property_fields(raw: bytes, version: int, span: dict[str, object]) -> list[bytes]:
    """Expand one untagged nested body within its containing record and exact byte extent; e.g. nested_property_fields(raw, version, span)."""
    from cosworks_mesh_nested_record import parse
    start, end = cast(int, span['offset']), cast(int, span['end'])
    parsed = parse(raw, version, offset=start, require_eof=False)
    assert parsed['success'] and parsed['end'] == end, parsed
    fields: list[bytes] = []
    for nested in parsed['spans']:
        assert raw[nested['offset']:nested['end']].hex() == nested['raw_hex'], nested
        shape = [(cast(str, span['label']) + '.' + path, tag, width)
                 for path, tag, width in property_nested_shape(nested['label'], nested['end'] - nested['offset'])]
        fields.extend(projected_fields(raw, [nested], lambda label, size: shape))
    return fields


def property_fields(raw: bytes, version: int, parsed: dict[str, object]) -> list[bytes]:
    """Project every complete primitive field, including all scoped nested occurrences; e.g. property_fields(raw, version, parsed)."""
    assert parsed['success'] and parsed['end'] == len(raw), parsed
    fields: list[bytes] = []
    for span in cast(list[dict[str, object]], parsed['spans']):
        label = cast(str, span['label'])
        if re.fullmatch(r'mesh\.record\d+\.extension(?:2c|38)\.nested\d+', label):
            fields.extend(nested_property_fields(raw, version, span))
            continue
        fields.extend(projected_fields(raw, [span], property_span_shape))
    return fields


def property_projection(version: int, raw: bytes, parsed: dict[str, object]) -> bytes:
    """Use the established complete operation packet format with explicit version; e.g. property_projection(version, raw, parsed)."""
    fields = property_fields(raw, version, parsed)
    return struct.pack('<III', 0, version, len(raw)) + raw + struct.pack('<I', len(fields)) + b''.join(fields)
