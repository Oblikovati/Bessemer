"""Compare complete process output with Source reads and independent whole-carrier patches."""
import gzip
import struct
import zlib
from freeze_grid import ROOT, native_cases, owned_cases, flowresults


def carrier(payload: bytes) -> bytes:
    records = [(101, b"unselected"), (101, payload), (999, b"opaque\0")]
    return b"\7\0" + b"".join(struct.pack("<IQ", kind, len(raw)) + raw for kind, raw in records)


def blob(raw: bytes, position: int) -> tuple[bytes, int]:
    count = struct.unpack_from("<I", raw, position)[0]
    start = position + 4
    assert start + count <= len(raw)
    return raw[start:start + count], start + count


def check_wire(wire: bytes, expected: bytes, compressed: bool) -> None:
    parsed = flowresults.read(wire)
    records = parsed["records"]
    assert parsed["version"] == 7 and len(records) == 3
    assert [(r["identifier"], r["payload"]) for r in (records[0], records[2])] == [(101, b"unselected"), (999, b"opaque\0")]
    assert records[1]["identifier"] == 101
    payload = records[1]["payload"]
    if not compressed:
        assert wire == carrier(expected)
        return
    decoder = zlib.decompressobj()
    decoded = decoder.decompress(payload, len(expected) + 1)
    assert decoder.eof and not decoder.unused_data and not decoder.unconsumed_tail
    assert decoded == expected


def check_request(raw: bytes, position: int, case: int, array: int, index: int,
                  decoded: bytes, stored: bytes, compressed: bool, offset: int) -> int:
    assert struct.unpack_from("<3I", raw, position) == (case, array, index)
    position += 12
    wires = []
    for _ in range(4):
        wire, position = blob(raw, position)
        wires.append(wire)
    original = struct.unpack_from("<Q", decoded, offset)[0]
    patched = decoded[:offset] + struct.pack("<Q", original ^ 0x8000000000000001) + decoded[offset + 8:]
    assert wires[0] == wires[1] == carrier(stored)
    check_wire(wires[2], patched, compressed)
    check_wire(wires[3], decoded, compressed)
    assert struct.unpack_from("<I", raw, position)[0] == 3
    position += 4
    for ordinal in range(3):
        assert struct.unpack_from("<IIB", raw, position) == (ordinal, ordinal, ordinal == 1)
        position += 9
    return position


def check_packet(raw: bytes) -> dict[str, int]:
    baseline = gzip.decompress((ROOT / "tests/fixtures/flow-base-grid/grid.bin.gz").read_bytes())
    assert raw[:len(baseline)] == baseline
    position = len(baseline)
    assert raw[position:position + 4] == b"FGE1"
    assert struct.unpack_from("<I", raw, position + 4)[0] == 452
    position += 8
    request_count = 0
    for case, (_, decoded) in enumerate(native_cases() + owned_cases()):
        for compressed in (False, True):
            stored = zlib.compress(decoded, 9) if compressed else decoded
            position, checked = check_case(raw, position, case * 2 + compressed, decoded, stored, compressed)
            request_count += checked
    assert position == len(raw) and request_count == 452
    return dict(views=56, changed_fields=request_count, original_noops=request_count, fresh_reversals=request_count)


def check_case(raw: bytes, position: int, case: int, decoded: bytes, stored: bytes,
               compressed: bool) -> tuple[int, int]:
    offset, count = 12, 0
    for array, dimension in enumerate(struct.unpack_from("<3i", decoded)):
        for index in range(dimension + 1):
            position = check_request(raw, position, case, array, index, decoded, stored, compressed, offset)
            count += 1
            offset += 8
    return position, count
