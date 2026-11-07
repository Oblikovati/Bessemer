"""Check Bessemer native member edits with actual Composer and independent ZIP readers (#100)."""
from pathlib import Path
import hashlib
import io
import json
import struct
import sys
import tempfile
import unittest
import zipfile

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
sys.path.insert(0, str(FEATURE.parent / 'model-archives'))
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot


def verify_pair(original: bytes, edited: bytes) -> dict[str, object]:
    """Require exact contents and encoded preservation without normalizing unrelated metadata."""
    import composer
    before, after = composer.Document(original), composer.Document(edited)
    assert before.names() == after.names() and before.comment == after.comment
    with zipfile.ZipFile(io.BytesIO(edited)) as independent:
        for index, (source, result, info) in enumerate(zip(before.entries, after.entries, independent.infolist(), strict=True)):
            expected = b'Bessemer native member replacement' if index == 0 else before.read(source)
            assert after.read(result) == independent.read(info) == expected
            assert source.local_extra == result.local_extra and source.central_extra == result.central_extra
            assert source.name_raw == result.name_raw and source.comment == result.comment
            assert source.local[:6] == result.local[:6] and source.local[9:] == result.local[9:]
            assert source.central[:7] == result.central[:7] and source.central[10:16] == result.central[10:16]
            if index != 0:
                assert original[source.data_at:source.data_at + source.csize] == edited[result.data_at:result.data_at + result.csize]
                assert source.local == result.local and source.central[:16] == result.central[:16]
    return dict(members=len(after.entries), original_sha256=hashlib.sha256(original).hexdigest(),
                edited_sha256=hashlib.sha256(edited).hexdigest())


def verify_edits() -> None:
    """Check all 38 published replacements and all 550 output members through EOF."""
    raw = (ROOT / 'target/composer-native-edits.bin').read_bytes()
    assert raw[:4] == b'SMGE' and struct.unpack_from('<I', raw, 4)[0] == 38
    position, records = 8, []
    baseline = json.loads((FEATURE / 'native-reference.json').read_bytes())['records']
    for index in range(38):
        source_size, result_size = struct.unpack_from('<II', raw, position)
        position += 8
        original = raw[position:position + source_size]
        position += source_size
        edited = raw[position:position + result_size]
        position += result_size
        record = verify_pair(original, edited)
        assert record['original_sha256'] == baseline[index]['sha256']
        records.append(record)
    assert position == len(raw) and sum(row['members'] for row in records) == 550
    report = dict(files=38, members=550, unchanged_encoded_members=512, records=records)
    (FEATURE / 'native-edit-reference.json').write_bytes((json.dumps(report, indent=2) + '\n').encode())


def main() -> None:
    """Keep the reader sources immutable and retain their exact imported origins."""
    with tempfile.TemporaryDirectory(prefix='bessemer-composer-edits-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin))
        result = unittest.TextTestRunner(verbosity=2).run(unittest.FunctionTestCase(verify_edits))
        assert result.wasSuccessful() and result.testsRun == 1 and not result.skipped
        record_snapshot(pin, hashes, result, FEATURE, ('composer',), ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'native-edit-source-snapshot.json')


if __name__ == '__main__':
    main()
