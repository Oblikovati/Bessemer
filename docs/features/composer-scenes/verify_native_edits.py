"""Reopen complete Rust scene edits through actual frozen readers and independent gzip (#103)."""
from pathlib import Path
import gzip
import hashlib
import json
import struct
import sys
import tempfile
import unittest
import freeze_scenes as baseline


def without_offsets(value: object) -> object:
    """Remove only reader positions and derived scene byte count when comparing content."""
    if isinstance(value, dict):
        skipped = {'start', 'end', 'info_start', 'info_end', 'scene_graph_bytes'}
        return {key: without_offsets(child) for key, child in value.items() if key not in skipped}
    if isinstance(value, (list, tuple)):
        return [without_offsets(child) for child in value]
    return value


def verify_pair(original: bytes, changed: bytes) -> dict[str, object]:
    """Require exact requested name replacement and every other stored semantic field unchanged."""
    import composergeometry
    import composerscene
    assert original[:10] == changed[:10]
    before_bytes, after_bytes = gzip.decompress(original), gzip.decompress(changed)
    assert composergeometry.decode(original)['decoded'] == before_bytes
    assert composergeometry.decode(changed)['decoded'] == after_bytes
    before = composerscene.read_decoded(before_bytes, texture_import_enabled=True)
    after = composerscene.read_decoded(after_bytes, texture_import_enabled=True)
    assert before['end'] == len(before_bytes) and after['end'] == len(after_bytes)
    expected = 'Bessemer complete scene Ω'
    name = before['entries'][0]['name']
    name.update(value=expected, code_units=len(expected.encode('utf-16-le')) // 2,
                raw_hex=expected.encode('utf-16-le').hex())
    assert without_offsets(before) == without_offsets(after)
    return dict(original_sha256=hashlib.sha256(original).hexdigest(),
                edited_sha256=hashlib.sha256(changed).hexdigest(), decoded_bytes=len(after_bytes))


def verify_bundle() -> None:
    """Bind all 38 original pairs to reviewed member identities before comparing edited scenes."""
    raw = (baseline.ROOT / 'target/composer-scenes-native-edits.bin').read_bytes()
    records = json.loads((baseline.FEATURE / 'native-reference.json').read_bytes())['records']
    assert raw[:4] == b'SGED' and struct.unpack_from('<I', raw, 4)[0] == len(records) == 38
    cursor, results = 8, []
    for expected in records:
        original_size, changed_size = struct.unpack_from('<II', raw, cursor)
        cursor += 8
        original = raw[cursor:cursor + original_size]
        cursor += original_size
        changed = raw[cursor:cursor + changed_size]
        cursor += changed_size
        assert hashlib.sha256(original).hexdigest() == expected['encoded_sha256']
        results.append(verify_pair(original, changed))
    assert cursor == len(raw)
    baseline.retained_report('native-edit-reference.json', dict(files=38, bundle_sha256=hashlib.sha256(raw).hexdigest(), records=results))


def main() -> None:
    """Use a pinned reference tree for all complete-scene checks, e.g. python verify_native_edits.py."""
    with tempfile.TemporaryDirectory(prefix='bessemer-scene-edits-') as temporary:
        pin = Path(temporary)
        hashes = baseline.copy_sources(pin)
        sys.meta_path.insert(0, baseline.FrozenSourceImports(pin))
        sys.path.insert(0, str(pin))
        result = unittest.TextTestRunner(verbosity=2).run(unittest.FunctionTestCase(verify_bundle))
        assert result.wasSuccessful() and result.testsRun == 1 and not result.skipped
        baseline.record_snapshot(pin, hashes, result, baseline.FEATURE, ('composergeometry', 'composerscene'), ())
        (baseline.FEATURE / 'fixture-reference-snapshot.json').replace(baseline.FEATURE / 'native-edit-source-snapshot.json')


if __name__ == '__main__':
    main()
