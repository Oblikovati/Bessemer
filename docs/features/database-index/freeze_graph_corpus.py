"""Freeze the authenticated selected native graph census into cross-language cases (#230)."""
from collections import Counter
import gzip
import hashlib
import json
from pathlib import Path
import struct
from corpus_reference import corpus_file


def frozen_files(image_root: Path, census: Path) -> tuple[bytes, dict[str, int]]:
    """Authenticate retained images before deriving expectations; e.g. frozen_files(images,census)."""
    report = json.loads(gzip.decompress(census.read_bytes()))
    assert report['streams_checked'] == report['complete_selected_graphs'] == 2486
    assert report['failures'] == []
    records: list[bytes] = []
    totals: Counter[str] = Counter()
    for row in report['observations']:
        raw = (image_root / f"{row['ordinal']}.bin").read_bytes()
        assert hashlib.sha256(raw).hexdigest() == row['database_sha256']
        buckets = tuple(identity for identity in range(2, len(raw) // 4096)
                        if identity in {entry[1] for entry in row['entries']})
        buckets = complete_bucket_ids(raw, buckets)
        record, checked = corpus_file(raw, row['ordinal'], buckets)
        records.append(record)
        totals.update(checked)
    assert dict(totals) == report['totals']
    return b'BFG1' + struct.pack('<I', len(records)) + b''.join(records), dict(totals)


def complete_bucket_ids(raw: bytes, roots: tuple[int, ...]) -> tuple[int, ...]:
    """Include every linked continuation bucket in physical order; e.g. complete_bucket_ids(raw,(2,))."""
    visited: set[int] = set()
    for root in roots:
        current = root
        while current:
            assert current not in visited and current < len(raw) // 4096
            visited.add(current)
            current = struct.unpack_from('>Q', raw, current * 4096 + 4)[0]
    return tuple(sorted(visited))


def freeze_corpus(image_root: Path, output: Path, census: Path) -> None:
    """Write complete native graph cases plus integrity metadata; e.g. freeze_corpus(images,output,census)."""
    raw, totals = frozen_files(image_root, census)
    compressed = gzip.compress(raw, mtime=0)
    output.mkdir(parents=True, exist_ok=True)
    (output / 'native-files.bin.gz').write_bytes(compressed)
    proof = {'issue': 230, 'magic': 'BFG1', 'files': 2486, 'totals': totals,
             'raw_bytes': len(raw), 'raw_sha256': hashlib.sha256(raw).hexdigest(),
             'compressed_bytes': len(compressed), 'compressed_sha256': hashlib.sha256(compressed).hexdigest(),
             'census_sha256': hashlib.sha256(census.read_bytes()).hexdigest(),
             'expected_results': 'Authenticated original bytes and independently addressed complete cell value spans',
             'original_native_implementation_executed': False}
    (output / 'native-files-integrity.json').write_text(json.dumps(proof, indent=2) + '\n')


if __name__ == '__main__':
    repository = Path(__file__).resolve().parents[3]
    freeze_corpus(repository / 'target/database-overflow-native-images',
                  repository / 'tests/fixtures/database-index',
                  Path(__file__).resolve().parent / 'native-index-census.json.gz')
