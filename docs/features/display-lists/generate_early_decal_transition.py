"""Populated decal lists and maps on both sides of their 0x239D boundary."""
from pathlib import Path
import json
from generate_decals import generate as generate_decal
from generate_grouped_components import generate as generate_root
from generate_earlier_range import write_group
from decal_sources import swcontainer
from reference_corpus import frame_at


def source(version: int, kind: str) -> tuple[Path,object,bytes,int]:
    path = Path("tests/fixtures/blank-part.SLDPRT" if kind=="part" else "tests/fixtures/blank-assembly.SLDASM")
    document = swcontainer.Document(str(path))
    framed,row = generate_root(version,kind)
    raw,_ = frame_at(framed,6)
    return path,document,raw,row["aggregate_entries"]


def generate(version: int, kind: str, scenario: int) -> tuple[bytes,dict[str,object]]:
    framed,row = generate_decal(version,kind,scenario,source)
    row["oracle_fields"] = row["scalar_fields"]
    return framed,row


if __name__=="__main__":
    cases = [generate(version,kind,0) for version in [0x239c,0x239d] for kind in ["part","assembly"]]
    count,fields = write_group("early-decal-transition",cases)
    print(json.dumps({"fixtures":count,"fields":fields}))
