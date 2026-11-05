"""Hash-bound complete component-property display roots reused as owned seeds (#181)."""
from pathlib import Path
from typing import Literal, NamedTuple, cast
import hashlib
import json
import struct

COMPONENT_VISUAL_SEED_STEMS = ('component-properties', 'component-property-holders-part', 'component-property-holders-assembly')


class ComponentVisualSeed(NamedTuple):
    version: int
    kind: Literal['part', 'assembly']
    scenario: int
    raw: bytes


def skip_component_seed_oracles(packet: bytes, position: int) -> int:
    """Keep both persisted oracle sections outside the actual display stream; e.g. skip_component_seed_oracles(packet, end)."""
    for _ in range(2):
        assert position + 4 <= len(packet), f'{position}/{len(packet)}; expected another four-byte oracle extent'
        size = struct.unpack_from('<I', packet, position)[0]
        position += 4 + size
        assert position <= len(packet), f'{position}/{len(packet)}; expected the complete bounded oracle section'
    return position


def parse_component_visual_seed(packet: bytes, position: int, row: dict[str, object]) -> tuple[ComponentVisualSeed, int]:
    """Check framing, kind and complete stream hash against an owned baseline; e.g. parse_component_visual_seed(packet, 4, row)."""
    assert position + 10 <= len(packet), f'{position}/{len(packet)}; expected the complete ten-byte case header'
    version, kind, scenario, size = struct.unpack_from('<IBBI', packet, position)
    start, end = position + 10, position + 10 + size
    assert kind in (0, 1), f'{kind}; expected part0 or assembly1'
    name: Literal['part', 'assembly'] = 'part' if kind == 0 else 'assembly'
    actual = (version, name, scenario, size)
    expected = (row['version'], row['kind'], row['scenario'], row['bytes'])
    assert actual == expected, f'{actual}; expected complete owned metadata header {expected}'
    assert end <= len(packet), f'{end}/{len(packet)}; expected the complete declared display stream'
    raw = packet[start:end]
    digest = hashlib.sha256(raw).hexdigest()
    assert digest == row['sha256'], f'{digest}; expected owned stream hash {row["sha256"]}'
    return ComponentVisualSeed(version, name, scenario, raw), skip_component_seed_oracles(packet, end)


def load_component_visual_seeds(root: Path, stem: str) -> list[ComponentVisualSeed]:
    """Load complete part/assembly roots without invoking historical patching generators; e.g. load_component_visual_seeds(root, 'component-properties')."""
    assert stem in COMPONENT_VISUAL_SEED_STEMS, f'{stem}; expected one of {COMPONENT_VISUAL_SEED_STEMS}'
    packet = (root / 'tests/fixtures/display-lists' / (stem + '.bin')).read_bytes()
    manifest = cast(dict[str, object], json.loads((root / 'docs/features/display-lists' / (stem + '.json')).read_bytes()))
    digest = hashlib.sha256(packet).hexdigest()
    assert digest == manifest['sha256'], f'{digest}; expected complete packet hash {manifest["sha256"]}'
    rows = cast(list[dict[str, object]], manifest['fixtures'])
    assert len(packet) >= 4 and struct.unpack_from('<I', packet)[0] == len(rows), f'{len(packet)}/{len(rows)}; expected count header matching all metadata cases'
    seeds: list[ComponentVisualSeed] = []
    position = 4
    for row in rows:
        seed, position = parse_component_visual_seed(packet, position, row)
        seeds.append(seed)
    assert position == len(packet), f'{position}/{len(packet)}; expected all complete cases and no trailing bytes'
    return seeds
