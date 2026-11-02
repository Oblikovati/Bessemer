"""Independent complete first-word edits and ordered occurrence parents (#164)."""
import struct
from thermal_history_projection import outer_history
from results_projection import results_fields


def results_history(level: int, version: int, raw: bytes) -> bytes:
    """Reparse the literal edited image and retain all copied/derived parents; e.g. results_history(1, 810010, raw)."""
    replacement = struct.unpack_from('<i', raw)[0] ^ 32
    edited = struct.pack('<i', replacement) + raw[4:]
    fields = results_fields(level, version, edited)
    path = ('member58', 'word10', 'temporary_word_and_member28[0]')[level]
    return (struct.pack('<IIiII', level, version, replacement, 1, len(raw)) + raw + edited
            + struct.pack('<I', len(fields)) + b''.join(fields)
            + outer_history(fields, True, path) + struct.pack('<I', 0))
