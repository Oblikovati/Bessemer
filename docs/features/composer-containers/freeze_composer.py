"""Bind complete SMG container fixtures to composer.py and independent zipfile (#100)."""
from pathlib import Path
import hashlib
import io
import json
import struct
import sys
import tempfile
import unittest
import warnings
import zipfile

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
sys.path.insert(0, str(FEATURE.parent / 'model-archives'))
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot
from pinned_reference_tests import SOURCE


def zip_bytes(items: list[tuple[str, bytes]], method: int, comment: bytes) -> bytes:
    """Use the standard-library writer with fixed timestamps and unrelated metadata."""
    stream = io.BytesIO()
    with warnings.catch_warnings():
        warnings.simplefilter('ignore', UserWarning)
        with zipfile.ZipFile(stream, 'w', compression=method) as archive:
            archive.comment = comment
            for name, payload in items:
                info = zipfile.ZipInfo(name)
                info.compress_type = method
                info.extra = struct.pack('<HH3s', 0xcafe, 3, b'xyz')
                info.comment = b'entry\x00\xff'
                archive.writestr(info, payload)
    return b'SMG' + stream.getvalue()


def composer_cases() -> list[tuple[str, bytes, bytes]]:
    """Cover empty, stored, compressed, Unicode, duplicate and false-end cases."""
    items = [('α/file', b'data' * 200), ('empty', b''), ('same', b'first'), ('same', b'second')]
    result = [('empty.smg', zip_bytes([], zipfile.ZIP_STORED, b'empty'))]
    for method, label in ((zipfile.ZIP_STORED, 'stored'), (zipfile.ZIP_DEFLATED, 'deflated')):
        result.append((label + '.smg', zip_bytes(items, method, b'archive\x00\xff')))
    base = zip_bytes([('a', b'hello')], zipfile.ZIP_DEFLATED, b'prefix' + bytes(22))
    fake = struct.pack('<4s4H2IH', b'PK\x05\x06', 0, 0, 0, 0, 0, len(base) - 25, 0)
    return [(name, raw, raw) for name, raw in result] + [('false-end.smg', base[:-22] + fake, base)]


def verify_archive(raw: bytes, independent_raw: bytes) -> dict[str, object]:
    """Require every byte region and every decoded member to match independent ZIP evidence."""
    import composer
    document = composer.Document(raw)
    regions = document.regions()
    assert regions[0][0] == 0 and regions[-1][1] == len(raw)
    assert all(left[1] == right[0] for left, right in zip(regions, regions[1:]))
    members = []
    # zipfile selects the false end signature, so the producer's unchanged members are its oracle.
    with zipfile.ZipFile(io.BytesIO(independent_raw)) as independent:
        for entry, info in zip(document.entries, independent.infolist(), strict=True):
            payload = document.read(entry)
            assert entry.name == info.filename and payload == independent.read(info)
            members.append(dict(name=entry.name, bytes=len(payload), crc=entry.crc, method=entry.method,
                                stored_bytes=entry.csize, decoded_sha256=hashlib.sha256(payload).hexdigest()))
    return dict(bytes=len(raw), sha256=hashlib.sha256(raw).hexdigest(), members=members,
                comment_hex=document.comment.hex(), regions=regions,
                independent_archive_sha256=hashlib.sha256(independent_raw).hexdigest())


def frozen(path: Path, raw: bytes) -> None:
    """Check reviewed fixture bytes unless replacement is explicitly requested."""
    if '--write' in sys.argv:
        path.write_bytes(raw)
        return
    assert path.read_bytes() == raw, f'{path}; expected unchanged reference bytes'


def freeze_owned() -> None:
    """Retain independent inputs and complete payload oracles for both readers."""
    report = []
    for name, raw, independent in composer_cases():
        facts = verify_archive(raw, independent)
        frozen(ROOT / 'tests/fixtures/composer' / name, raw)
        facts['name'] = name
        report.append(facts)
    frozen(FEATURE / 'owned-reference.json', (json.dumps(report, indent=2, ensure_ascii=False) + '\n').encode())


def verify_native() -> None:
    """Check all installed carriers and members against the reviewed source corpus."""
    baseline = json.loads((SOURCE / 'research/composer-project-corpus.json').read_bytes())
    records = []
    for row in baseline['files']:
        raw = Path(row['path']).read_bytes()
        facts = verify_archive(raw, raw)
        assert facts['sha256'] == row['sha256'] and facts['regions'] == [tuple(r) for r in row['regions']]
        assert len(facts['members']) == len(row['members'])
        for observed, expected in zip(facts['members'], row['members'], strict=True):
            assert observed['decoded_sha256'] == expected['decoded_sha256']
        facts['source_path'] = row['path']
        records.append(facts)
    report = dict(files=len(records), members=sum(len(r['members']) for r in records),
                  file_bytes=sum(r['bytes'] for r in records),
                  decoded_bytes=sum(m['bytes'] for r in records for m in r['members']), records=records)
    assert (report['files'], report['members'], report['file_bytes'], report['decoded_bytes']) == (38, 550, 44056222, 106834828)
    frozen(FEATURE / 'native-reference.json', (json.dumps(report, indent=2, ensure_ascii=False) + '\n').encode())


def main() -> None:
    """Pin all reference sources before loading the complete Composer container reader."""
    with tempfile.TemporaryDirectory(prefix='bessemer-composer-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin))
        suite = unittest.TestSuite([unittest.FunctionTestCase(freeze_owned), unittest.FunctionTestCase(verify_native)])
        result = unittest.TextTestRunner(verbosity=2).run(suite)
        assert result.wasSuccessful() and result.testsRun == 2 and not result.skipped
        record_snapshot(pin, hashes, result, FEATURE, ('composer',), ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'owned-source-snapshot.json')


if __name__ == '__main__':
    main()
