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
import reference_corpus
import scene_corpus
import item_corpus


def collect() -> tuple[bytes, list[tuple[str, bytes, int, bytes]], str]:
    records: list[tuple[str, bytes, int, bytes]] = []
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
        root = displaylists.display_lists(archive, archive.doc_kind)
        assert archive.p == len(raw)
        name = path.relative_to(Path(swcover.SAMPLES)).as_posix()
        records.append((name, raw, archive.version & 0xffffffff, reference_corpus.trace(root)+scene_corpus.trace(root)+item_corpus.trace(root)))
        manifest.update(name.encode()+b"\0"+raw)
        components = sorted(getattr(archive, "face_tess_comp_ids", ()))
        framed.extend(context_frame(archive.version, archive.doc_kind, components)+struct.pack("<I", len(raw))+raw)
    return struct.pack("<I", len(records))+framed, records, manifest.hexdigest()


def context_frame(version: int, kind: str, components: list[str]) -> bytes:
    framed = struct.pack("<IIBI", version & 0xffffffff, 19000, int(kind == "assembly"), len(components))
    for component in components:
        encoded = component.encode()
        framed += struct.pack("<I", len(encoded))+encoded
    return framed


def results(raw: bytes, records: list[tuple[str, bytes, int, bytes]], traces: list[bytes]) -> dict[str, object]:
    count, = struct.unpack_from("<I", raw)
    assert count == len(records) == len(traces)
    position = 4
    accepted: list[str] = []
    failures: Counter[str] = Counter()
    versions: Counter[int] = Counter()
    fields: Counter[str] = Counter()
    for (name, source, version, expected), trace in zip(records, traces, strict=True):
        status = raw[position]
        payload, position = reference_corpus.frame_at(raw, position+1)
        if status == 0:
            assert payload == source, name
            fields.update(item_corpus.field_coverage(trace,expected,name))
            accepted.append(name)
        else:
            assert (status, trace) == (1, b""), f"{name}: failure trace {(status, trace)!r}; expected (1, b'')"
            failures[payload.decode()] += 1
            versions[version] += 1
    assert position == len(raw)
    return {"accepted": accepted, "failures": dict(failures.most_common()), "missing_by_version": dict(sorted(versions.items())), "independent_reference_fields": fields["reference"], "independent_scene_fields": fields["scene"], "independent_item_fields": fields["items"]}


def audit(executable: str) -> None:
    framed, records, manifest = collect()
    with tempfile.TemporaryDirectory(prefix="bessemer-display-archives-") as temporary:
        root = Path(temporary)
        local = root/Path(executable).name
        shutil.copyfile(executable, local)
        source, destination, trace = root/"source.bin", root/"result.bin", root/"trace.bin"
        source.write_bytes(framed)
        subprocess.run([str(local), str(source), str(destination), str(trace)], check=True)
        first = destination.read_bytes()
        first_trace = trace.read_bytes()
        subprocess.run([str(local), str(source), str(destination), str(trace)], check=True)
        assert first == destination.read_bytes()
        assert first_trace == trace.read_bytes()
        report = results(first, records, reference_corpus.frames(first_trace))
    report.update({"status": "incomplete implementation audit; missing streams are required work", "documents": len(records), "manifest_sha256": manifest, "accepted_count": len(report["accepted"]), "two_processes_identical": True})
    write_report(report)
    print(json.dumps({key: value for key, value in report.items() if key not in ("accepted", "failures")}))


def accepted_paths(report: dict[str, object]) -> set[str]:
    if "accepted" in report:
        return set(report["accepted"])
    return {name for filename in report["accepted_manifests"] for name in Path(__file__).with_name(filename).read_text(encoding="utf-8").splitlines()}


def write_report(report: dict[str, object]) -> None:
    destination = Path(__file__).with_name("archive-audit.json")
    previous = json.loads(destination.read_text(encoding="utf-8"))
    lost = accepted_paths(previous)-set(report["accepted"])
    assert not lost, f"lost complete reconstruction for {sorted(lost)}; expected previous successes to remain accepted"
    accepted = report.pop("accepted")
    manifests: list[str] = []
    for start in range(0, len(accepted), 400):
        name = f"archive-accepted-{start//400:03}.txt"
        Path(__file__).with_name(name).write_text("\n".join(accepted[start:start+400])+"\n", encoding="utf-8", newline="\n")
        manifests.append(name)
    report["accepted_manifests"] = manifests
    destination.write_text(json.dumps(report, indent=2)+"\n", encoding="utf-8", newline="\n")


if __name__ == "__main__":
    audit(sys.argv[1])
