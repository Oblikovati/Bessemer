"""Closed-form scan maps and recognition/failure boundaries independent of Bessemer."""
import tempfile
import unittest
from pathlib import Path
import index_requests as requests
import freeze_replay as freeze
from swarchive import ArchiveError


def scan_equations(raw: bytes) -> tuple[dict, dict]:
    records = requests.wire.controls.definitions.objects.flowresults.read(raw)['records']
    groups = {}
    for record in records:
        groups.setdefault(record['identifier'], []).append((record['start'], record['end'] - record['start']))
    active, saved = {}, {}
    for identifier, spans in groups.items():
        earlier = spans if identifier == requests.wire.DELETED else spans[:-1]
        for start, span in earlier:
            saved[start] = span
        if identifier != requests.wire.DELETED:
            start, span = spans[-1]
            active[identifier] = dict(record_start=start, record_span=span)
    return active, saved


def case_named(rows: list[tuple[str, str, bytes]], name: str) -> bytes:
    return next(raw for label, _, raw in rows if label == name)


class ReplayFixtureTests(unittest.TestCase):
    def test_native_and_original_synthetic_projections_are_fully_authenticated(self):
        rows, proof = requests.reference_cases()
        self.assertEqual(len(rows), 5)
        native = [raw for _, family, raw in rows if family == 'native']
        self.assertEqual(len(native), 3)
        self.assertEqual(sum(map(len, native)), 2936303)
        self.assertTrue(proof['limitations'])
        active_count = sum(len(requests.reference.read(raw)['active_index']) for raw in native)
        self.assertEqual(active_count, 66)

    def test_all_forced_scans_match_closed_form_last_occurrence_and_discard_equations(self):
        for name, _, raw in requests.cases():
            projection = requests.reference.read(raw, force_scan=True)
            active, saved = scan_equations(raw)
            self.assertEqual(projection['method'], 'physical_scan', name)
            self.assertEqual(projection['active_index'], active, name)
            self.assertEqual(projection['saved_spans'], saved, name)
            self.assertFalse(projection['ordinary_payloads_decoded'])
            self.assertFalse(projection['indexed_ordinary_targets_validated'])
            self.assertFalse(projection['complete_document_schema'])

    def test_recognition_misses_scan_without_decoding_even_invalid_control_payloads(self):
        rows = requests.recognition_cases()
        for name, _, raw in rows:
            preferred = requests.outcome(raw, False)
            forced = requests.outcome(raw, True)
            self.assertTrue(forced['completed'], name)
            if name == 'matched-truncated-marker':
                self.assertFalse(preferred['completed'])
                self.assertIn('truncated', preferred['cause'])
                continue
            self.assertTrue(preferred['completed'], name)
            self.assertEqual(preferred['projection']['method'], 'physical_scan', name)
            self.assertEqual(preferred['projection']['active_index'], forced['projection']['active_index'])

    def test_recognized_bad_stream_or_table_fails_instead_of_falling_back(self):
        for name, _, raw in requests.matched_failure_cases():
            preferred = requests.outcome(raw, False)
            self.assertFalse(preferred['completed'], name)
            self.assertTrue(preferred['cause'])
            self.assertTrue(requests.outcome(raw, True)['completed'])

    def test_terminal_last_wins_self_override_and_unvalidated_targets(self):
        rows = requests.terminal_cases()
        raw = case_named(rows, 'terminal-8-duplicates-self-reserved')
        value = requests.reference.read(raw)
        self.assertEqual(value['active_index'][999], dict(record_start=901, record_span=2))
        record = requests.wire.controls.definitions.objects.flowresults.read(raw)['records'][-1]
        self.assertEqual(value['active_index'][requests.wire.INDEX], dict(record_start=record['start'], record_span=record['end'] - record['start']))
        self.assertIn(requests.wire.DELETED, value['active_index'])
        raw = case_named(rows, 'terminal-wide-1-ffffffffffffffff')
        value = requests.reference.read(raw)
        self.assertEqual(value['active_index'][999]['record_start'], 0xffffffffffffffff)
        self.assertFalse(value['indexed_ordinary_targets_validated'])

    def test_saved_reference_uses_selected_physical_occurrence_and_last_duplicate_key(self):
        rows = requests.saved_cases()
        for name, expected in (('saved-select-earlier', 1), ('saved-select-later', 2),
                               ('saved-directive-last-earlier', 1), ('saved-directive-last-later', 2)):
            self.assertEqual(requests.reference.read(case_named(rows, name))['saved_spans'], {7: expected})
        raw = case_named(rows, 'saved-duplicate-last-wins')
        self.assertEqual(requests.reference.read(raw)['saved_spans'], {100: 4, 0: 1})
        for name, _, raw in rows:
            if name.startswith('saved-mismatch') or name in ('saved-missing', 'saved-malformed-selected'):
                self.assertFalse(requests.outcome(raw, False)['completed'], name)
                self.assertTrue(requests.outcome(raw, True)['completed'], name)

    def test_force_scan_type_and_physical_carrier_boundaries_are_strict(self):
        raw = requests.wire.carrier([])
        for context in (1, 0, None, 'false'):
            with self.assertRaises(ArchiveError):
                requests.reference.read(raw, force_scan=context)
        raw = requests.wire.carrier([(7, b'payload')])
        for size in (0, 1, 3, len(raw) - 1):
            for forced in (False, True):
                with self.assertRaises(ArchiveError):
                    requests.reference.read(raw[:size], force_scan=forced)

    def test_terminal_and_saved_cardinality_boundaries_and_complete_empty_cases(self):
        raw = case_named(requests.terminal_cases(), 'terminal-8-unique-65536')
        self.assertEqual(len(requests.reference.read(raw)['active_index']), 65537)
        raw = case_named(requests.saved_cases(), 'saved-map-65536')
        self.assertEqual(len(requests.reference.read(raw)['saved_spans']), 65536)
        raw = case_named(requests.terminal_cases(), 'terminal-8-unique-0')
        self.assertEqual(list(requests.reference.read(raw)['active_index']), [requests.wire.INDEX])
        raw = requests.wire.carrier([])
        self.assertEqual(requests.reference.read(raw)['active_index'], {})
        self.assertEqual(requests.reference.read(raw)['saved_spans'], {})

    def test_all_packets_regenerate_under_a_temporary_root_without_native_execution(self):
        with tempfile.TemporaryDirectory() as folder:
            root = Path(folder)
            freeze.freeze(root)
            for family in ('native', 'scan7', 'scan8', 'terminal'):
                relative = f'tests/fixtures/flow-index-replay/{family}.bin.gz'
                self.assertEqual((root / relative).read_bytes(), (requests.ROOT / relative).read_bytes())
            relative = 'docs/features/flow-index-replay/baseline-verification.json'
            self.assertEqual((root / relative).read_bytes(), (requests.ROOT / relative).read_bytes())


if __name__ == '__main__':
    unittest.main()
