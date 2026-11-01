"""Independent complete dialog tail gates and interleaved mesh-quality fields (#166)."""
from graph_literals import graph_words, graph_raw64
from dialog_literal_values import DialogBlocks, DialogLiteralProfile, dialog_words, dialog_text, dialog_raw64
from dialog_reference_literals import dialog_reference_container
from dialog_collection_literals import dialog_sensor_blocks, dialog_integer_blocks, dialog_string_list, dialog_equation_blocks

EARLY_WORD_GATES = ((830010, (0xb10,)), (830012, (0xc44,)), (830017, (0xcd0, 0xcd4, 0xcdc)),
    (830021, (0xd04,)), (830022, (0xaec,)), (830024, (0xce0,)), (830026, (0x2c,)),
    (830030, (0xd08, 0xd0c)), (850003, (0xcfc, 0xd00)))
LATE_FIELD_GATES = ((880002, 0xeb8, 4), (880003, 0xebc, 4), (880005, 0xec0, 4),
    (880017, 0xec8, 8), (880018, 0xed0, 4), (890004, 0xef4, 4))


def dialog_modern_tail(version: int, profile: DialogLiteralProfile) -> DialogBlocks:
    """Keep the entire modern continuation absent below800004; e.g. dialog_modern_tail(890620, profile)."""
    if version < 800004: return []
    blocks: DialogBlocks = []
    if version >= 810015: blocks.extend(dialog_text('dialog.text_b08', profile))
    if version >= 810020: blocks.append(('dialog.raw64_bc8_bd0_words_bd8_bdc', graph_raw64(2, profile.rotation) + graph_words(2, profile.rotation)))
    if version >= 820008: blocks.extend(dialog_words('dialog', (0xbe4,), profile.rotation))
    if version >= 820021: blocks.extend(dialog_words('dialog', (0xbec,), profile.rotation))
    if version >= 820026:
        blocks.extend(dialog_words('dialog', (0xbf0,), profile.rotation))
        blocks.extend(dialog_reference_container('dialog.reference_bf8', version, profile))
    if version >= 830008: blocks.extend(dialog_words('moving_objects', (0x48, 0x50), profile.rotation))
    for gate, offsets in EARLY_WORD_GATES:
        if version >= gate: blocks.extend(dialog_words('dialog', offsets, profile.rotation))
    return blocks + dialog_mid_tail(version, profile)


def dialog_mid_tail(version: int, profile: DialogLiteralProfile) -> DialogBlocks:
    """Encode the populated lists between the early scalar and late fixed-field gates; e.g. dialog_mid_tail(890611, profile)."""
    blocks: DialogBlocks = []
    if version >= 850010:
        blocks.extend(dialog_words('dialog', (0xd38,), profile.rotation))
        blocks.append(dialog_raw64('dialog.raw64_d40', 1, profile))
    if version >= 850017:
        blocks.extend(dialog_words('dialog', (0xd48, 0xd4c, 0xd50, 0xd54, 0xd58, 0xd5c, 0xd60, 0xd64, 0xd68), profile.rotation))
        blocks.extend(dialog_text('dialog.text_d78', profile))
    if version >= 850021: blocks.extend(dialog_words('dialog', (0xd6c, 0xd70, 0xd74), profile.rotation))
    if version >= 850022: blocks.extend(dialog_sensor_blocks(profile))
    if version >= 850029: blocks.extend(dialog_words('dialog', (0x230,), profile.rotation))
    if version >= 860007: blocks.extend(dialog_integer_blocks(profile))
    if version >= 870007:
        blocks.extend(dialog_words('dialog', (0xbc0,), profile.rotation))
        blocks.extend(dialog_reference_container('dialog.reference_b78', version, profile))
    if version >= 870009: blocks.extend(dialog_string_list(version, profile))
    if version >= 870015: blocks.extend(dialog_words('dialog', (0x234,), profile.rotation))
    return blocks + dialog_fixed_late(version, profile)


def dialog_fixed_late(version: int, profile: DialogLiteralProfile) -> DialogBlocks:
    """Preserve each declared four/eight-byte late field width; e.g. dialog_fixed_late(890004, profile)."""
    return [(f'dialog.field_{offset:x}', graph_words(1, profile.rotation) if width == 4 else graph_raw64(1, profile.rotation))
            for gate, offset, width in LATE_FIELD_GATES if version >= gate]


def dialog_common_final(version: int, profile: DialogLiteralProfile) -> DialogBlocks:
    """Keep equation and monotonically nested common-final gates separate from modern-only fields; e.g. dialog_common_final(890620, profile)."""
    blocks: DialogBlocks = []
    for gate, offsets in ((890008, (0xef8,)), (890014, (0xf04,)), (890107, (0xae0, 0xadc))):
        if version >= gate: blocks.extend(dialog_words('dialog', offsets, profile.rotation))
    blocks.extend(dialog_equation_blocks(version, profile))
    if version >= 890118: blocks.extend(dialog_words('dialog', (0xf90,), profile.rotation))
    if version >= 890206: blocks.extend(dialog_words('dialog', (0xf94,), profile.rotation))
    if version >= 890212:
        blocks.extend(dialog_words('dialog', (0xf98, 0xf9c), profile.rotation))
        blocks.extend(dialog_text('dialog.text_fa8', profile))
        blocks.extend(dialog_text('dialog.text_fa0', profile, 1))
    if version >= 890307: blocks.extend(dialog_words('dialog', (0xfb0,), profile.rotation))
    if version >= 890311:
        blocks.extend(dialog_words('dialog', (0xfd0, 0xfd4, 0xfb4), profile.rotation))
        blocks.append(dialog_raw64('dialog.raw64_fc0', 1, profile))
        blocks.append(('dialog.color_fd8', graph_words(4, profile.rotation)))
    return blocks + dialog_mesh_quality(version, profile)


def dialog_mesh_quality(version: int, profile: DialogLiteralProfile) -> DialogBlocks:
    """Keep the twelve raw64 pairs interleaved and the parent1168 word separate; e.g. dialog_mesh_quality(890513, profile)."""
    blocks: DialogBlocks = []
    if version >= 890319: blocks.extend(dialog_words('dialog', (0xfe8,), profile.rotation))
    if version >= 890504: blocks.extend(dialog_words('dialog', (0xfec,), profile.rotation))
    if version >= 890507:
        blocks.extend(dialog_words('mesh_quality', (0xffc, 0x1000, 0x1004, 0x1008, 0x1018, 0x1118, 0x111c, 0x1120, 0x1124, 0x1128), profile.rotation))
        blocks.extend((f'mesh_quality.raw64_pair[{index}]', graph_raw64(2, profile.rotation + index)) for index in range(12))
        blocks.extend((f'mesh_quality.word_array[{index}]', graph_words(1, profile.rotation + index)) for index in range(12))
    if version >= 890508: blocks.append(dialog_raw64('mesh_quality.raw64_1110', 1, profile))
    if version >= 890510: blocks.extend(dialog_words('mesh_quality', (0x112c,), profile.rotation))
    if version >= 890511: blocks.extend(dialog_words('mesh_quality', (0x1160,), profile.rotation))
    if version >= 890513: blocks.extend(dialog_words('dialog', (0x1168,), profile.rotation))
    return blocks
