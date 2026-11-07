"""Persist complete independent saved-index mutation results (#230)."""
import gzip
import hashlib
import json
from pathlib import Path
import struct
from saved_index_fixtures import SavedIndexMutation, saved_cases


def encode_saved_mutation(change: SavedIndexMutation) -> bytes:
    """Frame a typed request and complete expected file; e.g. encode_saved_mutation(change)."""
    header = struct.pack('<BI', change.kind, change.ordinal)
    words = b''.join(struct.pack('<Q', word) for word in change.words)
    return header + words + struct.pack('<QQ', *change.span) + change.expected


def saved_packet(fixtures: Path) -> tuple[bytes, int]:
    """Build seven owned/native cases and every mutation expectation; e.g. saved_packet(root)."""
    cases = saved_cases(fixtures)
    records = tuple(struct.pack('<II', len(raw), len(changes)) + raw
                    + b''.join(encode_saved_mutation(change) for change in changes)
                    for raw, changes in cases)
    edits = sum(len(changes) for _, changes in cases)
    return b'BSI1' + struct.pack('<I', len(cases)) + b''.join(records), edits


def freeze_saved(fixtures: Path, output: Path) -> None:
    """Write deterministic saved-index expectations and integrity metadata; e.g. freeze_saved(root,output)."""
    raw, edits = saved_packet(fixtures)
    compressed = gzip.compress(raw, mtime=0)
    output.mkdir(parents=True, exist_ok=True)
    (output / 'saved-indexes.bin.gz').write_bytes(compressed)
    proof = {'issue': 230, 'behavior_version': 'saved-hash-index-v1', 'cases': 7, 'edits': edits,
             'raw_bytes': len(raw), 'raw_sha256': hashlib.sha256(raw).hexdigest(),
             'compressed_bytes': len(compressed), 'compressed_sha256': hashlib.sha256(compressed).hexdigest(),
             'expected_results': 'Literal word spans and complete files, including ordered duplicates and noncanonical words',
             'file_graph_validity_claimed': False}
    (output / 'saved-indexes-integrity.json').write_text(json.dumps(proof, indent=2) + '\n')


if __name__ == '__main__':
    repository = Path(__file__).resolve().parents[3]
    freeze_saved(repository / 'tests/fixtures/database-overflow',
                 repository / 'tests/fixtures/database-index')
