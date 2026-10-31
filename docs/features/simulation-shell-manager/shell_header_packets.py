"""Complete independent header fields/history for explicit reader and current-writer profiles (#156)."""
from pathlib import Path
from typing import cast
import json
import struct
from native_carriers import RESEARCH, digest, recover_sample
from owned_shell import owned_shell_headers
from shell_literal_oracles import shell_header_spans
from thermal_history_projection import outer_history
from thermal_nested_history import nested_copy_rows


def header_primitive_fields(raw: bytes, words: list[int]) -> list[bytes]:
    """Project known signed words and documented destinations independently of Rust; e.g. header_primitive_fields(raw, [-1, 2])."""
    fields: list[bytes] = []
    spans = shell_header_spans(words)
    assert b''.join(bytes.fromhex(cast(str, span['raw_hex'])) for span in spans) == raw
    for span in spans:
        path = cast(str, span['label']).encode()
        fields.append(struct.pack('<I', len(path)) + path + struct.pack('<III', 0, cast(int, span['offset']), cast(int, span['end']))
                      + bytes.fromhex(cast(str, span['raw_hex'])))
    return fields


def require_header_words(profile: int, version: int, raw: bytes, words: list[int]) -> None:
    """Check static writer bytes using an explicit compatible-reader control; e.g. require_header_words(1, 0, raw, [-1, 2])."""
    from cosworks_shell_manager_layout import parse
    assert profile in (0, 1), f'{profile}; expected explicit reader/current-writer profile'
    control_version = version if profile == 0 else 800028
    parsed = parse(raw, control_version, 'Property')
    assert parsed['success'] and parsed['values'] == words and parsed['end'] == len(raw)
    assert parsed['reconstruction_sha256'] == digest(raw)


def header_case_packets(profile: int, version: int, raw: bytes, words: list[int]) -> tuple[bytes, bytes]:
    """Keep full field and independently re-read edited/history expectations together; e.g. header_case_packets(0, 800028, raw, words)."""
    require_header_words(profile, version, raw, words)
    fields = header_primitive_fields(raw, words)
    projection = struct.pack('<III', profile, version, len(raw)) + raw + struct.pack('<I', len(fields)) + b''.join(fields)
    replacement = words[0] ^ 32
    changed_words = [replacement, *words[1:]]
    edited = struct.pack('<i', replacement) + raw[4:]
    require_header_words(profile, version, edited, changed_words)
    edited_fields = header_primitive_fields(edited, changed_words)
    history = (struct.pack('<IIiII', profile, version, replacement, 1, len(raw)) + raw + edited
               + struct.pack('<I', len(edited_fields)) + b''.join(edited_fields)
               + outer_history(edited_fields, True, 'wordc8') + nested_copy_rows(edited_fields))
    return projection, history


def owned_header_packets() -> tuple[bytes, bytes]:
    """Cover historical reader gates and unconditional current-writer words at every supplied version; e.g. owned_header_packets()."""
    packets = [bytearray(), bytearray()]
    for version, reader_words, reader_raw in owned_shell_headers():
        writer_words = [reader_words[0], -reader_words[0] - 1]
        writer_raw = b''.join(struct.pack('<i', word) for word in writer_words)
        for profile, words, raw in ((0, reader_words, reader_raw), (1, writer_words, writer_raw)):
            fragments = header_case_packets(profile, version, raw, words)
            packets[0].extend(fragments[0])
            packets[1].extend(fragments[1])
    return bytes(packets[0]), bytes(packets[1])


def native_header_row(row: dict[str, object], streams: dict[str, bytes]) -> tuple[bytes, bytes, bytes]:
    """Keep all original accepted/failed header arguments and complete accepted projections; e.g. native_header_row(row, streams)."""
    from cosworks_shell_manager_layout import parse
    raw = streams[cast(str, row['stream'])]
    assert len(raw) == row['bytes'] and digest(raw) == row['sha256']
    packets = [bytearray(), bytearray(), bytearray()]
    for expected in cast(list[dict[str, object]], row['attempts']):
        version = cast(int, expected['version'])
        parsed = parse(raw, version, 'Property')
        assert dict(version=version, **parsed) == expected
        packets[0].extend(struct.pack('<III', version, int(cast(bool, parsed['success'])), len(raw)) + raw)
        if not parsed['success']: continue
        projection, history = header_case_packets(0, version, raw, cast(list[int], expected['values']))
        packets[1].extend(projection)
        packets[2].extend(history)
    return bytes(packets[0]), bytes(packets[1]), bytes(packets[2])


def native_header_packets(pin: Path) -> tuple[bytes, bytes, bytes]:
    """Freshly recover the 361 direct headers without conflating empty group streams; e.g. native_header_packets(pin)."""
    report = json.loads((RESEARCH / 'cosworks-shell-manager-native-evidence.json').read_bytes())
    rows = [row for row in report['rows'] if row['stream'].endswith('/ShellManager/Property')]
    assert len(rows) == 361 and sum(row['bytes'] for row in rows) == 2888
    packets = [bytearray(), bytearray(), bytearray()]
    for document in sorted({row['path'] for row in rows}):
        streams = recover_sample(document, pin)
        for row in (row for row in rows if row['path'] == document):
            fragments = native_header_row(row, streams)
            packets[0].extend(fragments[0])
            packets[1].extend(fragments[1])
            packets[2].extend(fragments[2])
    return bytes(packets[0]), bytes(packets[1]), bytes(packets[2])
