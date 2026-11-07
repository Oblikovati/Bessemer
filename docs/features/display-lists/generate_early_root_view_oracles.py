"""Independent complete-root oracles for all three nested view collection types."""
from pathlib import Path
import hashlib
import json
import struct
from generate_early_view_roots import generate
from generate_alias_fixture import displaylists,swcontainer
from reference_corpus import frame_at
from view_corpus import semantic_trace


def record(row: dict[str,object]) -> tuple[bytes,int]:
    framed,actual = generate(row["version"],row["kind"],row["scenario"])
    assert actual==row
    raw,_ = frame_at(framed,6)
    path = Path("tests/fixtures/blank-part.SLDPRT")
    archive = displaylists.archive_for(swcontainer.Document(str(path)),str(path),raw)
    archive.version,archive.doc_kind,archive.face_tess_comp_ids = row["version"],row["kind"],set()
    root = displaylists.display_lists(archive,row["kind"])
    assert archive.p==len(raw)
    traces = [semantic_trace(root[key]) for key in ["named_view_transform","view_3d_transform","annotation_view_transform"]]
    encoded = struct.pack("<IBB",row["version"],row["kind"]=="assembly",row["scenario"])+b"".join(struct.pack("<I",len(trace))+trace for trace in traces)
    return encoded,sum(len(trace.splitlines()) for trace in traces)


if __name__=="__main__":
    manifests = [json.loads(Path(__file__).with_name(name+".json").read_text(encoding="utf-8")) for name in ["early-view-roots-part","early-view-roots-assembly","earliest-minimal-annotations"]]
    cases = [record(row) for manifest in manifests for row in manifest["fixtures"]]
    encoded = struct.pack("<I",len(cases))+b"".join(raw for raw,_ in cases)
    Path("tests/fixtures/display-lists/early-root-view-oracles.bin").write_bytes(encoded)
    report = {"scope":"independent semantic fields of all nested view collections in complete owned roots","fixtures":len(cases),"fields":sum(fields for _,fields in cases),"sha256":hashlib.sha256(encoded).hexdigest(),"root_manifests_sha256":[manifest["sha256"] for manifest in manifests]}
    Path(__file__).with_name("early-root-view-oracles.json").write_text(json.dumps(report,indent=2)+"\n",encoding="utf-8",newline="\n")
    print(json.dumps(report))
