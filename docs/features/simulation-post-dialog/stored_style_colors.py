"""Shared canonical complete-body color observations without initialization defaults (#166)."""
from typing import cast
from dialog_projection import actual_post_record


def canonical_style_spans(family: int, version: int, raw: bytes) -> dict[str, dict[str, object]]:
    """Require complete EOF before exposing canonical physical fields; e.g. canonical_style_spans(2, 700006, raw)."""
    parsed = actual_post_record(family, version, raw)
    assert parsed['success'] and parsed['end'] == len(raw)
    return {span['label']: span for span in cast(list[dict[str, object]], parsed['spans'])}


def optional_style_rgba(spans: dict[str, dict[str, object]], raw: bytes, prefix: str) -> bytes | None:
    """Retain complete named groups or explicit absence, never a partial/default color; e.g. optional_style_rgba(spans, raw, 'style.color_1bc')."""
    present = [f'{prefix}.component_{index}' in spans for index in range(4)]
    assert all(present) or not any(present)
    if not all(present): return None
    words: list[bytes] = []
    for index in range(4):
        span = spans[f'{prefix}.component_{index}']
        word = bytes.fromhex(cast(str, span['raw_hex']))
        offset = cast(int, span['offset'])
        assert len(word) == 4 and raw[offset:offset + 4] == word
        words.append(word)
    return b''.join(words)



def require_style_rgba_rejection(spans: dict[str, dict[str, object]], raw: bytes) -> None:
    """Keep incomplete or byte-inconsistent groups fatal; e.g. require_style_rgba_rejection(spans, raw)."""
    try:
        optional_style_rgba(spans, raw, 'style.color_fixture')
    except AssertionError:
        return
    raise AssertionError((spans, raw.hex(), 'expected complete unchanged four-word color rejection'))


def verify_stored_style_rgba_boundaries() -> None:
    """Pin absent, complete, partial and changed-byte groups without filesystem I/O; e.g. verify_stored_style_rgba_boundaries()."""
    raw = bytes.fromhex('00000000000000804200a07f0000803f')
    spans: dict[str, dict[str, object]] = {f'style.color_fixture.component_{index}':
        dict(offset=index * 4, raw_hex=raw[index * 4:index * 4 + 4].hex()) for index in range(4)}
    assert optional_style_rgba({}, raw, 'style.color_fixture') is None
    assert optional_style_rgba(spans, raw, 'style.color_fixture') == raw
    for missing in spans:
        require_style_rgba_rejection({key: value for key, value in spans.items() if key != missing}, raw)
    require_style_rgba_rejection(spans, bytes(16))
