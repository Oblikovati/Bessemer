"""Replay all accepted native configurations with actual immutable Source readers (#54)."""
from pathlib import Path
import hashlib
import json
import shutil
import sys
import tempfile
import time
import unittest

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
sys.path.insert(0, str(FEATURE.parent / 'model-archives'))
from pinned_reference_tests import SOURCE, FrozenSourceImports, copy_sources, record_snapshot


def replay_records() -> None:
    """Bind all four context streams and require exact EOF in every Source read; e.g. replay_records()."""
    from swcontainer import Document
    from resolvedfeatures import read_supported
    manifest = json.loads((ROOT / 'target/resolved-native-inputs.json').read_bytes())
    assert manifest['cases'] == len(manifest['records']) == 906
    started = time.perf_counter()
    records = []
    for row in manifest['records']:
        document = Document(Path(row['path']), table={})
        configuration = row['configuration']
        names = [configuration + '-ModelHeader', configuration, 'Contents/CMgr', configuration + '-ResolvedFeatures']
        raw = [document.read(name) for name in names]
        assert [hashlib.sha256(payload).hexdigest() for payload in raw] == row['sha256'], row['index']
        parsed = read_supported(document, names[-1])
        assert parsed['end'] == len(raw[-1]) and parsed['tail'] == b'', row['index']
        records.append(dict(index=row['index'], bytes=parsed['end'], unknown_bytes=parsed['unknown_bytes']))
        if len(records) % 100 == 0:
            print(f'SOURCE_ACCEPTED\t{len(records)}', flush=True)
    report = dict(cases=len(records), resolved_bytes=sum(row['bytes'] for row in records), records=records,
                  seconds=time.perf_counter() - started, reader_functions_replaced=False)
    (FEATURE / 'accepted-reference.json').write_bytes((json.dumps(report, indent=2) + '\n').encode())


def main() -> None:
    """Pin all readers and retain origin/hash evidence after the complete audit; e.g. main()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-forming-corpus-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        (pin / 'cache').mkdir(exist_ok=True)
        shutil.copyfile(SOURCE / 'cache/keytable.json', pin / 'cache/keytable.json')
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin))
        result = unittest.TextTestRunner(verbosity=2).run(unittest.FunctionTestCase(replay_records))
        assert result.wasSuccessful() and result.testsRun == 1 and not result.skipped
        record_snapshot(pin, hashes, result, FEATURE, ('formtools', 'embeddedparts', 'modelheader'))
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'accepted-source-snapshot.json')


if __name__ == '__main__':
    main()
