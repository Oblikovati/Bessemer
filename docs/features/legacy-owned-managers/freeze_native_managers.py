"""Freeze independently decoded version-18000 manager streams (#54)."""
from pathlib import Path
import hashlib
import json
import sys
import tempfile

WORKSPACE = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(WORKSPACE/'docs/features/model-archives'))
sys.path.insert(0, str(WORKSPACE/'docs/features/resolved-features'))
from pinned_reference_tests import copy_sources, FrozenSourceImports, SOURCE
from primitive_inventory import observe_primitives, verify_primitive_coverage


def freeze_case(row: dict, destination: Path, index: int) -> dict:
    """Require independent EOF and primitive coverage; e.g. freeze_case(row, root, 0)."""
    import configurationmanager, swcontainer, swarchive
    from swcover import SAMPLES
    document = swcontainer.Document((Path(SAMPLES)/row['path']).read_bytes(), table={})
    raw = document.read('Contents/CMgr')
    assert hashlib.sha256(raw).hexdigest() == row['result']['sha256']
    with observe_primitives(swarchive.Archive, raw) as reads:
        parsed = configurationmanager.read_supported(document, decals=False)
    assert parsed['end'] == parsed['archive'].p == len(raw)
    assert parsed['root']['display_maps_presence'] is None
    probes = [r for r in reads if r.source_file == 'configurationscenes.py' and r.source_function == 'scene_data']
    spans = [r for r in reads if r.end > r.start and r not in probes]
    verify_primitive_coverage(raw, spans)
    oracle = ''.join(f'{r.start}\t{r.end}\t{r.kind}\t{r.raw_hex}\n' for r in spans).encode()
    stem = f'manager-{index}'
    (destination/f'{stem}.bin').write_bytes(raw)
    (destination/f'{stem}-primitives.tsv').write_bytes(oracle)
    return dict(stem=stem, path=row['path'], bytes=len(raw), reads=len(spans),
                sha256=hashlib.sha256(raw).hexdigest(), oracle_sha256=hashlib.sha256(oracle).hexdigest())


def main() -> None:
    """Pin actual readers before collecting owned fixtures; e.g. main()."""
    destination = WORKSPACE/'tests/fixtures/legacy-owned-managers'
    destination.mkdir(parents=True, exist_ok=True)
    rows = json.loads((SOURCE/'research/configuration-legacy-owned-corpus.json').read_text())
    with tempfile.TemporaryDirectory(prefix='bessemer-legacy-manager-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin))
        cases = [freeze_case(row, destination, index) for index, row in enumerate(rows)]
        assert hashes == {p.relative_to(pin).as_posix(): hashlib.sha256(p.read_bytes()).hexdigest() for p in pin.rglob('*.py')}
    report = dict(issue=54, version=18000, cases=cases, source_hashes=hashes)
    Path(__file__).with_name('native-reference.json').write_text(json.dumps(report, indent=2)+'\n', encoding='utf-8')
    print([(r['stem'], r['bytes'], r['reads']) for r in cases])


if __name__ == '__main__':
    main()
