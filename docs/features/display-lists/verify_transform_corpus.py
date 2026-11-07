"""Extract transform spans during complete reference DisplayLists parsing, then force Rust rebuilds."""
from pathlib import Path
from typing import Callable,Protocol
import hashlib
import json
import shutil
import struct
import subprocess
import sys
import tempfile
sys.path.insert(0,str(Path("../inventor-interface/solidworks").resolve()))
import swcontainer
import swcover
import displaylists
import dl_ui
import dl_tess


class TransformArchive(Protocol):
    b: bytes
    p: int


TransformReader = Callable[[TransformArchive],object]


def capture(original: TransformReader, kind: int, unique: set[tuple[int,bytes]], counts: list[int]) -> TransformReader:
    def wrapped(archive: TransformArchive) -> object:
        start = archive.p
        result = original(archive)
        raw = archive.b[start:archive.p]
        unique.add((kind,raw))
        counts[kind] += 1
        counts[2] += len(raw)
        return result
    return wrapped


def scan_documents() -> tuple[int,str]:
    count = 0
    manifest = hashlib.sha256()
    for path in sorted(map(Path,swcover.collect([]))):
        if path.suffix.lower() not in (".sldprt",".sldasm"):
            continue
        document = swcontainer.Document(str(path))
        if "Contents/DisplayLists" not in document.names():
            continue
        displaylists.read(document,str(path))
        count += 1
        relative = path.relative_to(Path(swcover.SAMPLES)).as_posix().encode()
        manifest.update(relative+b"\0"+document.read("Contents/DisplayLists"))
    return count,manifest.hexdigest()


def extract() -> tuple[list[tuple[int,bytes]],dict[str,int | str]]:
    unique: set[tuple[int,bytes]] = set()
    counts = [0,0,0]
    original_compact,original_fixed = dl_ui.read_mgXform_c,dl_tess.read_mgXform_struct
    dl_ui.read_mgXform_c = capture(original_compact,0,unique,counts)
    dl_tess.read_mgXform_struct = capture(original_fixed,1,unique,counts)
    try:
        documents,manifest = scan_documents()
    finally:
        dl_ui.read_mgXform_c,dl_tess.read_mgXform_struct = original_compact,original_fixed
    evidence = {"documents":documents,"compact":counts[0],"fixed":counts[1],"transform_bytes":counts[2],"manifest_sha256":manifest,"unique_transforms":len(unique)}
    return sorted(unique),evidence


def verify(executable: str) -> None:
    records,counts = extract()
    output = bytearray(struct.pack("<I",len(records)))
    for kind,payload in records:
        output.extend(struct.pack("<BI",kind,len(payload))+payload)
    with tempfile.TemporaryDirectory(prefix="bessemer-display-transforms-") as temporary:
        root = Path(temporary)
        local = root/Path(executable).name
        shutil.copyfile(executable,local)
        source,destination = root/"source.bin",root/"rebuilt.bin"
        source.write_bytes(output)
        subprocess.run([str(local),str(source),str(destination)],check=True)
        assert destination.read_bytes() == output
    print(json.dumps(counts))


if __name__ == "__main__":
    verify(sys.argv[1])
