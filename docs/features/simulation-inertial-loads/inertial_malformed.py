"""Independent malformed images and exact source budgets for complete inertial loads (#136)."""
import re
import struct
from typing import cast
from owned_inertial import LOAD_KINDS, owned_inertial_cases
from inertial_edit_oracle import actual_inertial_parser
from inertial_projection import inertial_shape
from thermal_projection import projected_fields
from thermal_malformed import span_mutations, is_text_span, source_budget_metrics, encode_mutation_cases
from embedded_field_projection import embedded_field_case


def is_load_text(label: str) -> bool:
    """Identify actual common and derived persisted text groups; e.g. is_load_text('record0.gravity.text0')."""
    return is_text_span(label) or re.search(r'\.(gravity|centrifugal|excitation)\.text\d+$', label) is not None


def inertial_mutation_packet() -> tuple[bytes, int, int]:
    """Retain every actual reference outcome, including any accepted reinterpretation; e.g. inertial_mutation_packet()."""
    cases: list[tuple[int, int, bytes, list[tuple[bool, bytes]]]] = []
    for kind, version, raw in owned_inertial_cases():
        parsed = actual_inertial_parser(raw, version, LOAD_KINDS[kind])
        assert parsed['success']
        spans = cast(list[dict[str, object]], parsed['spans'])
        images = {image for span in spans for image in span_mutations(span, raw, is_load_text)}
        outcomes = [(cast(bool, actual_inertial_parser(image, version, LOAD_KINDS[kind])['success']), image)
                    for image in sorted(images)]
        cases.append((kind, version, raw, outcomes))
    return encode_mutation_cases(cases)


def maximum_embedded_fields(raw: bytes, archives: list[dict[str, object]]) -> int:
    """Derive per-root field limits from observed original source calls; e.g. maximum_embedded_fields(raw, archives)."""
    counts: list[int] = []
    for archive in archives:
        image = raw[cast(int, archive['offset']):cast(int, archive['end'])]
        packet = embedded_field_case(image)
        counts.append(struct.unpack_from('<I', packet, 4 + len(image))[0])
    return max(counts, default=0)


def inertial_budget_packet() -> bytes:
    """Freeze exact outer and nested budgets independently of Rust counters; e.g. inertial_budget_packet()."""
    packet = bytearray()
    for kind, version, raw in owned_inertial_cases():
        parsed = actual_inertial_parser(raw, version, LOAD_KINDS[kind])
        assert parsed['success']
        spans = cast(list[dict[str, object]], parsed['spans'])
        archives = cast(list[dict[str, object]], parsed['archive_records'])
        metrics = source_budget_metrics(raw, spans, archives, is_load_text)
        fields = len(projected_fields(raw, spans, inertial_shape))
        embedded_fields = maximum_embedded_fields(raw, archives)
        packet.extend(struct.pack('<III', kind, version, len(raw)) + raw)
        packet.extend(struct.pack('<IIIIII', *metrics, fields, embedded_fields))
    return bytes(packet)
