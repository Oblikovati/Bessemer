"""Earlier detailing-tail gates reuse the independent double-coordinate bucket vectors."""
from pathlib import Path
import hashlib
import json
import struct
import generate_middle_buckets as middle
from generate_middle_buckets import generate as middle_bucket
import bucket_vector_wire as wire


def omit_final_word(original: object) -> object:
    def encode(*arguments: object) -> bytes:
        return original(*arguments)[:-4]
    return encode


def arc_layout(original: object) -> object:
    def factory(polygon: object) -> object:
        encode_legacy = original(polygon)
        def encode(count: int, dimensions: int, kind: int = 1, arc: bool = True, line: bool = True) -> bytes:
            raw = encode_legacy(count,dimensions,kind,arc,line)
            if count<=0 or not arc:
                return raw
            position = len(raw)-(32+4+83)+73
            return raw[:position]+raw[position+1:]
        return encode
    return factory


def generate(version: int, kind: str, scenario: int) -> tuple[bytes,dict[str,object]]:
    original_bucket,original_view,original_poly = wire.bucket,wire.view_bucket,middle.legacy_poly
    try:
        if version<0x3724:
            middle.legacy_poly = arc_layout(original_poly)
        if version<0x372a:
            wire.bucket = omit_final_word(original_bucket)
        if version<0x374b:
            wire.view_bucket = omit_final_word(original_view)
        return middle_bucket(version,kind,scenario)
    finally:
        wire.bucket,wire.view_bucket,middle.legacy_poly = original_bucket,original_view,original_poly


def main() -> None:
    versions = [14000,0x3723,0x3724,0x3729,0x372a,0x372b,0x374a,0x374b,15999]
    cases = [generate(version,"part",scenario) for version in versions for scenario in [0,16]]+[generate(14000,"part",17),generate(14000,"part",2)]
    framed = struct.pack("<I",len(cases))+b"".join(raw for raw,_ in cases)
    Path("tests/fixtures/display-lists/lower-buckets.bin").write_bytes(framed)
    evidence = {"scope":"complete unshaded part roots in both coordinate scopes; no CAD validity claim","fixtures":[row for _,row in cases],"sha256":hashlib.sha256(framed).hexdigest()}
    evidence["source_hashes"] = {name:hashlib.sha256(Path(f"../inventor-interface/solidworks/{name}").read_bytes()).hexdigest() for name in ["dl_doc.py","dl_ui.py","mfcobjects.py"]}
    Path(__file__).with_name("lower-buckets.json").write_text(json.dumps(evidence,indent=2)+"\n",encoding="utf-8",newline="\n")
    print(json.dumps({"fixtures":len(cases),"fields":sum(row["trace_fields"] for _,row in cases)}))


if __name__=="__main__":
    main()
