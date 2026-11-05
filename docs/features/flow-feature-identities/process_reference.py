"""Independent raw patch oracle for complete record-143 carriers and all edit requests."""
import struct
from collections.abc import Iterator
from freeze_identities import blob, case_packet, identities, native_cases, owned_cases


def identity_cases() -> list[tuple[str, int, bytes]]:
    return [case for case in native_cases() + owned_cases() if case[1] == 143]


def carrier(payload: bytes) -> bytes:
    records = [(999, b"before"), (143, payload), (111, b"after")]
    return b"\x08\0" + b"".join(struct.pack("<IQ", identifier, len(raw)) + raw for identifier, raw in records)


def text_bytes(raw: bytes) -> bytes:
    return struct.pack("<i", len(raw) // 2) + raw


def object_requests(obj: dict) -> list[tuple[int, bytes]]:
    original = [(0, struct.pack("<I", obj["header_version"])), (1, struct.pack("<I", obj["type_code"])),
                (2, struct.pack("<I", obj["body_version"])), (3, bytes([obj["state_byte"]])),
                (4, text_bytes(obj["text_bytes"]))]
    changed = [(0, struct.pack("<I", obj["header_version"] ^ 0x80000001)),
               (1, struct.pack("<I", obj["type_code"] ^ 0x40000000)),
               (2, struct.pack("<I", obj["body_version"] ^ 0x80000001)), (3, bytes([obj["state_byte"] ^ 255]))]
    texts = [(4, text_bytes(raw)) for raw in [b"", b"\0\0", struct.pack("<4H", 0xd800, 0, 0xdc00, 0)]]
    return original + changed + texts


def patched_object(payload: bytes, obj: dict, kind: int, replacement: bytes) -> bytes:
    spans = [(obj["start"], obj["start"] + 4), (obj["start"] + 4, obj["start"] + 8),
             (obj["start"] + 8, obj["start"] + 12), (obj["end"] - 1, obj["end"]),
             (obj["text_start"] - 4, obj["end"] - 1)]
    start, end = spans[kind]
    patched = payload[:start] + replacement + payload[end:]
    assert identities.read143(patched)["end"] == len(patched)
    return patched


def edit_row(case_index: int, tag: int, index: int, kind: int, replacement: bytes,
             original: bytes, patched: bytes) -> bytes:
    prefix = struct.pack("<IBIB", case_index, tag, index, kind) + blob(replacement)
    wires = blob(carrier(original)) + blob(carrier(patched)) + blob(carrier(original))
    history = struct.pack("<I", 3) + b"".join(struct.pack("<IIB", ordinal, ordinal,
                 ordinal == 1 and original != patched) for ordinal in range(3))
    return prefix + wires + history


def edit_cases() -> Iterator[tuple[bytes, bool]]:
    for case_index, (_, _, payload) in enumerate(identity_cases()):
        parsed = identities.read143(payload)
        for index, obj in enumerate(parsed["objects"]):
            for kind, replacement in object_requests(obj):
                patched = patched_object(payload, obj, kind, replacement)
                yield edit_row(case_index, 0, index, kind, replacement, payload, patched), patched != payload
        tail = parsed["tail_values"]
        if tail is None:
            continue
        for index, before in enumerate(tail):
            start = len(payload) - len(tail) * 4 + index * 4
            for value in (before, before ^ 0x40000000):
                replacement = struct.pack("<i", value)
                patched = payload[:start] + replacement + payload[start + 4:]
                yield edit_row(case_index, 1, index, 5, replacement, payload, patched), patched != payload


def packet() -> bytes:
    cases = identity_cases()
    views = b"FIA1" + struct.pack("<I", len(cases)) + b"".join(map(case_packet, cases))
    edits = list(edit_cases())
    assert len(edits) == 1080
    return views + b"FIE1" + struct.pack("<I", len(edits)) + b"".join(row for row, _ in edits)
