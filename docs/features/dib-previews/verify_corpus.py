"""External corpus oracle; uses the approved sibling reader and Pillow, never production code."""
import hashlib
import io
import json
from pathlib import Path
import struct
import shutil
import subprocess
import sys
import tempfile
from PIL import Image

sys.path.insert(0, str(Path("../inventor-interface/solidworks").resolve()))
import swcover
import swcontainer as sc

SOURCE_EXECUTABLE = Path(sys.argv[1]).resolve()


def oracle_pixels(raw: bytes, raster: bytes) -> tuple[int, int]:
    header = struct.unpack_from("<IiiHHIIiiII", raw, 4)
    palette = header[9] or (256 if header[4] == 8 else 0)
    bitmap = struct.pack("<2sIHHI", b"BM", len(raw) + 10, 0, 0, 54 + 4 * palette) + raw[4:]
    image = Image.open(io.BytesIO(bitmap))
    image.load()
    if header[4] == 8:
        return indexed_oracle(image.tobytes(), raster)
    expected = image.convert("RGB").tobytes()
    assert len(raster) // 4 == len(expected) // 3
    assert raster[2::4] == expected[0::3] and raster[1::4] == expected[1::3] and raster[0::4] == expected[2::3], "BGRX pixel oracle mismatch"
    return len(expected) // 3, 0


def indexed_oracle(expected: bytes, raster: bytes) -> tuple[int, int]:
    assert len(raster) == 2 * len(expected), "index raster length mismatch"
    if raster[::2].count(0) == 0:
        assert raster[1::2] == expected, "index pixel oracle mismatch"
        return len(expected), 0
    unwritten = 0
    for index, value in enumerate(expected):
        if raster[2 * index]:
            assert raster[2 * index + 1] == value, f"pixel {index}: oracle mismatch"
        else:
            unwritten += 1
    return len(expected), unwritten


def verify_stream(raw: bytes, root: Path) -> tuple[int, int]:
    source, output, raster = (root / name for name in ("source", "output", "raster"))
    source.write_bytes(raw)
    process = subprocess.run([EXECUTABLE, str(source), str(output), str(raster)], capture_output=True, text=True)
    assert process.returncode == 0, process.stderr
    assert output.read_bytes() == raw, "forced reconstruction changed stored bytes"
    return oracle_pixels(raw, raster.read_bytes())


counts = {"streams": 0, "bytes": 0, "pixels": 0, "unwritten": 0}
digest = hashlib.sha256()
samples = Path(swcover.SAMPLES)
with tempfile.TemporaryDirectory(prefix="bessemer-dib-corpus-") as directory:
    EXECUTABLE = str(Path(directory) / "rebuild_dib.exe")
    shutil.copyfile(SOURCE_EXECUTABLE, EXECUTABLE)
    for path in sorted(map(Path, swcover.collect([]))):
        document = sc.Document(str(path))
        for entry in document.entries:
            if not swcover.THUMBNAILS[0][0].match(entry.name):
                continue
            raw = document.read(entry.name)
            if not swcover.sized_dib(raw):
                continue
            try:
                pixels, holes = verify_stream(raw, Path(directory))
            except AssertionError as error:
                raise AssertionError(f"{path}: {entry.name}: {error}") from error
            counts["streams"] += 1
            counts["bytes"] += len(raw)
            counts["pixels"] += pixels
            counts["unwritten"] += holes
            digest.update(path.relative_to(samples).as_posix().encode() + b"\0" + entry.name.encode() + b"\0" + raw)
print(json.dumps({**counts, "manifest": digest.hexdigest(), "pillow": Image.__version__}))
