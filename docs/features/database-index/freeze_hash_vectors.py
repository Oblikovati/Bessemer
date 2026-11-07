"""Independent polynomial-hash expectations consumed through complete Rust files (#230)."""
import gzip
from pathlib import Path
import struct
from hash_reference import closed_form_hash

LENGTHS = (0, 1, 2, 3, 4, 7, 8, 15, 31, 255, 256, 1023, 1024, 2047, 2048, 2049, 4096)


def hash_packet() -> bytes:
    """Frame both unsigned-key families at every prefix boundary; e.g. hash_packet()."""
    records: list[bytes] = []
    for length in LENGTHS:
        pattern = bytes((index * 73 + index // 251) % 256 for index in range(length))
        for key in (pattern, bytes([255]) * length):
            records.append(struct.pack('<II', len(key), closed_form_hash(key)) + key)
    return b'BHV1' + struct.pack('<I', len(records)) + b''.join(records)


def freeze_hashes(root: Path) -> None:
    """Write deterministic hash goldens under an explicit root; e.g. freeze_hashes(root)."""
    root.mkdir(parents=True, exist_ok=True)
    compressed = gzip.compress(hash_packet(), mtime=0)
    (root / 'hash-vectors.bin.gz').write_bytes(compressed)


if __name__ == '__main__':
    freeze_hashes(Path(__file__).resolve().parents[3] / 'tests/fixtures/database-index')
