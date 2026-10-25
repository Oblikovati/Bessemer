"""Independent traces and exact reconstruction for context-explicit native view bodies."""
from pathlib import Path
from typing import Callable,Protocol
import shutil
import struct
import subprocess
import sys
import tempfile
from verify_transform_corpus import scan_documents
import dl_doc
from mfcobjects import ObjectArchive


class ViewArchive(Protocol):
    b: bytes
    p: int
    version: int


ViewReader = Callable[[ViewArchive],object]


def capture(original: ViewReader, unique: dict[tuple[int,bytes],bytes], counts: list[int], count_views: Callable[[object],int]) -> ViewReader:
    def wrapped(archive: ViewArchive) -> object:
        start = archive.p
        result = original(archive)
        raw = archive.b[start:archive.p]
        unique[(archive.version & 0xffffffff,raw)] = semantic_trace(result)
        counts[0] += 1
        counts[1] += len(raw)
        counts[2] += count_views(result)
        return result
    return wrapped


def extract_component(reader_name: str, count_views: Callable[[object],int]) -> tuple[list[tuple[int,bytes,bytes]],dict[str,int | str]]:
    unique: dict[tuple[int,bytes],bytes] = {}
    counts = [0,0,0]
    original = getattr(dl_doc,reader_name)
    setattr(dl_doc,reader_name,capture(original,unique,counts,count_views))
    try:
        documents,manifest = scan_documents()
    finally:
        setattr(dl_doc,reader_name,original)
    evidence = {"documents":documents,"collections":counts[0],"view_bytes":counts[1],"views":counts[2],"distinct_collections":len(unique),"display_stream_manifest_sha256":manifest}
    return [(version,raw,trace) for (version,raw),trace in sorted(unique.items())],evidence


def semantic_trace(value: object) -> bytes:
    rows: dict[str,str] = {}
    trace_value(value,"root",rows)
    return "".join(f"{path}={text}\n" for path,text in sorted(rows.items())).encode()


def trace_scalar(value: object, path: str, rows: dict[str,str]) -> bool:
    if value is None:
        rows[path] = "none"
    elif isinstance(value,str):
        rows[path] = "s:"+value.encode().hex()
    elif isinstance(value,bytes):
        rows[path] = "b:"+value.hex()
    elif isinstance(value,int):
        rows[path] = f"i:{int(value)}"
    elif isinstance(value,float):
        rows[path] = "f:"+struct.pack(">d",value).hex()
    else:
        return False
    return True


def trace_value(value: object, path: str, rows: dict[str,str]) -> None:
    if trace_scalar(value,path,rows):
        return
    if isinstance(value,(list,tuple)):
        rows[path] = f"a:{len(value)}"
        for index,nested in enumerate(value):
            trace_value(nested,f"{path}[{index}]",rows)
    elif isinstance(value,dict):
        for key,nested in value.items():
            assert isinstance(key,str)
            trace_value(nested,f"{path}.{key}",rows)
    else:
        raise ValueError(f"trace value {value!r}; expected explicit scalar, sequence or named fields")


def owned_records(path: Path, reader: ViewReader) -> list[tuple[int,bytes,bytes]]:
    raw = path.read_bytes()
    count, = struct.unpack_from("<I",raw)
    offset = 4
    records: list[tuple[int,bytes,bytes]] = []
    for _ in range(count):
        version,_,length = struct.unpack_from("<IBI",raw,offset)
        payload = raw[offset+9:offset+9+length]
        archive = ObjectArchive(payload,version)
        result = reader(archive)
        assert archive.p == len(payload)
        records.append((version,payload,semantic_trace(result)))
        offset += 9+length
    assert offset == len(raw)
    return records


def verify_records(executable: Path, root: Path, records: list[tuple[int,bytes,bytes]]) -> None:
    output = bytearray(struct.pack("<I",len(records)))
    expected = bytearray(struct.pack("<I",len(records)))
    for version,payload,trace in records:
        output.extend(struct.pack("<IBI",version,255,len(payload))+payload)
        expected.extend(struct.pack("<I",len(trace))+trace)
    source,destination,traces = root/"source.bin",root/"rebuilt.bin",root/"traces.bin"
    source.write_bytes(output)
    subprocess.run([str(executable),str(source),str(destination),str(traces)],check=True)
    assert destination.read_bytes() == output
    actual = traces.read_bytes()
    difference = next((index for index,(left,right) in enumerate(zip(actual,expected)) if left != right),min(len(actual),len(expected)))
    assert actual == expected,f"semantic trace mismatch at {difference}: {actual[max(0,difference-40):difference+120]!r} != {expected[max(0,difference-40):difference+120]!r}"


def verify(executable: str, reader_name: str, fixture: Path, count_views: Callable[[object],int]) -> dict[str,int | str]:
    records,evidence = extract_component(reader_name,count_views)
    with tempfile.TemporaryDirectory(prefix="bessemer-view-corpus-") as temporary:
        root = Path(temporary)
        local = root/Path(executable).name
        shutil.copyfile(executable,local)
        verify_records(local,root,records)
        verify_records(local,root,owned_records(fixture,getattr(dl_doc,reader_name)))
    return evidence

def list_view_count(value:object)->int:
    if value is None:
        return 0
    if isinstance(value,list):
        return len(value)
    raise ValueError(f"view result {value!r}; expected None or list")
