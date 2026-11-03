"""Complete owned folder-feature carriers and original full-byte primitive partitions (#181)."""
from typing import cast
import struct
from visual_owned_words import (VisualStoredWords, visual_word_bytes, contrasting_visual_words,
    verify_visual_scalar_projection)


def owned_visual_feature(words: VisualStoredWords) -> bytes:
    """Use the complete147-byte folder body with its literal visual span; e.g. owned_visual_feature(words)."""
    original = bytes(147)
    result = original[:76] + visual_word_bytes(words) + original[93:]
    assert len(result) == 147, f'{len(result)}; expected the complete folder feature with its17-byte visual span'
    return result


def owned_visual_stream(words: VisualStoredWords) -> tuple[bytes, list[int]]:
    """Use new/loaded definitions and a real alias against the inherited97-slot map; e.g. owned_visual_stream(words)."""
    name = b'moFolder_c'
    tag = struct.pack('<3H', 65535, 1, len(name)) + name
    header = struct.pack('<iH', -71, 3)
    first = owned_visual_feature(words)
    second = owned_visual_feature(contrasting_visual_words(words))
    starts = [len(header) + len(tag) + 76, len(header) + len(tag) + len(first) + 2 + 76]
    return header + tag + first + struct.pack('<H', 0x8061) + second + struct.pack('<HI', 98, 0), starts


def verify_visual_feature_items(parsed: dict[str, object], words: VisualStoredWords) -> None:
    """Keep physical feature definitions and alias identity distinct; e.g. verify_visual_feature_items(parsed, words)."""
    items = cast(list[dict[str, object]], parsed['items'])
    assert len(items) == 3 and items[0] is items[2] and items[0] is not items[1], parsed['end']
    assert len(cast(list[object], getattr(parsed['archive'], 'map'))) == 100, parsed['end']
    for item, expected in zip(items[:2], (words, contrasting_visual_words(words)), strict=True):
        assert item['_class'] == 'moFolder_c', item['_class']
        verify_visual_scalar_projection(cast(dict[str, object], item['entity_visual_properties']), expected)
        children = cast(dict[str, object], item['feature_list'])
        assert children['nodes'] == [] and item['unknown_358'] == item['unknown_368'] == 0, item['_class']


def verify_owned_visual_model_carriers() -> None:
    """Prove one complete caller before extending the visual operation; e.g. verify_owned_visual_model_carriers()."""
    from visual_owned_corpus import verify_visual_caller_corpus
    verify_visual_caller_corpus('model', 'complete moFolder_c features in resolved-feature carriers',
        owned_visual_stream, verify_visual_feature_items)
