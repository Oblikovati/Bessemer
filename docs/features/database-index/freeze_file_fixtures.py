"""Persist complete original/result expectations for cross-language checks (#230)."""
import gzip
import hashlib
import json
from pathlib import Path
import struct
from file_fixtures import FileValueCase, complete_file, expected_edit


def edit_record(raw: bytes, cell: FileValueCase) -> bytes:
    """Frame one exact cell identity and independently edited file; e.g. edit_record(raw,cell)."""
    replacement = bytes(byte ^ 0xa5 for byte in cell.value)
    result = expected_edit(raw, cell, replacement)
    header = struct.pack('<QHII', cell.bucket, cell.cell, len(cell.key), len(cell.value))
    spans = struct.pack('<I', len(cell.spans)) + b''.join(struct.pack('<QQ', *span) for span in cell.spans)
    return header + cell.key + cell.value + replacement + spans + result


def file_packet() -> bytes:
    """Build a BFI1 packet with one original and four complete results; e.g. file_packet()."""
    raw, cells = complete_file()
    records = tuple(edit_record(raw, cell) for cell in cells)
    header = b'BFI1' + struct.pack('<II', len(raw), len(records))
    return header + raw + b''.join(struct.pack('<I', len(record)) + record for record in records)


def freeze_packet(root: Path) -> None:
    """Write deterministic fixtures and integrity metadata under an injected root; e.g. freeze_packet(root)."""
    raw = file_packet()
    compressed = gzip.compress(raw, mtime=0)
    root.mkdir(parents=True, exist_ok=True)
    (root / 'complete-file.bin.gz').write_bytes(compressed)
    proof = {'issue': 230, 'magic': 'BFI1', 'files': 1, 'edits': 4,
             'raw_bytes': len(raw), 'raw_sha256': hashlib.sha256(raw).hexdigest(),
             'compressed_bytes': len(compressed), 'compressed_sha256': hashlib.sha256(compressed).hexdigest(),
             'expected_results': 'Literal value spans; complete file bytes, including unrelated roles',
             'original_native_implementation_executed': False}
    (root / 'complete-file-integrity.json').write_text(json.dumps(proof, indent=2) + '\n')


if __name__ == '__main__':
    freeze_packet(Path(__file__).resolve().parents[3] / 'tests/fixtures/database-index')
