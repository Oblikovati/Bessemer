"""Complete explicit dialog physical value expansion and current role-based scalar typing (#166)."""
import importlib
import re
from thermal_projection import FieldShape, repeated_shape
from dialog_postinfo_projection import postinfo_field_shapes

WORD_GROUPS = {'fourteen_words0c_28_30_3c_518_51c': 14, 'eight_words528_544': 8,
    'three_words568_570': 3, 'eleven_words': 11, 'id_header': 2, 'words38_48': 5,
    'legacy_header': 7, 'words58_5c': 2, 'two_trailing_words': 2}
RAW64_GROUPS = {'four_raw64_548_560': 4, 'sixteen_raw64_590_608': 16}


def dialog_span_shapes(label: str, size: int) -> list[FieldShape]:
    """Reject unassigned span shapes instead of swallowing them as opaque fallback bytes; e.g. dialog_span_shapes('post_info.raw_block', 460)."""
    if label == 'post_info.raw_block': return postinfo_field_shapes(size)
    if label.endswith(('.persist_text', '.legacy_persist_text')): return [(label, 3, size)]
    if re.search(r'\.archive\[[01]\]$', label): return [(label, 5, size)]
    special = dialog_group_shapes(label, size)
    if special is not None: return special
    if '.color_' in label and '.component_' not in label:
        assert size == 16, (label, size)
        return [(f'{label}.component_{index}', 8, 4) for index in range(4)]
    assert size == 4, f'{label}/{size}; expected a documented complete primitive shape'
    return [(label, dialog_scalar_tag(label), 4)]


def dialog_group_shapes(label: str, size: int) -> list[FieldShape] | None:
    """Expand only the fixed or explicitly counted groups established by the wire grammar; e.g. dialog_group_shapes('eight_words528_544', 32)."""
    suffix = label.rsplit('.', 1)[-1]
    count = WORD_GROUPS.get(label, WORD_GROUPS.get(suffix))
    if count is not None:
        assert size == count * 4, (label, size, count)
        return dialog_word_group_shapes(label, count)
    if label == 'dialog.da8_integer_array': return repeated_shape(label, 0, 4, size // 4)
    if label in RAW64_GROUPS: return repeated_shape(label, 9, 8, RAW64_GROUPS[label])
    if re.fullmatch(r'mesh_quality\.raw64_pair\[\d+\]', label): return repeated_shape(label, 9, 8, 2)
    if label == 'dialog.raw64_bc8_bd0_words_bd8_bdc':
        return repeated_shape(label + '.raw64', 9, 8, 2) + repeated_shape(label + '.words', 7, 4, 2)
    if label.endswith(('.word60_raw64_68', '.two_words_raw64')):
        count = 1 if label.endswith('.word60_raw64_68') else 2
        return repeated_shape(label + '.words', 7, 4, count) + [(label + '.raw64', 9, 8)]
    if size == 8 and ('raw64' in label or label == 'dialog.field_ec8'): return [(label, 9, 8)]
    return None


def dialog_scalar_tag(label: str) -> int:
    """Use established scalar roles while leaving unresolved words as raw32 bits; e.g. dialog_scalar_tag('legend.word_ac')."""
    if label.endswith('.count') or label == 'temporary_state_word': return 0
    if '.component_' in label and (label.startswith(('style.color_', 'legend.color_', 'v7.'))): return 8
    description = importlib.import_module('cosworks_post_style_field_catalog').describe(label)
    if description['wire_type'] == 'binary32_le': return 8
    if description['wire_type'] == 'int32_le': return 0
    return 7


def dialog_word_group_shapes(label: str, count: int) -> list[FieldShape]:
    """Bind parent scalar types from schema14.492–493 without changing unknown word bits; e.g. dialog_word_group_shapes('eight_words528_544', 8)."""
    fields = repeated_shape(label, 7, 4, count)
    if label == 'fourteen_words0c_28_30_3c_518_51c':
        for index in (1, 13): fields[index] = (fields[index][0], 0, 4)
    if label == 'eight_words528_544':
        for index in range(6): fields[index] = (fields[index][0], 8, 4)
    return fields


def verify_dialog_scalar_shapes() -> None:
    """Check independently specified parent offsets and preserve unresolved words; e.g. verify_dialog_scalar_shapes()."""
    assert [tag for _, tag, _ in dialog_span_shapes('eight_words528_544', 32)] == [8] * 6 + [7, 7]
    assert [tag for _, tag, _ in dialog_span_shapes('fourteen_words0c_28_30_3c_518_51c', 56)] == [7, 0] + [7] * 11 + [0]
    assert dialog_span_shapes('temporary_state_word', 4) == [('temporary_state_word', 0, 4)]
    assert dialog_span_shapes('unresolved_parent_word', 4) == [('unresolved_parent_word', 7, 4)]
