"""Actual SurfaceBody count/text mutations and independent outer-only requirements (#153)."""
import struct
from typing import cast
from collections.abc import Callable
from body_adversarial import BodyParser, body_mutation_packet
from thermal_malformed import span_mutations, source_budget_metrics
from owned_surface import owned_surface_bodies, surface_literal
from surface_projection import surface_fields


def surface_text_span(label: str) -> bool:
    """Exclude the physical signed list count from text frames; e.g. surface_text_span('list108.0')."""
    return label == 'text68' or label.startswith('list108.') and label != 'list108.count'


def surface_mutations(span: dict[str, object], raw: bytes) -> list[bytes]:
    """Use the exact list108 cap and preserve every reinterpretation outcome; e.g. surface_mutations(span, raw)."""
    if span['label'] != 'list108.count': return span_mutations(span, raw, surface_text_span)
    offset = cast(int, span['offset'])
    return [raw[:offset] + struct.pack('<i', count) + raw[offset + 4:]
            for count in (-1, 0, 1, 99999, 100000, 100001, 2147483647)]


def surface_mutation_packet(parser: BodyParser) -> tuple[bytes, int, int]:
    """Keep all accepted/rejected source outcomes including unchanged controls; e.g. surface_mutation_packet(parser)."""
    return body_mutation_packet(parser, owned_surface_bodies(), surface_mutations)


def surface_budget_packet(parser: BodyParser) -> bytes:
    """Measure the four physical header requirements without unrelated embedded options; e.g. surface_budget_packet(parser)."""
    packet = bytearray()
    for _, version, raw in owned_surface_bodies():
        parsed = parser(raw, version)
        assert parsed['success']
        spans = cast(list[dict[str, object]], parsed['spans'])
        entries, text_bytes, _, _ = source_budget_metrics(raw, spans, [], surface_text_span)
        packet.extend(struct.pack('<II', version, len(raw)) + raw)
        packet.extend(struct.pack('<III', entries, text_bytes, len(surface_fields(raw, version))))
    return bytes(packet)


def surface_list_cap(parser: BodyParser, save: Callable[[str, object], None]) -> None:
    """Require sufficient complete frames at and above the fixed cap; e.g. surface_list_cap(parser, save)."""
    prefix = surface_literal(870008, 0)[:-4]
    outcomes: list[dict[str, object]] = []
    for count in (100000, 100001):
        raw = prefix + struct.pack('<i', count) + bytes(count * 4)
        success = parser(raw, 870008)['success']
        assert success == (count == 100000)
        outcomes.append(dict(count=count, success=success))
    save('list-cap-reference.json', outcomes)
