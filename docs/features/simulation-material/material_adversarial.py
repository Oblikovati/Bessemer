"""Complete actual mutation acceptance and independent Material resource limits (#140)."""
import struct
from typing import cast
from owned_material import owned_material_cases
from material_reference import actual_material_parser
from material_projection import material_shape, material_text_label
from thermal_projection import projected_fields
from thermal_malformed import count_mutations, span_mutations, encode_mutation_cases


def material_mutations(raw: bytes, span: dict[str, object]) -> list[bytes]:
    """Exercise control counts and accepted raw text/presence alternatives; e.g. material_mutations(raw, span)."""
    label = cast(str, span['label'])
    start, end = cast(int, span['offset']), cast(int, span['end'])
    images = span_mutations(span, raw, material_text_label)
    if label == 'property_count': images.extend(count_mutations(label, start, raw))
    if label == 'property_count' or label.endswith(('.count', '.pair_count')):
        images.extend(raw[:start] + struct.pack('<i', count) + raw[start + 4:] for count in (0, 1, 9999, 10000))
    if label.endswith('.curve_present'):
        images.extend(raw[:start] + struct.pack('<i', value) + raw[start + 4:] for value in (-1, 0, 1, 2))
    if material_text_label(label):
        frames = (bytes(4), struct.pack('<I', 2) + bytes(2), struct.pack('<I', 3) + b'A\0\0')
        images.extend(raw[:start] + frame + raw[end:] for frame in frames)
    return images


def material_mutation_packet() -> tuple[bytes, int, int]:
    """Retain all actual outcomes, including accepted reinterpretations; e.g. material_mutation_packet()."""
    cases: list[tuple[int, int, bytes, list[tuple[bool, bytes]]]] = []
    for version, raw in owned_material_cases():
        parsed = actual_material_parser(raw, version)
        assert parsed['success']
        spans = cast(list[dict[str, object]], parsed['spans'])
        images = {image for span in spans for image in material_mutations(raw, span)}
        outcomes = [(cast(bool, actual_material_parser(image, version)['success']), image) for image in sorted(images)]
        cases.append((0, version, raw, outcomes))
    return encode_mutation_cases(cases)


def material_budget_metrics(raw: bytes, spans: list[dict[str, object]]) -> tuple[int, int, int]:
    """Count declared entries and every physical field without consulting Rust counters; e.g. material_budget_metrics(raw, spans)."""
    entries, text = 0, 0
    for span in spans:
        label, start = cast(str, span['label']), cast(int, span['offset'])
        if label == 'property_count' or label.endswith(('.count', '.pair_count')):
            entries += struct.unpack_from('<i', raw, start)[0]
        if material_text_label(label):
            text = max(text, struct.unpack_from('<I', raw, start)[0])
    fields = len(projected_fields(raw, spans, material_shape))
    return entries, text, fields


def material_budget_packet() -> bytes:
    """Freeze independent exact standalone/prefix limits for every owned profile; e.g. material_budget_packet()."""
    packet = bytearray()
    for version, raw in owned_material_cases():
        parsed = actual_material_parser(raw, version)
        assert parsed['success']
        metrics = material_budget_metrics(raw, cast(list[dict[str, object]], parsed['spans']))
        packet.extend(struct.pack('<III', 0, version, len(raw)) + raw)
        packet.extend(struct.pack('<III', *metrics))
    return bytes(packet)
