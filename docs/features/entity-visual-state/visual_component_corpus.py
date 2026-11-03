"""Full component word matrices with unchanged complete templates retained once (#181)."""
from pathlib import Path
import hashlib
import json
import struct
import sys
from visual_component_matrix import ComponentVisualTemplate, component_visual_templates, verify_component_word_case
from visual_component_baselines import verify_component_visual_baselines
from visual_owned_corpus import lock_visual_fixture
from visual_owned_words import visual_rgb_cases, visual_flag_cases, visual_width_layer_cases
from visual_state_evidence import FEATURE


def component_template_packet(templates: list[ComponentVisualTemplate]) -> bytes:
    """Retain complete original root bytes and explicit context without repeating unchanged owners; e.g. component_template_packet(templates)."""
    packet = bytearray(struct.pack('<I', len(templates)))
    for template in templates:
        seed = template.seed
        packet.extend(struct.pack('<IBBII', seed.version, seed.kind == 'assembly', seed.scenario, len(seed.raw), template.offset))
        packet.extend(seed.raw)
    return bytes(packet)


def component_template_primitives(templates: list[ComponentVisualTemplate]) -> bytes:
    """Store every original read and attribution for each unchanged complete owner; e.g. component_template_primitives(templates)."""
    rows: list[str] = []
    for template in templates:
        for read in template.captured.physical:
            rows.append(f'{template.seed.kind}\t{read.start}\t{read.end}\t{read.kind}\t{read.raw_hex}\t{read.source_file}\t{read.source_function}\t{read.source_line}\n')
    return ''.join(rows).encode()


def collect_component_visual_matrix(root: Path, templates: list[ComponentVisualTemplate]) -> bytes:
    """Reparse every independent word case in both complete document kinds; e.g. collect_component_visual_matrix(root, templates)."""
    cases = visual_rgb_cases() + visual_flag_cases() + visual_width_layer_cases()
    assert len(cases) == 3831, f'{len(cases)}; expected the full3831-case independent visual matrix'
    rows: list[str] = []
    for template in templates:
        for family, words in cases:
            rows.append(verify_component_word_case(root, template, family, words))
    assert len(rows) == 7662, f'{len(rows)}; expected3831 complete cases for both part and assembly'
    return ''.join(rows).encode()


def persist_component_visual_matrix(templates: list[ComponentVisualTemplate], rows: bytes, controls: list[dict[str, object]]) -> None:
    """Lock factored oracles while retaining per-case whole-root hashes and full checks; e.g. persist_component_visual_matrix(templates, rows, controls)."""
    count = len(rows.splitlines())
    assert count == 7662, f'{count}; expected all7662 completed component cases before persistence'
    packet, primitives = component_template_packet(templates), component_template_primitives(templates)
    for suffix, raw in (('templates.bin', packet), ('words.tsv', rows), ('primitives.tsv', primitives)):
        lock_visual_fixture('component-' + suffix, raw)
    result = dict(cases=7662, cases_per_kind=3831, physical_visual_definitions=7662,
        selected_scalar_spans=7 * 7662, complete_source_reads_per_case=True, no_reader_overrides=True, coverage_credit=0,
        primitive_reads_checked=3831 * sum(len(template.captured.physical) for template in templates),
        baseline_controls=controls, template_bytes=len(packet), template_sha256=hashlib.sha256(packet).hexdigest(),
        word_oracle_bytes=len(rows), word_oracle_sha256=hashlib.sha256(rows).hexdigest(),
        primitive_oracle_bytes=len(primitives), primitive_oracle_sha256=hashlib.sha256(primitives).hexdigest())
    raw = (json.dumps(result, indent=2) + '\n').encode()
    path = FEATURE / 'owned-component-reference.json'
    if '--write' in sys.argv:
        path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected complete unchanged component matrix evidence'


def verify_component_visual_matrix() -> None:
    """Keep baseline controls, both whole-owner matrices and every alias check together; e.g. verify_component_visual_matrix()."""
    root = FEATURE.parents[2]
    controls = verify_component_visual_baselines(root)
    templates = component_visual_templates(root)
    rows = collect_component_visual_matrix(root, templates)
    persist_component_visual_matrix(templates, rows, controls)
