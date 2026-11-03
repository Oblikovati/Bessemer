"""Shared complete visual caller corpus and immutable oracle persistence (#181)."""
from collections.abc import Callable
from typing import TYPE_CHECKING
import importlib
import hashlib
import json
import struct
import sys
from visual_state_evidence import FEATURE
from visual_owned_words import VisualStoredWords, visual_rgb_cases, visual_flag_cases, visual_width_layer_cases
if TYPE_CHECKING:
    from freeze_owned_features import ReferenceDocument


def capture_visual_stream(document: 'ReferenceDocument', raw: bytes, starts: list[int], index: int) -> tuple[dict[str, object], bytes]:
    """Check every original read and literal visual scalar span; e.g. capture_visual_stream(doc, raw, starts, 0)."""
    from freeze_owned_features import ReplacedFeatureDocument, STREAM
    from primitive_inventory import observe_primitives, verify_primitive_coverage
    archive = importlib.import_module('swarchive')
    with observe_primitives(archive.Archive, raw) as reads:
        parsed = importlib.import_module('resolvedfeatures').read_supported(ReplacedFeatureDocument(document, raw), STREAM)
    physical = [read for read in reads if read.end > read.start]
    verify_primitive_coverage(raw, physical)
    spans = [(start + offset, start + offset + length, kind) for start in starts
             for offset, length, kind in ((0, 2, 'H'), (2, 2, 'H'), (4, 1, 'B'), (5, 4, 'I'), (9, 2, 'H'), (11, 2, 'h'), (13, 4, 'f'))]
    selected = [read for read in physical if (read.start, read.end, read.kind) in spans]
    assert [(read.start, read.end, read.kind) for read in selected] == spans, f'{index}; expected all14literal visual spans {spans}'
    callers = {'H': ('mfcobjects.py', 'u16'), 'h': ('mfcobjects.py', 'i16'),
               'B': ('dl_light.py', 'read_moEntVisProp_c'), 'I': ('dl_light.py', 'read_moEntVisProp_c'),
               'f': ('dl_ui.py', 'read_utLineWidth_c')}
    assert all((read.source_file, read.source_function) == callers[read.kind] for read in selected), selected
    oracle = ''.join(f'{index}\t{read.start}\t{read.end}\t{read.kind}\t{read.raw_hex}\n' for read in physical).encode()
    return parsed, oracle


def lock_visual_fixture(name: str, raw: bytes | bytearray) -> None:
    """Keep owned fixture outputs immutable outside explicit initialization; e.g. lock_visual_fixture('model.bin', packet)."""
    path = FEATURE.parents[2] / 'tests/fixtures/entity-visual-state' / name
    path.parent.mkdir(parents=True, exist_ok=True)
    if '--write' in sys.argv or ('--write-sketch' in sys.argv and name.startswith('sketch')):
        path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected complete unchanged visual carrier evidence'


def persist_visual_carrier_oracles(prefix: str, caller: str, packet: bytearray, rows: list[str], primitives: bytearray, contexts: list[str]) -> None:
    """Retain complete literals and original scalar partitions for each caller; e.g. persist_visual_carrier_oracles('model', caller, packet, rows, primitives, contexts)."""
    oracle = ''.join(rows).encode()
    for suffix, raw in (('.bin', packet), ('-words.tsv', oracle), ('-primitives.tsv', primitives)):
        lock_visual_fixture(prefix + suffix, raw)
    result = dict(caller=caller, cases=len(rows), physical_records=2 * len(rows),
        instance_aliases=len(rows), inherited_slots=97, final_slots=100,
        context_streams_sha256=contexts, packet_bytes=len(packet), wire_bytes=len(packet) - 4 * len(rows),
        packet_sha256=hashlib.sha256(packet).hexdigest(), word_oracle_sha256=hashlib.sha256(oracle).hexdigest(),
        primitive_reads=len(primitives.splitlines()), primitives_sha256=hashlib.sha256(primitives).hexdigest(), literal_scalar_spans=14 * len(rows))
    path = FEATURE / f'owned-{prefix}-reference.json'
    raw = (json.dumps(result, indent=2) + '\n').encode()
    if '--write' in sys.argv or ('--write-sketch' in sys.argv and prefix == 'sketch'):
        path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected complete unchanged owned visual metadata'


def verify_visual_caller_corpus(prefix: str, caller: str,
        stream: Callable[[VisualStoredWords], tuple[bytes, list[int]]],
        verify: Callable[[dict[str, object], VisualStoredWords], None]) -> None:
    """Use the same complete independent matrix for each real owner; e.g. verify_visual_caller_corpus('model', caller, stream, verify)."""
    from dimension_state_owned import owned_dimension_context
    document, contexts = owned_dimension_context()
    cases = visual_rgb_cases() + visual_flag_cases() + visual_width_layer_cases()
    assert len(cases) == 3831, f'{len(cases)}; expected3072RGB,704flag and55width/layer/style cases'
    packet, primitives = bytearray(), bytearray()
    rows: list[str] = []
    for family, words in cases:
        raw, starts = stream(words)
        parsed, observed = capture_visual_stream(document, raw, starts, len(rows))
        assert parsed['end'] == len(raw), f'{len(rows)}; expected complete EOF at{len(raw)}'
        verify(parsed, words)
        packet.extend(struct.pack('<I', len(raw)) + raw)
        primitives.extend(observed)
        rows.append(f'{family}\t' + '\t'.join(map(str, words)) + f'\t{starts[0]}\t{starts[1]}\t{len(raw)}\n')
    persist_visual_carrier_oracles(prefix, caller, packet, rows, primitives, contexts)
