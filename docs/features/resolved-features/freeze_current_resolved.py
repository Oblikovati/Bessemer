"""Replay the current original resolved-feature regression from immutable sources (#54)."""
from pathlib import Path
import hashlib
import importlib
from functools import partial, update_wrapper
import sys
import tempfile
import unittest

FEATURE = Path(__file__).resolve().parent
for folder in ('model-archives', 'simulation-binary-thermal'):
    sys.path.insert(0, str(FEATURE.parent / folder))
from pinned_reference_tests import SOURCE, FrozenSourceImports, copy_sources, record_snapshot
from native_carriers import pinned_auxiliaries
from current_resolved_measurement import measure_current_resolved_corpus

RESOLVED_REPORT_PINS = {
    'resolved-library-audit-0.382.json': '69e35d935f644fb5297e58974de703aee99fa41c48cae3c4f0f9ebb2e87dae7c',
    'resolved-curve-file-audit-0.353.json': '0bd73c88e01eddb2eef064eedd59bab0027173c46e0f26e5db42de5dbacedda5',
}


def copy_resolved_test_inputs(pin: Path) -> None:
    """Bind reports and the container's default cache without patching readers; e.g. copy_resolved_test_inputs(pin)."""
    cached = (pin / 'keytable.json').read_bytes()
    assert hashlib.sha256(cached).hexdigest() == '3d4b864346a859652b8662655479d8669556db28434da9ed15ba7bf9c677fac7', pin
    cache = pin / 'cache/keytable.json'
    if cache.exists():
        assert cache.read_bytes() == cached, f'{cache}; expected unchanged frozen signature cache'
    else:
        cache.parent.mkdir(exist_ok=True)
        cache.write_bytes(cached)
    for name, expected in RESOLVED_REPORT_PINS.items():
        raw = (SOURCE / 'research' / name).read_bytes()
        actual = hashlib.sha256(raw).hexdigest()
        assert actual == expected, f'{name}/{actual}; expected unchanged report {expected}'
        destination = pin / 'research' / name
        if destination.exists():
            assert destination.read_bytes() == raw, f'{destination}; expected unchanged frozen report {expected}'
        else:
            destination.write_bytes(raw)


def current_resolved_suite(pin: Path) -> unittest.TestSuite:
    """Keep the original failing gate separate from diagnostic measurement; e.g. current_resolved_suite(pin)."""
    suite = unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module('test_resolvedcoverage'))
    assert suite.countTestCases() == 3, f'{suite.countTestCases()}; expected all three original resolved tests'
    if '--measure' not in sys.argv:
        return suite
    measure = update_wrapper(partial(measure_current_resolved_corpus, pin), measure_current_resolved_corpus)
    return unittest.TestSuite([unittest.FunctionTestCase(measure)])


def run_current_resolved() -> bool:
    """Run every original test without replacing parser functions or expectations; e.g. run_current_resolved()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-current-resolved-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        pinned_auxiliaries(pin, hashes)
        copy_resolved_test_inputs(pin)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        result = unittest.TextTestRunner(verbosity=2).run(current_resolved_suite(pin))
        copy_resolved_test_inputs(pin)
        record_snapshot(pin, hashes, result, FEATURE, ('test_resolvedcoverage', 'resolvedfeatures', 'swcover'),
                        ('swcontainer', 'mfcobjects'))
        name = 'current-measured-source-snapshot.json' if '--measure' in sys.argv else 'current-source-snapshot.json'
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / name)
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run_current_resolved() else 1)
