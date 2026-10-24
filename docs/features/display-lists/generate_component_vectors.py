"""Independent modern component bodies with populated arrays and both display-mode layouts."""
from pathlib import Path
import hashlib
import json
import struct
from generate_owner_cycle import scalar_oracle
from generate_alias_fixture import collect, swcontainer
from generate_named_vectors import cstring
from generate_transform_vectors import compact, doubles


def integers(*values: int) -> bytes:
    return struct.pack("<"+"i"*len(values), *values)


def string_list(values: list[str]) -> bytes:
    return integers(len(values))+b"".join(cstring(value, True) for value in values)


def viewer(mode: int) -> bytes:
    raw = cstring("component 鋼", True)+compact(2, 0x40)+doubles(10)
    raw += struct.pack("<H", 0)+cstring("configuration", True)
    raw += struct.pack("<iII", -3, 0xffffffff, 123)
    raw += cstring("model", True)+cstring("C:\\example\\model", True)+integers(-17)
    raw += integers(3, -1, 0, 0x7fffffff)
    raw += integers(mode) if mode == 6 else integers(mode, -4, 8)
    raw += struct.pack("<Q", 0x8000000000000000)+integers(-1, 2, 3, -4)
    raw += struct.pack("<I", 0xfedcba98)+integers(7, 8, 9, 10, 11)
    raw += b"".join(cstring(value, True) for value in ["ref", "spool", "description 鋼", "config"])
    raw += string_list(["first", "second 鋼"])+integers(2)
    for key, values in [("key0", []), ("key1", ["value0", "value1"] )]:
        raw += cstring(key, True)+string_list(values)
    raw += integers(12, 13, 14, 15, 16, 17, 18)+struct.pack("<H", 0xffff)
    raw += integers(19, 20, 0)+cstring("seed", True)
    raw += struct.pack("<II", 0, 0)+integers(2, -1, 2, 3, -4, 5, 6)
    raw += cstring("state 鋼", True)+integers(-99)+bytes([255])
    return raw


def shaded(mode: int) -> bytes:
    raw = viewer(mode)+struct.pack("<H", 0)+integers(-21)
    raw += struct.pack("<HIH", 0xffff, 0, 0)
    raw += integers(17)+struct.pack("<I", 0)+integers(-22)
    return raw+integers(3, -1, 0, 0x7fffffff)


def main() -> None:
    path = Path("tests/fixtures/blank-assembly.SLDASM")
    document = swcontainer.Document(str(path))
    source = document.read("Contents/DisplayLists")
    start, end, _, _ = next(record for record in collect(document, path, source) if record[3] == "uoTempSubAssemblySHDData_c")
    assert source[start:start+2] == b"\xff\xff"
    name_length, = struct.unpack_from("<H", source, start+4)
    body_start = start+6+name_length
    rows: list[dict[str, object]] = []
    for mode in [6, -3]:
        derived = source[:body_start]+shaded(mode)+source[end:]
        destination = Path(f"tests/fixtures/display-lists/component-mode-{mode}.bin")
        destination.write_bytes(derived)
        fields = scalar_oracle(document, path, derived, destination.with_suffix(".txt"))
        rows.append({"mode": mode, "bytes": len(derived), "sha256": hashlib.sha256(derived).hexdigest(), "scalar_oracle_fields": fields})
    Path(__file__).with_name("component-vectors.json").write_text(json.dumps(rows, indent=2)+"\n", encoding="utf-8", newline="\n")


if __name__ == "__main__":
    main()
