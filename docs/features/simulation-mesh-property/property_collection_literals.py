"""Independent inline reference, direct-record and counted-array literals (#163)."""
import struct
from owned_thermal import thermal_reference_blocks
from property_literal_values import PropertyBlocks, assemble_property_blocks, property_count, property_words, property_raw64, property_text
from property_nested_literals import nested_property_blocks


def property_settings_blocks(version: int, profile: int) -> PropertyBlocks:
    """Reuse the original physical reference grammar with repeated and null archive occurrences; e.g. property_settings_blocks(840002, 4)."""
    count = (0, 1, 2, 3, 2)[profile]
    blocks = [property_count('mesh.text_list.count', count)]
    blocks.extend((f'mesh.text_list.{index}', property_text(profile + index)) for index in range(count))
    blocks.append(property_count('mesh.settings.references.count', count))
    for index in range(count):
        for label, raw in thermal_reference_blocks(version):
            if profile == 4 and index == 1 and label in ('archive0', 'archive1'): raw = raw[:4] + b'\0\0'
            blocks.append((f'mesh.settings.reference{index}.{label}', raw))
    blocks.append(('mesh.settings.references.tail', property_words(2, profile)))
    if version >= 800040: blocks.append(('mesh.word738', property_words(1, profile)))
    return blocks


def direct_property_extension(version: int, profile: int, record: int, extension: str) -> PropertyBlocks:
    """Exercise disabled arbitrary modes, enabled-empty and populated continuations; e.g. direct_property_extension(850030, 3, 0, 'extension38')."""
    mode = (0, 2, 1, (1, -2147483648, 1)[record % 3], 1)[profile]
    count = 0 if profile == 3 and record == 0 and extension == 'extension2c' else (1, 1, 2, 2, 1)[profile]
    prefix = f'mesh.record{record}.{extension}'
    blocks = [property_count(prefix + '.mode', mode)]
    if mode != 1: return blocks
    blocks.append(property_count(prefix + '.count', count))
    for index in range(count):
        counts = property_nested_counts(profile, record, index, extension)
        raw, _ = assemble_property_blocks(nested_property_blocks(version, profile + record + index, counts))
        blocks.append((f'{prefix}.nested{index}', raw))
    return blocks


def property_nested_counts(profile: int, record: int, index: int, extension: str) -> tuple[int, int]:
    """Select all independent empty/populated array combinations without consulting source bytes; e.g. property_nested_counts(2, 0, 1, 'extension38')."""
    choices = ((0, 0), (2, 0), (0, 3), (2, 3))
    return choices[(profile + record + index + (extension == 'extension38')) % len(choices)]


def direct_property_blocks(version: int, profile: int) -> PropertyBlocks:
    """Keep every direct record contiguous and both extension gates independent; e.g. direct_property_blocks(870018, 2)."""
    count = (0, 1, 2, 3, 1)[profile]
    blocks = [property_count('mesh.direct_records.count', count)]
    for index in range(count):
        prefix = f'mesh.record{index}'
        blocks.append((prefix + '.five_words', property_words(5, profile + index)))
        if version >= 870018: blocks.append((prefix + '.words14_18', property_words(2, index)))
        blocks.append((prefix + '.four_words1c_28', property_words(4, index + 1)))
        for gate, extension in ((850002, 'extension2c'), (850030, 'extension38')):
            if version >= gate: blocks.extend(direct_property_extension(version, profile, index, extension))
    return blocks


def property_array_objects(profile: int) -> PropertyBlocks:
    """Preserve two independently empty/populated word arrays in each object; e.g. property_array_objects(3)."""
    count = (0, 1, 2, 3, 2)[profile]
    blocks = [property_count('mesh.array_objects.count', count)]
    shapes = ((0, 0), (2, 0), (0, 3), (2, 3))
    for index in range(count):
        prefix = f'mesh.array_object{index}'
        first, second = shapes[(index + profile) % len(shapes)]
        blocks.extend([(prefix + '.five_words', property_words(5, profile + index)),
                       property_count(prefix + '.count0', first), property_count(prefix + '.count1', second),
                       (prefix + '.array0', property_words(first, index)), (prefix + '.array1', property_words(second, index + 1))])
    return blocks


def property_keyed_objects(profile: int) -> PropertyBlocks:
    """Keep the older wider keyed region distinct from newest set keys; e.g. property_keyed_objects(2)."""
    count = (0, 1, 2, 3, 2)[profile]
    blocks = [property_count('mesh.map.count', count)]
    for index in range(count):
        prefix = f'mesh.map{index}'
        children = (index + profile) % 3
        header = property_words(4, index) + property_raw64(3, profile) + property_words(1, profile)
        blocks.extend([(prefix + '.key_three_words_three_doubles_word', header),
                       property_count(prefix + '.child_count', children),
                       (prefix + '.children', property_words(10 * children, profile + index))])
    return blocks


def newest_property_keys(profile: int) -> list[int]:
    """Preserve negative values and duplicates in original wire order; e.g. newest_property_keys(2)."""
    return ([], [-7], [-7, 3, -7, -2147483648, 2147483647], [1, 1, -1], [0, -2147483648, 0])[profile]


def property_late_blocks(version: int, profile: int) -> PropertyBlocks:
    """Encode all late count and scalar gates without a guessed remainder; e.g. property_late_blocks(891103, 2)."""
    blocks: PropertyBlocks = []
    if version >= 890104: blocks.append(('mesh.temporary_word', property_words(1, profile)))
    if version >= 890111: blocks.append(('mesh.word2ac', property_words(1, profile + 1)))
    if version >= 890512: blocks.append(('mesh.words89c_8a0', property_words(2, profile)))
    if version >= 890703: blocks.extend(property_keyed_objects(profile))
    if version >= 890907:
        count = (0, 1, 2, 3, 2)[profile]
        blocks.extend([property_count('mesh.double_array.count', count), ('mesh.double_array', property_raw64(count, profile))])
    if version >= 890909: blocks.append(('mesh.word73c', property_words(1, profile + 2)))
    if version >= 891103:
        keys = newest_property_keys(profile)
        blocks.extend([property_count('mesh.newest_set.count', len(keys)), ('mesh.newest_set.keys', struct.pack('<' + 'i' * len(keys), *keys))])
    return blocks
