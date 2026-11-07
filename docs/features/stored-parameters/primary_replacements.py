"""Independent literal primary replacements, using no Bessemer encoder (#202)."""
import struct


def string_replacement(text: str, raw: bytes) -> tuple[str, bytes]:
    """Change one scalar without assuming a native setter; e.g. string_replacement('abc',raw)."""
    utf16 = raw.startswith(b'\xff\xfe\xff')
    first = 'X' if not text or text[0] != 'X' else 'Y'
    if text and ord(text[0]) > 0xffff:
        first = '😀' if text[0] != '😀' else '😁'
    changed = first + text[1:] if text else first
    encoded = changed.encode('utf-16le' if utf16 else 'latin1')
    count = len(encoded) // (2 if utf16 else 1)
    word_limit = 65535 if utf16 else 65534
    length = bytes([count]) if count < 255 else b'\xff' + struct.pack('<H', count) if count < word_limit else b'\xff\xff\xff' + struct.pack('<I', count)
    return changed, (b'\xff\xfe\xff' if utf16 else b'') + length + encoded


def primary_replacement(primary: tuple[object, ...], raw: bytes) -> tuple[tuple[object, ...], bytes]:
    """Flip one numeric bit or change one string scalar with explicit framing; e.g. primary_replacement(primary,raw)."""
    kind = primary[0]
    if kind in (0, 1):
        assert raw == primary[1], 'expected exact original numeric image'
        changed = bytes([raw[0] ^ 1]) + raw[1:]
        return (kind, changed), changed
    assert kind == 2 and raw == primary[2], 'expected complete original CString frame'
    text, changed = string_replacement(primary[1], raw)
    return (2, text, changed), changed
