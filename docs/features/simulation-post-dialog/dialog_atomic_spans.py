"""Atomic character/byte frames and nonoverlapping legacy structural spans (#166)."""
from typing import cast


def dialog_atomic_spans(parsed: dict[str, object]) -> list[dict[str, object]]:
    """Keep framing headers with their variable payload and consume each input byte once; e.g. dialog_atomic_spans(parsed)."""
    spans = cast(list[dict[str, object]], parsed['spans'])
    output: list[dict[str, object]] = []
    index = 0
    while index < len(spans):
        span = spans[index]
        label = cast(str, span['label'])
        frame = dialog_frame_parts(label)
        if frame is not None:
            output.append(dialog_join_frame(span, spans[index + 1], frame))
            index += 2
            continue
        if label == 'style.legacy_raw': output.extend(dialog_legacy_detail_spans(parsed, span))
        else: output.append(span)
        index += 1
    return output


def dialog_frame_parts(label: str) -> tuple[str, str, int] | None:
    """Distinguish character counts, reference blobs and legacy byte-list items; e.g. dialog_frame_parts('style.character_count')."""
    if label.endswith('.character_count'):
        prefix = label.removesuffix('.character_count')
        path = prefix + '.text' if prefix in ('style', 'legacy') else prefix
        return path, prefix + '.utf16', 6
    if label.endswith('.blob_count'):
        prefix = label.removesuffix('.blob_count')
        return prefix + '.blob', prefix + '.blob', 10
    if label.endswith('.byte_count'):
        prefix = label.removesuffix('.byte_count')
        return prefix, prefix + '.raw_bytes', 10
    return None


def dialog_join_frame(header: dict[str, object], payload: dict[str, object], frame: tuple[str, str, int]) -> dict[str, object]:
    """Require the recorded adjacent complete payload rather than searching by a basename; e.g. dialog_join_frame(header, payload, frame)."""
    path, expected, tag = frame
    assert header['end'] == payload['offset'] and payload['label'] == expected, (header, payload, frame)
    assert cast(int, header['end']) - cast(int, header['offset']) == 4, header
    return dict(label=path, offset=header['offset'], end=payload['end'], atomic_tag=tag)


def dialog_legacy_detail_spans(parsed: dict[str, object], outer: dict[str, object]) -> list[dict[str, object]]:
    """Replace the overlapping raw368 view with its contiguous detailed partition for field accounting; e.g. dialog_legacy_detail_spans(parsed, outer)."""
    view = cast(dict[str, object], parsed['legacy_view'])
    spans = cast(list[dict[str, object]], view['spans'])
    assert len(spans) == 38 and spans[0]['offset'] == outer['offset'] and spans[-1]['end'] == outer['end']
    assert all(first['end'] == second['offset'] for first, second in zip(spans, spans[1:]))
    return spans
