"""Independent Source views and whole-carrier fixed-width edit expectations."""
import struct
from collections.abc import Iterator
from freeze_mapped import blob, case_packet, flowmappedobjects, owned_cases, source_cases


def cases() -> list[tuple[str, bytes, list[int | None] | None]]:
    return source_cases()[0] + owned_cases()


def carrier(payload: bytes) -> bytes:
    records = [(999, b"before"), (144, payload), (111, b"after")]
    return b"\x08\0" + b"".join(struct.pack("<IQ", identifier, len(raw)) + raw for identifier, raw in records)


def numeric_blocks(obj: dict) -> Iterator[tuple[int, int, tuple[int, ...]]]:
    start = obj["start"] + 6
    yield 3, start, obj["numeric"]["raw_bits"]
    start += 9 * 8 + 4 + len(obj["pairs"]) * 8 + 4
    yield 4, start, obj["mapped_values"]["raw_bits"]
    start += len(obj["mapped_values"]["raw_bits"]) * 8
    legacy = obj["legacy_version_5"]
    if legacy is None:
        return
    yield 5, start, legacy["prefix"]["raw_bits"]
    start += 16 * 8
    for kind, block in enumerate(legacy["arrays"], 6):
        start += 4
        yield kind, start, block["raw_bits"]
        start += len(block["raw_bits"]) * 8
    yield 9, start, legacy["tripled_values"]["raw_bits"]
    assert start + len(legacy["tripled_values"]["raw_bits"]) * 8 == obj["end"]


def field_requests(obj: dict) -> Iterator[tuple[int, int, int, bytes, bytes]]:
    state = obj["state_byte"]
    yield 0, 0, obj["start"] + 5, bytes([state]), bytes([state ^ 255])
    start = obj["start"] + 6 + 9 * 8 + 4
    for index, pair in enumerate(obj["pairs"]):
        for column, value in enumerate(pair):
            yield column + 1, index, start + index * 8 + column * 4, struct.pack("<i", value), struct.pack("<i", value ^ 0x40000000)
    for kind, start, values in numeric_blocks(obj):
        for index, bits in enumerate(values):
            yield kind, index, start + index * 8, struct.pack("<Q", bits), struct.pack("<Q", bits ^ 0x8000000000000001)


def edit_row(case_index: int, entry: int, kind: int, index: int, original: bytes,
             patched: bytes, before: bytes, after: bytes) -> bytes:
    prefix = struct.pack("<IIBI", case_index, entry, kind, index) + blob(before) + blob(after)
    original_wire = blob(carrier(original))
    wires = original_wire * 2 + blob(carrier(patched)) + original_wire
    history = struct.pack("<I", 3) + b"".join(struct.pack("<IIB", ordinal, ordinal, ordinal == 1) for ordinal in range(3))
    return prefix + wires + history


def edit_rows() -> Iterator[bytes]:
    for case_index, (_, payload, context) in enumerate(cases()):
        parsed = flowmappedobjects.read_payload(payload, context)
        for entry, obj in enumerate(parsed["objects"]):
            if not obj["present"]:
                continue
            for kind, index, offset, before, after in field_requests(obj):
                assert payload[offset:offset + len(before)] == before
                patched = payload[:offset] + after + payload[offset + len(after):]
                assert flowmappedobjects.read_payload(patched, context)["end"] == len(payload)
                yield edit_row(case_index, entry, kind, index, payload, patched, before, after)


def packet() -> bytes:
    selected = cases()
    views = b"FMO1" + struct.pack("<I", len(selected)) + b"".join(map(case_packet, selected))
    edits = list(edit_rows())
    assert len(edits) == 3894
    return views + b"FME1" + struct.pack("<I", len(edits)) + b"".join(edits)
