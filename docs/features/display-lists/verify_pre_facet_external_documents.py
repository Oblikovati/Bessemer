"""Exact external face bytes, context resolution and complete document reconstruction."""
from pathlib import Path
import json
import hashlib
import platform
import sys
import tempfile
from external_link_reference import expected
from verify_external_display_links import two_reports
from verify_document_contexts import verify_batch


if __name__=="__main__":
    rows = json.loads(Path(__file__).with_name("pre-facet-external-documents.json").read_text())["fixtures"]
    records,hashes = [],[]
    with tempfile.TemporaryDirectory(prefix="bessemer-old-external-") as folder:
        for row in rows:
            path = Path("tests/fixtures/display-lists")/row["fixture"]
            raw = path.read_bytes()
            assert hashlib.sha256(raw).hexdigest()==row["sha256"]
            result = two_reports(Path(sys.argv[1]).resolve(),Path(folder)/"links",path,expected(str(path)))
            hashes.append(hashlib.sha256(result).hexdigest())
            records.append((row,raw,row["component_ids"]))
    context_hash = verify_batch(sys.argv[2],records)
    report = {"documents":len(rows),"requested_face_references":len(rows)*2,"all_complete_facet_bytes_match":True,"all_complete_documents_exact":True,"two_processes_identical":True,"context_sha256":context_hash,"link_report_sha256":hashes,"platform":platform.platform(),"scope":"exact cache links and stored bytes; geometric and native application validity unclaimed"}
    Path(__file__).with_name("pre-facet-external-verification.json").write_text(json.dumps(report,indent=2)+"\n",encoding="utf-8",newline="\n")
    print(json.dumps(report))
