"""Complete unchanged Source graphs and literal scalar partitions for owned roots (#184)."""
from typing import cast
import importlib
import struct
from freeze_owned_features import ReferenceDocument, ReplacedFeatureDocument, STREAM
from primitive_inventory import PrimitiveRead, observe_primitives, verify_primitive_coverage
from owned_intersection_match import ReferenceMatchWords


def read_intersection_matrix_root(document: ReferenceDocument, raw: bytes) -> tuple[dict[str, object], list[PrimitiveRead]]:
    """Delegate the complete root reader without field overrides; e.g. read_intersection_matrix_root(document, raw)."""
    archive = importlib.import_module('swarchive')
    with observe_primitives(archive.Archive, raw) as observed:
        parsed = importlib.import_module('resolvedfeatures').read_supported(ReplacedFeatureDocument(document, raw), STREAM)
    physical = [read for read in observed if read.end > read.start]
    verify_primitive_coverage(raw, physical)
    return cast(dict[str, object], parsed), physical


def verify_intersection_matrix_graph(parsed: dict[str, object], length: int, expected: tuple[tuple[ReferenceMatchWords, ...], ...]) -> None:
    """Keep definitions, physical groups and aliases distinct; e.g. verify_intersection_matrix_graph(parsed, len(raw), groups)."""
    assert parsed['end'] == length and parsed['list_complete'], 'Expected complete original-reader EOF'
    items = cast(list[dict[str, object]], parsed['items'])
    assert len(items) == 3 and items[0] is items[2] and items[0] is not items[1], 'Expected two definitions and first-instance alias'
    assert len(getattr(parsed['archive'], 'map')) == 100, 'Expected97 inherited slots plus class and two definitions'
    for item, words in zip(items[:2], expected, strict=True):
        assert item['_class'] == 'sgSketch', item['_class']
        collection = cast(dict[str, object], item['styled_intersections'])
        intersections = cast(list[dict[str, object]], collection['intersections'])
        assert len(intersections) == 2 and intersections[0]['marker'] == 65535
        full = intersections[1]
        edges = cast(list[dict[str, object]], full['edges_48'])
        assert full['marker'] == 0 and full['handle_count'] == 1 and len(edges) == 1
        for group, values in zip(('38', '80', 'c8', '110'), words, strict=True):
            actual = cast(dict[str, object], edges[0]['group_' + group])
            assert tuple(actual[name] for name in ('raw_00', 'raw_20', 'raw_24', 'raw_28')) == values


def verify_intersection_matrix_spans(raw: bytes, starts: list[int], groups: tuple[ReferenceMatchWords, ...], physical: list[PrimitiveRead]) -> None:
    """Bind all32 selected scalar reads to literal wire positions; e.g. verify_intersection_matrix_spans(raw, starts, groups, reads)."""
    assert len(starts) == len(groups) == 8, 'Expected eight physical reference groups'
    spans = [(start + offset, start + offset + length, kind) for start in starts
             for offset, length, kind in ((0, 2, 'H'), (2, 4, 'i'), (6, 4, 'i'), (10, 4, 'i'))]
    selected = [read for read in physical if (read.start, read.end, read.kind) in spans]
    assert [(read.start, read.end, read.kind) for read in selected] == spans, 'Expected all32 literal scalar spans in source order'
    for start, words in zip(starts, groups, strict=True):
        assert raw[start:start + 14] == struct.pack('<H3i', *words), (start, words)
        assert raw[start + 14:start + 20] == bytes(6), 'Expected unchanged null reference siblings'


def intersection_matrix_primitive_rows(index: int, physical: list[PrimitiveRead]) -> bytes:
    """Retain every raw primitive rather than decoded float values; e.g. intersection_matrix_primitive_rows(0, reads)."""
    assert len(physical) == 451, f'{len(physical)}; expected complete451-read control shape'
    assert all(read.end > read.start for read in physical), 'Expected physical reads only'
    return ''.join(f'{index}\t{read.start}\t{read.end}\t{read.kind}\t{read.raw_hex}\n' for read in physical).encode()
