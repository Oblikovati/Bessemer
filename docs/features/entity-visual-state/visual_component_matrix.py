"""Complete part/assembly component visual cases with factored unchanged root oracles (#181)."""
from pathlib import Path
from typing import NamedTuple
import hashlib
from primitive_inventory import PrimitiveRead
from visual_component_seeds import ComponentVisualSeed, load_component_visual_seeds
from visual_component_source import CapturedComponentSeed, capture_component_visual_seed, component_seed_visual_spans
from visual_component_baselines import COMPONENT_BASELINE_WORDS, verify_component_visual_projection
from visual_owned_words import VisualStoredWords, visual_word_bytes


class ComponentVisualTemplate(NamedTuple):
    seed: ComponentVisualSeed
    captured: CapturedComponentSeed
    offset: int


def component_visual_templates(root: Path) -> list[ComponentVisualTemplate]:
    """Use complete shared-property/cyclic-holder roots for both document kinds; e.g. component_visual_templates(root)."""
    templates: list[ComponentVisualTemplate] = []
    for kind in ('part', 'assembly'):
        seeds = load_component_visual_seeds(root, 'component-property-holders-' + kind)
        matches = [seed for seed in seeds if seed.version == 19000 and seed.scenario == 21]
        assert len(matches) == 1, f'{len(matches)}; expected one complete modern nested-holder {kind} seed'
        seed = matches[0]
        captured = capture_component_visual_seed(root, seed)
        verify_component_visual_projection(seed, captured)
        spans = component_seed_visual_spans(captured)
        assert len(spans) == 1, f'{spans}; expected one shared physical visual definition'
        templates.append(ComponentVisualTemplate(seed, captured, spans[0]))
    return templates


def replace_component_visual_words(template: ComponentVisualTemplate, words: VisualStoredWords) -> ComponentVisualSeed:
    """Change only the independently bounded visual bytes in a complete owner; e.g. replace_component_visual_words(template, words)."""
    start, end = template.offset, template.offset + 17
    original = template.seed.raw
    assert original[start:end] == visual_word_bytes(COMPONENT_BASELINE_WORDS), f'{start}..{end}; expected complete original visual literals'
    raw = original[:start] + visual_word_bytes(words) + original[end:]
    assert len(raw) == len(original), f'{len(raw)}; expected unchanged complete root extent {len(original)}'
    return template.seed._replace(raw=raw)


def component_read_shape(read: PrimitiveRead) -> tuple[int, int, str, str, str, int]:
    """Keep every original read boundary, type and source attribution load-bearing; e.g. component_read_shape(read)."""
    return read.start, read.end, read.kind, read.source_file, read.source_function, read.source_line


def verify_component_read_sequence(template: ComponentVisualTemplate, captured: CapturedComponentSeed) -> None:
    """Check every fresh read and preserve all unselected fields without repeating their stored oracles; e.g. verify_component_read_sequence(template, captured)."""
    before, after = template.captured.physical, captured.physical
    assert len(before) == len(after), f'{len(after)}; expected all{len(before)} original primitive reads'
    for original, current in zip(before, after, strict=True):
        assert component_read_shape(current) == component_read_shape(original), f'{component_read_shape(current)}; expected original read shape {component_read_shape(original)}'
        if current.end <= template.offset or current.start >= template.offset + 17:
            assert current.raw_hex == original.raw_hex, f'{current.start}/{current.raw_hex}; expected unchanged original bytes {original.raw_hex}'


def verify_component_word_case(root: Path, template: ComponentVisualTemplate, family: str, words: VisualStoredWords) -> str:
    """Reparse the entire owner and verify every primitive and alias for each word case; e.g. verify_component_word_case(root, template, 'rgb', words)."""
    seed = replace_component_visual_words(template, words)
    captured = capture_component_visual_seed(root, seed)
    verify_component_visual_projection(seed, captured, words)
    verify_component_read_sequence(template, captured)
    assert len(captured.archive.map) == len(template.captured.archive.map), f'{len(captured.archive.map)}; expected unchanged {len(template.captured.archive.map)} map slots'
    digest = hashlib.sha256(seed.raw).hexdigest()
    return f'{seed.kind}\t{family}\t' + '\t'.join(map(str, words)) + f'\t{template.offset}\t{len(seed.raw)}\t{len(captured.physical)}\t{digest}\n'
