"""Resolved complete roots need not contain the unresolved old facet-object body."""
from pathlib import Path
from functools import lru_cache
import json
import struct
from generate_alias_fixture import collect,swcontainer,displaylists
from generate_version_range import apply_patches
from reference_corpus import frame_at
from mfcobjects import ObjectArchive
from generate_component_vectors import integers
from generate_named_vectors import cstring
from generate_transform_vectors import doubles
from generate_item_vectors import record
import dl_tess
import dl_doc
import dl_light


@lru_cache(maxsize=12)
def prototype(kind: str, light_kind: int | None = None) -> bytes:
    name = "earliest-visual-"+kind if light_kind is None else "earliest-light-state-kinds"
    scenario = 208 if light_kind is None else 224+2*light_kind
    manifest = json.loads(Path(__file__).with_name(name+".json").read_text())
    raw = Path(f"tests/fixtures/display-lists/{name}.bin").read_bytes()
    position = 4
    for row in manifest["fixtures"]:
        payload,position = frame_at(raw,position+6)
        _,position = frame_at(raw,position)
        _,position = frame_at(raw,position)
        if row["version"]==3196 and row["scenario"]==scenario and row["kind"]==kind:return payload
    raise ValueError(f"prototype {kind!r}/{light_kind!r}; expected a frozen version-3196 scenario-{scenario} root")


def capture(original: object, label: str, spans: dict[str,tuple[int,int]]) -> object:
    def read(archive: object,*args: object) -> object:
        start = archive.p
        result = original(archive,*args)
        assert label not in spans,f"duplicate observed {label}; expected one root call"
        spans[label] = (start,archive.p)
        return result
    return read


def observed(raw: bytes, kind: str) -> dict[str,tuple[int,int]]:
    bindings = [(dl_tess,"tessellation"),(dl_doc,"read_display_item_list"),(dl_light,"read_old_lights")]
    labels,spans = ["tessellation","items","lights"],{}
    originals = [getattr(module,name) for module,name in bindings]
    try:
        for (module,name),label,original in zip(bindings,labels,originals):
            setattr(module,name,capture(original,label,spans))
        archive = ObjectArchive(raw,3196)
        archive.doc_kind,archive.file_version,archive.face_tess_comp_ids = kind,19000,set()
        root = displaylists.display_lists(archive,kind)
        assert archive.p==len(raw) and root["textured_planes"]==[]
        # No object follows the model, so removing its object slots needs no tag relocation.
        assert raw[spans["tessellation"][1]:]==bytes(4)
    finally:
        for (module,name),original in zip(bindings,originals):setattr(module,name,original)
    return spans


def items(version: int, edited: bool) -> bytes:
    pen = struct.pack("<Ii",0xffffffff,-2147483648+int(edited))
    if version>0xc57:pen += integers(-32768+int(edited))
    pen += cstring("legacy pen 鋼",True)
    parabola = doubles(15)+(doubles(2) if version>0xc58 else b"")
    return struct.pack("<H",2)+record(12,pen)+record(13,parabola)


def assembly(raw: bytes, span: tuple[int,int], version: int) -> bytes:
    path = Path("tests/fixtures/blank-assembly.SLDASM")
    document = swcontainer.Document(str(path))
    pointers = [row for row in collect(document,path,raw,3196,set()) if row[3]=="uoTempPartTessData_c"]
    assert len(pointers)==1,f"part pointers {pointers}; expected one nested part object"
    first,last = pointers[0][:2]
    start,end = span
    result = raw[start:first]+bytes(2)+raw[last:end]
    return result[:-4] if version<=0xc6e else result


def converted(version: int, kind: str, scenario: int) -> bytes:
    raw = prototype(kind,scenario-16 if scenario>=16 else None)
    spans = observed(raw,kind)
    patches = [(*spans["items"],items(version,scenario==1))]
    if scenario==2:
        return apply_patches(raw,patches+[(spans["lights"][0]-4,len(raw),integers(0))])
    tessellation = struct.pack("<HHIi",0,0,0xffffffff,0) if kind=="part" else assembly(raw,spans["tessellation"],version)
    patches.append((*spans["tessellation"],tessellation))
    return apply_patches(raw,patches)
