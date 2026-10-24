"""Full XML corpus oracle: exact framing reconstruction and independent ElementTree models."""
from collections import Counter
import hashlib
import json
from pathlib import Path
import re
import shutil
import struct
import subprocess
import sys
import tempfile
import xml.etree.ElementTree as ET

sys.path.insert(0,str(Path("../inventor-interface/solidworks").resolve()))
import swcontainer as sc
import swcover

SOURCE = Path(sys.argv[1]).resolve()


def append_text(output: bytearray, text: str | None) -> None:
    if text is None:
        output.extend(struct.pack("<I",0xffffffff))
        return
    encoded = text.encode("utf-8")
    output.extend(struct.pack("<I",len(encoded)))
    output.extend(encoded)


def expanded(name: str) -> tuple[str | None,str]:
    if not name.startswith("{"):
        return None,name
    namespace,local = name[1:].rsplit("}",1)
    return namespace,local


def append_name(output: bytearray, name: str) -> None:
    namespace,local = expanded(name)
    append_text(output,namespace)
    append_text(output,local)


def oracle(raw: bytes) -> tuple[bytes,int,int]:
    if not raw:
        return struct.pack("<I",0),0,0
    if raw[1:6] == b"<?xml":
        raw = raw[1:]
    root = ET.fromstring(raw)
    stack = [(0xffffffff,root)]
    entries = []
    while stack:
        parent,element = stack.pop()
        ordinal = len(entries)
        entries.append((parent,element))
        stack.extend((ordinal,child) for child in reversed(list(element)))
    output = bytearray(struct.pack("<I",len(entries)))
    for parent,element in entries:
        append_element(output,parent,element)
    return bytes(output),len(entries),sum(len(element.attrib) for _,element in entries)


def append_element(output: bytearray, parent: int, element: ET.Element) -> None:
    output.extend(struct.pack("<I",parent))
    append_name(output,element.tag)
    append_text(output,(element.text or "")+"".join(child.tail or "" for child in element))
    output.extend(struct.pack("<I",len(element.attrib)))
    for name,value in element.attrib.items():
        append_name(output,name)
        append_text(output,value)
    output.extend(struct.pack("<I",len(element)))


def verify_batch(batch: list[tuple[str,Path,Path,Path,bytes]], executable: str) -> None:
    arguments = [executable]
    for name,source,output,trace,_ in batch:
        arguments.extend([name,str(source),str(output),str(trace)])
    result = subprocess.run(arguments,capture_output=True,text=True)
    assert result.returncode == 0,result.stderr
    for name,source,output,trace,expected in batch:
        assert output.read_bytes() == source.read_bytes(),f"{name}: forced byte reconstruction mismatch"
        actual = trace.read_bytes()
        difference = next((index for index,(left,right) in enumerate(zip(actual,expected)) if left != right),min(len(actual),len(expected)))
        assert actual == expected,f"{name} {source.name}: oracle mismatch at {difference}: actual {actual[max(0,difference-40):difference+80]!r}, expected {expected[max(0,difference-40):difference+80]!r}"


counts = {"streams":0,"bytes":0,"elements":0,"attributes":0,"empty":0}
families = Counter()
manifest = hashlib.sha256()
samples = Path(swcover.SAMPLES)
with tempfile.TemporaryDirectory(prefix="bessemer-xml-corpus-") as temporary:
    root = Path(temporary)
    executable = str(root / "rebuild_xml.exe")
    shutil.copyfile(SOURCE,executable)
    batch = []
    for path in sorted(map(Path,swcover.collect([]))):
        document = sc.Document(str(path))
        for entry in document.entries:
            if not(entry.name.startswith(("docProps/","swXmlContents/","_rels/")) or entry.name == "[Content_Types].xml"):
                continue
            raw = document.read(entry.name)
            expected,elements,attributes = oracle(raw)
            index = counts["streams"]
            source,output,trace = (root / f"{index}-{suffix}" for suffix in ("input","output","trace"))
            source.write_bytes(raw)
            batch.append((entry.name,source,output,trace,expected))
            counts["streams"] += 1
            counts["bytes"] += len(raw)
            counts["elements"] += elements
            counts["attributes"] += attributes
            counts["empty"] += int(not raw)
            families[re.sub(r"Config-\d+","Config-N",entry.name)] += 1
            manifest.update(path.relative_to(samples).as_posix().encode()+b"\0"+entry.name.encode()+b"\0"+raw)
            if len(batch) == 32:
                verify_batch(batch,executable)
                batch.clear()
    if batch:
        verify_batch(batch,executable)
print(json.dumps({**counts,"families":len(families),"manifest":manifest.hexdigest()}))
