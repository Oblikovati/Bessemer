"""Frozen owned bytes, independent field expectations and two complete Rust processes."""
from pathlib import Path
import sys
import json
from verify_corpus import verify,frames,reference,provenance
from generate_vectors import generate
from range_wire import generate as ranged,VERSIONS


if __name__=="__main__":
    roots = [generate(index) for index in range(5)]
    ranges = [ranged(version) for version in VERSIONS]
    destination = Path("tests/fixtures/drawing-buckets")
    assert frames(roots)==(destination/"vectors.bin").read_bytes()
    assert frames(ranges)==(destination/"range.bin").read_bytes()
    assert frames([reference(raw) for raw in roots])==(destination/"fields.bin").read_bytes()
    assert frames([reference(raw,v) for raw,v in zip(ranges,VERSIONS)])==(destination/"range-fields.bin").read_bytes()
    cases = [(f"scenario-{index}",raw) for index,raw in enumerate(roots)]
    cases += [(f"version-{v}",raw) for v,raw in zip(VERSIONS,ranges)]
    report = verify(sys.argv[1],cases,[19000]*5+VERSIONS)
    report["source_sha256"] = provenance()
    Path(__file__).with_name("owned-verification.json").write_text(json.dumps(report,indent=2)+"\n",encoding="utf-8",newline="\n")
    print(json.dumps(report))
