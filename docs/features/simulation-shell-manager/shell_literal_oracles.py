"""Independent primitive spans and isolated scalar expectations for ShellManager (#156)."""
from typing import cast
import struct
from owned_shell import OwnedShellGroup, shell_string_frame


def shell_header_spans(words: list[int]) -> list[dict[str, object]]:
    """Retain each documented destination separately from its physical position; e.g. shell_header_spans([-1, 2])."""
    assert len(words) in (1, 2), f'{words}; expected one or two complete signed header words'
    return [dict(label=label, offset=index * 4, end=(index + 1) * 4, raw_hex=struct.pack('<i', word).hex())
            for index, (label, word) in enumerate(zip(('wordc8', 'word10c')[:len(words)], words, strict=True))]


def shell_header_edit(words: list[int]) -> tuple[bytes, list[int], list[dict[str, object]]]:
    """Change one mandatory header word with independent surrounding-byte expectations; e.g. shell_header_edit([-1, 2])."""
    replacement = [314159, *words[1:]]
    raw = b''.join(struct.pack('<i', word) for word in replacement)
    history = [dict(label=span['label'], result_ordinal=index, relation='derived' if index == 0 else 'copied',
               parents=[dict(owner='source', ordinal=index)] + ([dict(owner='replacement', ordinal=None)] if index == 0 else []))
               for index, span in enumerate(shell_header_spans(words))]
    return raw, replacement, history


def shell_record_segments(case: OwnedShellGroup, record: dict[str, object]) -> list[tuple[str, bytes]]:
    """Rebuild named primitive pieces from known literal values; e.g. shell_record_segments(case, record)."""
    texts = cast(list[str], record['strings'])
    widths = (1, 4 if case['pattern'] == 5 else 2, 4, 2)
    segments = [(f'string{index}', shell_string_frame(text, index in (1, 3), widths[index]))
                for index, text in enumerate(texts)]
    words = cast(list[int], record['words'])
    segments.extend((name, struct.pack('<i', word)) for name, word in zip(('word28', 'word2c'), words[:2], strict=True))
    segments.append(('double30', bytes.fromhex(cast(str, record['double_bits']))))
    segments.extend((name, struct.pack('<i', word)) for name, word in zip(('word70', 'word74', 'word78'), words[2:], strict=True))
    late = cast(int | None, record['late_word'])
    if late is not None: segments.append(('word84', struct.pack('<i', late)))
    return segments


def shell_literal_spans(case: OwnedShellGroup) -> list[dict[str, object]]:
    """Derive every boundary from independent record literals, without parsing; e.g. shell_literal_spans(case)."""
    segments = [('count', struct.pack('<i', len(case['records'])))]
    segments.extend((f'record{index}.{name}', raw) for index, record in enumerate(case['records'])
                    for name, raw in shell_record_segments(case, record))
    spans: list[dict[str, object]] = []
    position = 0
    for label, raw in segments:
        spans.append(dict(label=label, offset=position, end=position + len(raw), raw_hex=raw.hex()))
        position += len(raw)
    assert b''.join(raw for _, raw in segments) == case['raw']
    return spans


def shell_literal_edit(case: OwnedShellGroup) -> tuple[bytes, list[dict[str, object]], int]:
    """Change a known word, retaining an empty-group no-op instead of inventing records; e.g. shell_literal_edit(case)."""
    spans = shell_literal_spans(case)
    if not case['records']: return case['raw'], [], 0
    ordinal = next(index for index, span in enumerate(spans) if span['label'] == 'record0.word28')
    start, end = cast(int, spans[ordinal]['offset']), cast(int, spans[ordinal]['end'])
    replacement = struct.pack('<i', 314159)
    raw = case['raw'][:start] + replacement + case['raw'][end:]
    records = [dict(record) for record in case['records']]
    words = list(cast(list[int], records[0]['words']))
    words[0] = 314159
    records[0]['words'] = words
    return raw, records, ordinal


def shell_literal_history(case: OwnedShellGroup, ordinal: int) -> list[dict[str, object]]:
    """Retain one ordered relation per physical field and all contributing parents; e.g. shell_literal_history(case, 5)."""
    changed = bool(case['records'])
    relations: list[dict[str, object]] = []
    for index, span in enumerate(shell_literal_spans(case)):
        derived = changed and index == ordinal
        parents = [dict(owner='source', ordinal=index)]
        if derived: parents.append(dict(owner='replacement', ordinal=None))
        relations.append(dict(label=span['label'], result_ordinal=index,
                              relation='derived' if derived else 'copied', parents=parents))
    return relations
