"""Independent complete VBLists reconstruction and stored drawing-field oracles."""
from pathlib import Path
import sys
import json
import hashlib
import struct
import subprocess
import tempfile
import platform
sys.path.insert(0,str(Path(__file__).resolve().parents[1]/"display-lists"))
from generate_alias_fixture import swcontainer
import drawingvblists


def scalar(value: object) -> str:
    if isinstance(value,str):return "s:"+value.encode().hex()
    assert isinstance(value,int),value
    return f"i:{value}"


def reference(raw: bytes, version: int = 19000) -> bytes:
    root = drawingvblists.read_data(raw,version,through_models=True)
    rows = {"root."+key:scalar(root[key]) for key in ["view_bucket_count","shaded_data_present"]}
    header = root["shaded_prefix"]
    if header is not None:header_fields(header,rows)
    for index,model in enumerate(root["models"]):
        for key in ["is_part_word","model_reference","raw_30"]:
            rows[f"root.models[{index}].{key}"] = scalar(model[key])
    for key,lists in (root["trailing_word_lists"] or {}).items():
        for index,values in enumerate(lists):direct_list(rows,f"root.{key}[{index}]",values,"words")
    return "".join(f"{key}={value}\n" for key,value in sorted(rows.items())).encode()


def header_fields(header: dict[str,object], rows: dict[str,str]) -> None:
    rows["root.model_count"] = scalar(header["model_count"])
    for key in ["shaded_model_indices","raw_words_20"]:
        for index,value in enumerate(header[key]):rows[f"root.{key}[{index}]"] = scalar(value)
    for key in ["string_lists_28","string_lists_68"]:
        for index,values in enumerate(header[key]):
            direct_list(rows,f"root.{key}[{index}]",[item["value"] for item in values],"strings")


def direct_list(rows: dict[str,str], prefix: str, values: list[object], member: str) -> None:
    rows[prefix+".count"] = scalar(len(values))
    for index,value in enumerate(values):rows[f"{prefix}.{member}[{index}]"] = scalar(value)


def frames(values: list[bytes]) -> bytes:
    return struct.pack("<I",len(values))+b"".join(struct.pack("<I",len(value))+value for value in values)


def verify(executable: str, records: list[tuple[str,bytes]], versions: list[int] | None = None) -> dict[str,object]:
    versions = versions or [19000]*len(records)
    assert len(versions)==len(records)
    raw = frames([struct.pack("<I",v)+value for (_,value),v in zip(records,versions)])
    expected_rows = [reference(value,v) for (_,value),v in zip(records,versions)]
    expected = frames(expected_rows)
    with tempfile.TemporaryDirectory(prefix="bessemer-drawing-") as directory:
        folder = Path(directory)
        (folder/"input").write_bytes(raw)
        outputs = []
        for index in range(2):
            out,trace = folder/f"out{index}",folder/f"trace{index}"
            subprocess.run([str(Path(executable).resolve()),str(folder/"input"),str(out),str(trace)],check=True)
            assert out.read_bytes()==raw,"complete forced reconstruction differs"
            assert trace.read_bytes()==expected,"drawing field oracle differs"
            outputs.append((out.read_bytes(),trace.read_bytes()))
        assert outputs[0]==outputs[1]
    return {"streams":len(records),"two_processes_identical":True,"all_forced_bytes_exact":True,"independent_drawing_fields":sum(len(value.splitlines()) for value in expected_rows),"manifest_sha256":hashlib.sha256(b"".join(name.encode()+b"\0"+value for name,value in records)).hexdigest(),"trace_sha256":hashlib.sha256(expected).hexdigest(),"platform":platform.platform()}


def provenance() -> dict[str,str]:
    rust = ["archive","archive_encode","archive_layout","drawing_buckets","drawing_bucket_read","drawing_document","modern_body_tess"]
    paths = [Path("src/display_lists")/(name+".rs") for name in rust]
    paths += [Path("../inventor-interface/solidworks")/name for name in ["drawingvblists.py","dl_tess.py","dl_light.py","dl_ui.py","facettess.py","mfcobjects.py"]]
    return {str(path):hashlib.sha256(path.read_bytes()).hexdigest() for path in paths}


def native_records() -> list[tuple[str,bytes]]:
    root = Path("C:/Users/Public/Documents/SOLIDWORKS/SOLIDWORKS 2026")
    paths = sorted(path for path in root.rglob("*") if path.suffix.lower()==".slddrw")
    return [(path.relative_to(root).as_posix(),swcontainer.Document(str(path)).read("Contents/VBLists")) for path in paths]


def verify_documents(executable: str, records: list[tuple[str,bytes]]) -> None:
    root = Path("C:/Users/Public/Documents/SOLIDWORKS/SOLIDWORKS 2026")
    with tempfile.TemporaryDirectory(prefix="bessemer-drawing-docs-") as directory:
        for index,(name,_) in enumerate(records):
            source = root/name
            expected = source.read_bytes()
            for process in range(2):
                output = Path(directory)/f"document-{index}-{process}"
                subprocess.run([str(Path(executable).resolve()),"--document",str(source),str(output)],check=True)
                assert output.read_bytes()==expected,f"whole drawing bytes differ: {name}"


if __name__=="__main__":
    records = native_records()
    assert len(records)==20,len(records)
    report = verify(sys.argv[1],records)
    verify_documents(sys.argv[1],records)
    report["whole_documents_exact_in_two_processes"] = len(records)
    report["source_sha256"] = provenance()
    parsed = [drawingvblists.read_data(raw,19000,through_models=True) for _,raw in records]
    models = [model for root in parsed for model in root["models"]]
    report.update({"decoded_bytes":sum(len(raw) for _,raw in records),"shaded_models":len(models),"part_models":sum(model["is_part_word"] for model in models),"assembly_models":sum(1-model["is_part_word"] for model in models)})
    assert len(models)==33 and report["decoded_bytes"]==8215597
    report["fixtures"] = [{"path":name,"bytes":len(raw),"sha256":hashlib.sha256(raw).hexdigest()} for name,raw in records]
    Path(__file__).with_name("native-verification.json").write_text(json.dumps(report,indent=2)+"\n",encoding="utf-8",newline="\n")
    print(json.dumps({key:value for key,value in report.items() if key!="fixtures"}))
