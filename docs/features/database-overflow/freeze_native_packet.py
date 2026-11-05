"""Create or verify the bounded native overflow regression corpus (#229)."""
import argparse
import gzip
import hashlib
import json
from pathlib import Path
from typing import cast
from native_packet import NativeCensusRow, native_packet

ROOT = Path(__file__).resolve().parents[3]
FOLDER = Path(__file__).resolve().parent
PACKET = ROOT / 'tests/fixtures/database-overflow/native-census.bin.gz'
REFERENCE = FOLDER / 'native-packet-reference.json'


def packet_reference(raw: bytes, compressed: bytes) -> dict[str, object]:
    """Bind frozen packet lengths/hashes to the retained census; e.g. packet_reference(raw,gz)."""
    return {'issue': 229, 'format': 'BOV1', 'cases': 5140,
            'bytes': len(raw), 'sha256': hashlib.sha256(raw).hexdigest(),
            'gzip_bytes': len(compressed), 'gzip_sha256': hashlib.sha256(compressed).hexdigest(),
            'census_sha256': hashlib.sha256((FOLDER / 'native-candidate-census.json.gz').read_bytes()).hexdigest(),
            'basis': 'Extracted native database bytes and independently checked retained payload spans',
            'native_target_executed': False}


def freeze(image_root: Path) -> None:
    """Explicitly regenerate every native source-bound record; e.g. freeze(Path('target/images'))."""
    census = json.loads(gzip.decompress((FOLDER / 'native-candidate-census.json.gz').read_bytes()))
    raw = native_packet(cast(list[NativeCensusRow], census['samples']), image_root)
    compressed = gzip.compress(raw, mtime=0)
    PACKET.write_bytes(compressed)
    REFERENCE.write_text(json.dumps(packet_reference(raw, compressed), indent=2) + '\n', encoding='utf-8')


def verify() -> None:
    """Verify a frozen image without installed sources; e.g. verify()."""
    compressed = PACKET.read_bytes()
    raw = gzip.decompress(compressed)
    reference = json.loads(REFERENCE.read_text(encoding='utf-8'))
    assert packet_reference(raw, compressed) == reference
    assert raw[:8] == b'BOV1\x14\x14\0\0'
    print(json.dumps(reference))


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--write-from', type=Path)
    arguments = parser.parse_args()
    if arguments.write_from is not None:
        freeze(arguments.write_from)
    verify()
