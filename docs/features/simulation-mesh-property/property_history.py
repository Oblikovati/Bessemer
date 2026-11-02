"""Independent literal edit and complete ordered Mesh Property provenance expectations (#163)."""
import struct
from native_carriers import digest
from property_native import actual_property
from property_projection import property_fields
from thermal_history_projection import outer_history
from thermal_nested_history import nested_copy_rows
from sheet_projection import sheet_embedded_fields


def property_history(version: int, raw: bytes) -> bytes:
    """Change only the first nonstructural signed word and observe all resulting fields and parents; e.g. property_history(891103, raw)."""
    before = actual_property(raw, version)
    assert before['success'] and before['end'] == len(raw), before
    replacement = struct.unpack_from('<i', raw)[0] ^ 32
    expected = struct.pack('<i', replacement) + raw[4:]
    after = actual_property(expected, version)
    assert after['success'] and after['end'] == len(expected), after
    assert after['reconstruction_sha256'] == digest(expected), after
    assert after['newest_keys'] == before['newest_keys'], after
    fields = property_fields(expected, version, after)
    packet = struct.pack('<IIiII', 0, version, replacement, 1, len(raw)) + raw + expected
    packet += struct.pack('<I', len(fields)) + b''.join(fields)
    return packet + outer_history(fields, True, 'mesh.word594') + nested_copy_rows(fields, sheet_embedded_fields)
