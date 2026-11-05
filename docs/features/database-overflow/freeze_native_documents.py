"""Bind native document trials to an explicit source root and full input digests (#229)."""
import argparse
import gzip
import hashlib
import json
from pathlib import Path
import struct
from typing import cast
from native_packet import NativeCensusRow

FOLDER = Path(__file__).resolve().parent
PACKET = FOLDER.parents[2] / 'tests/fixtures/database-overflow/native-documents.bin.gz'
REFERENCE = FOLDER / 'native-documents-reference.json'


def framed_text(text: str) -> bytes:
    """Frame exact UTF8 path/stream text; e.g. framed_text('Contents/PMISemanticDataDB')."""
    encoded = text.encode('utf-8')
    return struct.pack('<I', len(encoded)) + encoded


def document_record(row: NativeCensusRow, source_root: Path) -> bytes:
    """Bind a source under the injected root; e.g. document_record(row,Path('samples'))."""
    path = Path(row['path'])
    relative = path.relative_to(source_root).as_posix()
    original = path.read_bytes()
    return (struct.pack('<I', row['ordinal']) + framed_text(relative) + framed_text(row['stream']) +
            struct.pack('<Q', len(original)) + hashlib.sha256(original).digest())


def document_packet(source_root: Path) -> bytes:
    """Retain every cell-bearing source stream once; e.g. document_packet(root)."""
    census = json.loads(gzip.decompress((FOLDER / 'native-candidate-census.json.gz').read_bytes()))
    rows = cast(list[NativeCensusRow], census['samples'])
    unique: dict[int, NativeCensusRow] = {row['ordinal']: row for row in rows}
    assert len(unique) == 440
    records = (document_record(unique[ordinal], source_root) for ordinal in sorted(unique))
    return b'BDD1' + struct.pack('<I', len(unique)) + b''.join(records)


def freeze_or_check(source_root: Path, write: bool) -> None:
    """Explicitly freeze or verify full source fingerprints; e.g. freeze_or_check(root,False)."""
    raw = document_packet(source_root)
    compressed = gzip.compress(raw, mtime=0)
    reference = {'issue': 229, 'format': 'BDD1', 'streams': 440, 'documents': 357,
                 'raw_bytes': len(raw), 'sha256': hashlib.sha256(raw).hexdigest(),
                 'gzip_bytes': len(compressed), 'gzip_sha256': hashlib.sha256(compressed).hexdigest(),
                 'source_binding': 'Relative paths under explicit samples root; every complete original file length and SHA256'}
    if write:
        PACKET.write_bytes(compressed)
        REFERENCE.write_text(json.dumps(reference, indent=2) + '\n', encoding='utf-8')
        return
    assert gzip.decompress(PACKET.read_bytes()) == raw
    assert json.loads(REFERENCE.read_text(encoding='utf-8')) == reference
    print(json.dumps(reference))


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--source-root', type=Path, required=True)
    parser.add_argument('--write', action='store_true')
    args = parser.parse_args()
    freeze_or_check(args.source_root, args.write)
