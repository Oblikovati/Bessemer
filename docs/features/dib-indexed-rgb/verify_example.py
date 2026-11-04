"""Exercise the shipped indexed raster caller in two independent processes per image."""
from pathlib import Path
from itertools import product
from typing import Iterator
import argparse
import hashlib
import io
import json
import struct
import subprocess
import tempfile


def read_blob(cursor: io.BytesIO) -> bytes:
    length, = struct.unpack('<I', cursor.read(4))
    blob = cursor.read(length)
    assert len(blob) == length
    return blob


def indexed_images(packet: bytes) -> Iterator[tuple[bytes, bytes]]:
    cursor = io.BytesIO(packet)
    assert cursor.read(4) == b'BIX1'
    assert struct.unpack('<I', cursor.read(4)) == (864,)
    for _ in range(864):
        original, edited = read_blob(cursor), read_blob(cursor)
        indices, edited_indices = read_blob(cursor), read_blob(cursor)
        yield original, indices
        yield edited, edited_indices
    assert cursor.read() == b''


PARSER = argparse.ArgumentParser()
PARSER.add_argument('executable', type=Path)
ARGS = PARSER.parse_args()
EXECUTABLE = ARGS.executable.resolve()
ROOT = Path(__file__).resolve().parents[3]
PACKET = (ROOT / 'tests/fixtures/preview-indexed-rgb/cases.bin').read_bytes()
IMAGES = list(indexed_images(PACKET))
MINIMAL = [ROOT / f'tests/fixtures/preview-indexed-rgb/one-entry-{bits}.bin' for bits in (1, 4, 8)]
IMAGES.extend((path.read_bytes(), bytes((0,))) for path in MINIMAL)
TRIALS = 0
SEEN = {}
with tempfile.TemporaryDirectory(prefix='bessemer-indexed-rgb-example-') as temporary:
    source, rebuilt, raster = (Path(temporary) / name for name in ('source', 'rebuilt', 'raster'))
    for (number, (original, indices)), replay in product(enumerate(IMAGES), range(2)):
        source.write_bytes(original)
        subprocess.run([str(EXECUTABLE), str(source), str(rebuilt), str(raster)], check=True)
        actual = (rebuilt.read_bytes(), raster.read_bytes())
        expected = b''.join(bytes((1, index)) for index in indices)
        assert actual == (original, expected), (number, replay)
        assert SEEN.get(number, actual) == actual, (number, replay)
        SEEN[number] = actual
        TRIALS += 1
assert TRIALS == 3462
print(json.dumps({'images': len(IMAGES), 'fresh_process_trials': TRIALS,
                  'executable_sha256': hashlib.sha256(EXECUTABLE.read_bytes()).hexdigest(),
                  'packet_sha256': hashlib.sha256(PACKET).hexdigest(),
                  'one_entry_sha256': {path.name: hashlib.sha256(path.read_bytes()).hexdigest() for path in MINIMAL}}, indent=2))
