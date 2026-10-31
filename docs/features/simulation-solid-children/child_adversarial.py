"""Actual component framing mutations and independent literal resource measures (#149)."""
import struct
from typing import cast
from owned_children import owned_component_headers, child_versions, folder_fields
from thermal_malformed import span_mutations, encode_mutation_cases


def component_text(label: str) -> bool:
    """Identify the actual single counted frame; e.g. component_text('text68')."""
    return label == 'text68'


def component_mutations() -> tuple[bytes, int, int]:
    """Keep every actual accepted/rejected mutation including unchanged controls; e.g. component_mutations()."""
    from cosworks_solid_component_layout import parse
    cases: list[tuple[int, int, bytes, list[tuple[bool, bytes]]]] = []
    for _, version, raw in owned_component_headers():
        parsed = parse(raw, version)
        assert parsed['success']
        images = {image for span in cast(list[dict[str, object]], parsed['spans']) for image in span_mutations(span, raw, component_text)}
        images.add(raw)
        outcomes = [(cast(bool, parse(image, version)['success']), image) for image in sorted(images)]
        cases.append((0, version, raw, outcomes))
    return encode_mutation_cases(cases)


def child_budgets() -> bytes:
    """Measure component calls and static folder vectors without Rust-derived requirements; e.g. child_budgets()."""
    from cosworks_solid_component_layout import parse
    from cosworkspersist import read_text
    packet = bytearray()
    for _, version, raw in owned_component_headers():
        parsed = parse(raw, version)
        text = read_text(raw)
        packet.extend(struct.pack('<III', 0, version, len(raw)) + raw)
        packet.extend(struct.pack('<III', 0, len(text.payload), len(parsed['spans'])))
    for version in child_versions():
        for profile in range(4):
            fields = folder_fields(version, profile)
            raw = b''.join(value for _, _, value in fields)
            text_sizes = [len(value) - len(read_text(value).header) for _, tag, value in fields if tag == 3]
            entries = sum(label.startswith('folder.list68.') and label != 'folder.list68.count' for label, _, _ in fields)
            packet.extend(struct.pack('<III', 1, version, len(raw)) + raw)
            packet.extend(struct.pack('<III', entries, max(text_sizes), len(fields)))
    return bytes(packet)
