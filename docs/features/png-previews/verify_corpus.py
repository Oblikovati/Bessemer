"""Installed-corpus PNG oracle using the approved sibling reader and Pillow."""
import hashlib
import io
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
from PIL import Image

sys.path.insert(0,str(Path("../inventor-interface/solidworks").resolve()))
import swcontainer as sc
import swcover

SOURCE = Path(sys.argv[1]).resolve()


def verify(raw: bytes, root: Path, executable: str) -> int:
    source,output,samples,rgba = (root/name for name in ("source","output","samples","rgba"))
    source.write_bytes(raw)
    result = subprocess.run([executable,str(source),str(output),str(samples),str(rgba)],capture_output=True,text=True)
    assert result.returncode == 0, result.stderr
    assert output.read_bytes() == raw, "forced reconstruction mismatch"
    image = Image.open(io.BytesIO(raw))
    expected = image.convert("RGBA").tobytes()
    actual = rgba.read_bytes()
    assert actual[::2] == expected and actual[1::2] == expected, "exact expanded RGBA16 mismatch for native 8-bit PNG"
    return image.width * image.height


counts = {"streams":0,"bytes":0,"pixels":0}
manifest = hashlib.sha256()
samples_root = Path(swcover.SAMPLES)
with tempfile.TemporaryDirectory(prefix="bessemer-png-corpus-") as folder:
    root = Path(folder)
    executable = str(root / "rebuild_png.exe")
    shutil.copyfile(SOURCE,executable)
    for path in sorted(map(Path,swcover.collect([]))):
        document = sc.Document(str(path))
        for entry in document.entries:
            if not swcover.THUMBNAILS[0][0].match(entry.name):
                continue
            raw = document.read(entry.name)
            if not raw.startswith(b"\x89PNG\r\n\x1a\n"):
                continue
            try:
                pixels = verify(raw,root,executable)
            except AssertionError as error:
                raise AssertionError(f"{path}: {entry.name}: {error}") from error
            counts["streams"] += 1
            counts["bytes"] += len(raw)
            counts["pixels"] += pixels
            manifest.update(path.relative_to(samples_root).as_posix().encode()+b"\0"+entry.name.encode()+b"\0"+raw)
print(json.dumps({**counts,"manifest":manifest.hexdigest(),"pillow":Image.__version__}))
