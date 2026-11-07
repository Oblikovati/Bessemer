"""Frozen unsigned drawing streams, markers and complete documents in two processes."""
from pathlib import Path
import sys
import json
import struct
import subprocess
import tempfile
from generate_unsigned import generate,VERSIONS
from verify_corpus import verify,frames,reference,provenance,drawingvblists,swcontainer
from verify_unsigned_gates import NAMES,comparisons


if __name__=="__main__":
    for name in NAMES:comparisons(Path("../inventor-interface/solidworks")/(name+".py"))
    cases = [(version,index,generate(index)) for version in VERSIONS for index in range(5)]
    destination = Path("tests/fixtures/drawing-buckets")
    assert frames([raw for _,_,raw in cases])==(destination/"unsigned.bin").read_bytes()
    assert frames([reference(raw,v) for v,_,raw in cases])==(destination/"unsigned-fields.bin").read_bytes()
    report = verify(sys.argv[1],[(f"{v}-{index}",raw) for v,index,raw in cases],[v for v,_,_ in cases])
    document = destination/"unsigned-document.bin"
    assert drawingvblists.read(swcontainer.Document(str(document)),through_models=True)["fully_framed"]
    with tempfile.TemporaryDirectory(prefix="bessemer-unsigned-drawing-") as directory:
        for index in range(2):
            output = Path(directory)/f"document-{index}"
            subprocess.run([str(Path(sys.argv[1]).resolve()),"--document",str(document),str(output)],check=True)
            assert output.read_bytes()==document.read_bytes()
    report["whole_documents_exact_in_two_processes"] = 1
    report["source_sha256"] = provenance()
    Path(__file__).with_name("unsigned-verification.json").write_text(json.dumps(report,indent=2)+"\n",encoding="utf-8",newline="\n")
    print(json.dumps({key:value for key,value in report.items() if key!="source_sha256"}))
