"""Independent complete roots bracketing 6000-range scene and component layouts."""
from generate_earlier_range import generate as generate_root,converted as convert_root,write_group
from generate_early_view_roots import display_items,entry_budget
from historical_scene_patches import convert
import json
import struct
from generate_item_vectors import payloads,record

VERSIONS = [6000,0x17dc,0x17dd,0x17f3,0x17f4,0x17f5,0x17f6,0x1827,0x1828,0x183d,0x183e,0x1841,0x1842,0x184c,0x184d,0x1859,0x185a,0x185e,0x185f,0x186c,0x186d,6999,7000]


def converted(document: object, path: object, raw: bytes, version: int, kind: str, variant: int) -> bytes:
    original = display_items(version)
    pen = payloads(False)[9]
    if version<0x185a:pen = pen[:-4]
    items = struct.pack("<H",4)+original[2:]+record(9,pen)
    return convert_root(document,path,raw,version,kind,variant,convert,b"",items)


def generate(version: int, kind: str, scenario: int = 0) -> tuple[bytes,dict[str,object]]:
    variant = scenario%64
    framed,row = generate_root(version,kind,variant,converted,lambda v,k,s:entry_budget(v,k,s)+1)
    row["scenario"] = 192+variant
    return framed[:5]+bytes([192+variant])+framed[6:],row


if __name__=="__main__":
    totals = []
    for kind in ["part","assembly"]:
        cases = [generate(version,kind) for version in VERSIONS]+[generate(6000,kind,1)]
        totals.append(write_group("historical-roots-"+kind,cases))
    print(json.dumps({"fixtures":sum(count for count,_ in totals),"fields":sum(fields for _,fields in totals)}))
