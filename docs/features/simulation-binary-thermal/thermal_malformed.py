"""Independent malformed images located through actual reference spans (#134)."""
import struct
from typing import cast
from owned_thermal import owned_thermal_cases, THERMAL_KINDS


def count_mutations(label: str, offset: int, raw: bytes) -> list[bytes]:
    """Exercise negative and excessive declared counts; e.g. count_mutations('collection.count', 0, raw)."""
    maximum = 50 if label.endswith('.table.count') else 1000000 if label.endswith('.blob_count') else 10000
    return [raw[:offset] + struct.pack('<i', value) + raw[offset + 4:] for value in (-1, maximum + 1, 0x7fffffff)]


def is_text_span(label: str) -> bool:
    """Identify documented persisted text groups, not arbitrary byte substrings; e.g. is_text_span('record0.base.text0')."""
    return ('.base.text' in label or '.base.array_text' in label or '.thermal.text' in label
            or label.endswith(('.name', '.legacy_name', '.base.158', '.base.160')))


def span_mutations(span: dict[str, object], raw: bytes) -> list[bytes]:
    """Target actual count, mode, optional and text boundaries; e.g. span_mutations(span, raw)."""
    label, offset, end = cast(str, span['label']), cast(int, span['offset']), cast(int, span['end'])
    if label.endswith(('.count', '.blob_count', '.pair_count')):
        return count_mutations(label, offset, raw)
    if '.optional' in label and label.endswith('.header'):
        return [raw[:offset + 16] + struct.pack('<i', value) + raw[offset + 20:] for value in (-1, 100001, 0x7fffffff)]
    if label.endswith('.table.mode'):
        return [raw[:offset] + struct.pack('<i', value) + raw[offset + 4:] for value in (-101, 101)]
    if is_text_span(label):
        images = [raw[:offset] + struct.pack('<I', count) + raw[offset + 4:] for count in (1, 0xffffffff)]
        if end > offset + 4: images.append(raw[:end - 1] + b'\x7f' + raw[end:])
        return images
    if label.endswith(('.archive0', '.archive1')):
        return [raw[:offset] + struct.pack('<I', version) + raw[offset + 4:] for version in (0, 4306, 19001, 0xffffffff)]
    return []


def malformed_thermal_cases() -> list[tuple[int, int, bytes, list[tuple[bool, bytes]]]]:
    """Replay every targeted mutation on the actual complete callback; e.g. malformed_thermal_cases()."""
    from cosworks_binary_thermal_transform_audit import parse_native, archive_reader
    cases: list[tuple[int, int, bytes, list[tuple[bool, bytes]]]] = []
    for kind, version, raw in owned_thermal_cases():
        parsed = parse_native(raw, version, THERMAL_KINDS[kind], archive_reader)
        assert parsed['success']
        images = {image for span in parsed['spans'] for image in span_mutations(span, raw)}
        outcomes = [(parse_native(image, version, THERMAL_KINDS[kind], archive_reader)['success'], image)
                    for image in sorted(images)]
        cases.append((kind, version, raw, outcomes))
    return cases


def source_budget_metrics(raw: bytes, spans: list[dict[str, object]], archives: list[dict[str, object]]) -> tuple[int, int, int, int]:
    """Compute aggregate entries and exact text/root budgets from actual independent boundaries; e.g. source_budget_metrics(raw, spans, archives)."""
    entries, text_max = 0, 0
    for span in spans:
        label, offset = cast(str, span['label']), cast(int, span['offset'])
        if label.endswith(('.count', '.pair_count')):
            entries += struct.unpack_from('<i', raw, offset)[0]
        if '.optional' in label and label.endswith('.header'):
            entries += struct.unpack_from('<i', raw, offset + 16)[0]
        if is_text_span(label): text_max = max(text_max, struct.unpack_from('<I', raw, offset)[0])
    largest_root = max((cast(int, item['end']) - cast(int, item['offset']) for item in archives), default=0)
    objects = max((sum(cast(dict[str, int], item['classes']).values()) for item in archives), default=0)
    return entries, text_max, largest_root, objects


def thermal_budget_cases() -> list[tuple[int, int, bytes, tuple[int, int, int, int]]]:
    """Preserve exact caller-budget expectations without reading Rust counters; e.g. thermal_budget_cases()."""
    from cosworks_binary_thermal_transform_audit import parse_native, archive_reader
    cases: list[tuple[int, int, bytes, tuple[int, int, int, int]]] = []
    for kind, version, raw in owned_thermal_cases():
        parsed = parse_native(raw, version, THERMAL_KINDS[kind], archive_reader)
        assert parsed['success']
        metrics = source_budget_metrics(raw, parsed['spans'], parsed['archive_records'])
        cases.append((kind, version, raw, metrics))
    return cases


def malformed_packet() -> tuple[bytes, int, int]:
    """Retain reference acceptance for every mutant; e.g. malformed_packet()."""
    cases = malformed_thermal_cases()
    packet = bytearray()
    count, accepted_count = 0, 0
    for kind, version, raw, outcomes in cases:
        packet.extend(struct.pack('<III', kind, version, len(raw)) + raw + struct.pack('<I', len(outcomes)))
        for accepted, image in outcomes:
            packet.extend(struct.pack('<I', int(accepted)) + mutation_splice(raw, image))
            count += 1
            accepted_count += accepted
    return bytes(packet), count, accepted_count


def mutation_splice(raw: bytes, image: bytes) -> bytes:
    """Store exact mutations without repeating whole reference inputs; e.g. mutation_splice(raw, image)."""
    start = next((index for index, pair in enumerate(zip(raw, image)) if pair[0] != pair[1]), min(len(raw), len(image)))
    suffix = 0
    while suffix < min(len(raw), len(image)) - start and raw[len(raw) - suffix - 1] == image[len(image) - suffix - 1]:
        suffix += 1
    replacement = image[start:len(image) - suffix]
    removed = len(raw) - suffix - start
    assert raw[:start] + replacement + raw[start + removed:] == image
    return struct.pack('<III', start, removed, len(replacement)) + replacement


def budget_packet() -> bytes:
    """Keep exact reference budgets beside complete inputs; e.g. budget_packet()."""
    return b''.join(struct.pack('<III', kind, version, len(raw)) + raw + struct.pack('<IIII', *metrics)
                    for kind, version, raw, metrics in thermal_budget_cases())
