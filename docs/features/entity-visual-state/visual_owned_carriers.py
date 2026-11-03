"""Complete owned folder-feature carriers and original full-byte primitive partitions (#181)."""
from typing import TYPE_CHECKING, cast
import importlib
import hashlib
import json
import struct
import sys
from visual_state_evidence import FEATURE
from visual_owned_words import (VisualStoredWords, visual_word_bytes, contrasting_visual_words,
    visual_rgb_cases, visual_flag_cases, visual_width_layer_cases, verify_visual_scalar_projection)
if TYPE_CHECKING:
    from freeze_owned_features import ReferenceDocument


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


def lock_visual_fixture(name: str, raw: bytes | bytearray) -> None:
    """Keep owned fixture outputs immutable outside explicit initialization; e.g. lock_visual_fixture('model.bin', packet)."""
    path = FEATURE.parents[2] / 'tests/fixtures/entity-visual-state' / name
    path.parent.mkdir(parents=True, exist_ok=True)
    if '--write' in sys.argv:
        path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected complete unchanged visual carrier evidence'


def persist_visual_model_oracles(packet: bytearray, rows: list[str], primitives: bytearray, contexts: list[str]) -> None:
    """Retain complete literals and original scalar partitions without old baseline changes; e.g. persist_visual_model_oracles(packet, rows, primitives, contexts)."""
    oracle = ''.join(rows).encode()
    for name, raw in (('model.bin', packet), ('model-words.tsv', oracle), ('model-primitives.tsv', primitives)):
        lock_visual_fixture(name, raw)
    result = dict(caller='complete moFolder_c features in resolved-feature carriers', cases=len(rows),
        physical_records=2 * len(rows), instance_aliases=len(rows), inherited_slots=97, final_slots=100,
        context_streams_sha256=contexts, packet_bytes=len(packet), wire_bytes=len(packet) - 4 * len(rows),
        packet_sha256=hashlib.sha256(packet).hexdigest(), word_oracle_sha256=hashlib.sha256(oracle).hexdigest(),
        primitive_reads=len(primitives.splitlines()), primitives_sha256=hashlib.sha256(primitives).hexdigest(), literal_scalar_spans=14 * len(rows))
    path = FEATURE / 'owned-model-reference.json'
    raw = (json.dumps(result, indent=2) + '\n').encode()
    if '--write' in sys.argv:
        path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected complete unchanged owned model metadata'


def verify_owned_visual_model_carriers() -> None:
    """Prove one complete caller before extending the visual operation; e.g. verify_owned_visual_model_carriers()."""
    from dimension_state_owned import owned_dimension_context
    document, contexts = owned_dimension_context()
    cases = visual_rgb_cases() + visual_flag_cases() + visual_width_layer_cases()
    assert len(cases) == 3831, f'{len(cases)}; expected3072RGB,704flag and55width/layer/style cases'
    packet, primitives = bytearray(), bytearray()
    rows: list[str] = []
    for family, words in cases:
        raw, starts = owned_visual_stream(words)
        parsed, observed = capture_visual_stream(document, raw, starts, len(rows))
        assert parsed['end'] == len(raw), f'{len(rows)}; expected complete EOF at{len(raw)}'
        verify_visual_feature_items(parsed, words)
        packet.extend(struct.pack('<I', len(raw)) + raw)
        primitives.extend(observed)
        rows.append(f'{family}\t' + '\t'.join(map(str, words)) + f'\t{starts[0]}\t{starts[1]}\t{len(raw)}\n')
    persist_visual_model_oracles(packet, rows, primitives, contexts)
