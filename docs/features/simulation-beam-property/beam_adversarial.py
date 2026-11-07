"""Complete actual beam mutation outcomes and independent resource budgets (#142)."""
import struct
from typing import cast
from owned_beam import owned_beam_cases
from beam_projection import beam_parser, beam_fields
from thermal_malformed import span_mutations, is_text_span, source_budget_metrics, encode_mutation_cases
from inertial_malformed import maximum_embedded_fields


def beam_text(label: str) -> bool:
    """Include both direct beam strings and shared reference strings; e.g. beam_text('record0.beam.textd0')."""
    return is_text_span(label) or label.endswith(('.beam.textd0', '.beam.textc8'))


def count_spans(parsed: dict[str, object]) -> list[dict[str, object]]:
    """Adapt the beam point-count role to shared count accounting; e.g. count_spans(parsed)."""
    return [dict(span, label=cast(str, span['label']).replace('.point_count', '.points.count'))
            for span in cast(list[dict[str, object]], parsed['spans'])]


def mutation_packet() -> tuple[bytes, int, int]:
    """Keep accepted alternatives as well as rejected mutations; e.g. mutation_packet()."""
    cases: list[tuple[int, int, bytes, list[tuple[bool, bytes]]]] = []
    for _, version, raw in owned_beam_cases():
        parsed = beam_parser(raw, version)
        assert parsed['success']
        images = {image for span in count_spans(parsed) for image in span_mutations(span, raw, beam_text)}
        outcomes = [(cast(bool, beam_parser(image, version)['success']), image) for image in sorted(images)]
        cases.append((0, version, raw, outcomes))
    return encode_mutation_cases(cases)


def budget_packet() -> bytes:
    """Derive exact limits from complete Source calls and embedded classes; e.g. budget_packet()."""
    packet = bytearray()
    for _, version, raw in owned_beam_cases():
        parsed = beam_parser(raw, version)
        archives = cast(list[dict[str, object]], parsed['archive_records'])
        metrics = source_budget_metrics(raw, count_spans(parsed), archives, beam_text)
        fields = len(beam_fields(raw, parsed))
        nested_fields = maximum_embedded_fields(raw, archives)
        packet.extend(struct.pack('<III', 0, version, len(raw)) + raw)
        packet.extend(struct.pack('<IIIIII', *metrics, fields, nested_fields))
    return bytes(packet)
