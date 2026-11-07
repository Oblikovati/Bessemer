"""Independent physical expansion of the documented shell shared base (#159)."""
import re
from thermal_projection import FieldShape, span_shape, scalar_shape, optional_shape, repeated_shape


def shell_base_shape(label: str, size: int) -> list[FieldShape]:
    """Map source groups to scoped primitive roles without inferring semantics; e.g. shell_base_shape('base.word198', 4)."""
    if label.startswith(('references.', 'reference')): return span_shape('property.' + label, size)
    if label.startswith('base.virtual'): return shell_optional_shape(label, size)
    if label.startswith('base.curve') and label != 'base.curve_count': return shell_curve_shape(label, size)
    if label == 'base.doublef8_wordec_doublee0_worde8_words110_a8_1e0':
        assert size == 36, (label, size)
        return scalar_shape('property.base.scalar_block')
    texts = {'base.text78': 0, 'base.compat0': 1, 'base.compat1': 2, 'base.text60': 3}
    if label in texts: return [(f'property.base.text[{texts[label]}]', 3, size)]
    if label.startswith('base.arraytext'):
        index = int(label.removeprefix('base.arraytext'))
        assert 0 <= index < 6, label
        return [(f'property.base.array_text[{index}]', 3, size)]
    return shell_base_scalar_shape(label, size)


def shell_base_scalar_shape(label: str, size: int) -> list[FieldShape]:
    """Retain every exact base word/text/double gate as an independent field; e.g. shell_base_scalar_shape('base.double108', 8)."""
    groups = {'base.words68_58': ('property.base.two_integers', 2), 'base.words148_150': ('property.base.three_integers', 3)}
    if label in groups:
        path, count = groups[label]
        assert size == count * 4, (label, size)
        return repeated_shape(path, 0, 4, count)
    if label == 'base.six_doubles':
        assert size == 48, (label, size)
        return repeated_shape('property.base.six_doubles', 2, 8, 6)
    names = {'base.word198': ('base.integer', 0), 'base.word100': ('base.id', 0),
             'base.wordac': ('base.ac', 0), 'base.word48': ('base.48', 0), 'base.word230': ('base.230', 0),
             'base.wordc8': ('base.c8', 0), 'base.word170': ('base.170', 0), 'base.double108': ('base.108', 2),
             'base.text158': ('base.158', 3), 'base.text160': ('base.160', 3),
             'base.word2e0': ('table.mode', 0), 'base.curve_count': ('table.count', 0)}
    assert label in names, (label, size)
    path, tag = names[label]
    assert tag == 3 or size == (8 if tag == 2 else 4), (label, size)
    return [('property.' + path, tag, size)]


def shell_optional_shape(label: str, size: int) -> list[FieldShape]:
    """Expand documented virtual record headers and conditional arrays; e.g. shell_optional_shape('base.virtual0.body.header', 20)."""
    match = re.fullmatch(r'base\.virtual([01])\.(word|presence|body\..+)', label)
    assert match is not None, label
    prefix, suffix = 'property.optional' + match[1], match[2]
    if suffix in ('word', 'presence'):
        assert size == 4, (label, size)
        return [(prefix + ('.integer' if suffix == 'word' else '.present'), 0, 4)]
    role = suffix.removeprefix('body.')
    if role == 'name': return [(prefix + '.name', 3, size)]
    if role == 'header':
        assert size == 20, (label, size)
        return optional_shape(prefix + '.header')
    if role in ('nine_doubles', 'pairs'):
        assert size % 8 == 0 and (role != 'nine_doubles' or size == 72), (label, size)
        return repeated_shape(prefix + '.' + role, 2, 8, size // 8)
    assert role == 'version_integer' and size == 4, (label, size)
    return [(prefix + '.' + role, 0, 4)]


def shell_curve_shape(label: str, size: int) -> list[FieldShape]:
    """Keep all fifteen interleaved double/word pairs and their raw text; e.g. shell_curve_shape('base.curve0.textc0', 4)."""
    match = re.fullmatch(r'base\.curve(\d+)\.(textc0|fifteen_double_word_pairs)', label)
    assert match is not None, label
    prefix = 'property.table' + match[1]
    if match[2] == 'textc0': return [(prefix + '.name', 3, size)]
    assert size == 180, (label, size)
    return [(f'{prefix}.pairs[{index}].{name}', tag, width) for index in range(15)
            for name, tag, width in (('double', 2, 8), ('word', 0, 4))]
