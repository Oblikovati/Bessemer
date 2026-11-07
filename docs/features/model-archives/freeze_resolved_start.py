"""Independent resolved-manager wire cases and the owned native handoff."""
from pathlib import Path
import hashlib
import json
import struct
import sys
from freeze_blank_part import reference_snapshot
from freeze_node_units import object_tag, oracle

WORKSPACE = Path(__file__).resolve().parents[3]
REFERENCE = WORKSPACE.parent / "inventor-interface/solidworks"
sys.path.insert(0, str(REFERENCE))
import modelprefix
import modelresolved
import modelrelations
import modelheader
import dl_light
from mfcobjects import ObjectArchive
from swcontainer import Document


def node_list() -> bytes:
    return struct.pack("<HIHHi", 65535, 2, 0, 0, -19)


def atom_body(mode: int) -> bytes:
    node = struct.pack("<HiI", 0, -3, 0x80000001) + struct.pack("<iI", -7, 0xfedcba98) + b"\0"
    concept = struct.pack("<i", -1) + node_list() + b"\0\0" + node_list()
    tail = struct.pack("<iIiiIi", -11, 0x89abcdef, -13, 17, 0xffffffff, mode)
    if mode == 2: tail += struct.pack("<i", -23)
    return node + concept + tail + struct.pack("<7i", -29, 31, -37, 41, -43, 47, -53)


def ghost_body() -> bytes:
    out = struct.pack("<4ii3iiH", -1, 3, -5, 7, 3, 11, 11, -13, -1, 0)
    return out + struct.pack("<i6Q", 2, 0, 1 << 63, 0x7ff000000000002a, 1, 0x3ff0000000000000, 0x4000000000000000)


def relation_body(version: int) -> bytes:
    out = struct.pack("<H", 2) + object_tag("moNodeName_c") + b"\x01x" + struct.pack("<H", 2)
    out += struct.pack("<HI", 65535, 2) + b"\x01k" + struct.pack("<d", -0.0) + b"\x01k" + struct.pack("<d", 1.25)
    out += b"\x01z" + struct.pack("<iIiiHH", -3, 0xfedcba98, 5, -7, 1, 2)
    if version > 0x336a: out += struct.pack("<iiHH", -11, 2, 0, 0)
    return out


def cases() -> list[tuple[int, int, bytes]]:
    result = [(0, 19000, node_list())]
    for mode in (0, 2):
        atom = object_tag("moAtom_c") + atom_body(mode)
        result += [(5, 19000, atom), (1, 19000, struct.pack("<ii", -3, 2) + atom + struct.pack("<HI", 2, 0xfedcba98))]
    for flags in range(16): result.append((2, 19000, struct.pack("<iI", -7, flags << 28 | 1) + ghost_body()))
    for version in (0x27cc, 0x336a, 0x336b, 19000, 0xffffffff):
        result.append((3, version, object_tag("moRelMgr_c") + relation_body(version)))
    return result


def read_case(kind: int, archive: ObjectArchive) -> object:
    if kind == 0: return dl_light.read_moNodeList_c(archive)
    if kind == 1: return modelresolved.atom_manager(archive)
    if kind == 2:
        value = modelresolved.ghost_manager(archive)
        del value["flags"], value["count"]
        return value
    return archive.object()


def freeze_case(kind: int, version: int, wire: bytes) -> tuple[bytes, int]:
    archive = ObjectArchive(wire, version)
    archive.feature_context = "ordinary_external_xml"
    value = read_case(kind, archive)
    if archive.p != len(wire): raise ValueError(f"section {kind} end {archive.p}; expected {len(wire)}")
    expected = oracle("root.object" if kind in (3, 5) else "root", value)
    return struct.pack("<BIII", kind, version, len(wire), len(expected)) + wire + expected, len(expected.splitlines())


def native_handoff(destination: Path) -> dict[str, object]:
    document = Document(WORKSPACE / "tests/fixtures/blank-part.SLDPRT", table={})
    root, archive = modelprefix.read(document, through_relation_manager=True)
    value = {key: child for key, child in root["resolved"].items() if key not in ("size", "end")}
    ghost = value["ghost_manager"]
    del ghost["flags"], ghost["count"]  # Derived views of packed_count have no separate wire fields.
    value["array_b00"] = modelheader.object_list(archive, 1)
    start, end = root["lightweight_end"] + 4, archive.p
    raw, expected = archive.b[start:end], oracle("root", value)
    (destination / "native-resolved-start.bin").write_bytes(raw)
    (destination / "native-resolved-start.txt").write_bytes(expected)
    relation_index = next(index for index, item in enumerate(archive.map) if item is value["relation_manager"])
    return {"start": start, "end": end, "bytes": len(raw), "fields": len(expected.splitlines()),
            "relation_object_index": relation_index, "sha256": hashlib.sha256(raw).hexdigest()}


def generate() -> None:
    before = reference_snapshot(REFERENCE)
    frozen = [freeze_case(*case) for case in cases()]
    payload = b"".join(wire for wire, _ in frozen)
    destination = WORKSPACE / "tests/fixtures/model-archives"
    native = native_handoff(destination)
    if before != reference_snapshot(REFERENCE): raise ValueError("reference sources changed; expected stable hashes")
    (destination / "resolved-manager-cases.bin").write_bytes(payload)
    report = {"cases": len(frozen), "fields": sum(count for _, count in frozen), "sha256": hashlib.sha256(payload).hexdigest(),
              "native": native, "source_hashes": before, "generator_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    Path(__file__).with_name("resolved-start-reference.json").write_text(json.dumps(report, sort_keys=True, separators=(",", ":")) + "\n")
    print({key: value for key, value in report.items() if key != "source_hashes"})


if __name__ == "__main__":
    generate()
