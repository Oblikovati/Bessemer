"""Independent binary transmit vectors, checked with the Python reference parser."""
from pathlib import Path
import struct
import xtbin

FieldTuple = tuple[str, str, int, int]

ROOT: Path = Path(__file__).resolve().parents[3]
FIXTURES: Path = ROOT / 'tests/fixtures/parasolid'
SCALARS: list[FieldTuple] = [('integer', 'd', 0, 0), ('pointer', 'p', 3, 0), ('byte', 'u', 0, 0),
           ('character', 'c', 0, 0), ('logical', 'l', 0, 0), ('short', 'n', 0, 0),
           ('wide', 'w', 0, 0), ('real', 'f', 0, 0), ('interval', 'i', 0, 0),
           ('vector', 'v', 0, 0), ('hvec', 'h', 0, 0), ('box', 'b', 0, 0), ('tail', 'u', 0, 1)]
EDITED: list[FieldTuple] = SCALARS[:1] + [('replacement', 'p', 101, 0), ('Caf\xe9', 'u', 0, 0)] + SCALARS[2:]
DEFINED: list[FieldTuple] = [('real_pair', 'f', 0, 2), ('pointer_array', 'p', 200, 1)]
PARTITION: list[FieldTuple] = [('current_pmark', 'p', 4, 0), ('highest_id', 'd', 0, 0)]
PMARK: list[FieldTuple] = [(name, 'p', 4, 0) for name in ('preceding', 'first_following', 'next_sibling', 'prev_sibling')]
PMARK += [(name, 'd', 0, 0) for name in ('n_new_nodes', 'n_del_nodes', 'n_copy_mod_nodes', 'delta_key')]
PMARK += [('delta_is_forward', 'l', 0, 0), ('id', 'd', 0, 0)]


class FixtureWriter:
    def __init__(self, bare: bool, deltas: bool) -> None:
        self.endian = '<' if bare else '>'
        self.output = bytearray(b'B' if bare else b'PS')
        self.seen: set[int] = set()
        self.string32(': TRANSMIT FILE (%s) created by fixture' % ('deltas' if deltas else 'partition'))
        self.string32('SCH_3701250_37102_13006')
        self.pack('HI', 239, 0)

    def pack(self, shape: str, *values: int) -> None:
        self.output.extend(struct.pack(self.endian + shape, *values))

    def string8(self, text: str) -> None:
        encoded = text.encode('latin1')
        self.pack('B', len(encoded))
        self.output.extend(encoded)

    def string32(self, text: str) -> None:
        encoded = text.encode('latin1')
        self.pack('I', len(encoded))
        self.output.extend(encoded)

    def index(self, stored: int) -> None:
        if stored <= 0x7fff:
            self.pack('H', stored)
        else:
            self.pack('hH', -((stored & 0x7fff) + 1), stored >> 15)

    def definition(self, field: FieldTuple) -> None:
        name, kind, pointer_class, elements = field
        self.string8(name)
        self.pack('HH', pointer_class, elements + 1)
        if pointer_class == 0:
            self.string8(kind)

    def layout(self, code: int) -> None:
        if code in (3, 4):
            self.pack('B', 255)
        elif code == 200:
            self.pack('B', len(EDITED))
            self.output.extend(b'CDI')
            self.definition(EDITED[1])
            self.output.extend(b'A')
            self.definition(EDITED[2])
            self.output.extend(b'C' * 11 + b'Z')
        else:
            fields = SCALARS if code == 231 else DEFINED
            self.pack('B', len(fields))
            self.string8('ALL_KINDS_DEFINED' if code == 231 else 'NEW')
            self.string8('Independent complete type')
            for field in fields:
                self.definition(field)
            self.pack('B', 1)

    def field(self, field: FieldTuple, length: int | None, delta: bool) -> None:
        name, kind, _, extent = field
        count = length if extent == 1 else (extent or 1)
        assert count is not None
        for ordinal in range(count):
            if kind == 'p':
                self.index((32771, 1, 0x7fffffff)[ordinal % 3])
                if delta:
                    self.pack('B', (0, 255, 1)[ordinal % 3])
            elif kind in 'ucl':
                self.pack('B', 255 - ordinal % 256)
            elif kind == 'd':
                self.pack('i', -2147483648 + ordinal)
            elif kind == 'n':
                self.pack('h', -32768 + ordinal)
            elif kind == 'w':
                self.pack('H', 0xd800 + ordinal)
            else:
                self.floats(kind, ordinal, delta)

    def floats(self, kind: str, ordinal: int, delta: bool) -> None:
        count = {'f': 1, 'i': 2, 'v': 3, 'h': 11 if delta else 3, 'b': 6}[kind]
        patterns = [0x8000000000000000, 0x7ff8123456789abc, 0x7ff0000000000000,
                    0xfff0000000000000, 1, 0x3ff0000000000000]
        for coordinate in range(count):
            self.pack('Q', patterns[(coordinate + ordinal) % len(patterns)])

    def node(self, code: int, stored_index: int, length: int | None, delta: bool) -> None:
        self.pack('H', code)
        if code not in self.seen:
            self.layout(code)
            self.seen.add(code)
        fields = {3: PARTITION, 4: PMARK, 200: EDITED, 230: DEFINED, 231: SCALARS}[code]
        if any(field[3] == 1 for field in fields):
            assert length is not None
            self.pack('I', length)
        self.index(stored_index)
        for field in fields:
            self.field(field, length, delta)

    def terminator(self, stored_index: int = 1) -> None:
        self.pack('H', 1)
        self.index(stored_index)


def fixture(bare: bool, deltas: bool, initial_only: bool = False) -> bytes:
    writer = FixtureWriter(bare, deltas)
    writer.node(3, 2, None, False)
    writer.node(4, 3, None, False)
    writer.node(200, 32771, 3, deltas)
    writer.node(230, 0x7fffffff, 2, deltas)
    writer.node(200, 32770, 0, deltas)
    writer.node(231, 6, 3, deltas)
    writer.terminator(32771)
    if deltas and not initial_only:
        delta_changes(writer)
    return bytes(writer.output)


def delta_changes(writer: FixtureWriter) -> None:
    writer.index(32771)
    writer.pack('H', 200)
    writer.index(0x7fffffff)
    writer.pack('H', 230)
    writer.index(1)
    writer.pack('I', 0xffffffff)
    writer.node(3, 4, None, True)
    writer.node(4, 5, None, True)
    writer.node(200, 32771, 1, True)
    writer.terminator(0x7fffffff)
    writer.terminator()


def reference_schema() -> dict[int, xtbin.NodeType]:
    types = {}
    for code, fields in [(3, PARTITION), (4, PMARK), (200, SCALARS)]:
        node = xtbin.NodeType()
        node.code, node.name, node.desc, node.xmt, node.variable = code, str(code), '', 1, 0
        node.fields = [xtbin.Field(name, kind, 1, pointer_class, extent) for name, kind, pointer_class, extent in fields]
        types[code] = node
    return types


class FixtureSchemaSource:
    def load_schema(self, key: str) -> dict[int, xtbin.NodeType]:
        if key != '13006':
            raise ValueError(f'schema key {key!r}; expected fixture schema 13006')
        return reference_schema()


def write_case(bare: bool, deltas: bool) -> None:
    payload = fixture(bare, deltas)
    result = xtbin.parse(payload)
    assert not result['tail'] and len(result['nodes']) == (9 if deltas else 6)
    prefix = 'bare' if bare else 'neutral'
    name = '%s-%s.bin' % (prefix, 'deltas' if deltas else 'partition')
    (FIXTURES / name).write_bytes(payload)
    if deltas:
        initial = fixture(bare, True, initial_only=True)
        assert len(xtbin.parse(initial)['sections']) == 1
        (FIXTURES / (prefix + '-initial-deltas.bin')).write_bytes(initial)


def generate() -> None:
    source_loader = xtbin.load_schema
    xtbin.load_schema = FixtureSchemaSource().load_schema
    try:
        for bare, deltas in [(False, False), (False, True), (True, False), (True, True)]:
            write_case(bare, deltas)
    finally:
        xtbin.load_schema = source_loader
    print('Generated and reference-verified six all-kind transmit vectors')


if __name__ == '__main__':
    generate()
