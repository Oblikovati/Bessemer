"""Actual accepted/rejected MeshControl grammar mutations over every owned context (#161)."""
from typing import cast
import sys
from native_carriers import digest
from thermal_malformed import span_mutations, encode_mutation_cases
from shell_property_malformed import shell_span_variants, shell_text_span, shell_word_variants
from mesh_control_evidence import FEATURE, mesh_baseline
from mesh_record_packets import actual_mesh_records
from owned_mesh_records import OwnedMeshRecords, owned_mesh_records


def mesh_mutation_text(label: str) -> bool:
    """Include all derived texts alongside the documented shared-base frames; e.g. mesh_mutation_text('mesh.text318')."""
    return shell_text_span(label) or label in ('mesh.text2e8', 'mesh.text2f0', 'mesh.text310', 'mesh.text318')


def mesh_span_variants(raw: bytes, version: int, span: dict[str, object]) -> list[bytes]:
    """Reuse exact base controls and retain the collection/record continuation; e.g. mesh_span_variants(raw, version, span)."""
    label, offset = cast(str, span['label']), cast(int, span['offset'])
    if label == 'mesh_control.count':
        return shell_word_variants(raw, offset, (-1, 0, 1, 9999, 10000, 10001, 2147483647))
    if label.startswith(('references.', 'reference', 'base.')):
        return shell_span_variants(raw, version, span)
    return span_mutations(span, raw, mesh_mutation_text)


def complete_mesh_mutant(case: OwnedMeshRecords, raw: bytes) -> bool:
    """Keep accepted reinterpretations only at exact EOF with identical full bytes; e.g. complete_mesh_mutant(case, raw)."""
    parsed = actual_mesh_records(raw, case['version'])
    if not parsed['success']: return False
    assert parsed['end'] == len(raw) and parsed['reconstruction_sha256'] == digest(raw), parsed
    return True


def verify_mesh_record_mutations() -> None:
    """Freeze every actual outcome rather than treating every changed control as invalid; e.g. verify_mesh_record_mutations()."""
    packet, total, accepted = bytearray(), 0, 0
    cases = owned_mesh_records()
    for case in cases:
        parsed = actual_mesh_records(case['raw'], case['version'])
        assert parsed['success'], parsed
        images = {image for span in cast(list[dict[str, object]], parsed['spans'])
                  for image in mesh_span_variants(case['raw'], case['version'], span)}
        images.add(case['raw'])
        outcomes = [(complete_mesh_mutant(case, image), image) for image in sorted(images)]
        encoded, count, yes = encode_mutation_cases([(0, case['version'], case['raw'], outcomes)])
        packet.extend(encoded)
        total, accepted = total + count, accepted + yes
    path = FEATURE.parents[2] / 'tests/fixtures/simulation-mesh-control/records-malformed.bin'
    if '--write' in sys.argv: path.write_bytes(packet)
    assert path.read_bytes() == packet, f'{path}; expected every complete original mutation outcome'
    mesh_baseline('records-malformed-reference.json', dict(contexts=len(cases), mutations=total, accepted=accepted,
                  rejected=total - accepted, bytes=len(packet), sha256=digest(packet)))
