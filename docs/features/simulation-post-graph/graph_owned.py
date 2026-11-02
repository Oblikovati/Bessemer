"""Independent populated graph boundary controls and complete reference outcomes (#165)."""
from collections.abc import Iterator
from itertools import product
from typing import cast
import importlib
from native_carriers import digest
from graph_evidence import READER, graph_baseline
from graph_literals import graph_blocks

GRAPH_VERSIONS = (800018, 800019, 800025, 800026, 800027, 800061, 800062, 800063,
    820021, 820022, 820023, 840027, 840028, 840029, 870001, 870002, 870003,
    890001, 890002, 890003, 890004, 890005, 890006, 890109, 890110, 890111, 2147483647)


def graph_owned_cases() -> Iterator[tuple[int, int, bool, bytes]]:
    """Keep all 270 version/rotation/empty contexts in a fixed order; e.g. list(graph_owned_cases())."""
    for version, rotation, empty_only in product(GRAPH_VERSIONS, range(5), (False, True)):
        raw = b''.join(fragment for _, fragment in graph_blocks(version, rotation, empty_only))
        yield version, rotation, empty_only, raw


def graph_literal_spans(version: int, rotation: int, empty_only: bool) -> list[dict[str, object]]:
    """Derive every source span before consulting the parser; e.g. graph_literal_spans(890110, 1, False)."""
    spans: list[dict[str, object]] = []
    offset = 0
    for label, fragment in graph_blocks(version, rotation, empty_only):
        spans.append(dict(label=label, offset=offset, end=offset + len(fragment), raw_hex=fragment.hex()))
        offset += len(fragment)
    return spans


def graph_owned_context(version: int, rotation: int, empty_only: bool, raw: bytes) -> dict[str, object]:
    """Require full independent framing and raw string values at exact EOF; e.g. graph_owned_context(890110, 1, False, raw)."""
    parsed = importlib.import_module(READER).parse(raw, version)
    spans = graph_literal_spans(version, rotation, empty_only)
    assert parsed['success'] and parsed['end'] == len(raw), (version, rotation, parsed)
    assert parsed['spans'] == spans and parsed['reconstruction_sha256'] == digest(raw), (version, rotation)
    for row in cast(list[dict[str, object]], parsed['strings']):
        expected = next(span for span in spans if span['label'] == cast(str, row['label']) + '.utf16')
        text = bytes.fromhex(cast(str, expected['raw_hex']))
        assert (row['count'], row['raw_hex'], row['text']) == (len(text) // 2, text.hex(), text.decode('utf-16le', 'surrogatepass'))
    return dict(version=version, rotation=rotation, empty_only=empty_only, bytes=len(raw), sha256=digest(raw), result=parsed)


def verify_graph_owned() -> None:
    """Lock all boundary, scalar-pattern and populated-text outcomes; e.g. verify_graph_owned()."""
    rows = [graph_owned_context(*case) for case in graph_owned_cases()]
    assert len(rows) == 270, len(rows)
    graph_baseline('owned-reference.json', dict(contexts=len(rows), logical_bytes=sum(cast(int, row['bytes']) for row in rows), rows=rows))
