"""Reopen complete initial-image CWR edits using the actual frozen ELF reader (#115)."""
from pathlib import Path
import json
import struct
import sys
import tempfile
import unittest
import freeze_elf as frozen
from verify_out_edits import verify_entries
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot


def verify() -> None:
    """Require the independent parameter patch and every unrelated stream/metadata byte."""
    from cfb import CFB
    from verify_native_edits import records
    reference = json.loads((frozen.FEATURE / 'native-reference.json').read_text())
    original_path = Path(reference['path'])
    original = original_path.read_bytes()
    assert frozen.sha(original) == reference['carrier_sha256']
    edited_path = frozen.ROOT / 'target/simulation-elf-native-initial-edit.cwr'
    edited = edited_path.read_bytes()
    before, after = CFB(original_path), CFB(edited_path)
    golden = bytearray(before.read('ELF'))
    struct.pack_into('<I', golden, 16, 0x12345678)
    assert after.read('ELF') == golden
    oracle = frozen.initial_oracle(after.read('ELF'))
    with tempfile.TemporaryDirectory(prefix='bessemer-elf-initial-edit-') as directory:
        scratch = Path(directory) / 'carrier.cwr'
        previous = records(frozen.shared.compound.inspect(original, scratch)[0])
        current = records(frozen.shared.compound.inspect(edited, scratch)[0])
    verify_entries(previous, current, bytes(golden), stream_name='ELF')
    frozen.report('native-initial-edit-reference.json', dict(edited_bytes=len(edited), edited_sha256=frozen.sha(edited), initial_bytes=len(golden), initial_sha256=frozen.sha(golden), oracle_sha256=frozen.sha(oracle), streams_checked=sum(value[1][66] == 2 for value in previous.values()), complete_initial_image=True, populated_lifecycle_checked=False))


def run() -> bool:
    """Freeze actual reader contents before importing them, retaining all required origins."""
    with tempfile.TemporaryDirectory(prefix='bessemer-elf-initial-edit-source-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        frozen.shared.compound.pin_reader(pin, hashes)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        result = unittest.TextTestRunner(verbosity=2).run(unittest.FunctionTestCase(verify))
        record_snapshot(pin, hashes, result, frozen.FEATURE, ('cfb', 'simulationtransfer', 'simulationelf'), ())
        (frozen.FEATURE / 'fixture-reference-snapshot.json').replace(frozen.FEATURE / 'native-initial-edit-source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
