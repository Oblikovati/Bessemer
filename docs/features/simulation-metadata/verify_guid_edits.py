"""Reopen synthetic complete GUID-carrier edits with actual frozen readers (#116)."""
from pathlib import Path
import sys
import tempfile
import unittest
import freeze_metadata as frozen
from owned_metadata import guid_cases, stored_fields, carrier, REPLACEMENT
from verify_out_edits import verify_entries
from verify_native_edits import records, word, span
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot


def verify() -> None:
    """Keep synthetic populated GUID coverage separate from native absence evidence."""
    from cfb import CFB
    raw = (frozen.ROOT / 'target/simulation-metadata-guid-edits.bin').read_bytes()
    assert raw[:8] == b'SIMG\x06\0\0\0'
    cursor, seen, facts = 8, set(), []
    cases = guid_cases()
    with tempfile.TemporaryDirectory(prefix='bessemer-guid-edits-') as directory:
        scratch = Path(directory) / 'carrier.cwr'
        for _ in range(6):
            index, cursor = word(raw, cursor)
            original, cursor = span(raw, cursor)
            edited, cursor = span(raw, cursor)
            label, fields, tail = cases[index]
            assert not tail and index not in seen
            seen.add(index)
            assert original == carrier(stored_fields(fields))
            golden = stored_fields((REPLACEMENT,) + fields[1:])
            before = records(frozen.shared.compound.inspect(original, scratch)[0])
            after = records(frozen.shared.compound.inspect(edited, scratch)[0])
            verify_entries(before, after, golden, stream_name='TimeStampGUIDs')
            scratch.write_bytes(edited)
            actual = CFB(scratch).read('TimeStampGUIDs')
            assert actual == golden
            expected = frozen.guid_oracle(actual)
            facts.append(dict(label=label, edited_bytes=len(edited), edited_sha256=frozen.sha(edited), guid_bytes=len(actual), guid_sha256=frozen.sha(actual), oracle_sha256=frozen.sha(expected)))
    assert cursor == len(raw) and seen == {index for index, (_, _, tail) in enumerate(cases) if not tail}
    frozen.report('owned-guid-edit-reference.json', dict(synthetic_edits=len(facts), native_populated_guid_claims=0, bundle_bytes=len(raw), bundle_sha256=frozen.sha(raw), records=facts))


def run() -> bool:
    """Freeze actual reader modules and verify origins before interpreting the bundle."""
    with tempfile.TemporaryDirectory(prefix='bessemer-guid-edit-source-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        frozen.shared.compound.pin_reader(pin, hashes)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        result = unittest.TextTestRunner(verbosity=2).run(unittest.FunctionTestCase(verify))
        record_snapshot(pin, hashes, result, frozen.FEATURE, ('cfb', 'simulationguids'), ())
        (frozen.FEATURE / 'fixture-reference-snapshot.json').replace(frozen.FEATURE / 'owned-guid-edit-source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
