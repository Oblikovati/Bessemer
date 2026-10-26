"""Independent owned node/unit wire cases for the complete model-root implementation."""
from pathlib import Path
import hashlib
import json
import struct
import sys
from freeze_blank_part import reference_snapshot
from owned_name_edit import short_wide_name

WORKSPACE = Path(__file__).resolve().parents[3]
REFERENCE = WORKSPACE.parent / "inventor-interface/solidworks"
sys.path.insert(0, str(REFERENCE))
sys.path.insert(0, str(WORKSPACE / "docs/features/display-lists"))
import modelnode
import modelunits
import modelprefix
from mfcobjects import ObjectArchive
from swcontainer import Document
from generate_blank_scalars import flatten

VERSIONS = [0,0x68,0x69,0x1aa,0x1ab,0x305,0x306,0x822,0x823,0x926,0x927,
            0x92e,0x92f,0xac6,0xac7,0xdce,0xdcf,0x1041,0x1042,0x1bd1,0x1bd2,
            0x1bd6,0x1bd7,0x1faa,0x1fab,19000,0xffffffff]
BASE = ["moUserUnits_c","moNumberUserUnits_c","moFloatNumberUserUnits_c",
        "moSpringConstantUnits_c","moStressUnits_c","moGravityUnits_c",
        "moLinearMotorUnits_c","moRotaryMotorUnits_c","moFrequencyUserUnits_c"]
CODED = ["moAngleUserUnits_c","moUnitSysUnits_c","moForceUnits_c"]
ENERGY = ["moPowerUnits_c","moEnergyUnits_c","moTimeUnits_c"]
LENGTH = ["moLengthUserUnits_c","moPercentUserUnits_c"]


def object_tag(name: str, schema: int = 7) -> bytes:
    encoded = name.encode("ascii")
    return struct.pack("<HHH",65535,schema,len(encoded)) + encoded


def base_wire(version: int) -> bytes:
    out = bytearray(struct.pack("<iidii",-5,7,1.25,-9,11))
    if version>0x822:out.extend(struct.pack("<Q",0x8000000000000000))
    out.extend(struct.pack("<i",13))
    for threshold,value in ((0x1aa,-17),(0x305,19),(0x1041,-23)):
        if version>threshold:out.extend(struct.pack("<i",value))
    if version>=0x1bd2:out.extend(struct.pack("<QH",0x7ff000000000002a,65530))
    if version>0x1bd6:out.extend(struct.pack("<i",-29))
    return bytes(out)


def density_wire(version: int, present: bool) -> bytes:
    out = bytearray()
    if version>0x92e:out.extend(struct.pack("<HH",65529,65528))
    if version<=0xac6:return bytes(out)
    out.append(255 if present else 0)
    if present:out.extend(unit_object("moLengthUserUnits_c",version,4))
    if version>0x1faa:out.extend(struct.pack("<Hi",65527,-31))
    return bytes(out)


def unit_object(name: str, version: int, identity: int = 2, present: bool = True) -> bytes:
    tag = object_tag(name)
    if name=="moUnitsTable_c":
        return tag + struct.pack("<HI",65535,3) + unit_object("moLengthUserUnits_c",version,4) + struct.pack("<HH",4,0)
    body = bytearray(base_wire(version))
    if name in CODED or name in LENGTH or name in ENERGY and version>0xdce:
        body.extend(struct.pack("<H",65526))
    if name in LENGTH and version>0x926:body.extend(struct.pack("<H",identity))
    if name=="moDensityUnits_c":body.extend(density_wire(version,present))
    return tag + body


def node_wire(version: int) -> bytes:
    out = object_tag("moNodeName_c") + short_wide_name("Ωx") + struct.pack("<iIi",-3,0x80000001,-7)
    if version>0x143a:out+=struct.pack("<I",0xfffffffe)
    if version>0x23cd:out+=short_wide_name("tail")
    return out


def scalar_rows(path: str, value: object, rows: dict[str,str], seen: set[int]) -> None:
    if isinstance(value,dict):
        if id(value) in seen:return
        seen.add(id(value))
        for key,child in value.items():
            if not key.startswith("_"):scalar_rows(path+"."+key,child,rows,seen)
        return
    if isinstance(value,(list,tuple)):
        for index,child in enumerate(value):scalar_rows(f"{path}[{index}]",child,rows,seen)
        return
    if value is not None:flatten(path,value,rows)


def oracle(path: str, value: object) -> bytes:
    rows: dict[str,str] = {}
    scalar_rows(path,value,rows,set())
    return "".join(f"{key}={value}\n" for key,value in sorted(rows.items())).encode()


def owned_case(version: int, raw: bytes, node: bool) -> tuple[bytes,int]:
    archive = ObjectArchive(raw,version)
    value = modelnode.lightweight(archive) if node else archive.object()
    if archive.p!=len(raw):raise ValueError(f"reference end {archive.p}; expected {len(raw)}")
    expected = oracle("root" if node else "root.unit",value)
    return struct.pack("<III",version,len(raw),len(expected))+raw+expected,len(expected.splitlines())


def write_cases(destination: Path, cases: list[tuple[int,bytes]], node: bool) -> dict[str,object]:
    framed = [owned_case(version,raw,node) for version,raw in cases]
    payload = b"".join(frame for frame,_ in framed)
    destination.write_bytes(payload)
    return {"cases":len(cases),"fields":sum(count for _,count in framed),
            "sha256":hashlib.sha256(payload).hexdigest()}


def native_units_start(archive: ObjectArchive, schema: int) -> int:
    expected = object_tag("moUnitsTable_c",schema)
    start = archive.p-len(expected)
    actual = archive.b[start:archive.p]
    if actual!=expected:raise ValueError(f"native units tag {actual.hex()}; expected {expected.hex()}")
    return start


def native_records(destination: Path) -> dict[str,object]:
    document = Document(WORKSPACE/"tests/fixtures/blank-part.SLDPRT",table={})
    observed: list[tuple[int,int,object]] = []
    original = ObjectArchive.readers["moUnitsTable_c"]
    def capture(archive: ObjectArchive, schema: int) -> dict[str,object]:
        start = native_units_start(archive,schema)
        value = original(archive,schema)
        observed.append((start,archive.p,value))
        return value
    ObjectArchive.readers["moUnitsTable_c"] = capture
    try:_,archive=modelprefix.read(document,through_part_resolved=True)
    finally:ObjectArchive.readers["moUnitsTable_c"]=original
    if archive.p!=len(archive.b) or len(observed)!=1:raise ValueError("native model; expected EOF and one units table")
    start,end,value = observed[0]
    raw = archive.b[start:end]
    (destination/"native-units.bin").write_bytes(raw)
    (destination/"native-units.txt").write_bytes(oracle("root.units",value))
    return {"start":start,"end":end,"bytes":len(raw),"sha256":hashlib.sha256(raw).hexdigest()}


def generate() -> None:
    before = reference_snapshot(REFERENCE)
    destination = WORKSPACE/"tests/fixtures/model-archives"
    destination.mkdir(parents=True,exist_ok=True)
    names = BASE+CODED+ENERGY+LENGTH+["moDensityUnits_c","moUnitsTable_c"]
    cases = [(v,unit_object(n,v,present=p)) for v in VERSIONS for n in names
             if n!="moUnitsTable_c" or v>=0x69
             for p in ([False,True] if n=="moDensityUnits_c" and v>0xac6 else [True])]
    report = {"units":write_cases(destination/"unit-boundaries.bin",cases,False)}
    versions = [0xc7d,0x143a,0x143b,0x23cd,0x23ce,19000,0xffffffff]
    report["nodes"] = write_cases(destination/"node-boundaries.bin",[(v,node_wire(v)) for v in versions],True)
    report["native_units"] = native_records(destination)
    if before!=reference_snapshot(REFERENCE):raise ValueError("reference sources changed; expected stable hashes")
    report["source_hashes"] = before
    report["generator_sha256"] = hashlib.sha256(Path(__file__).read_bytes()).hexdigest()
    Path(__file__).with_name("node-unit-reference.json").write_text(json.dumps(report,sort_keys=True,separators=(",",":"))+"\n")
    print({key:value for key,value in report.items() if key!="source_hashes"})


if __name__=="__main__":
    generate()
