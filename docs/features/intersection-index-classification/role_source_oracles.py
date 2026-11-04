"""Independent complete Source graphs and physical primitive oracles (#185)."""
from typing import cast
import struct
from primitive_inventory import PrimitiveRead
from owned_intersection_roles import OwnedIntersectionRoles


def role_source_edges(parsed: dict[str, object], counts: tuple[int, int]) -> list[dict[str, object]]:
    """Keep both definitions and the real alias visible; e.g. role_source_edges(parsed, (1, 1))."""
    items = cast(list[dict[str, object]], parsed['items'])
    assert len(items) == 3 and items[0] is items[2] and items[0] is not items[1]
    assert len(getattr(parsed['archive'], 'map')) == 100
    edges: list[dict[str, object]] = []
    for item, count in zip(items[:2], counts, strict=True):
        assert item['_class'] == 'sgSketch'
        collection = cast(dict[str, object], item['styled_intersections'])
        intersections = cast(list[dict[str, object]], collection['intersections'])
        assert len(intersections) == 2 and intersections[0]['marker'] == 65535
        full = intersections[1]
        current = cast(list[dict[str, object]], full['edges_48'])
        assert full['marker'] == 0 and full['handle_count'] == count and len(current) == count
        assert len(cast(list[object], full['handles_40'])) == count
        edges.extend(current)
    return edges


def verify_role_source_graph(parsed: dict[str, object], owned: OwnedIntersectionRoles) -> None:
    """Compare every index and all four sibling words; e.g. verify_role_source_graph(parsed, owned)."""
    assert parsed['list_complete'] and parsed['end'] == len(owned.raw)
    edges = role_source_edges(parsed, owned.owner_edge_counts)
    assert [edge['raw_168'] for edge in edges] == [value for _, value in owned.indices]
    actual = []
    for edge in edges:
        for group in ('38', '80', 'c8', '110'):
            value = cast(dict[str, object], edge['group_' + group])
            actual.append(tuple(value[key] for key in ('raw_00', 'raw_20', 'raw_24', 'raw_28')))
    assert actual == [words for _, words in owned.references]


def role_expected_scalars(owned: OwnedIntersectionRoles) -> dict[tuple[int, int, str], bytes]:
    """Bind field widths and offsets independently of Rust; e.g. role_expected_scalars(owned)."""
    expected = {(start, start + 4, 'i'): struct.pack('<i', value) for start, value in owned.indices}
    for start, words in owned.references:
        expected[(start, start + 2, 'H')] = struct.pack('<H', words[0])
        for offset, value in zip((2, 6, 10), words[1:], strict=True):
            expected[(start + offset, start + offset + 4, 'i')] = struct.pack('<i', value)
    assert len(expected) == len(owned.indices) + 4 * len(owned.references)
    return expected


def verify_role_source_primitives(owned: OwnedIntersectionRoles, physical: list[PrimitiveRead]) -> None:
    """Require all literal scalar observations and full coverage; e.g. verify_role_source_primitives(owned, reads)."""
    expected = role_expected_scalars(owned)
    selected = {(read.start, read.end, read.kind): bytes.fromhex(read.raw_hex)
                for read in physical if (read.start, read.end, read.kind) in expected}
    assert selected == expected
    assert len(physical) == 451 + (sum(owned.owner_edge_counts) - 2) * 34
    for (start, end, _), raw in expected.items():
        assert owned.raw[start:end] == raw


def role_scalar_rows(case: int, owned: OwnedIntersectionRoles) -> bytes:
    """Retain every independent role and sibling value; e.g. role_scalar_rows(0, owned)."""
    rows = [f'{case}\tindex\t{start}\t{value}\n' for start, value in owned.indices]
    rows.extend(f'{case}\tgroup\t{start}\t' + '\t'.join(map(str, words)) + '\n'
                for start, words in owned.references)
    return ''.join(rows).encode()


def role_primitive_rows(case: int, physical: list[PrimitiveRead]) -> bytes:
    """Retain the whole original raw partition; e.g. role_primitive_rows(0, reads)."""
    return ''.join(f'{case}\t{read.start}\t{read.end}\t{read.kind}\t{read.raw_hex}\n'
                   for read in physical).encode()
