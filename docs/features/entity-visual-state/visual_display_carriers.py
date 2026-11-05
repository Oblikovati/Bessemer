"""Complete revolution owners with nested display-dimension visual records (#181)."""
from typing import cast
import struct
from visual_owned_words import VisualStoredWords, visual_word_bytes, contrasting_visual_words, verify_visual_scalar_projection


def owned_display_dimension(words: VisualStoredWords) -> tuple[bytes, int]:
    """Retain the complete reviewed dimension around its literal visual record; e.g. owned_display_dimension(VisualStoredWords())."""
    from display_dimension_fixture_bytes import tagged
    original = tagged(19000, 'moDisplayDim_c', False)
    start = 6 + len(b'moDisplayDim_c') + 16
    raw = original[:start] + visual_word_bytes(words) + original[start + 17:]
    assert len(raw) == len(original), f'{len(raw)}; expected complete unchanged dimension extent {len(original)}'
    return raw, start


def owned_visual_revolution(words: VisualStoredWords) -> tuple[bytes, int]:
    """Enter the display reader through its actual complete revolution owner; e.g. owned_visual_revolution(VisualStoredWords())."""
    from body_feature_fixture_bytes import header, atom_fields, body_suffix, suffix
    from sketch_tail_fixture_bytes import WireMap
    wire = WireMap()
    prefix = header(wire, 19000, False) + atom_fields(wire, False, 98) + body_suffix(False)
    tail = suffix(wire, 19000, 'moRevolution_c', False)
    assert tail[:2] == bytes(2), f'{tail[:2].hex()}; expected original null dimension_800'
    display, offset = owned_display_dimension(words)
    return prefix + display + tail[2:], len(prefix) + offset


def owned_visual_display_stream(words: VisualStoredWords) -> tuple[bytes, list[int]]:
    """Keep new/loaded owner definitions and an outer alias around nested visuals; e.g. owned_visual_display_stream(VisualStoredWords())."""
    name = b'moRevolution_c'
    tag = struct.pack('<3H', 65535, 1, len(name)) + name
    header = struct.pack('<iH', -71, 3)
    first, first_offset = owned_visual_revolution(words)
    second, second_offset = owned_visual_revolution(contrasting_visual_words(words))
    base = len(header) + len(tag)
    starts = [base + first_offset, base + len(first) + 2 + second_offset]
    return header + tag + first + struct.pack('<H', 0x8061) + second + struct.pack('<HI', 98, 0), starts


def verify_visual_display_items(parsed: dict[str, object], words: VisualStoredWords) -> None:
    """Verify the nested caller, preserved null neighbor and real owner alias; e.g. verify_visual_display_items(parsed, words)."""
    items = cast(list[dict[str, object]], parsed['items'])
    assert len(items) == 3 and items[0] is items[2] and items[0] is not items[1], f'{len(items)}; expected two definitions and a first-owner alias'
    slots = len(cast(list[object], getattr(parsed['archive'], 'map')))
    assert slots == 108, f'{slots}; expected97 inherited slots, one owner class, two owners and four nested class/object pairs'
    for item, expected in zip(items[:2], (words, contrasting_visual_words(words)), strict=True):
        assert item['_class'] == 'moRevolution_c', f'{item["_class"]}; expected the complete revolution owner'
        model = cast(dict[str, object], item['model_feature'])
        feature = cast(dict[str, object], model['feature'])
        original = VisualStoredWords(65530, 65529, 255, 0x89abcdef, 65528, -123, 0x3fa00000)
        verify_visual_scalar_projection(cast(dict[str, object], feature['entity_visual_properties']), original)
        suffix = cast(dict[str, object], item['revolution_suffix'])
        assert suffix['dimension_808'] is None, f'{suffix["dimension_808"]}; expected the unchanged null neighboring dimension'
        dimension = cast(dict[str, object], suffix['dimension_800'])
        assert dimension['_class'] == 'moDisplayDim_c', f'{dimension["_class"]}; expected complete nested display dimension'
        verify_visual_scalar_projection(cast(dict[str, object], dimension['visual_properties']), expected)


def verify_owned_visual_display_carriers() -> None:
    """Prove every visual matrix case through complete revolution/display owners; e.g. verify_owned_visual_display_carriers()."""
    from visual_owned_corpus import verify_visual_caller_corpus
    verify_visual_caller_corpus('display', 'complete display dimensions nested in revolution owners in resolved-feature carriers',
        owned_visual_display_stream, verify_visual_display_items, final_slots=108, other_visual_records=2)
