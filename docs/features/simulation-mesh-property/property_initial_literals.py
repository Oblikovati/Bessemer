"""Independent Mesh Property initial and non-monotonic gates (#163)."""
from property_literal_values import PropertyBlocks, property_words, property_raw64, property_text


def initial_property_blocks(version: int, rotation: int, null_texts: bool = False) -> PropertyBlocks:
    """Preserve the native wire order rather than sorting by version threshold; e.g. initial_property_blocks(810002, 1)."""
    initial = property_words(3, rotation) + property_raw64(3, rotation) + property_words(1, rotation + 3)
    blocks = [('mesh.initial_three_words_three_doubles_word284', initial),
              ('mesh.text5f0', property_text(rotation, null_texts)), ('mesh.text5f8', property_text(rotation + 1, null_texts))]
    if version >= 800005: blocks.append(('mesh.words708_70c', property_words(2, rotation)))
    if 800078 <= version < 810000 or version >= 810004: blocks.append(('mesh.word37c', property_words(1, rotation)))
    if version >= 800035: blocks.append(('mesh.four_doubles710_728', property_raw64(4, rotation + 1)))
    if version >= 800038: blocks.append(('mesh.word188', property_words(1, rotation + 1)))
    if version >= 830018: blocks.append(('mesh.doubles5b0_5b8', property_raw64(2, rotation + 2)))
    if version >= 830020: blocks.append(('mesh.word5e0', property_words(1, rotation + 2)))
    if version >= 830034: blocks.extend((f'mesh.text600_{index}', property_text(rotation + index, null_texts)) for index in range(4))
    if version >= 840018: blocks.append(('mesh.word378', property_words(1, rotation + 3)))
    if version >= 890514: blocks.append(('mesh.words730_734', property_words(2, rotation + 3)))
    if version >= 890601: blocks.append(('mesh.doubles560_568', property_raw64(2, rotation + 3)))
    if version >= 890605: blocks.append(('mesh.word280', property_words(1, rotation + 4)))
    return blocks
