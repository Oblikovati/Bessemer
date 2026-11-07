"""Independent complete PostStyle and embedded legend wire order (#166)."""
from dialog_literal_values import DialogBlocks, DialogLiteralProfile, dialog_words, dialog_color, dialog_text


def legend_blocks(version: int, profile: DialogLiteralProfile) -> DialogBlocks:
    """Encode every modern legend gate in documented physical order; e.g. legend_blocks(890103, profile)."""
    blocks = dialog_words('legend', (0xa8, 0xc), profile.rotation)
    for index, offset in enumerate(range(0x18, 0xa8, 16)): blocks.extend(dialog_color('legend', offset, profile.rotation + index))
    blocks.extend(dialog_words('legend', (0xac, 0xb0, 0x10, 0x14, 0xb8, 0xbc, 0xb4, 8), profile.rotation))
    if version > 810008: blocks.extend(dialog_words('legend', (0xd4,), profile.rotation))
    if version > 810018: blocks.extend(dialog_color('legend', 0xd8, profile.rotation))
    if version >= 840008: blocks.extend(dialog_words('legend', (0xc0,), profile.rotation))
    if version > 840009: blocks.extend(legend_late_blocks(version, profile))
    return blocks


def legend_late_blocks(version: int, profile: DialogLiteralProfile) -> DialogBlocks:
    """Keep the >=890103 additions inside the >840009 branch; e.g. legend_late_blocks(890103, profile)."""
    blocks = dialog_words('legend', (0xc4, 0xc8), profile.rotation)
    if version < 890103: return blocks
    blocks.extend(dialog_words('legend', (0xe8, 0xfc), profile.rotation))
    blocks.extend(dialog_color('legend', 0xec, profile.rotation))
    return blocks + dialog_color('legend', 0x100, profile.rotation + 1)


def style_blocks(version: int, profile: DialogLiteralProfile) -> DialogBlocks:
    """Keep raw historical style and modern word/color/legend/text profiles distinct; e.g. style_blocks(700006, profile)."""
    if version < 700006:
        return [('style.legacy_raw', bytes((index + profile.rotation * 17) % 256 for index in range(368)))]
    blocks = dialog_words('style', (0x238, 0x234, 0x21c, 0x240, 0x23c), profile.rotation)
    for index, offset in enumerate((0x1f4, 0x1bc, 0x220, 0x1dc)): blocks.extend(dialog_color('style', offset, profile.rotation + index))
    blocks.extend(dialog_words('style', (0x1b8, 0x1f0, 0x230), profile.rotation))
    blocks.extend(legend_blocks(version, profile))
    blocks.extend(dialog_text('style', profile))
    return blocks + style_tail_blocks(version, profile)


def style_tail_blocks(version: int, profile: DialogLiteralProfile) -> DialogBlocks:
    """Encode style suffixes without applying read-time migrations; e.g. style_tail_blocks(890702, profile)."""
    blocks: DialogBlocks = []
    if version > 700329:
        blocks.extend(dialog_words('style', (0x28c, 0x290, 0x294, 0x298), profile.rotation))
        blocks.extend(dialog_color('style', 0x29c, profile.rotation))
    if version > 820005:
        blocks.extend(dialog_color('style', 0x2ac, profile.rotation))
        blocks.extend(dialog_color('style', 0x2bc, profile.rotation + 1))
    if version >= 840008:
        blocks.extend(dialog_words('style', (0x1ec, 0x204, 0x208), profile.rotation))
        blocks.extend(dialog_color('style', 0x20c, profile.rotation))
    if version > 890701: blocks.extend(dialog_color('style', 0x1cc, profile.rotation))
    return blocks
