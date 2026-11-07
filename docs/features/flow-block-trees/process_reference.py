"""Independent tree-field byte patches and strict checks for noncanonical zlib output."""
import gzip
import struct
import zlib
from freeze_trees import ROOT, append_tree, flowblocktrees, flowresults, native_cases, owned_cases


def cases() -> list[dict]:
    return native_cases()[0] + owned_cases()


def forest(case: dict) -> tuple[list[dict], list[dict]]:
    parsed = flowblocktrees.read_decoded(case["decoded"], legacy_bounds=case["bounds"], max_depth=case["maximum_depth"])
    nodes, roots = [], []
    for root in parsed["roots"] or []:
        if root["present"]:
            append_tree(root["tree"], nodes)
        roots.append(root)
    return roots, nodes


def node_requests(nodes: list[dict]) -> list[tuple]:
    requests = []
    for index, node in enumerate(nodes):
        marker = node["marker"]
        if marker != 255:
            requests.append((0, index, 0, node["start"], bytes([marker]), bytes([0 if marker == 254 else marker + 1])))
        if node["word"] is not None:
            requests.append((2, index, 0, node["start"] + 1, struct.pack("<I", node["word"]), struct.pack("<I", node["word"] ^ 0x80000001)))
        for slot, child in enumerate(node["children"]):
            prefix = child["prefix"]
            if prefix is not None:
                requests.append((1, index, slot, nodes[child["index"]]["start"] - 1, bytes([prefix]), bytes([prefix ^ 255])))
    return requests


def requests(case: dict) -> list[tuple]:
    roots, nodes = forest(case)
    result = node_requests(nodes)
    for index, root in enumerate(roots):
        before = root["presence_byte"]
        if before is not None:
            after = 0 if before == 0 else 1 if before == 255 else 255
            result.append((3, index, 0, root["start"], bytes([before]), bytes([after])))
    if case["name"] not in ("depth255", "depth256"):
        return result
    deepest = max(node["depth"] for node in nodes)
    deep = next(index for index, node in enumerate(nodes) if node["depth"] == deepest)
    selected = [request for request in result if request[0] in (0, 2) and request[1] == deep]
    selected += [request for kind in (1, 3) if (request := next((item for item in result if item[0] == kind), None)) is not None]
    return selected


def edit_cases() -> list[tuple[int, tuple]]:
    return [(index, request) for index, case in enumerate(cases()) for request in requests(case)]


def blob(cursor: memoryview, position: int) -> tuple[bytes, int]:
    size = struct.unpack_from("<I", cursor, position)[0]
    start = position + 4
    assert start + size <= len(cursor)
    return bytes(cursor[start:start + size]), start + size


def carrier(payload: bytes) -> bytes:
    records = [(999, b"before"), (400, payload), (111, b"after")]
    return b"\x08\0" + b"".join(struct.pack("<IQ", identifier, len(raw)) + raw for identifier, raw in records)


def check_carrier(raw: bytes, case: dict, expected: bytes) -> None:
    parsed = flowresults.read(raw)
    records = parsed["records"]
    assert parsed["version"] == 8 and len(records) == 3
    assert [(item["identifier"], item["payload"]) for item in (records[0], records[2])] == [(999, b"before"), (111, b"after")]
    assert records[1]["identifier"] == 400
    payload = records[1]["payload"]
    if case["compression"] == "none":
        assert payload == expected
        return
    decoder = zlib.decompressobj()
    decoded = decoder.decompress(payload, len(expected) + 1)
    assert decoder.eof and not decoder.unused_data and not decoder.unconsumed_tail
    assert decoded == expected


def check_edit(cursor: memoryview, position: int, case: dict, case_index: int, request: tuple) -> tuple[int, bool]:
    kind, index, child, offset, before, after = request
    actual = struct.unpack_from("<IBII", cursor, position)
    assert actual == (case_index, kind, index, child)
    position += 13
    for expected in (before, after):
        actual_bytes, position = blob(cursor, position)
        assert actual_bytes == expected
    wires = []
    for _ in range(4):
        wire, position = blob(cursor, position)
        wires.append(wire)
    patched = case["decoded"][:offset] + after + case["decoded"][offset + len(after):]
    check_wires(wires, case, patched)
    changed = patched != case["decoded"]
    position = check_history(cursor, position, changed)
    return position, changed


def check_wires(wires: list[bytes], case: dict, patched: bytes) -> None:
    assert wires[0] == wires[1] == carrier(case["payload"])
    check_carrier(wires[2], case, patched)
    check_carrier(wires[3], case, case["decoded"])
    if case["compression"] == "none":
        assert wires[2] == carrier(patched)
        assert wires[3] == wires[0]


def check_history(cursor: memoryview, position: int, changed: bool) -> int:
    assert struct.unpack_from("<I", cursor, position)[0] == 3
    position += 4
    for ordinal in range(3):
        assert struct.unpack_from("<IIB", cursor, position) == (ordinal, ordinal, changed and ordinal == 1)
        position += 9
    return position


def check_packet(raw: bytes) -> dict:
    expected_views = gzip.decompress((ROOT / "tests/fixtures/flow-block-trees/trees.bin.gz").read_bytes())
    assert raw[:len(expected_views)] == expected_views
    cursor, position = memoryview(raw), len(expected_views)
    assert raw[position:position + 4] == b"FTE1"
    count = struct.unpack_from("<I", cursor, position + 4)[0]
    position += 8
    expected, selected = edit_cases(), cases()
    assert count == len(expected)
    changed = 0
    for case_index, request in expected:
        position, edited = check_edit(cursor, position, selected[case_index], case_index, request)
        changed += edited
    assert position == len(raw)
    return dict(views=len(selected), edit_requests=count, changed=changed, noop_after_requests=count - changed)
