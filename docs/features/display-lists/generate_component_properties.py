"""Complete independent component-property roots at every admitted body boundary."""
import json
import struct
import hashlib
from pathlib import Path
from functools import partial
from generate_earlier_range import converted as convert_root,generate as generate_root,write_group
from generate_early_appearance_roots import item_bytes,face_identity_trace
from generate_early_view_roots import entry_budget as early_entries
from early_appearance_patches import rv_source
from ui_object_patches import convert,native_root
from component_property_wire import body,entries
import grouped_component_patches
from generate_grouped_components import entry_budget as grouped_entries
import lower_range_patches
from generate_lower_range import entry_budget as lower_entries
from generate_earliest_scene_roots import raw_nan_reader
from swarchive import Archive

VERSIONS = [5000,0x13ee,0x13ef,0x147b,0x147c,0x23b2,0x23b3,0x3738,0x3739,19000]


def converted(document: object, path: object, raw: bytes, version: int, kind: str, scenario: int, class_name: str, variant: int) -> bytes:
    raw = rv_source(document,path,raw)
    if version==19000:return native_root(document,path,raw,class_name,variant,encoder=body)
    if version<6000:
        converter = partial(convert,class_name=class_name,variant=variant,encoder=body)
        return convert_root(document,path,raw,version,kind,0,converter,b"",item_bytes(version))
    parent = grouped_component_patches.convert if version<10000 else lower_range_patches.convert
    converter = partial(convert,class_name=class_name,variant=variant,encoder=body,base_converter=parent)
    configuration = b"" if version<10000 else None
    items = None if version<10000 else bytes(2)
    return convert_root(document,path,raw,version,kind,0,converter,configuration,items)


def base_entries(version: int, kind: str) -> int:
    if version==19000:return 143 if kind=="part" else 148
    if version<6000:return early_entries(version,kind,0)+2
    if version<10000:return grouped_entries(version,kind,0)
    return lower_entries(version,kind)


def property_root(version: int, kind: str, scenario: int) -> tuple[bytes,dict[str,object]]:
    class_name = "moComponentPropertyHolder" if scenario>=16 else "moComponentProperty"
    variant = scenario%16
    converter = partial(converted,class_name=class_name,variant=variant)
    counter = lambda v,k,s:base_entries(v,k)+entries(v,class_name,variant)
    framed,row = generate_root(version,kind,0,converter,counter,face_identity_trace)
    row["scenario"],row["class_name"] = scenario,class_name
    return framed[:5]+bytes([scenario])+framed[6:],row


def generate(version: int, kind: str, scenario: int) -> tuple[bytes,dict[str,object]]:
    # #35: preserve the frozen signaling payload even when Python's native promotion quiets it.
    original = Archive.f32
    Archive.f32 = raw_nan_reader(original)
    try:return property_root(version,kind,scenario)
    finally:Archive.f32 = original


def write_budgets() -> None:
    rows = []
    for name in ["component-properties","component-property-holders-part","component-property-holders-assembly"]:
        rows += json.loads(Path(__file__).with_name(name+".json").read_text(encoding="utf-8"))["fixtures"]
    raw = struct.pack("<I",len(rows))+b"".join(struct.pack("<II",row["aggregate_entries"],row["objects"]) for row in rows)
    Path("tests/fixtures/display-lists/component-property-budgets.bin").write_bytes(raw)
    report = {"fixtures":len(rows),"sha256":hashlib.sha256(raw).hexdigest(),"scope":"independent collection arithmetic and reference object loads in fixture order"}
    Path(__file__).with_name("component-property-budgets.json").write_text(json.dumps(report,indent=2)+"\n",encoding="utf-8",newline="\n")


if __name__=="__main__":
    totals = []
    cases = [generate(v,k,s) for v in VERSIONS for k in ["part","assembly"] for s in [0,2]]
    totals.append(write_group("component-properties",cases))
    for kind in ["part","assembly"]:
        cases = [generate(v,kind,16+s) for v in VERSIONS for s in [0,2]]
        cases += [generate(19000,kind,16+s) for s in [1,3,4,5]]
        totals.append(write_group("component-property-holders-"+kind,cases))
    write_budgets()
    print(json.dumps({"fixtures":sum(n for n,_ in totals),"fields":sum(n for _,n in totals)}))
