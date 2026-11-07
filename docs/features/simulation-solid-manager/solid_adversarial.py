"""Actual direct-header mutations and independent exact caller budgets (#148)."""
import struct
from typing import cast
from owned_solid_manager import owned_solid_headers
from solid_projection import solid_fields
from thermal_malformed import span_mutations, source_budget_metrics, encode_mutation_cases


def solid_text(label: str) -> bool:
    """Match only physical list texts, never their signed counts; e.g. solid_text('listb0.0')."""
    prefix, _, ordinal = label.partition('.')
    return prefix in ('listb0', 'liste8') and ordinal.isdecimal()


def solid_mutations(span: dict[str, object], raw: bytes) -> list[bytes]:
    """Use this grammar's 100000 cap and retain valid-count reinterpretations; e.g. solid_mutations(span, raw)."""
    label = cast(str, span['label'])
    if not label.endswith('.count'): return span_mutations(span, raw, solid_text)
    offset = cast(int, span['offset'])
    return [raw[:offset] + struct.pack('<i', count) + raw[offset + 4:]
            for count in (-1, 0, 1, 99999, 100000, 100001)]


def mutation_packet() -> tuple[bytes, int, int]:
    """Preserve all actual accepted/rejected outcomes without filtering; e.g. mutation_packet()."""
    from cosworks_solid_manager_layout import parse
    cases: list[tuple[int, int, bytes, list[tuple[bool, bytes]]]] = []
    for _, version, raw in owned_solid_headers():
        parsed = parse(raw, version)
        assert parsed['success']
        images = {image for span in cast(list[dict[str, object]], parsed['spans']) for image in solid_mutations(span, raw)}
        outcomes = [(cast(bool, parse(image, version)['success']), image) for image in sorted(images)]
        cases.append((0, version, raw, outcomes))
    return encode_mutation_cases(cases)


def budget_packet() -> bytes:
    """Measure aggregate lists, raw text payloads and complete physical fields; e.g. budget_packet()."""
    from cosworks_solid_manager_layout import parse
    packet = bytearray()
    for _, version, raw in owned_solid_headers():
        parsed = parse(raw, version)
        spans = cast(list[dict[str, object]], parsed['spans'])
        entries, text, _, _ = source_budget_metrics(raw, spans, [], solid_text)
        packet.extend(struct.pack('<II', version, len(raw)) + raw)
        packet.extend(struct.pack('<III', entries, text, len(solid_fields(raw, parsed))))
    return bytes(packet)
