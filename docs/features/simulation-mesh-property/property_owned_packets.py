"""Complete owned Mesh Property source and primitive operation expectations (#163)."""
import re
import struct
import sys
from native_carriers import digest
from property_evidence import FEATURE, property_baseline
from property_native import actual_property
from property_literal_values import PropertyLiteralSpan, assemble_property_blocks
from property_nested_literals import nested_property_blocks
from property_collection_literals import property_nested_counts, newest_property_keys
from owned_property import OwnedMeshProperty, owned_properties, owned_property_literal
from property_projection import property_projection, property_span_shape, property_nested_shape
from property_history import property_history
from property_metrics import property_budget_packet


def verify_owned_property(case: OwnedMeshProperty) -> dict[str, object]:
    """Require all independently assembled groups and exact EOF before projecting fields; e.g. verify_owned_property(case)."""
    parsed = actual_property(case['raw'], case['version'])
    assert parsed['success'] and parsed['end'] == len(case['raw']), (case['version'], case['profile'], parsed)
    assert parsed['reconstruction_sha256'] == digest(case['raw']), parsed
    assert parsed['spans'] == case['spans'], (case['version'], case['profile'])
    keys = newest_property_keys(case['profile']) if case['version'] >= 891103 else None
    assert parsed['newest_keys'] == keys, parsed['newest_keys']
    for span in case['spans']:
        if case['null_texts'] and span['label'].startswith(('mesh.text5f', 'mesh.text600_')):
            assert case['raw'][span['offset']:span['end']] == b'\0' * 4, span
        if '.nested' in span['label']: verify_owned_property_nested(case, span)
    return parsed


def verify_owned_property_nested(case: OwnedMeshProperty, span: PropertyLiteralSpan) -> None:
    """Check every independent nested boundary against the actual prefix parser; e.g. verify_owned_property_nested(case, span)."""
    from cosworks_mesh_nested_record import parse
    match = re.fullmatch(r'mesh\.record(\d+)\.(extension2c|extension38)\.nested(\d+)', span['label'])
    assert match is not None, span
    record, extension, index = int(match[1]), match[2], int(match[3])
    counts = property_nested_counts(case['profile'], record, index, extension)
    raw, expected = assemble_property_blocks(nested_property_blocks(case['version'], case['profile'] + record + index, counts))
    assert raw == case['raw'][span['offset']:span['end']], span
    parsed = parse(raw, case['version'])
    actual = [dict(label=row['label'], offset=row['offset'], end=row['end'], sha256=digest(bytes.fromhex(row['raw_hex'])))
              for row in parsed['spans']]
    assert parsed['success'] and parsed['end'] == len(raw) and actual == expected, parsed


def save_property_packet(name: str, raw: bytes) -> dict[str, object]:
    """Require full fixture equality after explicit first initialization; e.g. save_property_packet('projection', packet)."""
    folder = FEATURE.parents[2] / 'tests/fixtures/simulation-mesh-property'
    path = folder / f'{name}.bin'
    if '--write' in sys.argv:
        folder.mkdir(parents=True, exist_ok=True)
        path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected complete unchanged owned Mesh Property packet'
    return dict(bytes=len(raw), sha256=digest(raw))


def property_input_packet(case: OwnedMeshProperty) -> bytes:
    """Retain every complete original group and independent direct-record extent; e.g. property_input_packet(case)."""
    raw = case['raw']
    spans, records = case['spans'], case['records']
    packet = struct.pack('<IIII', 0, case['version'], 1, len(raw)) + raw + struct.pack('<I', len(spans))
    packet += b''.join(struct.pack('<II', span['offset'], span['end']) for span in spans)
    return packet + struct.pack('<I', len(records)) + b''.join(struct.pack('<III', record['index'], record['start'], record['end']) for record in records)


def verify_owned_property_packets() -> None:
    """Freeze full literals and primitive fields across every gate and populated extension; e.g. verify_owned_property_packets()."""
    cases = owned_properties()
    inputs, projection, history, budgets = bytearray(), bytearray(), bytearray(), bytearray()
    for case in cases:
        parsed = verify_owned_property(case)
        inputs.extend(property_input_packet(case))
        projection.extend(property_projection(case['version'], case['raw'], parsed))
        history.extend(property_history(case['version'], case['raw']))
        budgets.extend(property_budget_packet(case['version'], case['raw'], parsed))
    references = {name: save_property_packet(name, bytes(packet))
                  for name, packet in (('input', inputs), ('projection', projection), ('history', history), ('budgets', budgets))}
    property_baseline('owned-reference.json', dict(contexts=len(cases), versions=len({case['version'] for case in cases}),
        logical_bytes=sum(len(case['raw']) for case in cases), null_text_controls=sum(case['null_texts'] for case in cases),
        direct_records=sum(len(case['records']) for case in cases),
        nested_records=sum('.nested' in span['label'] for case in cases for span in case['spans']), packets=references))


def verify_property_literal_guards() -> None:
    """Reject invalid literal contexts and unknown/incomplete projection shapes; e.g. verify_property_literal_guards()."""
    for version, profile in ((-1, 0), (True, 0), (2147483648, 0), (0, -1), (0, 5), (0, True)):
        try: owned_property_literal(version, profile)
        except AssertionError: continue
        raise AssertionError(f'{version}/{profile}; expected rejection outside explicit literal contexts')
    for null_texts in (True, 1):
        try: owned_property_literal(0, 1, null_texts)
        except AssertionError: continue
        raise AssertionError(f'{null_texts}; expected null-only text controls restricted to empty profile 0')
    controls = ((property_span_shape, 'unassigned', 4), (property_span_shape, 'unassigned.count', 4),
                (property_span_shape, 'mesh.words708_70c', 7), (property_nested_shape, 'array2.nine_raw64', 72),
                (property_nested_shape, 'fixed_eight_byte_fields', 263), (property_nested_shape, 'array0.raw64_values', 7))
    for shape, label, size in controls:
        try: shape(label, size)
        except AssertionError: continue
        raise AssertionError(f'{label}/{size}; expected rejection outside complete documented field shapes')
