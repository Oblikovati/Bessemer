"""Populated decal faces bracket the 0x12DB field gate inside complete owned documents."""
import json
from pathlib import Path
from generate_decals import generate as generate_decal
from generate_earliest_components import generate as generate_root
from generate_earlier_range import write_group
from generate_earliest_scene_roots import raw_nan_reader
from decal_sources import swcontainer
from reference_corpus import frame_at
from swarchive import Archive


def source(version: int, kind: str) -> tuple[Path,object,bytes,int]:
    path = Path("tests/fixtures/blank-part.SLDPRT" if kind=="part" else "tests/fixtures/blank-assembly.SLDASM")
    document = swcontainer.Document(str(path))
    framed,row = generate_root(version,kind)
    raw,_ = frame_at(framed,6)
    return path,document,raw,row["aggregate_entries"]


def generate(version: int, kind: str, scenario: int) -> tuple[bytes,dict[str,object]]:
    original = Archive.f32
    Archive.f32 = raw_nan_reader(original)
    try:framed,row = generate_decal(version,kind,scenario,source)
    finally:Archive.f32 = original
    row["oracle_fields"] = row["scalar_fields"]
    return framed,row


if __name__=="__main__":
    cases = [generate(v,k,0) for v in [4800,0x12db,0x12dc,4850] for k in ["part","assembly"]]
    cases += [generate(4800,k,1) for k in ["part","assembly"]]
    count,fields = write_group("earliest-component-decals",cases)
    print(json.dumps({"fixtures":count,"fields":fields}))
