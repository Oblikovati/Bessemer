"""Independent version-3–6 grouped-word fixtures, with explicit compression context."""
from __future__ import annotations
import gzip
import hashlib
import json
import struct
import sys
import zlib
from pathlib import Path
from typing import Literal, NamedTuple, NotRequired, TypedDict, cast

ROOT = Path(__file__).resolve().parents[3]
SOURCE = ROOT.parent / "inventor-interface/solidworks"
sys.path.insert(0, str(SOURCE))
sys.path.insert(0, str(ROOT / "docs/features/flow-carriers"))
import flowgroups
import flowresults
from freeze_carriers import blob, owned

Compression = Literal["none", "zlib"]
Member = tuple[int, int, int]
GroupSpec = tuple[int, list[Member]]


class DecodedGroup(TypedDict):
    start: int
    end: int
    count: int
    raw_auxiliary: NotRequired[int]
    words: list[int]
    wide_words: list[int]
    flags: list[int] | None


class GroupPayload(TypedDict):
    version: int
    groups: list[DecodedGroup]


class GroupCase(NamedTuple):
    name: str
    wire: bytes
    ordinal: int
    compression: Compression


class NativeCapture(TypedDict):
    path: str
    source_sha256: str
    record_start: int
    record_end: int
    payload_sha256: str
    word_sha256: str
    wide_word_sha256: str
    flag_sha256: str


def group_wire(version: int, auxiliary: int, members: list[Member]) -> bytes:
    prefix = struct.pack("<i", len(members))
    if version == 4:
        prefix += struct.pack("<i", auxiliary)
    if version == 6:
        return prefix + b"".join(struct.pack("<IQB", *member) for member in members)
    words = b"".join(struct.pack("<I", member[0]) for member in members)
    wide = b"".join(struct.pack("<Q", member[1]) for member in members)
    flags = bytes(member[2] for member in members) if version == 5 else b""
    return prefix + words + wide + flags


def decoded_wire(version: int, groups: list[GroupSpec]) -> bytes:
    prefix = struct.pack("<ii", version, len(groups))
    return prefix + b"".join(group_wire(version, auxiliary, members) for auxiliary, members in groups)


def owned_version_cases(version: int) -> list[GroupCase]:
    members = [(0, 0, 0), (0xFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 255), (0x80000000, 0x7FF8000000000001, 128)]
    profiles: list[tuple[str, list[GroupSpec]]] = [
        ("no-groups", []), ("empty", [(-1, [])]),
        ("mixed", [(-0x80000000, []), (0x7FFFFFFF, members), (-1, members[:1])]),
    ]
    cases = []
    for name, groups in profiles:
        decoded = decoded_wire(version, groups)
        for compression in ("none", "zlib"):
            payload = decoded if compression == "none" else zlib.compress(decoded)
            wire = owned(8, [(123, bytes(32)), (404, payload), (19, b"opaque")])
            cases.append(GroupCase(f"v{version}-{name}-{compression}", wire, 1, compression))
    return cases


def decoded_case(case: GroupCase) -> GroupPayload:
    record = flowresults.read(case.wire)["records"][case.ordinal]
    return cast(GroupPayload, flowgroups.read_record(record, compression=case.compression))


def native_case(capture: NativeCapture) -> GroupCase:
    raw = Path(capture["path"]).read_bytes()
    assert hashlib.sha256(raw).hexdigest() == capture["source_sha256"]
    matches = [(index, record) for index, record in enumerate(flowresults.read(raw)["records"])
               if record["identifier"] == 404]
    assert len(matches) == 1
    ordinal, record = matches[0]
    assert (record["start"], record["end"]) == (capture["record_start"], capture["record_end"])
    assert hashlib.sha256(record["payload"]).hexdigest() == capture["payload_sha256"]
    case = GroupCase(Path(capture["path"]).name, raw, ordinal, "zlib")
    validate_native(decoded_case(case), capture)
    return case


def validate_native(decoded: GroupPayload, capture: NativeCapture) -> None:
    assert decoded["version"] == 5 and len(decoded["groups"]) == 4056
    for field, shape, digest in [("words", "<I", "word_sha256"),
                                  ("wide_words", "<Q", "wide_word_sha256"),
                                  ("flags", "<B", "flag_sha256")]:
        flattened = [value for group in decoded["groups"] for value in group[field]]
        wire = b"".join(struct.pack(shape, value) for value in flattened)
        assert hashlib.sha256(wire).hexdigest() == capture[digest]


def cases() -> list[GroupCase]:
    corpus = SOURCE / "research/flow-group-corpus.json"
    captures = cast(list[NativeCapture], json.loads(corpus.read_text()))
    native = [native_case(capture) for capture in captures]
    return native + [case for version in range(3, 7) for case in owned_version_cases(version)]


def expected_group(group: DecodedGroup) -> bytes:
    packet = struct.pack("<QQ", group["start"], group["end"])
    auxiliary = group.get("raw_auxiliary")
    packet += bytes([auxiliary is not None])
    if auxiliary is not None:
        packet += struct.pack("<i", auxiliary)
    flags = group["flags"]
    packet += bytes([flags is not None]) + struct.pack("<I", group["count"])
    for index in range(group["count"]):
        packet += struct.pack("<IQ", group["words"][index], group["wide_words"][index])
        if flags is not None:
            packet += bytes([flags[index]])
    return packet


def packet_case(case: GroupCase) -> bytes:
    decoded = decoded_case(case)
    record = flowresults.read(case.wire)["records"][case.ordinal]
    raw = record["payload"] if case.compression == "none" else zlib.decompress(record["payload"])
    prefix = blob(case.name.encode()) + blob(case.wire)
    prefix += struct.pack("<IB", case.ordinal, case.compression == "zlib") + blob(raw)
    prefix += struct.pack("<iI", decoded["version"], len(decoded["groups"]))
    return prefix + b"".join(expected_group(group) for group in decoded["groups"])


def freeze(output_root: Path = ROOT) -> None:
    selected = cases()
    packet = b"FGP1" + struct.pack("<I", len(selected)) + b"".join(packet_case(case) for case in selected)
    target = output_root / "tests/fixtures/flow-groups/groups.bin.gz"
    target.write_bytes(gzip.compress(packet, mtime=0))
    report = dict(cases=len(selected), native_carriers=2, unique_native_payloads=1,
                  raw_bytes=len(packet), sha256=hashlib.sha256(packet).hexdigest(),
                  reference_sha256=hashlib.sha256((SOURCE / "flowgroups.py").read_bytes()).hexdigest(),
                  inputs=[dict(name=case.name, compression=case.compression, bytes=len(case.wire),
                               sha256=hashlib.sha256(case.wire).hexdigest()) for case in selected])
    (output_root / "docs/features/flow-groups/fixture-verification.json").write_text(
        json.dumps(report, indent=2) + "\n", encoding="utf-8", newline="\n")
    print(dict(cases=len(selected), packet_bytes=len(packet)))


if __name__ == "__main__":
    freeze()
