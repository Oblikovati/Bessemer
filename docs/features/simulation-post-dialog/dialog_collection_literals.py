"""Independent populated sensor, integer-list, string-list and equation bodies (#166)."""
from graph_literals import graph_words, graph_raw64
from dialog_literal_values import DialogBlocks, DialogLiteralProfile, dialog_count, dialog_text, dialog_words


def dialog_record_count(profile: DialogLiteralProfile) -> int:
    """Choose real empty, one-record or duplicate-record controls; e.g. dialog_record_count(profile)."""
    return (2 if profile.duplicate_records else 1) if profile.populated else 0


def dialog_sensor_blocks(profile: DialogLiteralProfile) -> DialogBlocks:
    """Keep names and all eleven point words for each declared record; e.g. dialog_sensor_blocks(profile)."""
    count = dialog_record_count(profile)
    blocks = [dialog_count('sensors.count', count)]
    for index in range(count): blocks.extend(dialog_sensor_record(index, profile))
    return blocks


def dialog_sensor_record(index: int, profile: DialogLiteralProfile) -> DialogBlocks:
    """Vary both sensor and point list cardinalities independently of framing; e.g. dialog_sensor_record(0, profile)."""
    label = f'sensor[{index}]'
    count = 2 if profile.duplicate_records else 1
    blocks = dialog_text(label + '.name', profile, index)
    blocks.append(dialog_count(label + '.points.count', count))
    return blocks + [(f'{label}.point[{point}].eleven_words', graph_words(11, profile.rotation + point)) for point in range(count)]


def dialog_integer_blocks(profile: DialogLiteralProfile) -> DialogBlocks:
    """Retain the stored DA4 count and zero-width empty array span; e.g. dialog_integer_blocks(profile)."""
    count = dialog_record_count(profile)
    return [dialog_count('dialog.da4.count', count), ('dialog.da8_integer_array', graph_words(count, profile.rotation))]


def dialog_string_list(version: int, profile: DialogLiteralProfile) -> DialogBlocks:
    """Keep the 890611 byte-list/character-text transition explicit; e.g. dialog_string_list(890611, profile)."""
    count = dialog_record_count(profile)
    blocks = [dialog_count('dialog.string_list_dd0.count', count)]
    for index in range(count):
        label = f'dialog.string_list[{index}]'
        if version >= 890611:
            blocks.extend(dialog_text(label, profile, index))
            continue
        raw = bytes((0xff, index, 0)) if not profile.empty_texts else b''
        blocks.extend((dialog_count(label + '.byte_count', len(raw)), (label + '.raw_bytes', raw)))
    return blocks


def dialog_equation_blocks(version: int, profile: DialogLiteralProfile) -> DialogBlocks:
    """Use the actual initial word65 condition instead of inferring a kind from version; e.g. dialog_equation_blocks(890109, profile)."""
    if version < 890109 or not profile.equation: return []
    blocks = dialog_text('equation.text8', profile)
    blocks.extend(dialog_words('equation', (0x10,), profile.rotation))
    blocks.extend(dialog_text('equation.text18', profile, 1))
    blocks.extend(dialog_text('equation.text20', profile, 2))
    count = dialog_record_count(profile)
    blocks.append(dialog_count('equation.variables.count', count))
    for index in range(count):
        blocks.extend(dialog_text(f'equation.variable[{index}].text8', profile, index))
        raw = graph_words(2, profile.rotation + index) + graph_raw64(1, profile.rotation)
        blocks.append((f'equation.variable[{index}].two_words_raw64', raw))
    return blocks
