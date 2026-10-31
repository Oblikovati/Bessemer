"""Actual complete malformed acceptance and independent exact source budgets (#138)."""
import re
import struct
from typing import cast
from owned_mass import owned_mass_cases
from mass_reference import actual_mass_parser
from mass_projection import mass_shape
from thermal_projection import projected_fields
from thermal_malformed import span_mutations, is_text_span, source_budget_metrics, encode_mutation_cases
from inertial_malformed import maximum_embedded_fields


def is_mass_text(label: str) -> bool:
    """Identify common text and both mass-specific text arrays; e.g. is_mass_text('record0.mass.header_text0')."""
    return is_text_span(label) or re.search(r'\.mass\.(?:header_text|text)\d+$', label) is not None


def mass_mutation_packet() -> tuple[bytes, int, int]:
    """Preserve every complete actual parser outcome without filtering accepted mutations; e.g. mass_mutation_packet()."""
    cases: list[tuple[int, int, bytes, list[tuple[bool, bytes]]]] = []
    for _, version, raw in owned_mass_cases():
        parsed = actual_mass_parser(raw, version)
        assert parsed['success']
        spans = cast(list[dict[str, object]], parsed['spans'])
        images = {image for span in spans for image in span_mutations(span, raw, is_mass_text)}
        outcomes = [(cast(bool, actual_mass_parser(image, version)['success']), image)
                    for image in sorted(images)]
        cases.append((0, version, raw, outcomes))
    return encode_mutation_cases(cases)


def mass_budget_packet() -> bytes:
    """Measure limits from source spans, classes and original primitive calls; e.g. mass_budget_packet()."""
    packet = bytearray()
    for _, version, raw in owned_mass_cases():
        parsed = actual_mass_parser(raw, version)
        assert parsed['success']
        spans = cast(list[dict[str, object]], parsed['spans'])
        archives = cast(list[dict[str, object]], parsed['archive_records'])
        metrics = source_budget_metrics(raw, spans, archives, is_mass_text)
        fields = len(projected_fields(raw, spans, mass_shape))
        nested_fields = maximum_embedded_fields(raw, archives)
        packet.extend(struct.pack('<III', 0, version, len(raw)) + raw)
        packet.extend(struct.pack('<IIIIII', *metrics, fields, nested_fields))
    return bytes(packet)
