"""Independent ShellManager words and direct populated MFC group literals (#156)."""
from typing import TypedDict
import struct


class OwnedShellGroup(TypedDict):
    version: int
    wire_profile: str
    pattern: int
    raw: bytes
    records: list[dict[str, object]]


def shell_string_frame(text: str, wide: bool, width: int) -> bytes:
    """Construct the literal MFC frame independently of the reference reader; e.g. shell_string_frame('A', True, 1)."""
    raw = text.encode('utf-16-le' if wide else 'latin-1')
    count = len(raw) // (2 if wide else 1)
    assert width in (1, 2, 4) and (width != 1 or count < 255)
    assert width != 2 or count < 65534
    marker = b'\xff\xfe\xff' if wide else b''
    if width == 1: return marker + struct.pack('<B', count) + raw
    if width == 2: return marker + b'\xff' + struct.pack('<H', count) + raw
    return marker + b'\xff\xff\xff' + struct.pack('<I', count) + raw


def shell_record_literal(version: int, profile: str, pattern: int, ordinal: int) -> tuple[bytes, dict[str, object]]:
    """Keep known stored values distinct from destination meanings; e.g. shell_record_literal(890016, 'reader', 1, 0)."""
    texts = ['', 'A\x00B', '\xff\x00', '\u03a9\U0001f642']
    if pattern == 4: texts[1] = 'Z' * 255
    if pattern == 5: texts[1] = 'Q' * 65535
    widths = (1, 4 if pattern == 5 else 2, 4, 2)
    frames = [shell_string_frame(text, index in (1, 3), widths[index]) for index, text in enumerate(texts)]
    words = [-2147483648 + ordinal, 2147483647 - ordinal, -1, 0, 1]
    bits = (0, 0x8000000000000000, 0x7ff8000000000042, 0xfff0000000000001)[pattern % 4]
    late = -890016 - ordinal if version >= 890016 or profile == 'writer' else None
    raw = b''.join(frames) + struct.pack('<iiQiii', *words[:2], bits, *words[2:])
    if late is not None: raw += struct.pack('<i', late)
    return raw, dict(strings=texts, words=words, double_bits=struct.pack('<Q', bits).hex(), late_word=late)


def owned_shell_groups() -> list[OwnedShellGroup]:
    """Cover empty/multiple records, all length widths and both explicit late-word profiles; e.g. owned_shell_groups()."""
    cases: list[OwnedShellGroup] = []
    for version in (0, 890014, 890015, 890016, 890017, 2147483647):
        for profile, pattern in ((profile, pattern) for profile in ('reader', 'writer') for pattern in range(6)):
            count = 0 if pattern == 0 else 2 if pattern == 2 else 1
            literals = [shell_record_literal(version, profile, pattern, ordinal) for ordinal in range(count)]
            raw = struct.pack('<i', count) + b''.join(literal for literal, _ in literals)
            cases.append(OwnedShellGroup(version=version, wire_profile=profile, pattern=pattern,
                                        raw=raw, records=[record for _, record in literals]))
    return cases


def owned_shell_headers() -> list[tuple[int, list[int], bytes]]:
    """Cross every header threshold neighbor with exact signed patterns; e.g. owned_shell_headers()."""
    cases: list[tuple[int, list[int], bytes]] = []
    for version in (0, 800026, 800027, 800028, 800029, 2147483647):
        for pattern in (-2147483648, -1, 0, 1, 2147483647):
            words = [pattern, -pattern - 1] if version > 800027 else [pattern]
            raw = b''.join(struct.pack('<i', word) for word in words)
            cases.append((version, words, raw))
    return cases
