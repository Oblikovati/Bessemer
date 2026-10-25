"""Force all owned configuration roots through two isolated Rust reconstruction processes."""
from pathlib import Path
import hashlib
import json
import platform
import shutil
import struct
import subprocess
import sys
import tempfile
from audit_archive_corpus import context_frame
from reference_corpus import frame_at


def records() -> list[tuple[dict[str,object],bytes]]:
    raw = Path("tests/fixtures/display-lists/configurations.bin").read_bytes()
    manifest = json.loads(Path(__file__).with_name("configurations.json").read_text(encoding="utf-8"))
    assert hashlib.sha256(raw).hexdigest()==manifest["sha256"]
    count, = struct.unpack_from("<I",raw)
    assert count==len(manifest["fixtures"])
    position,result = 4,[]
    for row in manifest["fixtures"]:
        version,kind,scenario = struct.unpack_from("<IBB",raw,position)
        payload,position = frame_at(raw,position+6)
        assert (version,kind,scenario)==(row["version"],int(row["kind"]=="assembly"),row["scenario"])
        assert hashlib.sha256(payload).hexdigest()==row["sha256"]
        _,position = frame_at(raw,position)
        _,position = frame_at(raw,position)
        result.append((row,payload))
    assert position==len(raw)
    return result


def frames(cases: list[tuple[dict[str,object],bytes]]) -> tuple[bytes,bytes]:
    prefix = struct.pack("<I",len(cases))
    source,expected = bytearray(prefix),bytearray(prefix)
    for row,payload in cases:
        source.extend(context_frame(row["version"],row["kind"],row["external_ids"],row.get("file_version",19000)))
        blob = struct.pack("<I",len(payload))+payload
        source.extend(blob)
        expected.extend(b"\0"+blob)
    return bytes(source),bytes(expected)


def two_processes(executable: str, source: bytes, expected: bytes) -> str:
    with tempfile.TemporaryDirectory(prefix="bessemer-configurations-") as directory:
        root = Path(directory)
        local = root/Path(executable).name
        shutil.copyfile(executable,local)
        input_path,output,trace = root/"source.bin",root/"result.bin",root/"trace.bin"
        input_path.write_bytes(source)
        results: list[bytes] = []
        for _ in range(2):
            subprocess.run([str(local),str(input_path),str(output),str(trace)],check=True)
            assert output.read_bytes()==expected
            results.append(trace.read_bytes())
        assert results[0]==results[1]
    return hashlib.sha256(results[0]).hexdigest()


def main(executable: str) -> None:
    cases = records()
    source,expected = frames(cases)
    trace_hash = two_processes(executable,source,expected)
    modules = ["archive_decode","archive_scalars","configurations","configuration_views","configuration_tail","modern_root","modern_assembly","modern_body_tess","modern_reference"]
    provenance = {name:hashlib.sha256(Path(f"src/display_lists/{name}.rs").read_bytes()).hexdigest() for name in modules}
    report = {"fixtures":len(cases),"platform":platform.platform(),"two_processes_identical":True,"all_forced_bytes_match_independent_fixtures":True,"input_sha256":hashlib.sha256(source).hexdigest(),"result_sha256":hashlib.sha256(expected).hexdigest(),"trace_sha256":trace_hash,"source_sha256":provenance,"scope":"byte reconstruction and process determinism; independent scalar oracles are checked separately by integration tests"}
    Path(__file__).with_name("configuration-verification.json").write_text(json.dumps(report,indent=2)+"\n",encoding="utf-8",newline="\n")
    print(json.dumps(report))


if __name__=="__main__":
    main(sys.argv[1])
