"""Complete actual malformed outcomes and independent count-context budgets (#144)."""
import struct
from typing import cast
from owned_groups import owned_group_cases
from group_projection import actual_parser, fields_for
from thermal_malformed import is_text_span, span_mutations, source_budget_metrics, encode_mutation_cases
from inertial_malformed import maximum_embedded_fields


def group_text(label: str) -> bool:
    """Include direct group/folder text alongside shared reference names; e.g. group_text('group.textc8')."""
    return is_text_span(label) or label in ('group.textc8', 'folder.text28')


def count_spans(parsed: dict[str, object]) -> list[dict[str, object]]:
    """Preserve source offsets while identifying the beam-ID count; e.g. count_spans(parsed)."""
    return [dict(span, label=cast(str, span['label']).replace('joint.beam_count', 'joint.beams.count'))
            for span in cast(list[dict[str, object]], parsed['spans'])]


def mutation_packet() -> tuple[bytes, int, int]:
    """Retain every original context and every actual outcome; e.g. mutation_packet()."""
    packet = bytearray()
    total = accepted = 0
    for kind, version, expected, raw in owned_group_cases():
        parsed = actual_parser(raw, version, kind, expected)
        assert parsed['success']
        images = {image for span in count_spans(parsed) for image in span_mutations(span, raw, group_text)}
        outcomes = [(cast(bool, actual_parser(image, version, kind, expected)['success']), image) for image in sorted(images)]
        encoded, count, yes = encode_mutation_cases([(kind, version, raw, outcomes)])
        packet.extend(struct.pack('<i', -1 if expected is None else expected) + encoded)
        total += count
        accepted += yes
    return bytes(packet), total, accepted


def budget_packet() -> bytes:
    """Derive exact outer and embedded caps from actual primitive calls; e.g. budget_packet()."""
    packet = bytearray()
    for kind, version, expected, raw in owned_group_cases():
        parsed = actual_parser(raw, version, kind, expected)
        archives = cast(list[dict[str, object]], parsed['archive_records'])
        metrics = source_budget_metrics(raw, count_spans(parsed), archives, group_text)
        fields = len(fields_for(raw, parsed))
        nested_fields = maximum_embedded_fields(raw, archives)
        packet.extend(struct.pack('<IIiI', kind, version, -1 if expected is None else expected, len(raw)) + raw)
        packet.extend(struct.pack('<IIIIII', *metrics, fields, nested_fields))
    return bytes(packet)
