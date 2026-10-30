"""Reopen complete edited OUT studies through frozen actual readers (#114)."""
from pathlib import Path
import json
import struct
import sys
import tempfile
import unittest
import freeze_reports as frozen
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot


def verify() -> None:
    """Verify one exact native report edit and all unrelated carrier content."""
    from cfb import CFB
    from verify_native_edits import records
    import simulationmaster
    import simulationgen
    import simulationout
    reference = json.loads((frozen.FEATURE / 'native-reference.json').read_text())
    original_path = Path(reference['path'])
    original = original_path.read_bytes()
    assert frozen.sha(original) == reference['carrier_sha256']
    edited_path = frozen.ROOT / 'target/simulation-out-native-edit.cwr'
    edited = edited_path.read_bytes()
    before, after = CFB(original_path), CFB(edited_path)
    golden = frozen.golden(before.read('OUT'))
    assert after.read('OUT') == golden
    expected, facts = frozen.inspect(after.read('OUT'))
    paired = simulationgen.read(after.read('GEN'), master=simulationmaster.read(after.read('MAS')))
    report = simulationout.read(after.read('OUT'))
    totals = {field['symbol']: int(field['value']) for field in report['fields'] if field['symbol'] in ('NUMNP', 'NUME')}
    assert totals == dict(NUMNP=paired['prefix']['node_count'], NUME=paired['prefix']['element_count'])
    with tempfile.TemporaryDirectory(prefix='bessemer-out-carrier-') as directory:
        scratch = Path(directory) / 'carrier.cwr'
        previous = records(frozen.compound.inspect(original, scratch)[0])
        current = records(frozen.compound.inspect(edited, scratch)[0])
    verify_entries(previous, current, golden)
    frozen.report('native-edit-reference.json', dict(edited_bytes=len(edited), edited_sha256=frozen.sha(edited), stream_count=sum(value[1][66] == 2 for value in previous.values()), totals=totals, **facts))


def verify_entries(previous: dict, current: dict, golden: bytes, stream_name: str = 'OUT') -> None:
    """Require exact metadata and every logical stream except the explicit replacement."""
    assert previous.keys() == current.keys()
    changed = []
    for identifier, (parent, record, payload) in previous.items():
        current_parent, current_record, actual = current[identifier]
        assert (parent, record) == (current_parent, current_record)
        size = struct.unpack_from('<H', record, 64)[0]
        name = record[:max(0, size - 2)].decode('utf-16-le')
        if record[66] == 2 and name == stream_name:
            assert actual == golden
            changed.append(identifier)
            continue
        assert actual == payload
    assert len(changed) == 1


def run() -> bool:
    """Pin reader contents and origins before importing any actual parser."""
    with tempfile.TemporaryDirectory(prefix='bessemer-out-edit-source-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        frozen.compound.pin_reader(pin, hashes)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        result = unittest.TextTestRunner(verbosity=2).run(unittest.FunctionTestCase(verify))
        record_snapshot(pin, hashes, result, frozen.FEATURE, ('cfb', 'simulationtransfer', 'simulationout', 'simulationmaster', 'simulationgen'), ())
        (frozen.FEATURE / 'fixture-reference-snapshot.json').replace(frozen.FEATURE / 'native-edit-source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
