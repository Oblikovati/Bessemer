"""Check complete 4800-range containers and exact contexts in two isolated Rust processes."""
import sys
import json
import hashlib
import platform
from pathlib import Path
from verify_document_contexts import frames
from verify_configuration_processes import two_processes


def verify(executable: str) -> None:
    manifest = json.loads(Path(__file__).with_name("earliest-component-documents.json").read_text())
    records = []
    for row in manifest["fixtures"]:
        raw = Path(f"tests/fixtures/display-lists/earliest-component-document-{row['kind']}.bin").read_bytes()
        assert hashlib.sha256(raw).hexdigest()==row["sha256"]
        records.append((row,raw,[]))
    source,expected,contexts = frames(records)
    context_hash = two_processes(executable,source,expected,contexts)
    report = {"documents":len(records),"all_complete_documents_byte_identical":True,"contexts_match_independent_expectations":True,"two_processes_identical":True,"input_sha256":hashlib.sha256(source).hexdigest(),"result_sha256":hashlib.sha256(expected).hexdigest(),"context_sha256":context_hash,"platform":platform.platform()}
    Path(__file__).with_name("earliest-component-document-verification.json").write_text(json.dumps(report,indent=2)+"\n",encoding="utf-8",newline="\n")
    print(json.dumps(report))


if __name__=="__main__":
    verify(sys.argv[1])
