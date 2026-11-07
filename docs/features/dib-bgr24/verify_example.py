"""Check the shipped reconstruction executable against independent literal oracles."""
from pathlib import Path
from itertools import product
import argparse
import hashlib
import json
import subprocess
import tempfile

PARSER = argparse.ArgumentParser()
PARSER.add_argument('executable', type=Path)
ARGS = PARSER.parse_args()
EXECUTABLE = ARGS.executable.resolve()
ROOT = Path(__file__).resolve().parents[3]
FIXTURES = ROOT / 'tests/fixtures/preview-bgr24'
MANIFEST = json.loads((FIXTURES / 'manifest.json').read_text())
TRIALS = 0
SEEN = {}
with tempfile.TemporaryDirectory(prefix='bessemer-bgr24-example-') as temporary:
    destination = Path(temporary) / 'rebuilt.bin'
    raster = Path(temporary) / 'raster.bin'
    for case, replay in product(MANIFEST, range(2)):
        original = FIXTURES / f"{case['name']}.bin"
        expected = (FIXTURES / f"{case['name']}.bgr").read_bytes()
        subprocess.run([str(EXECUTABLE), str(original), str(destination), str(raster)], check=True)
        actual = (destination.read_bytes(), raster.read_bytes())
        assert actual == (original.read_bytes(), expected), case['name']
        assert SEEN.get(case['name'], actual) == actual, (case['name'], replay)
        SEEN[case['name']] = actual
        TRIALS += 1
assert TRIALS == 256
print(json.dumps({'images': len(MANIFEST), 'fresh_process_trials': TRIALS,
                  'executable_sha256': hashlib.sha256(EXECUTABLE.read_bytes()).hexdigest(),
                  'manifest_sha256': hashlib.sha256((FIXTURES / 'manifest.json').read_bytes()).hexdigest()}, indent=2))
