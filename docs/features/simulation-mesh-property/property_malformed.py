"""Actual complete Mesh Property grammar mutation outcomes, including scoped nested controls (#163)."""
from typing import cast
import re
import struct
import sys
from native_carriers import digest
from thermal_malformed import span_mutations, encode_mutation_cases
from shell_property_malformed import shell_word_variants
from property_evidence import FEATURE, property_baseline
from property_native import actual_property
from property_projection import property_count_shape
from owned_property import owned_properties


def property_mutation_spans(raw: bytes, version: int, spans: list[dict[str, object]]) -> list[dict[str, object]]:
    """Expand nested control offsets without treating raw64 contents as count words; e.g. property_mutation_spans(raw, version, spans)."""
    from cosworks_mesh_nested_record import parse
    expanded = list(spans)
    for span in spans:
        label = cast(str, span['label'])
        if not re.fullmatch(r'mesh\.record\d+\.extension(?:2c|38)\.nested\d+', label): continue
        parsed = parse(raw, version, offset=span['offset'], require_eof=False)
        assert parsed['success'] and parsed['end'] == span['end'], parsed
        expanded.extend(dict(row, label=label + '.' + row['label']) for row in parsed['spans'])
    return expanded


def property_control_maximum(label: str) -> int | None:
    """Use each documented cap, including both independent 100000-entry nested arrays; e.g. property_control_maximum('mesh.duration_count')."""
    if label.endswith('.blob_count'): return 1000000
    if re.fullmatch(r'mesh\.record\d+\.extension(?:2c|38)\.nested\d+\.array[01]\.count', label): return 100000
    if property_count_shape(label) and (label.endswith(('.count', '.count0', '.count1', '.child_count')) or label == 'mesh.duration_count'): return 10000
    if label == 'mesh.settings.references.count': return 10000
    return None


def property_mutation_text(label: str) -> bool:
    """Recognize every counted text frame, excluding text-list counts; e.g. property_mutation_text('mesh.text5f0')."""
    return bool(re.fullmatch(r'mesh\.(?:text5f[08]|text600_[0-3]|text_list\.\d+)', label)) or label.endswith(('.name', '.legacy_name'))


def property_span_mutations(raw: bytes, span: dict[str, object]) -> list[bytes]:
    """Keep the parent continuation intact while replacing actual grammar controls; e.g. property_span_mutations(raw, span)."""
    label, start = cast(str, span['label']), cast(int, span['offset'])
    maximum = property_control_maximum(label)
    if maximum is not None: return shell_word_variants(raw, start, (-1, 0, 1, maximum - 1, maximum, maximum + 1, 2147483647))
    if re.fullmatch(r'mesh\.record\d+\.extension(?:2c|38)\.mode', label):
        return shell_word_variants(raw, start, (-2147483648, -1, 0, 1, 2, 2147483647))
    images = span_mutations(span, raw, property_mutation_text)
    if label.endswith(('.archive0', '.archive1')):
        images.extend(shell_word_variants(raw, start, (4280, 4281, 4307, 19000)))
        images.extend(raw[:start + 4] + struct.pack('<H', tag) + raw[start + 6:] for tag in (0, 0x7fff, 0x8000, 0xffff))
    return images


def complete_property_mutant(raw: bytes, version: int) -> bool:
    """Retain accepted reinterpretations only at exact EOF and full reconstruction; e.g. complete_property_mutant(raw, version)."""
    parsed = actual_property(raw, version)
    if not parsed['success']: return False
    assert parsed['end'] == len(raw) and parsed['reconstruction_sha256'] == digest(raw), parsed
    return True


def verify_property_mutations() -> None:
    """Freeze all actual outcomes rather than assuming every mutated control fails; e.g. verify_property_mutations()."""
    cases = owned_properties()
    packet, total, accepted = bytearray(), 0, 0
    for case in cases:
        spans = property_mutation_spans(case['raw'], case['version'], cast(list[dict[str, object]], case['spans']))
        images = {image for span in spans for image in property_span_mutations(case['raw'], span)}
        images.add(case['raw'])
        outcomes = [(complete_property_mutant(image, case['version']), image) for image in sorted(images)]
        encoded, count, yes = encode_mutation_cases([(0, case['version'], case['raw'], outcomes)])
        packet.extend(encoded)
        total, accepted = total + count, accepted + yes
    path = FEATURE.parents[2] / 'tests/fixtures/simulation-mesh-property/malformed.bin'
    if '--write' in sys.argv: path.write_bytes(packet)
    assert path.read_bytes() == packet, f'{path}; expected complete unchanged actual grammar outcomes'
    property_baseline('malformed-reference.json', dict(contexts=len(cases), mutations=total, accepted=accepted,
                      rejected=total - accepted, bytes=len(packet), sha256=digest(packet)))
