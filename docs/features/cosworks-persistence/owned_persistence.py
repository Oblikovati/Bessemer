"""Independent literal framing inputs and edit splice goldens (#127)."""
import struct

PersistCase = tuple[int, int, int, int, bytes, bytes]


def word(value: int) -> bytes:
    """Encode an explicit four-byte pattern, e.g. word(-1)."""
    return struct.pack('<I', value & 0xffffffff)


def span(raw: bytes) -> bytes:
    """Frame a complete packet span, e.g. span(b'')."""
    return word(len(raw)) + raw


def literal_texts() -> list[bytes]:
    """Independent null, empty, odd, surrogate and embedded-NUL records."""
    payloads = [b'', b'\0\0', b'\0\xd8\0\0\xff\xdf\0\0']
    payloads += [bytes([value, 0, 0]) for value in range(256)]
    payloads += [bytes([value, 0xd8, 0, 0]) for value in range(256)]
    return [span(payload) for payload in payloads]


def study_inputs() -> list[bytes]:
    """Keep order, duplicates, nulls and odd payloads without semantic labels."""
    texts = literal_texts()[:5]
    sequences = [[], [texts[0]], [texts[1], texts[0], texts[1]], [texts[3], texts[2], texts[3]]]
    return [leading + word(len(items)) + b''.join(items) for leading in texts for items in sequences]


def primary_cases() -> list[PersistCase]:
    """Supply version/mode context independently of arbitrary text content."""
    cases = []
    for version in (0, 800018, 800019, 800076, 800077, 830031, 830032, 0x7fffffff):
        for full in (0, 1):
            count = int(version > 800018) + int(bool(full) and version > 800076) + int(bool(full) and version >= 830032)
            for text in literal_texts()[:5]:
                for values in ((0, 0x7fffffff, 0x80000000), (0xffffffff, 0x80000000, 0x7fffffff)):
                    suffix = b'' if full else b'\x81\0\xff'
                    raw = text + b''.join(word(value) for value in values[:count]) + suffix
                    cases.append((2, version, full, 0, raw, b''))
    return cases


def study_edit_cases(raw: bytes) -> list[PersistCase]:
    """Literal counts locate each source frame before independent byte splicing."""
    leading_end = 4 + int.from_bytes(raw[:4], 'little')
    count = int.from_bytes(raw[leading_end:leading_end + 4], 'little')
    originals = [raw[:leading_end]]
    cursor = leading_end + 4
    for _ in range(count):
        end = cursor + 4 + int.from_bytes(raw[cursor:cursor + 4], 'little')
        originals.append(raw[cursor:end])
        cursor = end
    assert cursor == len(raw)
    replacements = literal_texts()[:4]
    return [(5, 0, 0, ordinal, raw, replacement) for ordinal, original in enumerate(originals) for replacement in [original, *replacements]]


def owned_cases() -> list[PersistCase]:
    """Cover complete parents and every documented framing branch with owned inputs."""
    cases = [(0, 0, 0, 0, raw, b'') for raw in literal_texts()]
    cases += [(0, 0, 0, 3, b'\xaa\xbb\xcc' + raw + b'\xff', b'') for raw in literal_texts()]
    cases += [(1, 0, 0, 0, raw, b'') for raw in study_inputs()]
    cases += primary_cases()
    for version in (0, 1, 500070, 999999):
        for multiplier in (0, 1, 37):
            raw = f'{version:06}'.encode('utf-16-le') + b'\0\0' + bytes((index * multiplier) & 255 for index in range(114))
            cases.append((3, version, 0, 0, raw, b''))
    for raw in study_inputs():
        cases += study_edit_cases(raw)
    return cases
