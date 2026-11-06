"""Reconstruct Source-parsed values into an independent typed field transcript."""
from __future__ import annotations
import struct
import freeze_extras as baseline

ENTITIES = ("Record", "Block", "Body", "First", "FirstMixed", "Second", "Third", "Fourth",
            "NestedRow", "NestedEntry", "Final", "Extra", "ExtraMixed")
KINDS = ("Version", "Token", "BodyFlag", "FirstCount", "SecondCount", "ThirdCount", "PairBytes",
         "FirstWords", "MixedCount", "MixedWords", "MixedWide", "ReferenceWords", "ByteArrayCount",
         "ByteArray", "AdditionalFlag", "AdditionalWords", "TagByte", "ThirdWord", "ThirdWide",
         "FourthCount", "FourthWide", "FourthBytes", "OuterFlag", "NestedFlag", "NestedCount",
         "NestedElementCount", "NestedIndex", "NestedWords", "NestedWide", "FinalFlag", "FinalWord",
         "FinalCount", "FinalWords", "ExtraFlag", "ExtraWord", "LastByte")
STRUCTURAL = {"Version", "FirstCount", "SecondCount", "ThirdCount", "MixedCount", "ByteArrayCount",
              "FourthCount", "NestedCount", "NestedElementCount", "FinalCount"}
Entity = tuple[str, int, int, int]


def entity(kind: str, block: int = 0, row: int = 0, index: int = 0) -> Entity:
    return kind, block, row, index


def entity_packet(scope: Entity) -> bytes:
    return bytes((ENTITIES.index(scope[0]),)) + struct.pack("<3I", *scope[1:])


class FieldOracle:
    def __init__(self, version: int) -> None:
        self.version = version
        self.wire = bytearray()
        self.fields: list[bytes] = []
        self.scopes: list[tuple[Entity, int, int, int | None, bool]] = []
        self.layouts: list[tuple[int, int, int, int]] = []

    def begin(self, scope: Entity, native_index: int | None = None, stored: bool = False) -> int:
        ordinal = len(self.scopes)
        self.scopes.append((scope, len(self.wire), len(self.wire), native_index, stored))
        return ordinal

    def end(self, ordinal: int) -> None:
        scope, start, _, native_index, stored = self.scopes[ordinal]
        self.scopes[ordinal] = scope, start, len(self.wire), native_index, stored

    def register(self, scope: Entity, kind: str, shape: str, values: tuple | list | bytes, start: int, stride: int) -> None:
        encoding = {"B": 0, "i": 1, "Q": 2}[shape]
        width = struct.calcsize(shape)
        raw = baseline.numbers(values, shape)
        packet = entity_packet(scope) + bytes((KINDS.index(kind), encoding, kind not in STRUCTURAL))
        self.fields.append(packet + struct.pack("<4I", start, len(values), width, stride) + baseline.blob(raw))
        self.layouts.append((start, len(values), width, stride))

    def store(self, scope: Entity, kind: str, shape: str, values: tuple | list | bytes) -> None:
        self.register(scope, kind, shape, values, len(self.wire), struct.calcsize(shape))
        self.wire.extend(baseline.numbers(values, shape))

    def scalar(self, scope: Entity, kind: str, shape: str, value: int) -> None:
        self.store(scope, kind, shape, (value,))

    def byte_array(self, scope: Entity, payload: bytes) -> None:
        self.scalar(scope, "ByteArrayCount", "B", len(payload))
        self.store(scope, "ByteArray", "B", payload)

    def mixed(self, scope: Entity, parsed: dict) -> None:
        assert len(self.wire) == parsed["start"]
        ordinal = self.begin(scope)
        self.scalar(scope, "MixedCount", "i", parsed["count"])
        if self.version < 8:
            self.store(scope, "MixedWords", "i", parsed["words_i32"])
            self.store(scope, "MixedWide", "Q", parsed["wide_bits"])
        else:
            self.interleaved(scope, parsed)
        self.end(ordinal)
        assert len(self.wire) == parsed["end"]

    def verify_coverage(self) -> None:
        covered = bytearray(len(self.wire))
        for start, count, width, stride in self.layouts:
            self.cover_layout(covered, start, count, width, stride)
        assert all(covered)

    def cover_layout(self, covered: bytearray, start: int, count: int, width: int, stride: int) -> None:
        assert 0 <= start <= len(covered)
        for index in range(count):
            offset = start + index * stride
            for position in range(offset, offset + width):
                assert position < len(covered) and covered[position] == 0
                covered[position] = 1

    def interleaved(self, scope: Entity, parsed: dict) -> None:
        start = len(self.wire)
        self.register(scope, "MixedWords", "i", parsed["words_i32"], start, 12)
        self.register(scope, "MixedWide", "Q", parsed["wide_bits"], start + (4 if parsed["count"] else 0), 12)
        for word, wide in zip(parsed["words_i32"], parsed["wide_bits"]):
            self.wire.extend(struct.pack("<iQ", word, wide))

    def first(self, block: int, index: int, parsed: dict) -> None:
        assert len(self.wire) == parsed["start"]
        scope = entity("First", block, index=index)
        ordinal = self.begin(scope)
        self.store(scope, "PairBytes", "B", parsed["pair_bytes"])
        self.store(scope, "FirstWords", "i", parsed["words_i32"])
        self.mixed(entity("FirstMixed", block, index=index), parsed["mixed"])
        self.store(scope, "ReferenceWords", "i" if self.version < 5 else "Q", parsed["reference_words"])
        self.byte_array(scope, parsed["byte_array"])
        if parsed["additional_presence_byte"] is not None:
            self.scalar(scope, "AdditionalFlag", "B", parsed["additional_presence_byte"])
            if parsed["additional_presence_byte"]:
                self.store(scope, "AdditionalWords", "i", parsed["additional_i32"])
        self.end(ordinal)
        assert len(self.wire) == parsed["end"]

    def second(self, block: int, index: int, parsed: dict) -> None:
        assert len(self.wire) == parsed["start"]
        scope = entity("Second", block, index=index)
        ordinal = self.begin(scope)
        self.store(scope, "PairBytes", "B", parsed["pair_bytes"])
        self.scalar(scope, "TagByte", "B", parsed["tag_byte"])
        self.byte_array(scope, parsed["byte_array"])
        self.end(ordinal)
        assert len(self.wire) == parsed["end"]

    def third(self, block: int, index: int, parsed: dict) -> None:
        assert len(self.wire) == parsed["start"]
        scope = entity("Third", block, index=index)
        ordinal = self.begin(scope)
        self.scalar(scope, "ThirdWord", "i", parsed["word_i32"])
        self.store(scope, "ThirdWide", "Q", parsed["wide_bits"])
        self.end(ordinal)
        assert len(self.wire) == parsed["end"]

    def fourth(self, block: int, parsed: dict) -> None:
        scope = entity("Fourth", block)
        ordinal = self.begin(scope)
        self.scalar(scope, "FourthCount", "B", parsed["fourth_count"])
        self.store(scope, "FourthWide", "Q", parsed["fourth_wide_bits"])
        self.store(scope, "FourthBytes", "B", parsed["fourth_bytes"])
        self.end(ordinal)

    def nested(self, block: int, rows: list) -> None:
        for row, entries in enumerate(rows):
            scope = entity("NestedRow", block, row)
            ordinal = self.begin(scope)
            self.scalar(scope, "NestedCount", "B" if self.version < 5 else "i", len(entries))
            for index, entry in enumerate(entries):
                self.nested_entry(entity("NestedEntry", block, row, index), entry)
            self.end(ordinal)

    def nested_entry(self, scope: Entity, parsed: dict) -> None:
        assert len(self.wire) == parsed["start"]
        ordinal = self.begin(scope, parsed["native_index"], parsed["stored_index"] is not None)
        self.scalar(scope, "NestedElementCount", "B", parsed["count"])
        if parsed["stored_index"] is not None:
            self.scalar(scope, "NestedIndex", "i", parsed["stored_index"])
        self.store(scope, "NestedWords", "i", parsed["words_i32"])
        self.store(scope, "NestedWide", "Q", parsed["wide_bits"])
        self.end(ordinal)
        assert len(self.wire) == parsed["end"]

    def final_tail(self, block: int, parsed: dict) -> None:
        scope = entity("Final", block)
        ordinal = self.begin(scope)
        self.scalar(scope, "FinalWord", "i", parsed["word_i32"])
        self.scalar(scope, "FinalCount", "i", parsed["count"])
        self.store(scope, "FinalWords", "i", parsed["words_i32"])
        self.end(ordinal)

    def extra_tail(self, block: int, parsed: dict) -> None:
        scope = entity("Extra", block)
        ordinal = self.begin(scope)
        self.scalar(scope, "ExtraWord", "i", parsed["word_i32"])
        self.mixed(entity("ExtraMixed", block), parsed["mixed"])
        self.end(ordinal)

    def body(self, block: int, parsed: dict) -> None:
        assert len(self.wire) == parsed["start"]
        scope = entity("Body", block)
        ordinal = self.begin(scope)
        for name, kind, visit in (("first", "FirstCount", self.first), ("second", "SecondCount", self.second), ("third", "ThirdCount", self.third)):
            self.scalar(scope, kind, "B", len(parsed[name]))
            for index, item in enumerate(parsed[name]):
                visit(block, index, item)
        self.fourth(block, parsed)
        self.body_tail(block, parsed)
        self.end(ordinal)
        assert len(self.wire) == parsed["end"]

    def body_tail(self, block: int, parsed: dict) -> None:
        scope = entity("Body", block)
        self.scalar(scope, "OuterFlag", "B", parsed["outer_presence_byte"])
        if parsed["nested_presence_byte"] is not None:
            self.scalar(scope, "NestedFlag", "B", parsed["nested_presence_byte"])
        if parsed["nested"] is not None:
            self.nested(block, parsed["nested"])
        if parsed["final_presence_byte"] is not None:
            self.scalar(scope, "FinalFlag", "B", parsed["final_presence_byte"])
        if parsed["final"] is not None:
            self.final_tail(block, parsed["final"])
        self.extra_and_last(block, parsed)

    def extra_and_last(self, block: int, parsed: dict) -> None:
        scope = entity("Body", block)
        if parsed["extra_presence_byte"] is not None:
            self.scalar(scope, "ExtraFlag", "B", parsed["extra_presence_byte"])
        if parsed["extra"] is not None:
            self.extra_tail(block, parsed["extra"])
        if parsed["last_byte"] is not None:
            self.scalar(scope, "LastByte", "B", parsed["last_byte"])

    def record(self, parsed: dict) -> None:
        ordinal = self.begin(entity("Record"))
        self.scalar(entity("Record"), "Version", "i", parsed["version"])
        for block in parsed["blocks"]:
            self.block(block)
        self.end(ordinal)
        assert len(self.wire) == parsed["end"]

    def block(self, parsed: dict) -> None:
        assert len(self.wire) == parsed["start"]
        block = parsed["ordinal"]
        scope = entity("Block", block)
        ordinal = self.begin(scope)
        self.scalar(scope, "Token", "i", parsed["token_i32"])
        if parsed["body_flag_byte"] is not None:
            self.scalar(scope, "BodyFlag", "B", parsed["body_flag_byte"])
        if parsed["body"] is not None:
            self.body(block, parsed["body"])
        self.end(ordinal)
        assert len(self.wire) == parsed["end"]

    def packet(self) -> bytes:
        raw = struct.pack("<I", len(self.scopes))
        for scope, start, end, native_index, stored in self.scopes:
            raw += entity_packet(scope) + struct.pack("<II", start, end)
            raw += bytes((0 if native_index is None else 2 if stored else 1,))
            if native_index is not None:
                raw += struct.pack("<i", native_index)
        return raw + struct.pack("<I", len(self.fields)) + b"".join(self.fields)


def case_packet(case: dict) -> bytes:
    parsed = baseline.flowblockextras.read_decoded(case["decoded"], leaf_count=case["leaves"], enabled=True)
    oracle = FieldOracle(parsed["version"])
    oracle.record(parsed)
    assert oracle.wire == case["decoded"]
    oracle.verify_coverage()
    return baseline.blob(case["name"].encode()) + oracle.packet()
