"""Reopen complete edited CWR files through frozen actual readers (#112)."""
from pathlib import Path
import struct
import sys
import tempfile
import unittest
import freeze_images as frozen
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot


def verify() -> None:
    """Require one complete coordinate edit per paired installed study."""
    from cfb import CFB
    raw = (frozen.ROOT / 'target/simulation-images-native-edits.bin').read_bytes()
    assert raw[:8] == b'SIMW\x03\0\0\0'
    cursor, records = 8, []
    with tempfile.TemporaryDirectory(prefix='bessemer-simulation-edits-') as directory:
        for original, carrier, mas, gen in frozen.native_pairs():
            length = struct.unpack_from('<I', raw, cursor)[0]
            cursor += 4
            edited = raw[cursor:cursor + length]
            cursor += length
            path = Path(directory) / 'edited.cwr'
            path.write_bytes(edited)
            after = CFB(path)
            expected = bytearray(gen)
            struct.pack_into('<3I', expected, 1036, 0x40000000, 0x40400000, 0x40800000)
            assert after.read('MAS') == mas and after.read('GEN') == expected
            oracle, facts = frozen.inspect(mas, after.read('GEN'))
            verify_all_entries(carrier, edited, path)
            records.append(dict(path=original['path'], edited_bytes=length, edited_sha256=frozen.sha(edited), **facts))
    assert cursor == len(raw)
    frozen.report('native-edit-reference.json', dict(files=3, bundle_bytes=len(raw), bundle_sha256=frozen.sha(raw), records=records))


def verify_all_entries(original: bytes, edited: bytes, path: Path) -> None:
    """Require every metadata byte and unrelated logical stream to survive."""
    sys.path.insert(0, str(frozen.FEATURE.parent / 'compound-files'))
    from verify_native_edits import records as cfb_records
    before, _ = frozen.compound.inspect(original, path)
    after, _ = frozen.compound.inspect(edited, path)
    expected = cfb_records(before)
    actual = cfb_records(after)
    assert expected.keys() == actual.keys()
    for identifier, (parent, record, payload) in expected.items():
        other_parent, other_record, other_payload = actual[identifier]
        assert (parent, record) == (other_parent, other_record)
        name_length = struct.unpack_from('<H', record, 64)[0]
        name = record[:max(0, name_length - 2)].decode('utf-16-le')
        if record[66] != 2 or name != 'GEN':
            assert payload == other_payload


def run() -> bool:
    """Pin all reference inputs and import origins before decoding any carrier."""
    with tempfile.TemporaryDirectory(prefix='bessemer-simulation-edit-source-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        frozen.compound.pin_reader(pin, hashes)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        result = unittest.TextTestRunner(verbosity=2).run(unittest.FunctionTestCase(verify))
        record_snapshot(pin, hashes, result, frozen.FEATURE, ('cfb', 'simulationmaster', 'simulationgen', 'simulationmesh'), ())
        (frozen.FEATURE / 'fixture-reference-snapshot.json').replace(frozen.FEATURE / 'native-edit-source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
