"""Freeze all selected native mixed buckets against the earlier independent census (#229)."""
import argparse
import gzip
import hashlib
import json
from pathlib import Path
import struct
from typing import cast
from bucket_fixtures import BucketCase, encode_bucket, native_bucket_bytes
from native_packet import NativeCensusRow

FOLDER = Path(__file__).resolve().parent
PACKET = FOLDER.parents[2] / 'tests/fixtures/database-overflow/native-buckets.bin.gz'
REFERENCE = FOLDER / 'native-buckets-reference.json'


def grouped_rows() -> dict[tuple[int, int], list[NativeCensusRow]]:
    """Keep exact source ordinal/bucket identities; e.g. grouped_rows()[(65,2)]."""
    original = json.loads(gzip.decompress((FOLDER / 'native-candidate-census.json.gz').read_bytes()))
    groups: dict[tuple[int, int], list[NativeCensusRow]] = {}
    for row in cast(list[NativeCensusRow], original['samples']):
        groups.setdefault((row['ordinal'], row['bucket_page_index']), []).append(row)
    assert len(groups) == 1023
    return groups


def checked_bucket(rows: list[NativeCensusRow], raw: bytes) -> BucketCase:
    """Authenticate full DB bytes and every earlier overflow span; e.g. checked_bucket(rows,raw)."""
    assert hashlib.sha256(raw).hexdigest() == rows[0]['database_sha256']
    case = native_bucket_bytes(raw, rows[0]['bucket_page_index'])
    expected = {row['cell'][0]: row for row in rows}
    overflow = [cell for cell in case.cells if cell.first]
    assert {cell.offset for cell in overflow} == set(expected)
    for cell in overflow:
        row = expected[cell.offset]
        assert hashlib.sha256(cell.key).hexdigest() == row['key_sha256']
        assert hashlib.sha256(cell.value).hexdigest() == row['value_sha256']
        assert (cell.offset, cell.key_hash, len(cell.key), len(cell.value), cell.following, cell.first) == tuple(row['cell'])
    return case


def census_packet(image_root: Path) -> tuple[bytes, dict[str, int]]:
    """Bind selected buckets to an injected image root; e.g. census_packet(Path('target/images'))."""
    records: list[bytes] = []
    totals = {'buckets': 0, 'cells': 0, 'inline_cells': 0, 'overflow_cells': 0}
    images: dict[int, bytes] = {}
    for (ordinal, index), rows in sorted(grouped_rows().items()):
        if ordinal not in images:
            images[ordinal] = (image_root / f'{ordinal}.bin').read_bytes()
        case = checked_bucket(rows, images[ordinal])
        encoded = encode_bucket(case)
        records.append(struct.pack('<III', ordinal, index, len(encoded)) + encoded)
        totals['buckets'] += 1
        totals['cells'] += len(case.cells)
        totals['overflow_cells'] += sum(cell.first != 0 for cell in case.cells)
    totals['inline_cells'] = totals['cells'] - totals['overflow_cells']
    assert totals['overflow_cells'] == 5140
    return b'BCW1' + struct.pack('<I', len(records)) + b''.join(records), totals


def freeze_or_check(image_root: Path, write: bool) -> None:
    """Explicitly freeze or regenerate the complete packet; e.g. freeze_or_check(root,False)."""
    raw, totals = census_packet(image_root)
    compressed = gzip.compress(raw, mtime=0)
    reference = {'issue': 229, 'format': 'BCW1', **totals, 'raw_bytes': len(raw),
                 'sha256': hashlib.sha256(raw).hexdigest(), 'gzip_bytes': len(compressed),
                 'gzip_sha256': hashlib.sha256(compressed).hexdigest(),
                 'scope': 'Selected4096-byte bucket buffers from440 streams; no universal file role inference'}
    if write:
        PACKET.write_bytes(compressed)
        REFERENCE.write_text(json.dumps(reference, indent=2) + '\n', encoding='utf-8')
        return
    assert gzip.decompress(PACKET.read_bytes()) == raw
    assert json.loads(REFERENCE.read_text(encoding='utf-8')) == reference
    print(json.dumps(reference))


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--image-root', type=Path, required=True)
    parser.add_argument('--write', action='store_true')
    args = parser.parse_args()
    freeze_or_check(args.image_root, args.write)
