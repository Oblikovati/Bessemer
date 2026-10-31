"""Actual framing mutations and independent stored-field resource requirements (#150)."""
import struct
from collections.abc import Callable, Sequence
from typing import cast
from owned_body import owned_solid_bodies, solid_body_literal
from body_projection import body_shape
from body_history import body_embedded_case
from thermal_projection import FieldShape, projected_fields
from thermal_malformed import span_mutations, source_budget_metrics, encode_mutation_cases

BodyParser = Callable[[bytes, int], dict[str, object]]


def body_text_span(label: str) -> bool:
    """Exclude list counts from text frames; e.g. body_text_span('list128.0')."""
    return label.startswith('text') or label.startswith('list128.') and label != 'list128.count'


def body_mutations(span: dict[str, object], raw: bytes) -> list[bytes]:
    """Use each exact grammar cap rather than a shared collection cap; e.g. body_mutations(span, raw)."""
    label, offset = cast(str, span['label']), cast(int, span['offset'])
    if label not in ('persistid.blob_count', 'list128.count'): return span_mutations(span, raw, body_text_span)
    maximum = 1000000 if label == 'persistid.blob_count' else 100000
    return [raw[:offset] + struct.pack('<i', count) + raw[offset + 4:]
            for count in (-1, 0, 1, maximum - 1, maximum, maximum + 1, 2147483647)]


def body_mutation_packet(parser: BodyParser, contexts: Sequence[tuple[int, int, bytes]] | None = None) -> tuple[bytes, int, int]:
    """Retain every outcome including unchanged positive controls; e.g. body_mutation_packet()."""
    cases: list[tuple[int, int, bytes, list[tuple[bool, bytes]]]] = []
    for _, version, raw in owned_solid_bodies() if contexts is None else contexts:
        parsed = parser(raw, version)
        assert parsed['success']
        images = {image for span in cast(list[dict[str, object]], parsed['spans']) for image in body_mutations(span, raw)}
        images.add(raw)
        outcomes = [(cast(bool, parser(image, version)['success']), image) for image in sorted(images)]
        cases.append((0, version, raw, outcomes))
    return encode_mutation_cases(cases)


def largest_body_archive_fields(raw: bytes, archives: list[dict[str, object]], project: Callable[[bytes], bytes] = body_embedded_case) -> int:
    """Observe nested typed calls under the exact C-734 callback; e.g. largest_body_archive_fields(raw, archives)."""
    counts: list[int] = []
    for item in archives:
        start, end = cast(int, item['offset']), cast(int, item['end'])
        observed = project(raw[start:end])
        counts.append(struct.unpack_from('<I', observed, 4 + end - start)[0])
    return max(counts, default=0)


def body_budget_packet(parser: BodyParser, contexts: Sequence[tuple[int, int, bytes]] | None = None, shape: Callable[[str, int], list[FieldShape]] = body_shape, project: Callable[[bytes], bytes] = body_embedded_case) -> bytes:
    """Compute seven exact budgets from independent spans and primitive calls; e.g. body_budget_packet()."""
    packet = bytearray()
    for _, version, raw in owned_solid_bodies() if contexts is None else contexts:
        parsed = parser(raw, version)
        assert parsed['success']
        spans = cast(list[dict[str, object]], parsed['spans'])
        archives = cast(list[dict[str, object]], parsed['archives'])
        metrics = source_budget_metrics(raw, spans, archives, body_text_span)
        fields = len(projected_fields(raw, spans, shape))
        nested = largest_body_archive_fields(raw, archives, project)
        packet.extend(struct.pack('<III', 0, version, len(raw)) + raw)
        packet.extend(struct.pack('<IIIIII', *metrics, fields, nested))
    return bytes(packet)


def verify_body_hardcaps(parser: BodyParser, baseline: Callable[[str, object], None]) -> None:
    """Supply sufficient payload on both sides of each fixed cap; e.g. verify_body_hardcaps()."""
    outcomes: list[dict[str, object]] = []
    for count in (1000000, 1000001):
        original = solid_body_literal(0, 0)
        raw = original[:8] + struct.pack('<i', count) + b'\xaa' * count + original[12:]
        success = parser(raw, 0)['success']
        assert success == (count == 1000000)
        outcomes.append(dict(kind='blob', count=count, success=success))
    for count in (100000, 100001):
        raw = solid_body_literal(870008, 0)[:-4] + struct.pack('<i', count) + bytes(count * 4)
        success = parser(raw, 870008)['success']
        assert success == (count == 100000)
        outcomes.append(dict(kind='list', count=count, success=success))
    baseline('hardcap-reference.json', outcomes)
