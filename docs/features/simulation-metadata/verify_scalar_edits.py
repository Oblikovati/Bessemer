"""Reopen every installed scalar edit through actual frozen CFB/scalar readers (#116)."""
from pathlib import Path
import json
import struct
import sys
import tempfile
import unittest
import freeze_metadata as frozen
from verify_out_edits import verify_entries
from verify_native_edits import records, word, span
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot


def verify() -> None:
    """Bind every complete edited carrier to the installed original and independent scalar word."""
    from cfb import CFB
    import simulationscalars
    reference = json.loads((frozen.FEATURE / 'native-reference.json').read_text())['records']
    originals = {item['carrier_sha256']: item for item in reference}
    raw = (frozen.ROOT / 'target/simulation-metadata-native-edits.bin').read_bytes()
    assert raw[:8] == b'SIMU\x04\0\0\0'
    cursor, seen, facts = 8, set(), []
    with tempfile.TemporaryDirectory(prefix='bessemer-scalar-edits-') as directory:
        scratch = Path(directory) / 'carrier.cwr'
        for _ in range(4):
            original, cursor = span(raw, cursor)
            kind, cursor = word(raw, cursor)
            edited, cursor = span(raw, cursor)
            name = frozen.KINDS[kind]
            digest = frozen.sha(original)
            assert digest in originals and (digest, name) not in seen
            seen.add((digest, name))
            before = records(frozen.shared.compound.inspect(original, scratch)[0])
            after = records(frozen.shared.compound.inspect(edited, scratch)[0])
            golden = struct.pack('<I', 0x87654321)
            verify_entries(before, after, golden, stream_name=name)
            scratch.write_bytes(edited)
            scalar = CFB(scratch).read(name)
            parsed = simulationscalars.read(name, scalar)
            assert parsed['pattern'] == 0x87654321 and parsed['signed_value'] == -2023406815
            facts.append(dict(source=digest, name=name, edited_bytes=len(edited), edited_sha256=frozen.sha(edited), scalar_sha256=frozen.sha(scalar)))
    expected = {(item['carrier_sha256'], scalar['name']) for item in reference for scalar in item['scalars']}
    assert cursor == len(raw) and seen == expected
    frozen.report('native-scalar-edit-reference.json', dict(edits=len(facts), bundle_bytes=len(raw), bundle_sha256=frozen.sha(raw), records=facts))


def run() -> bool:
    """Freeze actual reader contents and all required import origins before verification."""
    with tempfile.TemporaryDirectory(prefix='bessemer-scalar-edit-source-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        frozen.shared.compound.pin_reader(pin, hashes)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        result = unittest.TextTestRunner(verbosity=2).run(unittest.FunctionTestCase(verify))
        record_snapshot(pin, hashes, result, frozen.FEATURE, ('cfb', 'simulationscalars'), ())
        (frozen.FEATURE / 'fixture-reference-snapshot.json').replace(frozen.FEATURE / 'native-scalar-edit-source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
