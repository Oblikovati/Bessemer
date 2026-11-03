"""Complete owned sgSketch owners with by-value visual point records (#181)."""
from typing import cast
import struct
from visual_owned_words import VisualStoredWords, visual_word_bytes, contrasting_visual_words


def owned_visual_point(words: VisualStoredWords) -> bytes:
    """Retain the reviewed complete point body around independent visual literals; e.g. owned_visual_point(VisualStoredWords())."""
    from sketch_geometry_fixture_bytes import point
    original = point(0x8010, False, False)
    result = original[:8] + visual_word_bytes(words) + original[25:]
    assert len(result) == len(original), f'{len(result)}; expected unchanged complete point extent {len(original)}'
    return result


def owned_visual_sketch(words: VisualStoredWords) -> bytes:
    """Embed the direct point in its complete ordinary sketch owner; e.g. owned_visual_sketch(VisualStoredWords())."""
    from sketch_tail_fixture_bytes import WireMap, ordinary
    original = ordinary(WireMap(), 19000, False)
    assert original[:4] == bytes(4), f'{original[:4].hex()}; expected an empty original point collection'
    return struct.pack('<iH', 1, 1) + owned_visual_point(words) + original[4:]


def owned_visual_sketch_stream(words: VisualStoredWords) -> tuple[bytes, list[int]]:
    """Keep new/loaded sketch identities and a real instance alias; e.g. owned_visual_sketch_stream(VisualStoredWords())."""
    name = b'sgSketch'
    tag = struct.pack('<3H', 65535, 1, len(name)) + name
    header = struct.pack('<iH', -71, 3)
    first = owned_visual_sketch(words)
    second = owned_visual_sketch(contrasting_visual_words(words))
    base = len(header) + len(tag)
    starts = [base + 14, base + len(first) + 2 + 14]
    return header + tag + first + struct.pack('<H', 0x8061) + second + struct.pack('<HI', 98, 0), starts


def verify_visual_sketch_items(parsed: dict[str, object], words: VisualStoredWords) -> None:
    """Check owner reachability, direct point fields and alias identity; e.g. verify_visual_sketch_items(parsed, words)."""
    from visual_owned_words import verify_visual_scalar_projection
    items = cast(list[dict[str, object]], parsed['items'])
    assert len(items) == 3 and items[0] is items[2] and items[0] is not items[1], f'{len(items)}; expected two distinct definitions and a first-instance alias'
    slots = len(cast(list[object], getattr(parsed['archive'], 'map')))
    assert slots == 100, f'{slots}; expected97 inherited slots plus one class and two definitions'
    for item, expected in zip(items[:2], (words, contrasting_visual_words(words)), strict=True):
        assert item['_class'] == 'sgSketch' and item['point_count'] == 1, f'{item["_class"]}/{item["point_count"]}; expected sgSketch with one direct point'
        points = cast(list[dict[str, object]], item['points'])
        assert len(points) == 1 and points[0]['stored_category'] == 1, item['point_count']
        entity = cast(dict[str, object], points[0]['entity'])
        verify_visual_scalar_projection(cast(dict[str, object], entity['visual_properties']), expected)


def verify_owned_visual_sketch_carriers() -> None:
    """Prove complete sketch owners without a top-level point registration; e.g. verify_owned_visual_sketch_carriers()."""
    from visual_owned_corpus import verify_visual_caller_corpus
    verify_visual_caller_corpus('sketch', 'complete sgSketch owners with direct points in resolved-feature carriers',
        owned_visual_sketch_stream, verify_visual_sketch_items)
