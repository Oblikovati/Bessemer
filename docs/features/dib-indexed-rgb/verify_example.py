"""Exercise the shipped DIB raster caller in two independent processes per image."""
from pathlib import Path
from itertools import product
from typing import Callable, Iterator
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


def packet_images(packet: bytes, magic: bytes, count: int) -> Iterator[tuple[bytes, bytes]]:
    cursor = io.BytesIO(packet)
    assert cursor.read(4) == magic
    assert struct.unpack('<I', cursor.read(4)) == (count,)
    for _ in range(count):
        original, edited = read_blob(cursor), read_blob(cursor)
        indices, edited_indices = read_blob(cursor), read_blob(cursor)
        yield original, indices
        yield edited, edited_indices
    assert cursor.read() == b''


def indexed_raster(indices: bytes) -> bytes:
    return b''.join(bytes((1, index)) for index in indices)


PARSER = argparse.ArgumentParser()
PARSER.add_argument('executable', type=Path)
PARSER.add_argument('--family', choices=('indexed-rgb', 'rgb555'), default='indexed-rgb')
ARGS = PARSER.parse_args()
EXECUTABLE = ARGS.executable.resolve()
ROOT = Path(__file__).resolve().parents[3]
FORMATS = {'indexed-rgb': (b'BIX1', 864), 'rgb555': (b'B165', 136)}
PACKET = (ROOT / f'tests/fixtures/preview-{ARGS.family}/cases.bin').read_bytes()
IMAGES = list(packet_images(PACKET, *FORMATS[ARGS.family]))
MINIMAL = [ROOT / f'tests/fixtures/preview-indexed-rgb/one-entry-{bits}.bin' for bits in (1, 4, 8)] if ARGS.family == 'indexed-rgb' else []
IMAGES.extend((path.read_bytes(), bytes((0,))) for path in MINIMAL)
EXPECTED_RASTER: Callable[[bytes], bytes] = indexed_raster if ARGS.family == 'indexed-rgb' else bytes
TRIALS = 0
SEEN = {}
with tempfile.TemporaryDirectory(prefix='bessemer-indexed-rgb-example-') as temporary:
    source, rebuilt, raster = (Path(temporary) / name for name in ('source', 'rebuilt', 'raster'))
    for (number, (original, indices)), replay in product(enumerate(IMAGES), range(2)):
        source.write_bytes(original)
        subprocess.run([str(EXECUTABLE), str(source), str(rebuilt), str(raster)], check=True)
        actual = (rebuilt.read_bytes(), raster.read_bytes())
        expected = EXPECTED_RASTER(indices)
        assert actual == (original, expected), (number, replay)
        assert SEEN.get(number, actual) == actual, (number, replay)
        SEEN[number] = actual
        TRIALS += 1
assert TRIALS == (3462 if ARGS.family == 'indexed-rgb' else 544)
print(json.dumps({'images': len(IMAGES), 'fresh_process_trials': TRIALS,
                  'executable_sha256': hashlib.sha256(EXECUTABLE.read_bytes()).hexdigest(),
                  'packet_sha256': hashlib.sha256(PACKET).hexdigest(),
                  'one_entry_sha256': {path.name: hashlib.sha256(path.read_bytes()).hexdigest() for path in MINIMAL}}, indent=2))
