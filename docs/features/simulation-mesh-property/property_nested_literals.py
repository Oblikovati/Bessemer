"""Independent populated direct Mesh nested bodies, with no tags or memory padding (#163)."""
from property_literal_values import PropertyBlocks, property_words, property_raw64, property_count


def nested_property_blocks(version: int, rotation: int, counts: tuple[int, int]) -> PropertyBlocks:
    """Build the 268-byte prefix and every versioned continuation in physical order; e.g. nested_property_blocks(850035, 1, (2, 3))."""
    blocks = [('word00', property_words(1, rotation)), ('fixed_eight_byte_fields', property_raw64(33, rotation))]
    if version >= 850029:
        for index, count in enumerate(counts): blocks.extend(nested_property_array(index, count, rotation + index))
    if version >= 850035: blocks.append(('eleven_raw64_tail', property_raw64(11, rotation + 2)))
    if version >= 890502: blocks.append(('two_raw64_tail', property_raw64(2, rotation + 3)))
    return blocks


def nested_property_array(index: int, count: int, rotation: int) -> PropertyBlocks:
    """Keep the two raw64 arrays independently counted, including empty sides; e.g. nested_property_array(1, 3, 2)."""
    prefix = f'array{index}'
    return [(prefix + '.nine_raw64', property_raw64(9, rotation)),
            (prefix + '.word', property_words(1, rotation)), property_count(prefix + '.count', count),
            (prefix + '.raw64_values', property_raw64(count, rotation + 1))]
