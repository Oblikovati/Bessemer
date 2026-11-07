"""Complete projections from the actual frozen reader and independent edit splices."""
from cosworkspersist import TextRecord, StudyOrder, PrimaryVersion, LegacyVersion
from cosworkspersist import read_text, read_study_order, read_primary_version, read_legacy_version, select_version_stream
from owned_persistence import PersistCase, word, span


def coverage(source: int, consumed: int, opaque: int, incomplete: bool) -> bytes:
    """Encode exact byte scope and explicit non-native diagnostics, e.g. coverage(4, 4, 0, False)."""
    return b''.join(word(value) for value in (source, consumed, opaque, int(incomplete), 0, 7))


def text_projection(parent: bytes, text: TextRecord) -> bytes:
    """Compare every frame boundary, raw view and complete parent."""
    has_units = not text.is_null and len(text.payload) % 2 == 0
    units = text.payload[:-2] if has_units else b''
    assert not has_units or text.text.encode('utf-16-le', 'surrogatepass') == units
    fields = (text.offset, text.offset + 4, text.end, text.declared_bytes, int(text.is_null), int(has_units))
    packet = span(parent) + span(text.header) + span(text.payload)
    packet += b''.join(word(value) for value in fields) + span(units)
    framed = text.end - text.offset
    return packet + coverage(len(parent), framed, len(parent) - framed, False)


def study_projection(raw: bytes, record: StudyOrder) -> bytes:
    """Serialize every complete stored text with count and EOF boundaries."""
    packet = span(raw) + word(record.count_offset) + word(len(record.items)) + span(record.count_header)
    for text in (record.leading, *record.items):
        packet += span(text_projection(raw, text))
    return packet + coverage(len(raw), record.end, 0, False)


def primary_projection(raw: bytes, record: PrimaryVersion, full: int) -> bytes:
    """Retain consumed/unassigned words, all owner updates and the complete suffix."""
    packet = span(raw) + span(text_projection(raw, record.text))
    packet += b''.join(word(value) for value in (record.numeric_version, full, record.end, len(record.words)))
    packet += span(record.word_payload) + span(b''.join(word(value) for value in record.words)) + span(record.remainder)
    packet += word(len(record.owner_updates))
    for destination, value in record.owner_updates:
        packet += word(destination) + word(value)
    return packet + coverage(len(raw), record.end, len(record.remainder), bool(record.remainder))


def legacy_projection(raw: bytes, record: LegacyVersion) -> bytes:
    """Keep the complete six-digit prefix and every opaque suffix bit."""
    packet = span(raw) + span(record.prefix) + span(record.opaque_suffix)
    packet += word(record.numeric_version) + span(record.prefix[:12])
    return packet + coverage(len(raw), 128, 114, False)


def edit_projection(raw: bytes, ordinal: int, replacement: bytes) -> bytes:
    """Splice independent original byte ranges and check every resulting text."""
    original = read_study_order(raw)
    texts = (original.leading, *original.items)
    requested = texts[ordinal]
    new_bytes = raw[:requested.offset] + replacement + raw[requested.end:]
    result = read_study_order(new_bytes)
    new_texts = (result.leading, *result.items)
    changed = new_bytes != raw
    packet = span(new_bytes) + span(study_projection(new_bytes, result)) + word(int(changed)) + word(len(texts))
    for index, (before, after) in enumerate(zip(texts, new_texts, strict=True)):
        derived = changed and index == ordinal
        expected = replacement if derived else before.header + before.payload
        assert after.header + after.payload == expected
        packet += word(index) + word(int(derived)) + span(before.header + before.payload) + span(expected)
    return packet + coverage(len(new_bytes), len(new_bytes), 0, False)


def project(case: PersistCase) -> bytes:
    """Dispatch explicit corpus contexts without inferring version or carrier availability."""
    kind, numeric, full, offset, raw, extra = case
    if kind == 0:
        return text_projection(raw, read_text(raw, offset))
    if kind == 1:
        return study_projection(raw, read_study_order(raw))
    if kind == 2:
        return primary_projection(raw, read_primary_version(raw, numeric, full_read=bool(full)), full)
    if kind == 3:
        return legacy_projection(raw, read_legacy_version(raw))
    if kind == 4:
        selected = select_version_stream(raw, extra, primary_numeric_version=numeric, full_read=bool(full))
        assert selected.source == 'primary'
        return word(1) + primary_projection(raw, selected.record, full)
    assert kind == 5
    return edit_projection(raw, offset, extra)


def case_packet(case: PersistCase) -> bytes:
    """Retain every caller input beside the complete exact reference result."""
    kind, numeric, full, offset, raw, extra = case
    return b''.join(word(value) for value in (kind, numeric, full, offset)) + span(raw) + span(extra) + span(project(case))


def packet(cases: list[PersistCase]) -> bytes:
    """Produce the deterministic full projection packet, e.g. packet(cases)."""
    return b'CPST' + word(len(cases)) + b''.join(case_packet(case) for case in cases)
