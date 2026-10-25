"""Compare exact linked facet bytes and explicit incomplete-cache failures with the independent reader."""
from pathlib import Path
from collections import Counter
import hashlib
import json
import platform
import shutil
import struct
import subprocess
import sys
import tempfile
from external_link_reference import expected
from reference_corpus import frame_at
from generate_alias_fixture import swcontainer
import swcover


def owned_paths() -> list[Path]:
    manifest = json.loads(Path(__file__).with_name("external-link-documents.json").read_text(encoding="utf-8"))
    paths = []
    for row in manifest["fixtures"]:
        path = Path("tests/fixtures/display-lists")/row["fixture"]
        assert hashlib.sha256(path.read_bytes()).hexdigest()==row["sha256"]
        paths.append(path)
    return paths


def native_paths() -> list[Path]:
    result = []
    for path in sorted(map(Path,swcover.collect([]))):
        if path.suffix.lower() not in (".sldprt",".sldasm"):continue
        if "FaceTessellations/Directory" in swcontainer.Document(str(path)).names():result.append(path)
    return result


def check_report(raw: bytes, reference: dict[str,object], path: Path) -> None:
    assert raw[0]==reference["status"],(str(path),raw[0],reference["status"],raw[1:300] if raw[0] else b"complete")
    if raw[0]==0:
        assert raw==reference["raw"],str(path)
        return
    position,fields = 1,[]
    for _ in range(3):
        field,position = frame_at(raw,position)
        fields.append(field.decode())
    assert position==len(raw),(str(path),"partial facets published after a failed check")
    assert fields[0]=="check external face links",(str(path),fields)
    assert fields[1] and "expected" in fields[2],(str(path),fields)


def two_reports(executable: Path, destination: Path, path: Path, reference: dict[str,object]) -> bytes:
    outputs = []
    kind = "assembly" if path.suffix.lower()==".sldasm" else "part"
    for _ in range(2):
        subprocess.run([str(executable),str(path.resolve()),kind,"19000","19000",str(destination)],check=True)
        raw = destination.read_bytes()
        check_report(raw,reference,path)
        outputs.append(raw)
    assert outputs[0]==outputs[1],str(path)
    return outputs[0]


def audit(executable: str, native: bool) -> dict[str,object]:
    paths = native_paths() if native else owned_paths()
    statuses,totals = Counter(),Counter()
    manifest,results = hashlib.sha256(),hashlib.sha256()
    incomplete = []
    with tempfile.TemporaryDirectory(prefix="bessemer-external-links-") as folder:
        root = Path(folder)
        private = root/Path(executable).name
        shutil.copyfile(executable,private)
        for number,path in enumerate(paths,1):
            reference = expected(str(path))
            raw = two_reports(private,root/"report.bin",path,reference)
            collect_counts(statuses,totals,reference)
            name = path.relative_to(Path(swcover.SAMPLES)).as_posix() if native else path.name
            manifest.update(name.encode()+b"\0"+path.read_bytes())
            results.update(name.encode()+b"\0"+raw)
            if reference["status"]:incomplete.append(f"{reference['status']}\t{name}")
            if number%25==0:print(json.dumps({"checked_documents":number,"total":len(paths)}),flush=True)
    return write_report(paths,statuses,totals,manifest.hexdigest(),results.hexdigest(),incomplete,native)


def collect_counts(statuses: Counter, totals: Counter, reference: dict[str,object]) -> None:
    statuses[reference["status"]] += 1
    for key in ["requested_faces","directory_entries","blocks","unlocated_entries"]:totals[key]+=reference[key]
    if reference["status"]==0:
        totals["resolved_face_references"]+=reference["coverage"][5]
        totals["unique_resolved_faces"]+=reference["coverage"][6]


def write_report(paths: list[Path], statuses: Counter, totals: Counter, manifest: str, results: str, incomplete: list[str], native: bool) -> dict[str,object]:
    label = "native" if native else "owned"
    report = {"documents":len(paths),"complete_link_documents":statuses[0],"incomplete_missing_targets":statuses[1],"invalid_links":statuses[2],"unsupported_ambiguous_links":statuses[3],"integrity_failures":statuses[4],"counts":dict(totals),"all_statuses_match_independent_reference":True,"all_complete_facet_bytes_match":True,"no_partial_facets_after_failure":True,"two_processes_identical":True,"input_manifest_sha256":manifest,"result_sha256":results,"platform":platform.platform(),"scope":"exact archive links only; incomplete caches remain incomplete and geometry validity is not certified"}
    modules = ["src/display_lists/external_face_links.rs","src/display_lists/external_face_coverage.rs","src/display_lists/external_face_errors.rs","examples/check_external_display_links.rs"]
    report["source_sha256"] = {name:hashlib.sha256(Path(name).read_bytes()).hexdigest() for name in modules}
    Path(__file__).with_name(f"external-link-{label}-verification.json").write_text(json.dumps(report,indent=2)+"\n",encoding="utf-8",newline="\n")
    Path(__file__).with_name(f"external-link-{label}-incomplete.tsv").write_text("\n".join(incomplete)+("\n" if incomplete else ""),encoding="utf-8",newline="\n")
    return report


if __name__=="__main__":
    print(json.dumps(audit(sys.argv[1],"--native" in sys.argv[2:])))
