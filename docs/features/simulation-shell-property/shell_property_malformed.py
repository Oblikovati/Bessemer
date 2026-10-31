"""Complete actual shell framing mutations, including inline Material and accepted controls (#159)."""
from typing import cast
import re
import struct
from thermal_malformed import span_mutations, encode_mutation_cases
from material_adversarial import material_mutations
from owned_shell_property import OwnedShellProperty, ShellRawContext, owned_shell_properties
from shell_property_projection import shell_presence_packet
from shell_property_evidence import shell_baseline, FEATURE
from native_carriers import digest


def actual_shell_case(case: ShellRawContext, raw: bytes) -> dict[str, object]:
    """Apply exactly the supplied version/presence context, without guessing; e.g. actual_shell_case(case, case['raw'])."""
    from cosworks_shell_property_layout import parse
    from cosworks_embedded_const_surface import archive_reader
    return cast(dict[str, object], parse(raw, case['version'], 'Shell', archive_reader=archive_reader, layer_presence=case['layer_presence']))


def shell_text_span(label: str) -> bool:
    """Identify documented CwPersist text boundaries rather than byte substrings; e.g. shell_text_span('base.text78')."""
    return (label in ('base.text78', 'base.compat0', 'base.compat1', 'base.text60', 'base.text158', 'base.text160',
                      'shell.text300', 'shell.text348', 'shell.text488')
            or label.startswith('base.arraytext') or label.endswith(('.name', '.legacy_name', '.textc0'))
            or bool(re.fullmatch(r'layer\d+\.descriptor[12]\.text', label)))


def shell_word_variants(raw: bytes, offset: int, values: tuple[int, ...]) -> list[bytes]:
    """Replace only one declared four-byte control value; e.g. shell_word_variants(raw, 0, (-1, 0, 1))."""
    return [raw[:offset] + struct.pack('<i', value) + raw[offset + 4:] for value in values]


def inline_material_variants(raw: bytes, version: int, span: dict[str, object]) -> list[bytes]:
    """Keep the containing Property continuation when mutating a bounded Material body; e.g. inline_material_variants(raw, 840005, span)."""
    from cosworks_material_layout import parse
    start, end = cast(int, span['offset']), cast(int, span['end'])
    material = raw[start:end]
    parsed = parse(material, version)
    assert parsed['success'] and parsed['end'] == len(material), parsed
    images = {image for item in cast(list[dict[str, object]], parsed['spans']) for image in material_mutations(material, item)}
    return [raw[:start] + image + raw[end:] for image in sorted(images)]


def shell_span_variants(raw: bytes, version: int, span: dict[str, object]) -> list[bytes]:
    """Use the correct fixed cap and conditional body for each source control; e.g. shell_span_variants(raw, 891106, span)."""
    label, start = cast(str, span['label']), cast(int, span['offset'])
    if re.fullmatch(r'layer\d+\.material', label): return inline_material_variants(raw, version, span)
    caps = {'references.count': 10000, 'base.curve_count': 50, 'shell.layer_count': 10000}
    if label.endswith('.blob_count'): caps[label] = 1000000
    if label in caps:
        maximum = caps[label]
        return shell_word_variants(raw, start, (-1, 0, 1, maximum - 1, maximum, maximum + 1, 2147483647))
    if re.fullmatch(r'base\.virtual[01]\.body\.header', label):
        return shell_word_variants(raw, start + 16, (-1, 0, 1, 100000, 100001, 2147483647))
    controls = {'base.word2e0': (-2147483648, -101, -100, 0, 100, 101, 2147483647),
                'shell.subtype334': (-1, 0, 1, 2, 3), 'shell.word368': (-1, 0, 1, 2), 'shell.word378': (-1, 0, 1, 2)}
    if label in controls: return shell_word_variants(raw, start, controls[label])
    if label.endswith('.material_presence'): return shell_word_variants(raw, start, (-1, 0, 1, 2))
    if re.fullmatch(r'layer\d+\.flags', label): return shell_word_variants(raw, start, (-2147483648, -1, 0, 1, 2, 3, 0x80, 0x83))
    images = span_mutations(span, raw, shell_text_span)
    if label.endswith(('.archive0', '.archive1')): images.extend(shell_word_variants(raw, start, (4280, 4281, 4307, 19000)))
    return images


def complete_shell_mutant(case: OwnedShellProperty, raw: bytes) -> bool:
    """Count accepted reinterpretations only at exact EOF with full reconstruction; e.g. complete_shell_mutant(case, case['raw'])."""
    parsed = actual_shell_case(case, raw)
    if not parsed['success']: return False
    assert parsed['end'] == len(raw) and parsed['reconstruction_sha256'] == digest(raw)
    return True


def verify_shell_malformed() -> None:
    """Retain each actual accepted/rejected mutation in a compact splice corpus; e.g. verify_shell_malformed()."""
    import sys
    packet, total, accepted = bytearray(), 0, 0
    cases = owned_shell_properties()
    for case in cases:
        parsed = actual_shell_case(case, case['raw'])
        assert parsed['success'], parsed
        images = {image for span in cast(list[dict[str, object]], parsed['spans']) for image in shell_span_variants(case['raw'], case['version'], span)}
        images.add(case['raw'])
        outcomes = [(complete_shell_mutant(case, image), image) for image in sorted(images)]
        encoded, count, yes = encode_mutation_cases([(case['kind'], case['version'], case['raw'], outcomes)])
        packet.extend(shell_presence_packet(case['layer_presence']) + encoded)
        total, accepted = total + count, accepted + yes
    path = FEATURE.parents[2] / 'tests/fixtures/simulation-shell-property/malformed.bin'
    if '--write' in sys.argv: path.write_bytes(packet)
    assert path.read_bytes() == packet
    shell_baseline('malformed-reference.json', dict(contexts=len(cases),mutations=total,accepted=accepted,rejected=total-accepted,bytes=len(packet),sha256=digest(packet)))
