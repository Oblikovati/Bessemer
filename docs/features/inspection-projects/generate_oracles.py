"""Independent ElementTree traces for the two approved native Inspection drawings."""
from pathlib import Path
import hashlib
import json
import sys
import xml.etree.ElementTree as ET
sys.path.insert(0,str(Path("../inventor-interface/solidworks").resolve()))
from swcontainer import Document

ROOT = Path("tests/fixtures/inspection")
STREAM = "ThirdPty/InspectionXpert"


def expanded(name: str) -> tuple[str | None,str]:
    if name.startswith("{"):
        namespace,local = name[1:].split("}",1)
        return namespace,local
    return None,name


def hex_text(value: str | None) -> str:
    return "-" if value is None else value.encode("utf-8").hex()


def trace(element: ET.Element, parent: int, entries: list[tuple[int,ET.Element]]) -> None:
    ordinal = len(entries)
    entries.append((parent,element))
    for child in element:
        trace(child,ordinal,entries)


def element_row(parent: int, element: ET.Element, ordinals: dict[int,int]) -> str:
    namespace,local = expanded(element.tag)
    text = (element.text or "")+"".join(child.tail or "" for child in element)
    attributes = [":".join(map(hex_text,(*expanded(name),value))) for name,value in element.attrib.items()]
    children = ",".join(str(ordinals[id(child)]) for child in element)
    return "\t".join((str(parent),hex_text(namespace),local,hex_text(text),",".join(attributes) or "-",children or "-"))+"\n"


def write_oracle(name: str) -> dict[str,str | int]:
    path = Path("tests/fixtures")/f"{name}.SLDDRW"
    raw = Document(str(path)).read(STREAM)
    root = ET.fromstring(raw)
    entries: list[tuple[int,ET.Element]] = []
    trace(root,-1,entries)
    ordinals = {id(element):ordinal for ordinal,(_,element) in enumerate(entries)}
    rows = "".join(element_row(parent,element,ordinals) for parent,element in entries)
    ROOT.joinpath(f"{name}.tsv").write_text(rows,encoding="utf-8",newline="\n")
    return {"fixture":path.name,"fixture_sha256":hashlib.sha256(path.read_bytes()).hexdigest(),"bytes":len(raw),"elements":len(entries),"xml_sha256":hashlib.sha256(raw).hexdigest(),"job_number":root.findtext("Settings/JobNumber") or ""}


def observed_paths(root: ET.Element) -> set[str]:
    paths: set[str] = set()
    stack = [("",root)]
    while stack:
        parent,element = stack.pop()
        path = parent+"/"+element.tag
        paths.add(path)
        stack.extend((path,child) for child in element)
    return paths


def verify_inventory() -> None:
    paths: set[str] = set()
    for name in ("inspection-drawing","inspection-job"):
        document = Document(str(Path("tests/fixtures")/f"{name}.SLDDRW"))
        paths.update(observed_paths(ET.fromstring(document.read(STREAM))))
    inventory = Path("docs/features/inspection-projects/inspection-schema.md").read_text(encoding="utf-8")
    expected = {line.split("`",2)[1] for line in inventory.splitlines() if line.startswith("| `/")}
    assert paths == expected and len(paths) == 233


if __name__ == "__main__":
    verify_inventory()
    ROOT.mkdir(parents=True,exist_ok=True)
    records = [write_oracle(name) for name in ("inspection-drawing","inspection-job")]
    Path("docs/features/inspection-projects/verification.json").write_text(json.dumps(records,indent=2)+"\n",encoding="utf-8",newline="\n")
    print(json.dumps(records))
