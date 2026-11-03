"""Complete original-reader component controls before the full visual edit matrix (#181)."""
from pathlib import Path
from typing import cast
from visual_component_seeds import COMPONENT_VISUAL_SEED_STEMS, ComponentVisualSeed, load_component_visual_seeds
from visual_component_source import (CapturedComponentSeed, capture_component_visual_seed,
    component_seed_properties, component_seed_visual_spans, verify_component_seed_aliases)
from visual_owned_words import VisualStoredWords, visual_word_bytes, verify_visual_scalar_projection

COMPONENT_BASELINE_WORDS = VisualStoredWords(0xffff, 0x8000, 254, 0xfedcba98, 0xffff, -32768, 0x7f800042)


def verify_component_visual_projection(seed: ComponentVisualSeed, captured: CapturedComponentSeed, words: VisualStoredWords = COMPONENT_BASELINE_WORDS) -> dict[str, object]:
    """Check every physical baseline visual and its unchanged all-views neighbor; e.g. verify_component_visual_projection(seed, captured)."""
    verify_component_seed_aliases(seed, captured)
    spans, properties = component_seed_visual_spans(captured), component_seed_properties(captured)
    expected = 0 if seed.scenario == 17 else 1
    assert len(spans) == len(properties) == expected, f'{len(spans)}/{len(properties)}; expected{expected} physical component visuals for scenario{seed.scenario}'
    for offset, prop in zip(spans, properties, strict=True):
        assert seed.raw[offset:offset + 17] == visual_word_bytes(words), f'{offset}; expected the complete17-byte baseline visual literals'
        verify_visual_scalar_projection(cast(dict[str, object], prop['ent_vis_prop']), words)
        all_views = 23 if seed.scenario % 16 == 2 else 255
        assert prop['all_views'] == all_views, f'{prop["all_views"]}; expected unchanged all_views byte{all_views}'
    return dict(kind=seed.kind, scenario=seed.scenario, bytes=len(seed.raw),
        primitive_reads=len(captured.physical), focus=spans, slots=len(captured.archive.map))


def verify_component_visual_baselines(root: Path) -> list[dict[str, object]]:
    """Admit both real document kinds with original readers and exact whole-stream coverage; e.g. verify_component_visual_baselines(root)."""
    rows: list[dict[str, object]] = []
    for stem in COMPONENT_VISUAL_SEED_STEMS:
        for seed in load_component_visual_seeds(root, stem):
            if seed.version != 19000:
                continue
            captured = capture_component_visual_seed(root, seed)
            rows.append(dict(stem=stem, **verify_component_visual_projection(seed, captured)))
    assert len(rows) == 16, f'{len(rows)}; expected all16 modern part/assembly baseline and null controls'
    physical = sum(len(cast(list[int], row['focus'])) for row in rows)
    assert physical == 14, f'{physical}; expected14 physical component visual definitions and two null controls'
    return rows
