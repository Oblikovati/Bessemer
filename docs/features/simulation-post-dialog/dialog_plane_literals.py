"""Independent complete modern dialog plane and reference prefix (#166)."""
from dialog_literal_values import DialogBlocks, DialogLiteralProfile, dialog_words, dialog_text
from dialog_reference_literals import dialog_reference_container

SECTION_WORDS = (8, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c, 0x34, 0x30, 0x38, 0x44, 0x3c, 0x40, 0x48)


def dialog_plane_blocks(version: int, profile: DialogLiteralProfile) -> DialogBlocks:
    """Keep the historical parent route free of modern plane bytes; e.g. dialog_plane_blocks(800004, profile)."""
    if version < 800004: return []
    blocks = dialog_words('dialog', (0xad0,), profile.rotation)
    for index in range(6): blocks.extend(dialog_words(f'iso[{index}]', (0x10, 8, 0xc, 0x14), profile.rotation + index))
    for index in range(6): blocks.extend(dialog_section_blocks(index, version, profile))
    for threshold, offset in ((800006, 0xad4), (800007, 0xad8), (800024, 0xae4), (800025, 0xae8), (800045, 0xaf0), (800075, 0xb00)):
        if version > threshold: blocks.extend(dialog_words('dialog', (offset,), profile.rotation))
    if version > 810008: blocks.extend(dialog_reference_container('dialog.reference_b30', version, profile))
    return blocks


def dialog_section_blocks(index: int, version: int, profile: DialogLiteralProfile) -> DialogBlocks:
    """Preserve seventeen words, atomic name framing and separately gated references; e.g. dialog_section_blocks(0, 810013, profile)."""
    label = f'section[{index}]'
    blocks = dialog_words(label, SECTION_WORDS, profile.rotation + index)
    blocks.extend(dialog_text(label + '.name', profile, index))
    if version > 810012: blocks.extend(dialog_reference_container(label + '.reference', version, profile))
    return blocks
