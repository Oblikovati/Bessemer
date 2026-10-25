"""Locate zero-count decal slots and retain material bytes from approved owned roots."""
from pathlib import Path
import struct
from generate_alias_fixture import displaylists,swcontainer
from generate_early_shaded import generate as early_root
from generate_old_shaded import generate as old_root
from generate_version_range import generate as version_root
import dl_tess
import dl_light


def source(version: int, kind: str) -> tuple[Path,object,bytes,int]:
    path = Path("tests/fixtures/blank-part.SLDPRT" if kind=="part" else "tests/fixtures/blank-assembly.SLDASM")
    document = swcontainer.Document(str(path))
    if version<=11000:
        framed,row = (early_root if version<10000 else old_root)(version,kind)
        length, = struct.unpack_from("<I",framed,6)
        return path,document,framed[10:10+length],row["aggregate_entries"]
    if version==18000:
        framed,row = version_root(version)
        length, = struct.unpack_from("<I",framed,5)
        return path,document,framed[9:9+length],row["aggregate_entries"]
    raw = Path("tests/fixtures/display-lists/inline-shaded-part.bin").read_bytes() if kind=="part" else document.read("Contents/DisplayLists")
    return path,document,raw,134 if kind=="part" else 53


def slot_reader(original: object, slots: list[tuple[int,int,bool]]) -> object:
    def read(archive: object) -> object:
        start = archive.p
        result = original(archive)
        assert archive.p==start+4
        slots.append((start,archive.p,False))
        return result
    return read


def count_reader(original: object, slots: list[tuple[int,int,bool]], unsigned: list[int]) -> object:
    def read(archive: object, count: int, minimum: int = 1) -> int:
        result = original(archive,count,minimum)
        if minimum==8 and unsigned==[archive.p-4]:
            assert count==0
            slots.append((archive.p-4,archive.p,True))
        return result
    return read


def material_reader(original: object, materials: list[bytes], raw: bytes) -> object:
    def read(archive: object) -> object:
        start = archive.p
        result = original(archive)
        materials.append(raw[start:archive.p])
        return result
    return read


def locate(document: object, path: Path, raw: bytes, version: int, kind: str) -> tuple[list[tuple[int,int,bool]],bytes]:
    archive = displaylists.archive_for(document,str(path),raw)
    archive.version,archive.doc_kind,archive.face_tess_comp_ids = version,kind,set()
    slots,materials,unsigned = [],[],[]
    original = archive.u32
    def read_unsigned() -> int:
        unsigned[:] = [archive.p]
        return original()
    archive.u32 = read_unsigned
    bindings = [(dl_tess,"_decal_list"),(dl_tess,"_count_guard"),(dl_light,"read_uoMaterialProperties")]
    originals = [getattr(module,name) for module,name in bindings]
    replacements = [slot_reader(originals[0],slots),count_reader(originals[1],slots,unsigned),material_reader(originals[2],materials,raw)]
    try:
        for (module,name),replacement in zip(bindings,replacements,strict=True):setattr(module,name,replacement)
        displaylists.display_lists(archive,kind)
    finally:
        for (module,name),reader in zip(bindings,originals,strict=True):setattr(module,name,reader)
    assert archive.p==len(raw) and slots and materials
    return sorted(slots),materials[0]
