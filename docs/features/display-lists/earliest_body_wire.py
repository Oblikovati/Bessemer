"""Independent direct body-property maps with duplicate keys and relocated later MFC tags."""
import struct
from generate_alias_fixture import collect
from generate_component_vectors import integers
from generate_version_range import apply_patches
from transient_viewer_wire import shifted_tag


def direct_properties(document: object, path: object, raw: bytes, version: int, null_only: bool) -> bytes:
    if version>=0x10f6:return raw
    records = collect(document,path,raw,version,set())
    targets = [record for record in records if record[3]=="uoBodyPropInfo_c"]
    assert len(targets)==1
    start,end,identity,_ = targets[0]
    visual_ids = {record[2] for record in records if record[3]=="moVisualProperties_c"}
    assert len(visual_ids)==1 and raw[start:start+2]==b"\xff\xff"
    visual = next(iter(visual_ids))
    assert visual<identity-1 and raw[start-8:start]==integers(1,-1)
    entries = [0] if null_only else [visual,0]
    replacement = integers(len(entries))+b"".join(integers(-1)+struct.pack("<H",index) for index in entries)
    patches = [(start-8,end,replacement)]
    for position,_,_,_ in records:
        if position<end:continue
        shifted = shifted_tag(raw,position,identity+1,-2)
        if shifted is not None:patches.append((position,position+len(shifted),shifted))
    return apply_patches(raw,patches)
