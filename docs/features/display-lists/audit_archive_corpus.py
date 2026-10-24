"""Report whole-stream implementation coverage; failures remain missing work, never passes."""
from collections import Counter
from pathlib import Path
import hashlib
import json
import shutil
import struct
import subprocess
import sys
import tempfile

sys.path.insert(0, str(Path("../inventor-interface/solidworks").resolve()))
import displaylists
import swcontainer
import swcover


def collect() -> tuple[bytes, list[tuple[str, bytes, int]], str]:
    records: list[tuple[str, bytes, int]] = []
    framed = bytearray()
    manifest = hashlib.sha256()
    for path in sorted(map(Path, swcover.collect([]))):
        if path.suffix.lower() not in (".sldprt", ".sldasm"):
            continue
        document = swcontainer.Document(str(path))
        if "Contents/DisplayLists" not in document.names():
            continue
        raw = document.read("Contents/DisplayLists")
        archive = displaylists.archive_for(document, str(path), raw)
        displaylists.display_lists(archive, archive.doc_kind)
        assert archive.p == len(raw)
        name = path.relative_to(Path(swcover.SAMPLES)).as_posix()
        records.append((name, raw, archive.version & 0xffffffff))
        manifest.update(name.encode()+b"\0"+raw)
        framed.extend(struct.pack("<IIBI", archive.version & 0xffffffff, 19000, int(archive.doc_kind == "assembly"), len(raw))+raw)
    return struct.pack("<I", len(records))+framed, records, manifest.hexdigest()


def results(raw: bytes, records: list[tuple[str, bytes, int]]) -> dict[str, object]:
    count, = struct.unpack_from("<I", raw)
    assert count == len(records)
    position = 4
    accepted: list[str] = []
    failures: Counter[str] = Counter()
    versions: Counter[int] = Counter()
    for name, source, version in records:
        status, length = struct.unpack_from("<BI", raw, position)
        position += 5
        payload = raw[position:position+length]
        position += length
        if status == 0:
            assert payload == source, name
            accepted.append(name)
        else:
            assert status == 1
            failures[payload.decode()] += 1
            versions[version] += 1
    assert position == len(raw)
    return {"accepted": accepted, "failures": dict(failures.most_common()), "missing_by_version": dict(sorted(versions.items()))}


def audit(executable: str) -> None:
    framed, records, manifest = collect()
    with tempfile.TemporaryDirectory(prefix="bessemer-display-archives-") as temporary:
        root = Path(temporary)
        local = root/Path(executable).name
        shutil.copyfile(executable, local)
        source, destination = root/"source.bin", root/"result.bin"
        source.write_bytes(framed)
        subprocess.run([str(local), str(source), str(destination)], check=True)
        first = destination.read_bytes()
        subprocess.run([str(local), str(source), str(destination)], check=True)
        assert first == destination.read_bytes()
        report = results(first, records)
    report.update({"status": "incomplete implementation audit; missing streams are required work", "documents": len(records), "manifest_sha256": manifest, "accepted_count": len(report["accepted"]), "two_processes_identical": True})
    Path(__file__).with_name("archive-audit.json").write_text(json.dumps(report, indent=2)+"\n", encoding="utf-8", newline="\n")
    print(json.dumps({key: value for key, value in report.items() if key not in ("accepted", "failures")}))


if __name__ == "__main__":
    audit(sys.argv[1])
