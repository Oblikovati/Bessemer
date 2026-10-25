"""Freeze full roots and independent named-view field oracles at the later boundaries."""
from pathlib import Path
from functools import partial
import hashlib
import json
import struct
from named_branch_roots import named_branch_root,VERSIONS
from legacy_component_reference import component_reference,activity_scalars
from generate_legacy_visuals import visual_scalars
from complete_root_case import frame_case
from generate_earlier_range import write_group
from generate_earliest_components import write_documents
from view_corpus import semantic_trace


def generate(version: int, kind: str, scenario: int) -> tuple[bytes,dict[str,object]]:
    source,raw = named_branch_root(version,kind,scenario)
    root = component_reference(raw,version,kind,source.external_ids)
    framed,row = frame_case(version,kind,scenario,raw,source.external_ids,root,
        visual_scalars(root)+activity_scalars(root))
    row.update(source_label=source.label,source_sha256=hashlib.sha256(source.payload).hexdigest())
    return framed,row


def named_oracle(version: int, kind: str, scenario: int) -> tuple[bytes,int]:
    source,raw = named_branch_root(version,kind,scenario)
    root = component_reference(raw,version,kind,source.external_ids)
    trace = semantic_trace(root["named_view_transform"])
    return struct.pack("<IBBI",version,kind=="assembly",scenario,len(trace))+trace,len(trace.splitlines())


def write_oracles() -> None:
    records = [named_oracle(version,kind,scenario) for version in VERSIONS
        for kind in ("part","assembly") for scenario in range(192,197)]
    encoded = struct.pack("<I",len(records))+b"".join(raw for raw,_ in records)
    Path("tests/fixtures/display-lists/named-branch-oracles.bin").write_bytes(encoded)
    report = {"fixtures":len(records),"fields":sum(count for _,count in records),
        "sha256":hashlib.sha256(encoded).hexdigest()}
    Path(__file__).with_name("named-branch-oracles.json").write_text(json.dumps(report,indent=2)+"\n")


if __name__=="__main__":
    records = [generate(version,kind,scenario) for version in VERSIONS
        for kind in ("part","assembly") for scenario in range(192,197)]
    print(write_group("named-branch-roots",records,("dl_doc.py",)))
    write_oracles()
    write_documents(partial(generate,scenario=195),19000,"named-branch")
    write_documents(partial(generate,scenario=196),19000,"named-branch-edited")
