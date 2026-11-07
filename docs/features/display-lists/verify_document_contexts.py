"""Compare resolved document context and complete forced display reconstruction in two processes."""
from pathlib import Path
from collections.abc import Iterator
import hashlib
import json
import platform
import struct
import sys
from audit_archive_corpus import context_frame,accepted_paths
from verify_configuration_processes import two_processes
from generate_alias_fixture import swcontainer,displaylists
import facettess
import swcover


def frames(records: list[tuple[dict[str,object],bytes,list[str]]]) -> tuple[bytes,bytes,bytes]:
    prefix = struct.pack("<I",len(records))
    source,expected,contexts = bytearray(prefix),bytearray(prefix),bytearray(prefix)
    for row,raw,identities in records:
        blob = struct.pack("<I",len(raw))+raw
        source.extend(struct.pack("<BII",row["kind"]=="assembly",row["file_version"],row["directory_version"])+blob)
        expected.extend(b"\0"+blob)
        context = context_frame(row["display_version"],row["kind"],identities,row["file_version"])
        contexts.extend(struct.pack("<I",len(context))+context)
    return bytes(source),bytes(expected),bytes(contexts)


def owned_records() -> list[tuple[dict[str,object],bytes,list[str]]]:
    manifest = json.loads(Path(__file__).with_name("document-contexts.json").read_text(encoding="utf-8"))
    result = []
    for row in manifest["fixtures"]:
        raw = (Path("tests/fixtures/display-lists")/(row["fixture"]+".bin")).read_bytes()
        assert hashlib.sha256(raw).hexdigest()==row["sha256"]
        result.append((row,raw,row["component_ids"]))
    return result


def native_records() -> Iterator[tuple[dict[str,object],bytes,list[str]]]:
    previous = json.loads(Path(__file__).with_name("archive-audit.json").read_text(encoding="utf-8"))
    accepted = accepted_paths(previous)
    found = set()
    for path in sorted(map(Path,swcover.collect([]))):
        name = path.relative_to(Path(swcover.SAMPLES)).as_posix()
        if name not in accepted:continue
        document = swcontainer.Document(str(path))
        names = document.names()
        version = displaylists.dl_version(names)
        assert version is not None
        identities = sorted({entry["comp_id"] for entry in facettess.directory(document.read("FaceTessellations/Directory"),19000)["entries"]}) if "FaceTessellations/Directory" in names else []
        row = {"fixture":name,"kind":"part" if path.suffix.lower()==".sldprt" else "assembly","display_version":version&0xffffffff,"file_version":19000,"directory_version":19000}
        found.add(name)
        yield row,document.b,identities
    assert found==accepted,(len(found),len(accepted))


def verify_batch(executable: str, records: list[tuple[dict[str,object],bytes,list[str]]]) -> str:
    source,expected,contexts = frames(records)
    return two_processes(executable,source,expected,contexts)


def audit(executable: str, native: bool) -> dict[str,object]:
    owned = owned_records()
    hashes = [verify_batch(executable,owned)]
    manifest = hashlib.sha256()
    count,identities,batch = 0,0,[]
    for row,raw,components in native_records() if native else []:
        manifest.update(row["fixture"].encode()+b"\0"+raw)
        count += 1
        identities += len(components)
        batch.append((row,raw,components))
        if len(batch)==64:
            hashes.append(verify_batch(executable,batch))
            batch.clear()
    if batch:hashes.append(verify_batch(executable,batch))
    return {"owned_documents":len(owned),"native_documents":count,"resolved_native_component_ids":identities,"native_document_manifest_sha256":manifest.hexdigest() if native else None,"batch_context_sha256":hashes}


def main(executable: str, native: bool) -> None:
    report = audit(executable,native)
    report.update({"platform":platform.platform(),"all_complete_documents_byte_identical":True,"all_contexts_match_independent_declarations":True,"two_processes_identical":True,"scope":"context declarations and forced complete display-stream reconstruction; external block and geometry validity excluded"})
    report["source_sha256"] = {name:hashlib.sha256(Path(name).read_bytes()).hexdigest() for name in ["src/display_lists/document_context.rs","examples/rebuild_display_documents.rs"]}
    name = "document-context-native-verification.json" if native else "document-context-owned-verification.json"
    Path(__file__).with_name(name).write_text(json.dumps(report,indent=2)+"\n",encoding="utf-8",newline="\n")
    print(json.dumps(report))


if __name__=="__main__":
    main(sys.argv[1],"--native" in sys.argv[2:])
