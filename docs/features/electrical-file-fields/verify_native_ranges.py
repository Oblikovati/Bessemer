"""Authenticate every retained descriptor, literal and wrapper against original PE bytes."""
from __future__ import annotations
import hashlib
import json
from pathlib import Path
from typing import TypedDict, cast
import pefile
from freeze_field_maps import ROOT, SOURCE


class NativeRange(TypedDict):
    address: str
    end: str
    native_bytes: str


class Initializer(NativeRange):
    name: str
    literal: str
    literal_bytes: str


class Descriptor(TypedDict):
    name: str
    initializer: Initializer


class Artifact(TypedDict):
    sha256: str


class NativeReport(TypedDict):
    artifact: Artifact
    reader: NativeRange
    fields: list[Descriptor]
    recordset_wrappers: dict[str,NativeRange]


def byte_binding(raw: bytes, executable: pefile.PE, address: str, expected: bytes) -> dict[str,str | int]:
    rva = int(address,16)-executable.OPTIONAL_HEADER.ImageBase
    offset = executable.get_offset_from_rva(rva)
    actual = raw[offset:offset+len(expected)]
    assert actual==expected,(address,len(actual),len(expected))
    return dict(address=address,file_offset=offset,bytes=len(actual),sha256=hashlib.sha256(actual).hexdigest())


def native_binding(raw: bytes, executable: pefile.PE, record: NativeRange) -> dict[str,str | int]:
    expected = bytes.fromhex(record["native_bytes"])
    assert int(record["end"],16)-int(record["address"],16)==len(expected)
    return byte_binding(raw,executable,record["address"],expected)


def verify() -> None:
    report = cast(NativeReport,json.loads(SOURCE.read_text(encoding="utf-8")))
    binary = Path("C:/Program Files/SOLIDWORKS Corp/SOLIDWORKS Electrical/bin/ewadobject.dll")
    raw = binary.read_bytes()
    digest = hashlib.sha256(raw).hexdigest()
    assert digest==report["artifact"]["sha256"]
    executable = pefile.PE(data=raw,fast_load=True)
    bindings = [native_binding(raw,executable,report["reader"])]
    bindings += [native_binding(raw,executable,record) for record in report["recordset_wrappers"].values()]
    for field in report["fields"]:
        initializer = field["initializer"]
        literal = bytes.fromhex(initializer["literal_bytes"])
        assert literal==(field["name"]+"\0").encode("utf-16-le")
        bindings.append(native_binding(raw,executable,initializer))
        bindings.append(byte_binding(raw,executable,initializer["literal"],literal))
    assert len(bindings)==82 and hashlib.sha256(binary.read_bytes()).hexdigest()==digest
    result = dict(artifact=str(binary),sha256=digest,source_sha256=hashlib.sha256(SOURCE.read_bytes()).hexdigest(),checked_ranges=bindings,native_execution=False,scope="Complete reader, five typed wrappers, 38 complete descriptor initializers and 38 exact UTF-16 literals; provider conversions and final native effects not executed")
    (ROOT / "docs/features/electrical-file-fields/native-byte-verification.json").write_text(json.dumps(result,indent=2)+"\n",encoding="utf-8")
    print(dict(sha256=digest,ranges=len(bindings)))


if __name__=="__main__":
    verify()
