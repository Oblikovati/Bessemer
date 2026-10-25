"""Owned complete roots at every material, scene and spatial boundary in the modern range."""
from pathlib import Path
from collections.abc import Callable
import hashlib
import json
import struct
from generate_alias_fixture import displaylists,swcontainer
from generate_shaded_vectors import ShadedArchive
from generate_spatial_vectors import collection
from version_range_wire import dspbr,symbol
import dl_doc
import dl_light
import material_corpus
import scene_corpus

VERSIONS = [18000,0x46b4,0x46b5,0x46b9,0x46ba,0x46bb,0x46c9,0x46ca,0x4971,0x4972,19000]
Patch = tuple[int,int,bytes]


def capture_plain(original: Callable[[ShadedArchive],object], replacement: bytes, patches: list[Patch]) -> Callable[[ShadedArchive],object]:
    def capture(archive: ShadedArchive) -> object:
        start = archive.p
        result = original(archive)
        patches.append((start,archive.p,replacement))
        return result
    return capture


def capture_rv(original: Callable[[ShadedArchive],object], version: int, patches: list[Patch]) -> Callable[[ShadedArchive],object]:
    def capture(archive: ShadedArchive) -> object:
        result = original(archive)
        if version < 0x46ba:
            patches.append((archive.p,archive.p,symbol()))
        return result
    return capture


def capture_scene(original: Callable[[ShadedArchive,int],object], version: int, edited: bool, patches: list[Patch]) -> Callable[[ShadedArchive,int],object]:
    def capture(archive: ShadedArchive, kind: int) -> object:
        result = original(archive,kind)
        if edited:
            patches.append((archive.p-20,archive.p-12,struct.pack("<d",23.0)))
        if version <= 0x46ba:
            patches.append((archive.p-8,archive.p,b""))
        return result
    return capture


def capture_patches(document: swcontainer.Document, path: Path, raw: bytes, version: int, edited: bool) -> list[Patch]:
    patches: list[Patch] = []
    bindings = [(dl_light,"read_uoDSPBRProperties_c"),(dl_light,"read_uoRVAppearanceProperties"),(dl_light,"read_scene"),(dl_doc,"read_3DViewTransform")]
    originals = [getattr(module,name) for module,name in bindings]
    replacements = [capture_plain(originals[0],dspbr(version),patches),capture_rv(originals[1],version,patches),capture_scene(originals[2],version,edited,patches),capture_plain(originals[3],collection(version,2),patches)]
    try:
        for (module,name),replacement in zip(bindings,replacements,strict=True):
            setattr(module,name,replacement)
        archive = displaylists.archive_for(document,str(path),raw)
        displaylists.display_lists(archive,archive.doc_kind)
        assert archive.p == len(raw)
    finally:
        for (module,name),original in zip(bindings,originals,strict=True):
            setattr(module,name,original)
    return patches


def apply_patches(raw: bytes, patches: list[Patch]) -> bytes:
    output = bytearray()
    position = 0
    for start,end,replacement in sorted(patches):
        assert position <= start <= end <= len(raw)
        output.extend(raw[position:start]+replacement)
        position = end
    return bytes(output)+raw[position:]


def generate(version: int, edited: bool = False) -> tuple[bytes,dict[str,object]]:
    path = Path("tests/fixtures/blank-part.SLDPRT")
    document = swcontainer.Document(str(path))
    source = Path("tests/fixtures/display-lists/scene-cubical.bin").read_bytes()
    patches = capture_patches(document,path,source,version,edited)
    derived = apply_patches(source,patches)
    archive = displaylists.archive_for(document,str(path),derived)
    archive.version = version
    root = displaylists.display_lists(archive,archive.doc_kind)
    assert archive.p == len(derived) and len(root["view_3d_transform"]) == 2
    trace = material_corpus.trace(root)+scene_corpus.trace(root)
    framed = struct.pack("<IBI",version,edited,len(derived))+derived+struct.pack("<I",len(trace))+trace
    evidence = {"version":version,"edited":edited,"bytes":len(derived),"sha256":hashlib.sha256(derived).hexdigest(),"trace_fields":len(trace.splitlines()),"aggregate_entries":68,"new_objects":49}
    return framed,evidence


def main() -> None:
    cases = [generate(version) for version in VERSIONS]+[generate(18000,True)]
    framed = struct.pack("<I",len(cases))+b"".join(raw for raw,_ in cases)
    Path("tests/fixtures/display-lists/version-range.bin").write_bytes(framed)
    evidence = {"scope":"complete roots with explicit display versions 18000..19000, file version 19000; no model validity claim","fixtures":[row for _,row in cases],"fixture_sha256":hashlib.sha256(framed).hexdigest()}
    evidence["source_hashes"] = {name:hashlib.sha256(Path(f"../inventor-interface/solidworks/{name}").read_bytes()).hexdigest() for name in ["dl_light.py","dl_doc.py","dl_ui.py","dl_tess.py","facettess.py"]}
    Path(__file__).with_name("version-range.json").write_text(json.dumps(evidence,indent=2)+"\n",encoding="utf-8",newline="\n")
    print(json.dumps(evidence))


if __name__ == "__main__":
    main()
