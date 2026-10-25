"""Complete populated decals around the two older appearance-option word gates."""
import json
from pathlib import Path
from generate_decals import generate as decal_root
from generate_earliest_lights import generate as root
from generate_earlier_range import write_group
from generate_earliest_scene_roots import raw_nan_reader
from decal_sources import swcontainer
from reference_corpus import frame_at
from swarchive import Archive


def source(version: int, kind: str) -> tuple[Path,object,bytes,int]:
    path = Path("tests/fixtures/blank-part.SLDPRT" if kind=="part" else "tests/fixtures/blank-assembly.SLDASM")
    framed,row = root(version,kind)
    raw,_ = frame_at(framed,6)
    return path,swcontainer.Document(str(path)),raw,row["aggregate_entries"]


def generate(version: int, kind: str, scenario: int) -> tuple[bytes,dict[str,object]]:
    original = Archive.f32
    Archive.f32 = raw_nan_reader(original)
    try:framed,row = decal_root(version,kind,scenario,source)
    finally:Archive.f32 = original
    row["oracle_fields"] = row["scalar_fields"]
    return framed,row


if __name__=="__main__":
    cases = [generate(v,k,0) for v in [4500,0x1197,0x1198,0x11bb,0x11bc,4600] for k in ["part","assembly"]]
    cases += [generate(4500,k,1) for k in ["part","assembly"]]
    count,fields = write_group("earliest-light-decals",cases,("displaylists.py",))
    print(json.dumps({"fixtures":count,"fields":fields}))
