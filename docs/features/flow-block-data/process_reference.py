"""Independent complete-carrier patches, with explicit native edit sampling scope."""
import gzip
import struct
import zlib
from freeze_blocks import ROOT, native_cases, owned_cases, flowblockdata, flowresults


def carrier(payload: bytes) -> bytes:
    records = [(406, b"unselected"), (406, payload), (999, b"opaque\0")]
    return b"\x08\0" + b"".join(struct.pack("<IQ", kind, len(raw)) + raw for kind, raw in records)


def bitmap_size(version: int, first: int, second: int) -> int:
    total = (first + second) & 0xffffffff
    return total // 8 + 1 if version >= 8 and 0 < total <= 0x7fffffff else 0


def next_global(parsed: dict, count: int, other: int) -> int:
    for candidate in (min(count + 1, 0x7fffffff), max(count - 1, 0), 0, 1, 0x7fffffff):
        if candidate != count and bitmap_size(parsed["version"], candidate, other) == len(parsed["bitmap_bytes"]):
            return candidate
    return count


def request(case: dict, scope: str, index: int, offset: int, after: bytes) -> tuple:
    before = case["decoded"][offset:offset + len(after)]
    assert len(before) == len(after)
    return scope, index, offset, before, after


def prefix_requests(case: dict, parsed: dict) -> list[tuple]:
    fluid, solid = parsed["g_fluid"], parsed["g_solid"]
    result = [request(case, "global", 0, 4, struct.pack("<i", next_global(parsed, fluid, solid))),
              request(case, "global", 1, 8, struct.pack("<i", next_global(parsed, solid, fluid)))]
    result += [request(case, "bitmap", i, 12 + i, bytes([byte ^ 255]))
               for i, byte in enumerate(parsed["bitmap_bytes"])]
    return result


def header_requests(case: dict, entry: dict, index: int) -> tuple[list[tuple], int]:
    version = int.from_bytes(case["decoded"][:4], "little", signed=True)
    names = [("identifier_i32", 4), ("legacy_index_i32", 4)]
    if version != 1:
        names.append(("member_20_i32", 4))
    names += [("member_28_i32", 4), ("discarded_prefix_byte", 1), ("state_byte", 1), ("flag_2_byte", 1), ("flag_4_byte", 1)]
    offset, result = entry["start"], []
    for name, width in names:
        value = entry[name]
        if value is None:
            continue
        bits = value & ((1 << (width * 8)) - 1)
        after = (bits ^ (255 if width == 1 else 0x80000001)).to_bytes(width, "little")
        result.append(request(case, "entry", index, offset, after))
        offset += width
    return result, offset


def array_requests(case: dict, index: int, offset: int, words: tuple[int, ...]) -> tuple[list[tuple], int]:
    result = [request(case, "entry", index, offset + 4 * i, struct.pack("<I", word ^ 0x80000001))
              for i, word in enumerate(words)]
    return result, offset + 4 * len(words)


def descriptor_requests(case: dict, entry: dict, index: int, offset: int, version: int) -> tuple[list[tuple], int]:
    result = []
    width = 1 if version == 1 else 2
    for slot, word in enumerate(entry["descriptors"]):
        next_word = word if slot == 6 else word ^ (1 if width == 1 else 0x8001)
        result.append(request(case, "entry", index, offset, next_word.to_bytes(width, "little")))
        offset += width
    return result, offset


def entry_requests(case: dict, entry: dict, index: int, version: int) -> list[tuple]:
    result, offset = header_requests(case, entry, index)
    more, offset = descriptor_requests(case, entry, index, offset, version)
    result += more
    more, offset = array_requests(case, index, offset, entry["first_words"])
    result += more
    count = entry["second_count_i32"]
    next_count = (-2 if count == -1 else -1) if count <= 0 and not 2 <= version <= 9 else count
    width = 1 if version == 1 else 2 if version < 10 else 4
    result.append(request(case, "entry", index, offset, (next_count & ((1 << (8 * width)) - 1)).to_bytes(width, "little")))
    more, offset = array_requests(case, index, offset + width, entry["second_words"])
    result += more
    if entry["discarded_suffix_word"] is not None:
        result.append(request(case, "entry", index, offset, struct.pack("<H", entry["discarded_suffix_word"] ^ 0x8001)))
        offset += 2
    assert offset == entry["end"]
    return result


def requests(case: dict) -> list[tuple]:
    parsed = flowblockdata.read_decoded(case["decoded"], leaf_count=case["leaf_count"])
    result, entry_index = prefix_requests(case, parsed), 0
    for block in parsed["blocks"]:
        start, ordinal = block["start"], block["ordinal"]
        raw_count = block["count_12_raw_u16"]
        next_count = (raw_count ^ 1) | 0x8000 if block["count_12_i16"] <= 0 else raw_count
        result += [request(case, "block", ordinal, start, bytes([block["block_byte"] ^ 255])),
                   request(case, "block", ordinal, start + 1, struct.pack("<H", block["word_10_u16"] ^ 0x8001)),
                   request(case, "block", ordinal, start + 3, struct.pack("<H", next_count))]
        for entry in block["entries"]:
            result += entry_requests(case, entry, entry_index, parsed["version"])
            entry_index += 1
    if case["name"] != "native-cpt":
        return result
    last = dict(bitmap=len(parsed["bitmap_bytes"]) - 1, block=len(parsed["blocks"]) - 1, entry=entry_index - 1)
    return [item for item in result if item[0] == "global" or item[1] in (0, last[item[0]])]


def blob(raw: bytes, position: int) -> tuple[bytes, int]:
    count = struct.unpack_from("<I", raw, position)[0]
    start = position + 4
    assert start + count <= len(raw)
    return raw[start:start + count], start + count


def check_wire(wire: bytes, expected: bytes, compressed: bool) -> None:
    parsed = flowresults.read(wire)
    records = parsed["records"]
    assert parsed["version"] == 8 and len(records) == 3
    assert [(r["identifier"], r["payload"]) for r in (records[0], records[2])] == [(406, b"unselected"), (999, b"opaque\0")]
    assert records[1]["identifier"] == 406
    if not compressed:
        assert wire == carrier(expected)
        return
    decoder = zlib.decompressobj()
    decoded = decoder.decompress(records[1]["payload"], len(expected) + 1)
    assert decoder.eof and not decoder.unused_data and not decoder.unconsumed_tail
    assert decoded == expected


def check_request(raw: bytes, position: int, case_index: int, case: dict, expected: tuple) -> tuple[int, bool]:
    _, _, offset, before, after = expected
    assert struct.unpack_from("<II", raw, position) == (case_index, offset)
    position += 8
    for expected_bytes in (before, after):
        actual, position = blob(raw, position)
        assert actual == expected_bytes
    wires = []
    for _ in range(4):
        wire, position = blob(raw, position)
        wires.append(wire)
    patched = case["decoded"][:offset] + after + case["decoded"][offset + len(after):]
    assert wires[0] == wires[1] == carrier(case["stored"])
    check_wire(wires[2], patched, case["compressed"])
    check_wire(wires[3], case["decoded"], case["compressed"])
    changed = before != after
    return check_history(raw, position, changed), changed


def check_history(raw: bytes, position: int, changed: bool) -> int:
    assert struct.unpack_from("<I", raw, position)[0] == 3
    position += 4
    for ordinal in range(3):
        assert struct.unpack_from("<IIB", raw, position) == (ordinal, ordinal, changed and ordinal == 1)
        position += 9
    return position


def check_packet(raw: bytes) -> dict[str, int]:
    baseline = gzip.decompress((ROOT / "tests/fixtures/flow-block-data/blocks.bin.gz").read_bytes())
    assert raw[:len(baseline)] == baseline
    position = len(baseline)
    assert raw[position:position + 4] == b"FBE1"
    actual_count = struct.unpack_from("<I", raw, position + 4)[0]
    position += 8
    total, native, changed = 0, 0, 0
    for index, case in enumerate(native_cases() + owned_cases()):
        for item in requests(case):
            position, differs = check_request(raw, position, index, case, item)
            total += 1
            changed += differs
            native += index == 0
    assert position == len(raw) and actual_count == total
    return dict(views=273, requests=total, changed=changed, noop_after_requests=total - changed,
                native_sample_requests=native, owned_requests=total - native)
