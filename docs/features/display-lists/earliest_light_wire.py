"""Independent modern light definitions and populated hidden atoms before version conversion."""
import struct
from generate_alias_fixture import collect,displaylists
from generate_light_vectors import base,payload
from generate_transform_vectors import doubles
from generate_component_vectors import integers
from generate_version_range import apply_patches
from mfcobjects import ObjectArchive

KINDS = ["moAmbientLight_c","moLightFeature_c","moDirectionLight_c","moSpotLight_c","moPointLight_c"]


def replace_light(document: object, path: object, raw: bytes, kind: int) -> bytes:
    slots = [row for row in collect(document,path,raw) if row[3]=="moDirectionLight_c"]
    if not slots:return raw
    assert len(slots)==1
    start,end,_,_ = slots[0]
    name = KINDS[kind]
    body = base(False) if kind<2 else base(False)+doubles(4) if kind==2 else payload(kind==3,False,False)
    prefix = struct.pack("<HHH",0xffff,1,len(name))+name.encode()
    return raw[:start]+prefix+body+raw[end:]


def hidden_reader(original: object, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, schema: int) -> object:
        result = original(archive,schema)
        assert result["hidden_body_atoms"]==[]
        patches.append((archive.p-4,archive.p,integers(3,-1,0,2147483647)))
        return result
    return read


def populate_hidden(document: object, path: object, raw: bytes) -> bytes:
    names = ["uoTempAssemblySHDData_c","uoTempSubAssemblySHDData_c"]
    originals = {name:ObjectArchive.readers[name] for name in names}
    patches = []
    for name in names:ObjectArchive.readers[name] = hidden_reader(originals[name],patches)
    try:
        archive = displaylists.archive_for(document,str(path),raw)
        displaylists.display_lists(archive,"assembly")
        assert archive.p==len(raw)
    finally:ObjectArchive.readers.update(originals)
    return apply_patches(raw,patches)
