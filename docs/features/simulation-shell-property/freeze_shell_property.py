"""Fresh original/refined shell Property framing while upstream schemas evolve (#159)."""
from pathlib import Path
from typing import cast
import json
import struct
import sys
import tempfile
import unittest

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
for folder in ('simulation-binary-thermal', 'model-archives', 'simulation-solid-manager', 'simulation-solid-body', 'simulation-material'):
    sys.path.insert(0, str(FEATURE.parent / folder))
sys.path.insert(0, str(ROOT / 'target/reference-python'))
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot
from native_carriers import digest, recover_sample, pinned_auxiliaries
from freeze_body import verify_unimported_boundary
from shell_property_evidence import shell_baseline, shell_reports, verify_shell_sources, verify_shell_refinement_guards, verify_shell_clauses
from shell_property_bindings import verify_shell_bindings
from owned_shell_property import owned_shell_properties


def actual_shell_family(raw: bytes, version: int, kind: str) -> dict[str, object]:
    """Replay the exact concrete family with the unchanged scoped callback; e.g. actual_shell_family(raw, 840002, 'Shell')."""
    from cosworks_shell_property_layout import parse as shell
    from cosworks_mesh_control_layout import parse as mesh
    from cosworks_contact_component_layout import parse as component
    from cosworks_contact_pair_layout import parse as pair
    from cosworks_embedded_const_surface import archive_reader
    readers = {'Shell': shell, 'MeshControl': mesh, 'ContactComponent': component, 'ContactPair': pair}
    assert kind in readers, kind
    return cast(dict[str, object], readers[kind](raw, version, kind, archive_reader=archive_reader))


def replay_shell_row(row: dict[str, object], streams: dict[str, bytes], shell_kind: int | None) -> bytes:
    """Compare every original full outcome, including failures, without version selection; e.g. replay_shell_row(row, streams, 0)."""
    raw = streams[cast(str, row['stream'])]
    assert len(raw) == row['bytes'] and digest(raw) == row['sha256']
    kind = cast(str, row.get('kind', 'Shell'))
    packet = bytearray()
    for expected in cast(list[dict[str, object]], row['attempts']):
        version = cast(int, expected['version'])
        actual = actual_shell_family(raw, version, kind)
        assert dict(version=version, **actual) == expected, (row['stream'], version, actual, expected)
        if shell_kind is not None:
            packet.extend(struct.pack('<IIII', shell_kind, version, int(cast(bool, actual['success'])), len(raw)) + raw)
    return bytes(packet)


def shell_report_summary(report: dict[str, object]) -> dict[str, object]:
    """Keep each report denominator separate from unique native identities; e.g. shell_report_summary(report)."""
    rows = cast(list[dict[str, object]], report['rows'])
    attempts = [attempt for row in rows for attempt in cast(list[dict[str, object]], row['attempts'])]
    return dict(evidence=report['evidence'], occurrences=len(rows), logical_bytes=sum(cast(int, row['bytes']) for row in rows),
                arguments=len(attempts), accepted=sum(cast(bool, attempt['success']) for attempt in attempts),
                rejected=sum(not cast(bool, attempt['success']) for attempt in attempts))


def verify_owned_shell_properties() -> None:
    """Compare independent base ends, subtypes and complete inline boundaries; e.g. verify_owned_shell_properties()."""
    from cosworks_shell_property_layout import parse
    from cosworks_embedded_const_surface import archive_reader
    outcomes: list[dict[str, object]] = []
    for case in owned_shell_properties():
        parsed = parse(case['raw'], case['version'], 'Shell', archive_reader=archive_reader, layer_presence=case['layer_presence'])
        assert parsed['success'] and parsed['end'] == len(case['raw']), (case, parsed)
        assert parsed['base_end'] == case['base_end'] and parsed['subtype'] == case['subtype']
        assert parsed['reconstruction_sha256'] == digest(case['raw'])
        spans = cast(list[dict[str, object]], parsed['spans'])
        material_spans = [span for span in spans if cast(str, span['label']).endswith('.material')]
        assert [span['sha256'] for span in material_spans] == [digest(raw) for raw in case['inline_materials']]
        outcomes.append(dict(kind=case['kind'], version=case['version'], layer_presence=case['layer_presence'],
                             raw_hex=case['raw'].hex(), expected=parsed))
    shell_baseline('owned-reference.json', outcomes)


def verify_shell_presence_boundaries() -> None:
    """Keep missing or malformed required runtime context unresolved; e.g. verify_shell_presence_boundaries()."""
    from cosworks_shell_property_layout import parse
    from cosworks_embedded_const_surface import archive_reader
    rejected = 0
    for case in owned_shell_properties():
        presence = case['layer_presence']
        if not presence: continue
        for invalid in (None, [], [True] * (len(presence) + 1), [1] * len(presence)):
            parsed = parse(case['raw'], case['version'], 'Shell', archive_reader=archive_reader, layer_presence=invalid)
            assert not parsed['success'] and 'newest layer presence vector required' in parsed['error']
            rejected += 1
    shell_baseline('presence-reference.json', dict(rejected_missing_wrong_length_or_element_type=rejected))


def replay_shell_native(pin: Path) -> None:
    """Recover all original and refined identities once, then check each historical argument; e.g. replay_shell_native(pin)."""
    reports = shell_reports()
    all_rows = [cast(list[dict[str, object]], report['rows']) for report in reports]
    packet = bytearray()
    for document in sorted({cast(str, row['path']) for rows in all_rows for row in rows}):
        streams = recover_sample(document, pin)
        for index, rows in enumerate(all_rows):
            for row in (row for row in rows if row['path'] == document):
                packet.extend(replay_shell_row(row, streams, index if index < 2 else None))
    summaries = [shell_report_summary(report) for report in reports]
    assert (summaries[0]['occurrences'], summaries[0]['logical_bytes']) == (234, 826556)
    assert (summaries[1]['occurrences'], summaries[1]['logical_bytes']) == (15, 15770)
    original_keys = [(row['path'], row['stream']) for rows in all_rows[:2] for row in rows]
    assert len(set(original_keys)) == len(original_keys) == 249
    for summary in summaries[2:]: assert (summary['occurrences'], summary['logical_bytes']) == (378, 1489023)
    (ROOT / 'target/simulation-shell-property-native.bin').write_bytes(packet)
    shell_baseline('native-reference.json', dict(original_occurrences=249, original_logical_bytes=842326,
                   reports=summaries, packet_bytes=len(packet), packet_sha256=digest(packet)))


def run_shell_property_checks(pin: Path) -> unittest.TestResult:
    """Run the current pinned tests plus full original-outcome and source guards; e.g. run_shell_property_checks(pin)."""
    from functools import partial, update_wrapper
    import importlib
    replay = update_wrapper(partial(replay_shell_native, pin), replay_shell_native)
    checks = (verify_shell_sources, verify_shell_refinement_guards, verify_shell_clauses, verify_shell_bindings, verify_owned_shell_properties, verify_shell_presence_boundaries, replay, verify_unimported_boundary)
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    modules = ('test_cosworks_shell_property', 'test_cosworks_material', 'test_cosworks_embedded_body_ids',
               'test_cosworks_embedded_const_surface', 'test_cosworks_base_curves', 'test_cosworks_base_virtual',
               'test_cosworks_shell_layer_descriptor', 'test_cosworks_mesh_control', 'test_cosworks_contact_component', 'test_cosworks_contact_pair')
    for name in modules: suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module(name)))
    return unittest.TextTestRunner(verbosity=2).run(suite)


def run() -> bool:
    """Freeze all source imports before native/reference replay; e.g. run()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-shell-property-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        pinned_auxiliaries(pin, hashes)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin / 'research'))
        result = run_shell_property_checks(pin)
        verify_unimported_boundary()
        record_snapshot(pin, hashes, result, FEATURE, ('cosworks_shell_property_layout', 'cosworks_material_layout',
                        'cosworks_embedded_const_surface', 'cosworks_embedded_body_ids', 'cosworkspersist', 'cfb', 'swcontainer'), ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
