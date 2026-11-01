"""Independent composite budgets and actual grammar mutation outcomes (#157)."""
from collections.abc import Callable
from typing import cast
import struct
from owned_composite import owned_composite_cases
from composite_projection import composite_context, composite_fields
from body_adversarial import largest_body_archive_fields, body_mutations
from thermal_malformed import source_budget_metrics, encode_mutation_cases
from sheet_projection import sheet_embedded_fields

CompositeParser = Callable[[bytes, int, str, str], dict[str, object]]


def composite_budget_packet(parser: CompositeParser) -> bytes:
    """Measure each outer field and independently projected nested root; e.g. composite_budget_packet(parser)."""
    packet = bytearray()
    for case in owned_composite_cases():
        raw, version, kind, profile = case['raw'], case['version'], case['kind'], case['wire_profile']
        parsed = parser(raw, version, kind, profile)
        spans = cast(list[dict[str, object]], parsed['spans'])
        archives = cast(list[dict[str, object]], parsed['archives'])
        _, text, root, objects = source_budget_metrics(raw, spans, archives, lambda label: False)
        entries = struct.unpack_from('<i', raw)[0] if kind == 'CompositeFlipTurn' else 0
        fields = len(composite_fields(raw, parsed))
        nested = largest_body_archive_fields(raw, archives, sheet_embedded_fields)
        packet.extend(struct.pack('<III', composite_context(kind, profile), version, len(raw)) + raw)
        packet.extend(struct.pack('<6I', entries, text, root, objects, fields, nested))
    return bytes(packet)


def composite_mutations(parser: CompositeParser) -> tuple[bytes, int, int]:
    """Keep positive controls and every changed selector/count/archive outcome; e.g. composite_mutations(parser)."""
    cases: list[tuple[int, int, bytes, list[tuple[bool, bytes]]]] = []
    for case in owned_composite_cases():
        raw, version, kind, profile = case['raw'], case['version'], case['kind'], case['wire_profile']
        parsed = parser(raw, version, kind, profile)
        spans = cast(list[dict[str, object]], parsed['spans'])
        images = {image for span in spans for image in body_mutations(span, raw)}
        words = (0,) if kind == 'CompositeFlipTurn' else (8, 12) if len(raw) >= 64 else (8,)
        for offset in words:
            images.update(raw[:offset] + struct.pack('<i', value) + raw[offset + 4:] for value in (-1, 0, 1, 2, 3, 4, 100000, 100001, 2147483647))
        images.add(raw)
        outcomes = [(bool(parser(image, version, kind, profile)['success']), image) for image in sorted(images)]
        cases.append((composite_context(kind, profile), version, raw, outcomes))
    return encode_mutation_cases(cases)
