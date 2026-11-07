"""Complete group primitive values and typed scalar/CString edits, using actual frozen readers (#156)."""
from typing import TypedDict, cast
from pathlib import Path
import json
import struct
from owned_shell import OwnedShellGroup, shell_string_frame
from thermal_history_projection import outer_history
from native_carriers import RESEARCH, digest, recover_sample


class GroupPrimitive(TypedDict):
    path: str
    tag: int
    start: int
    end: int
    wire: bytes
    encoding: int
    text: str | None


def group_primitives(raw: bytes, parsed: dict[str, object]) -> list[GroupPrimitive]:
    """Preserve each actual primitive and its decoded CString representation; e.g. group_primitives(raw, parsed)."""
    assert parsed['success'] and parsed['end'] == len(raw)
    records = cast(list[dict[str, object]], parsed['records'])
    fields: list[GroupPrimitive] = []
    for span in cast(list[dict[str, object]], parsed['spans']):
        path, start, end = cast(str, span['label']), cast(int, span['offset']), cast(int, span['end'])
        wire = raw[start:end]
        tag, text = 0, None
        if '.string' in path:
            record, index = path.removeprefix('record').split('.string')
            tag, text = 3, cast(list[str], records[int(record)]['strings'])[int(index)]
        elif path.endswith('.double30'): tag = 2
        assert tag == 3 or end - start == (8 if tag == 2 else 4), path
        fields.append(GroupPrimitive(path=path, tag=tag, start=start, end=end, wire=wire,
                                    encoding=int(wire.startswith(b'\xff\xfe\xff')), text=text))
    return fields


def primitive_packet(field: GroupPrimitive) -> bytes:
    """Keep raw frame bytes beside independent decoded text/encoding metadata; e.g. primitive_packet(field)."""
    path = field['path'].encode()
    raw = struct.pack('<I', len(path)) + path + struct.pack('<III', field['tag'], field['start'], field['end']) + field['wire']
    if field['tag'] != 3: return raw
    text = cast(str, field['text']).encode()
    return raw + struct.pack('<II', field['encoding'], len(text)) + text


def group_projection(profile: int, version: int, raw: bytes, parsed: dict[str, object]) -> bytes:
    """Export every full field, offset and stored representation; e.g. group_projection(0, 890016, raw, parsed)."""
    fields = [primitive_packet(field) for field in group_primitives(raw, parsed)]
    return struct.pack('<III', profile, version, len(raw)) + raw + struct.pack('<I', len(fields)) + b''.join(fields)


def group_replacements(fields: list[GroupPrimitive]) -> list[tuple[int, bytes, str | None]]:
    """Exercise signed/raw-double/CString edits and semantic-equal prefix changes; e.g. group_replacements(fields)."""
    if len(fields) == 1: return [(0, bytes(4), None)]
    ordinals = {field['path']: index for index, field in enumerate(fields)}
    word, double, text = (ordinals['record0.' + name] for name in ('word28', 'double30', 'string1'))
    signed = struct.pack('<i', struct.unpack('<i', fields[word]['wire'])[0] ^ 32)
    bits = struct.pack('<Q', struct.unpack('<Q', fields[double]['wire'])[0] ^ 32)
    replacement = 'edited\x00\u03a9\U0001f642'
    original = cast(str, fields[text]['text'])
    count = len(original.encode('utf-16-le')) // 2
    width = 1 if count < 255 else 2 if count < 65535 else 4
    mode_text = ordinals['record0.string2']
    mode_original = cast(str, fields[mode_text]['text'])
    return [(word, signed, None), (double, bits, None),
            (text, shell_string_frame(replacement, True, 1), replacement),
            (text, shell_string_frame(original, True, width), original),
            (mode_text, shell_string_frame(mode_original, True, 1), mode_original)]


def replacement_packet(field: GroupPrimitive, wire: bytes, text: str | None) -> bytes:
    """Encode the complete caller replacement, including CString encoding/text; e.g. replacement_packet(field, wire, text)."""
    raw = struct.pack('<II', field['tag'], len(wire)) + wire
    if field['tag'] != 3: return raw
    assert text is not None
    encoded = text.encode()
    return raw + struct.pack('<II', 1, len(encoded)) + encoded


def require_edited_records(before: dict[str, object], after: dict[str, object], field: GroupPrimitive, wire: bytes, text: str | None) -> None:
    """Check known record values independently of parser reconstruction hashes; e.g. require_edited_records(before, after, field, wire, text)."""
    records = [dict(record) for record in cast(list[dict[str, object]], before['records'])]
    if field['path'] == 'record0.word28':
        words = list(cast(list[int], records[0]['words']))
        words[0] = struct.unpack('<i', wire)[0]
        records[0]['words'] = words
    elif field['path'] == 'record0.double30': records[0]['double_bits'] = wire.hex()
    elif field['path'].startswith('record0.string'):
        texts = list(cast(list[str], records[0]['strings']))
        texts[int(field['path'].rsplit('string', 1)[1])] = cast(str, text)
        records[0]['strings'] = texts
    else: assert field['path'] == 'count' and not records and wire == bytes(4)
    assert after['success'] and after['records'] == records


def group_edit_packet(profile: int, version: int, raw: bytes, parsed: dict[str, object], requested: tuple[int, bytes, str | None]) -> bytes:
    """Keep every source/replacement/result field parent for one exact typed edit; e.g. group_edit_packet(0, 890016, raw, parsed, requested)."""
    from cosworks_shell_group_layout import parse
    ordinal, wire, text = requested
    field = group_primitives(raw, parsed)[ordinal]
    edited = raw[:field['start']] + wire + raw[field['end']:]
    after = parse(edited, version, wire_profile='writer' if profile else 'reader')
    require_edited_records(parsed, after, field, wire, text)
    fields = [primitive_packet(value) for value in group_primitives(edited, after)]
    changed = wire != field['wire']
    return (struct.pack('<III', profile, version, ordinal) + replacement_packet(field, wire, text)
            + struct.pack('<III', int(changed), len(raw), len(edited)) + raw + edited
            + struct.pack('<I', len(fields)) + b''.join(fields) + outer_history(fields, changed, field['path']) + bytes(4))


def group_case_packets(profile: int, version: int, raw: bytes) -> tuple[bytes, bytes, bytes]:
    """Run one complete explicit profile and derive all typed-edit/resource expectations; e.g. group_case_packets(0, 890016, raw)."""
    from cosworks_shell_group_layout import parse
    parsed = parse(raw, version, wire_profile='writer' if profile else 'reader')
    fields = group_primitives(raw, parsed)
    histories = b''.join(group_edit_packet(profile, version, raw, parsed, replacement) for replacement in group_replacements(fields))
    text_bytes = sum(len(cast(str, field['text']).encode('utf-16-le' if field['encoding'] else 'latin-1')) for field in fields if field['tag'] == 3)
    budgets = struct.pack('<III', profile, version, len(raw)) + raw + struct.pack('<IIII', len(raw), len(fields), len(cast(list[object], parsed['records'])), text_bytes)
    return group_projection(profile, version, raw, parsed), histories, budgets


def owned_group_packets(cases: list[OwnedShellGroup]) -> tuple[bytes, bytes, bytes]:
    """Retain all empty/populated reader/writer contexts, including extended CString frames; e.g. owned_group_packets(cases)."""
    packets = [bytearray(), bytearray(), bytearray()]
    for case in cases:
        profile = int(case['wire_profile'] == 'writer')
        for packet, fragment in zip(packets, group_case_packets(profile, case['version'], case['raw']), strict=True):
            packet.extend(fragment)
    return bytes(packets[0]), bytes(packets[1]), bytes(packets[2])


def native_group_row(row: dict[str, object], streams: dict[str, bytes]) -> tuple[bytes, bytes, bytes, bytes]:
    """Keep every original group argument before projecting complete fields/no-op histories; e.g. native_group_row(row, streams)."""
    from cosworks_shell_group_layout import parse
    raw = streams[cast(str, row['stream'])]
    assert len(raw) == row['bytes'] and digest(raw) == row['sha256']
    packets = [bytearray(), bytearray(), bytearray(), bytearray()]
    for expected in cast(list[dict[str, object]], row['attempts']):
        version = cast(int, expected['version'])
        parsed = parse(raw, version)
        assert dict(version=version, **parsed) == expected
        packets[0].extend(struct.pack('<III', version, int(cast(bool, parsed['success'])), len(raw)) + raw)
        if not parsed['success']: continue
        fragments = group_case_packets(0, version, raw)
        for packet, fragment in zip(packets[1:], fragments, strict=True): packet.extend(fragment)
    return bytes(packets[0]), bytes(packets[1]), bytes(packets[2]), bytes(packets[3])


def native_group_packets(pin: Path) -> tuple[bytes, bytes, bytes, bytes]:
    """Freshly recover all 69 native empty groups without inventing populated evidence; e.g. native_group_packets(pin)."""
    report = json.loads((RESEARCH / 'cosworks-shell-group-native-evidence.json').read_bytes())
    rows = cast(list[dict[str, object]], report['rows'])
    assert len(rows) == 69 and sum(cast(int, row['bytes']) for row in rows) == 276
    packets = [bytearray(), bytearray(), bytearray(), bytearray()]
    for document in sorted({cast(str, row['path']) for row in rows}):
        streams = recover_sample(document, pin)
        fragments = [native_group_row(row, streams) for row in rows if row['path'] == document]
        for index, packet in enumerate(packets): packet.extend(b''.join(fragment[index] for fragment in fragments))
    return bytes(packets[0]), bytes(packets[1]), bytes(packets[2]), bytes(packets[3])
