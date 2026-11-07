"""Independent paired MAS/GEN bytes with actual complete context, never a seeded version."""
import struct


def put(raw: bytearray, offset: int, fmt: str, *values: int) -> None:
    """Keep every persisted scalar at its independently reviewed wire offset."""
    struct.pack_into('<' + fmt, raw, offset, *values)


def pattern(count: int, seed: int) -> bytes:
    """Make opaque regions nonzero so dropped bytes cannot pass a round trip."""
    return bytes((index * 17 + seed) & 255 for index in range(count))


def master_image(version: int = 830038, reversed_rows: bool = False) -> bytes:
    """Build all fixed tables, physical rows and padding with an explicit logical directory."""
    raw = bytearray(pattern(28420, 0x35))
    put(raw, 0, 'i9i', 1, 384, 70, 81, 0, 70, 0, 0, 0, 70)
    directory = list(range(70, 0, -1)) if reversed_rows else list(range(1, 71))
    for field in range(81):
        put(raw, 388 + field * 8, 'iI', field - 40, 4)
    for logical, physical in enumerate(directory):
        put(raw, 1156 + logical * 4, 'i', physical)
    for physical in range(70):
        words = [(physical + 1) * 1000 + field for field in range(81)]
        put(raw, 1540 + physical * 384, '81I', *words)
    put(raw, 1540 + (directory[20] - 1) * 384 + 8, 'i', version)
    return bytes(raw)


def mesh_records(nodes: int, elements: int, invalid_reference: bool) -> tuple[bytes, bytes]:
    """Pin raw activity, signed group tags, coordinate patterns and retained unused slots."""
    node_records, element_records = [], []
    for index in range(nodes):
        node = bytearray(pattern(52, index))
        node[0] = 2 if index % 2 else 0
        put(node, 22, 'h', -1 if index == 0 else index)
        put(node, 32, '3I', 0x3f800000 + index, 0x80000000, 0x7fc12345)
        node_records.append(bytes(node))
    for index in range(elements):
        element_records.append(element_record(index, nodes, invalid_reference))
    return b''.join(node_records), b''.join(element_records)


def element_record(index: int, nodes: int, invalid: bool) -> bytes:
    """Differentiate active prefix references from unused negative stored slots."""
    raw = bytearray(pattern(140, index + 6))
    raw[0], raw[23] = (42 if index % 2 else 0), min(2, nodes)
    put(raw, 64, '8i', *[index * 10 + n - 4 for n in range(8)])
    slots = [1, nodes] + [-123] * 9
    if invalid:
        slots[0] = 0
    put(raw, 96, '11i', *slots)
    return bytes(raw)


def auxiliary_record(index: int, old: bool, vector_length: int, unmatched: bool) -> bytes:
    """Force the documented old-stride retry through its actual range predicate."""
    raw = bytearray(pattern(284 if old else 288, index + 4))
    put(raw, 0, '2i', 99 if unmatched else index + 1, vector_length)
    put(raw, 12 if old else 16, '2i', 1, 2)
    if old:
        put(raw, 20, 'I', 0x3f800000)
    return bytes(raw)


def gen_image(version: int = 830038, *, old: bool = False, empty: bool = False,
              vectors: bool = False, unmatched: bool = False, invalid: bool = False) -> bytes:
    """Construct complete family-8 images through the stored version and exact EOF."""
    nodes, elements, auxiliary, extras = (0, 0, 0, 0) if empty else (2, 2, 2, 1)
    raw = bytearray(pattern(1004, 0x61))
    put(raw, 0, 'i', 1)
    put(raw, 404, '14i', 0, 0, 0, nodes, elements, 0, 0, 0, auxiliary, extras, 1, 2, 3, 4)
    mesh_nodes, mesh_elements = mesh_records(nodes, elements, invalid)
    raw += mesh_nodes + mesh_elements
    length = 3 if vectors else 0
    raw += b''.join(auxiliary_record(n, old, length, unmatched and n == 0) for n in range(auxiliary))
    raw += pattern(extras * 144, 0x91)
    for index in range(auxiliary):
        if not (unmatched and index == 0):
            raw += struct.pack('<' + 'I' * length, *[0xf1234567 + n for n in range(length)])
    raw += trailer(version)
    return bytes(raw)


def trailer(version: int) -> bytes:
    """Preserve all mixed-width trailer patterns without assigning engineering meanings."""
    return struct.pack('<3i2Q5I', version, -7, 91, 0x7ff81234567890ab, 0x8000000000000000,
                       0, 1, 0xdeadbeef, 0x80000000, 0xffffffff)


def owned_cases() -> list[tuple[str, bytes, bytes]]:
    """Cover empty images, both predicates, directory-selected versions and mesh exclusions."""
    ordinary = master_image()
    return [('modern', ordinary, gen_image()), ('old', ordinary, gen_image(old=True)),
            ('vectors', ordinary, gen_image(vectors=True)),
            ('old-vectors', ordinary, gen_image(old=True, vectors=True)),
            ('unmatched-vector', ordinary, gen_image(vectors=True, unmatched=True)),
            ('empty', ordinary, gen_image(empty=True)),
            ('reversed-master', master_image(reversed_rows=True), gen_image()),
            ('version-850023', master_image(850023), gen_image(850023)),
            ('invalid-reference', ordinary, gen_image(invalid=True))]
