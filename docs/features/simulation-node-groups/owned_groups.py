"""Independent signed node-tag and run cases for clause 14.120 (#122)."""
import struct
from itertools import groupby


def node_record(bits: int, activity: int = 1) -> bytes:
    """Keep unassigned bytes visible so a tag projection cannot replace the record."""
    raw = bytearray((index * 37 + 19) % 256 for index in range(52))
    raw[0] = activity
    struct.pack_into('<H', raw, 22, bits)
    return bytes(raw)


def group_cases() -> list[bytes]:
    """Include the entire stored word domain and continuous chunk-boundary runs."""
    domain = b''.join(node_record(bits, bits % 256) for bits in range(65536))
    separated = b''.join(node_record(bits, index % 2) for index, bits in enumerate((1, 1, 2, 1, 0, 65535, 65535, 32768)))
    crossing = node_record(65535, 0) * 16385 + node_record(32767) * 2
    return [b'', node_record(0, 0), node_record(32768), separated, crossing, domain]


def independent_runs(nodes: bytes) -> list[dict[str, int]]:
    """Group ordinal/tag pairs independently of the reader's mutable run cursor."""
    assert len(nodes) % 52 == 0
    tags = (struct.unpack_from('<h', nodes, offset + 22)[0] for offset in range(0, len(nodes), 52))
    result = []
    for tag, grouped in groupby(enumerate(tags, 1), key=lambda pair: pair[1]):
        ordinals = [ordinal for ordinal, _ in grouped]
        result.append(dict(tag=tag, first_node=ordinals[0], last_node=ordinals[-1]))
    return result


def independent_selection(nodes: bytes, elements: bytes, body: int) -> list[int]:
    """Derive the union directly from complete active element reference tuples."""
    selected = set()
    for offset in range(0, len(elements), 140):
        raw = elements[offset:offset + 140]
        if not raw[0] or struct.unpack_from('<H', raw, 28)[0] != body:
            continue
        refs = struct.unpack_from('<11i', raw, 96)[:raw[23]]
        assert raw[23] <= 11 and all(value == 0 or 1 <= value <= len(nodes) // 52 for value in refs)
        selected.update(value for value in refs if value and nodes[(value - 1) * 52])
    return sorted(selected)
