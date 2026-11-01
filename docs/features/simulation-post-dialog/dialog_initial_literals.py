"""Independent complete initial dialog and fixed-slot PostInfo literals (#166)."""
import struct
from graph_literals import graph_words
from dialog_literal_values import (DialogBlocks, DialogLiteralProfile, FLOAT32_PATTERNS,
    dialog_raw64, dialog_text)


def postinfo_block(profile: DialogLiteralProfile) -> bytes:
    """Keep all twelve physical slots regardless of active-count word values; e.g. postinfo_block(profile)."""
    raw = bytearray(graph_words(7, profile.rotation))
    for index in range(12):
        kind = (index + profile.rotation) % 5
        direction = (-1, 1, 2, -2147483648, 2147483647)[(index + profile.rotation) % 5]
        raw.extend(struct.pack('<ii', kind, direction))
        raw.extend(b''.join(struct.pack('<I', FLOAT32_PATTERNS[(index + component + profile.rotation) % 5]) for component in range(7)))
    assert len(raw) == 460, len(raw)
    return bytes(raw)


def postinfo_blocks(version: int, profile: DialogLiteralProfile) -> DialogBlocks:
    """Add the standalone suffix only at exact version700002; e.g. postinfo_blocks(700002, profile)."""
    blocks = [('post_info.raw_block', postinfo_block(profile))]
    if version != 700002: return blocks
    return blocks + [('legacy.word', graph_words(1, profile.rotation))] + dialog_text('legacy', profile)


def initial_dialog_blocks(profile: DialogLiteralProfile) -> DialogBlocks:
    """Encode the independently specified 296-byte empty-text initial prefix; e.g. initial_dialog_blocks(profile)."""
    first = bytearray(graph_words(14, profile.rotation))
    struct.pack_into('<i', first, 4, 65 if profile.equation else 0)
    blocks = [('fourteen_words0c_28_30_3c_518_51c', bytes(first)), ('temporary_state_word', graph_words(1, profile.rotation)),
              ('eight_words528_544', graph_words(8, profile.rotation)), dialog_raw64('four_raw64_548_560', 4, profile),
              ('three_words568_570', graph_words(3, profile.rotation))]
    blocks.extend(dialog_text('text578', profile))
    blocks.extend(dialog_text('text580', profile, 1))
    blocks.append(('word588', graph_words(1, profile.rotation)))
    blocks.append(dialog_raw64('sixteen_raw64_590_608', 16, profile))
    for index, label in enumerate(('text610', 'text618', 'text620', 'text628', 'text630')):
        blocks.extend(dialog_text(label, profile, index + 2))
    return blocks
