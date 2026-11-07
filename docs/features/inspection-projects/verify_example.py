"""Check the shipped edit example against an independent native-container/XML reader."""
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import xml.etree.ElementTree as ET
sys.path.insert(0,str(Path("../inventor-interface/solidworks").resolve()))
from swcontainer import Document

STREAM = "ThirdPty/InspectionXpert"
VALUE = "JOB-λ <&>"


def verify_streams(before: Document, after: Document) -> None:
    assert [entry.name for entry in before.entries] == [entry.name for entry in after.entries]
    for entry in before.entries:
        if entry.name == STREAM:
            continue
        matching = next(candidate for candidate in after.entries if candidate.name == entry.name)
        assert before.read(entry.name) == after.read(entry.name),entry.name
        assert before.b[entry.data_at:entry.data_at+entry.csize] == after.b[matching.data_at:matching.data_at+matching.csize],entry.name


def verify_drawing(executable: Path, root: Path, name: str) -> None:
    source = Path("tests/fixtures")/f"{name}.SLDDRW"
    original = source.read_bytes()
    outputs = [root/f"{name}-{index}.SLDDRW" for index in range(2)]
    for destination in outputs:
        subprocess.run([str(executable),str(source),VALUE,str(destination)],check=True)
    assert outputs[0].read_bytes() == outputs[1].read_bytes()
    before,after = Document(str(source)),Document(str(outputs[0]))
    expected = ET.fromstring(before.read(STREAM))
    expected.find("Settings/JobNumber").text = VALUE
    actual = ET.fromstring(after.read(STREAM))
    assert ET.tostring(expected) == ET.tostring(actual)
    assert after.read(STREAM).startswith(b"\xef\xbb\xbf")
    verify_streams(before,after)
    assert source.read_bytes() == original
    print(f"{name}: two processes identical; complete XML and all untouched compressed streams matched")


if __name__ == "__main__":
    with tempfile.TemporaryDirectory(prefix="bessemer-inspection-") as temporary:
        root = Path(temporary)
        executable = root/Path(sys.argv[1]).name
        shutil.copyfile(sys.argv[1],executable)
        for name in ("inspection-drawing","inspection-job"):
            verify_drawing(executable,root,name)
        rejected = subprocess.run([str(executable)],capture_output=True)
        assert rejected.returncode != 0 and b"expected SOURCE JOB_NUMBER DESTINATION" in rejected.stderr
