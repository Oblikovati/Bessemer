"""Freeze independently constructed section cases after reference EOF checks."""
from pathlib import Path
import hashlib
import json
import struct
import sys
from freeze_blank_part import reference_snapshot

WORKSPACE = Path(__file__).resolve().parents[3]
REFERENCE = WORKSPACE.parent / "inventor-interface/solidworks"
sys.path.insert(0, str(REFERENCE))
import modelprefix
import modelmanager
import modelpostmanager
import modelspatial
import partlightweight
from mfcobjects import ObjectArchive

READERS = [modelpostmanager.read, modelpostmanager.flags, modelpostmanager.collections,
           modelpostmanager.model_tail, partlightweight.tail, modelspatial.read,
           modelspatial.metadata, modelmanager.read]
VERSIONS = [0x27cc, 0x27cd, 0x27d0, 0x27d1, 0x2b6c, 0x2b6d, 0x2bce,
            0x2bcf, 0x2f72, 0x2f73, 0x46bb, 0x46bc, 0x46c5, 0x46c6, 19000, 0xffffffff]


def model_tail(version: int) -> bytes:
    out = bytes(range(16)) + struct.pack("<i", -3) + bytes(range(16, 32))
    out += struct.pack("<iiii", -5, 7, -9, 0)
    if version > 0x27cc: out += struct.pack("<i", 11)
    if version > 0x2b6c: out += struct.pack("<I", 0xfedcba98)
    if version > 0x46bb: out += struct.pack("<i", -13)
    if version > 0x46c5: out += struct.pack("<iIi", 1, 0x89abcdef, -17)
    if version > 0x2bce: out += b"\0\0"
    return out


def version_cases(version: int) -> list[tuple[int, int, bytes]]:
    flags = struct.pack("<ii", 13, 0)
    if version > 0x2f72: flags += struct.pack("<i", -1)
    flags += struct.pack("<i", 17)
    tail = struct.pack("<iii", -3, 5, -7)
    if version > 0x27d0: tail += struct.pack("<iii", 1, -11, 13)
    return [(1, version, flags), (3, version, model_tail(version)), (4, version, tail)]


def fixed_cases() -> list[tuple[int, int, bytes]]:
    arrays = [(0, 19000, struct.pack("<i", 0)), (0, 19000, struct.pack("<iI", -1, 0))]
    for size in (24, 48):
        arrays.append((0, 19000, struct.pack("<iIIII", -1, size, 9, 3, 2) + bytes(range(size * 2))))
    collections = struct.pack("<HIHHi", 65535, 2, 0, 0, -4)
    collections += struct.pack("<i", 1) + b"\x01x" + struct.pack("<iHH", 2, 0, 0)
    spatial = b"\xff\0\0\0\0\xff" + struct.pack("<10di", *[i / 4 for i in range(10)], -7)
    metadata = struct.pack("<iiHiii", 1, -5, 0, -7, 1, -9)
    metadata += struct.pack("<i", 1) + b"\x01x\0\0" + struct.pack("<dd", -0.0, 1.25)
    manager = struct.pack("<iHI", 2, 65535, 1) + b"\x01x\0\0" + struct.pack("<ii", -3, 5)
    return arrays + [(2, 19000, collections), (5, 19000, spatial), (6, 19000, metadata),
                     (7, 19000, manager), (7, 19000, struct.pack("<i", -1))]


def normalize(kind: int, value: dict[str, object]) -> dict[str, object]:
    if kind != 0 or value.get("array") is None:
        return {key: child for key, child in value.items() if child is not None}
    array = dict(value["array"])
    payload = array.pop("unknown_data")
    size = array["element_size"]
    array["elements"] = [list(payload[offset:offset + size]) for offset in range(0, len(payload), size)]
    return {"has_array": value["has_array"], **array}


def scalar_rows(path: str, value: object, rows: list[str]) -> None:
    if value is None: return
    if isinstance(value, dict):
        for key, child in value.items(): scalar_rows(f"{path}.{key}", child, rows)
    elif isinstance(value, (list, tuple, bytes)):
        for index, child in enumerate(value): scalar_rows(f"{path}[{index}]", child, rows)
    elif isinstance(value, float): rows.append(f"{path}=f:{struct.pack('>d', value).hex()}\n")
    elif isinstance(value, int): rows.append(f"{path}=i:{value}\n")
    elif isinstance(value, str): rows.append(f"{path}=s:{value.encode('utf-8').hex()}\n")
    else: raise ValueError(f"{path}: {value!r}; expected an independently readable scalar")


def freeze_case(kind: int, version: int, wire: bytes) -> tuple[bytes, int]:
    archive = ObjectArchive(wire, version)
    archive.feature_context = "ordinary_external_xml"
    archive.model_root_class = "moPart_c"
    result = READERS[kind](archive)
    if archive.p != len(wire): raise ValueError(f"section {kind} end {archive.p}; expected {len(wire)}")
    rows: list[str] = []
    scalar_rows("root", normalize(kind, result), rows)
    oracle = "".join(rows).encode("utf-8")
    return struct.pack("<BIII", kind, version, len(wire), len(oracle)) + wire + oracle, len(rows)


def generate() -> None:
    before = reference_snapshot(REFERENCE)
    cases = fixed_cases() + [case for version in VERSIONS for case in version_cases(version)]
    frozen = [freeze_case(*case) for case in cases]
    if before != reference_snapshot(REFERENCE): raise ValueError("reference sources changed; expected stable hashes")
    payload = b"".join(wire for wire, _ in frozen)
    destination = WORKSPACE / "tests/fixtures/model-archives/lightweight-sections.bin"
    destination.write_bytes(payload)
    report = {"cases": len(cases), "scalar_fields": sum(count for _, count in frozen),
              "sha256": hashlib.sha256(payload).hexdigest(), "source_hashes": before,
              "generator_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    Path(__file__).with_name("lightweight-section-reference.json").write_text(json.dumps(report, sort_keys=True, separators=(",", ":")) + "\n")
    print({key: value for key, value in report.items() if key != "source_hashes"})


if __name__ == "__main__":
    generate()
