"""Independent raw layouts, integer comparison oracle and selected-record failures."""
from __future__ import annotations
import gzip
import json
import struct
import tempfile
import unittest
from collections.abc import Callable
from pathlib import Path
import legacy_requests as requests
import freeze_legacy_fields as frozen
from swarchive import ArchiveError


def scalar_oracle(slot7: int, slot10: int) -> tuple[bool, bool, int | None]:
    magnitude = slot7 & 0x7fffffffffffffff
    nan = magnitude & 0x7ff0000000000000 == 0x7ff0000000000000 and magnitude & 0xfffffffffffff != 0
    status = None
    if not nan and (slot7 >> 63 or slot7 <= 0x406fa00000000000):
        status = -2
    elif not nan and slot7 > 0x4081a80000000000:
        status = -1
    return slot10 & 0x7fffffffffffffff != 0, status is None, status


class LegacyFieldBaselineTests(unittest.TestCase):
    def test_three_native_payloads_and_scalar_constants_authenticate(self) -> None:
        rows, evidence = requests.native_cases()
        self.assertEqual(len(rows), 3)
        self.assertEqual(sum(len(raw) for _, _, raw in rows), 32068)
        constants = evidence['scalar_constants']
        self.assertEqual({value['value'] for value in constants.values()}, {253.0, 565.0})
        for value in constants.values():
            self.assertEqual(struct.unpack('<d', bytes.fromhex(value['native_bytes']))[0], value['value'])

    def test_all_word_counts_and_fixed_payloads_have_independent_lengths(self) -> None:
        for _, identifier, raw in requests.word_cases():
            parsed = requests.reference.read_record(dict(identifier=identifier, size=len(raw), payload=raw))
            count = 320 if identifier == 105 else struct.unpack_from('<i', raw)[0]
            self.assertEqual(parsed['count'], count)
            self.assertEqual(len(raw), count * 8 + (4 if identifier == 113 else 0))
            self.assertEqual(parsed['end'], len(raw))
            self.assertFalse(parsed['engineering_fields_assigned'])

    def test_all_90601_query_pairs_match_integer_ieee_classification(self) -> None:
        domain = requests.query_domain()
        self.assertEqual(len(domain), 301)
        for slot7 in domain:
            for slot10 in domain:
                value = requests.query(slot7, slot10)
                self.assertEqual((value['slot10_nonzero'], value['slot7_calls_virtual_78'], value['slot7_precheck_status']), scalar_oracle(slot7, slot10))
                self.assertFalse(value['native_fp_faults_reproduced'])
                self.assertFalse(value['native_virtual_result_reproduced'])

    def test_threshold_neighbors_signed_zeros_and_signaling_nan_payloads_have_literal_results(self) -> None:
        expected = [(0x406fa00000000000, -2), (0x406fa00000000001, None),
                    (0x4081a80000000000, None), (0x4081a80000000001, -1),
                    (0x7ff0000000000001, None), (0xfff0000000000001, None)]
        for bits, status in expected:
            self.assertEqual(requests.query(bits, 0)['slot7_precheck_status'], status)
        for bits in (0, 0x8000000000000000):
            self.assertFalse(requests.query(0, bits)['slot10_nonzero'])
        for bits in (1, 0x8000000000000001, 0x7ff0000000000001):
            self.assertTrue(requests.query(0, bits)['slot10_nonzero'])

    def test_every_fallback_group_rebuilds_all_shared_fields_and_scope_extents(self) -> None:
        rows = requests.group_cases()
        self.assertEqual(len(rows), 1986)
        for _, raw in rows:
            expected = requests.group_snapshot(raw)
            self.assertEqual(bytes(expected.raw), raw)
            self.assertEqual(expected.scopes[0], ((), 0, len(raw)))
        inactive = requests.group_raw(requests.conditional.GroupSpec(mode=0, gate=0))
        empty = requests.group_raw(requests.conditional.GroupSpec(mode=1))
        self.assertIsNone(requests.reference.read_155(inactive)['group']['count'])
        self.assertEqual(requests.reference.read_155(empty)['group']['count'], 0)

    def test_compact_prefixes_extensions_and_reached_invalid_counts_are_rejected(self) -> None:
        for name, identifier, raw in requests.word_cases():
            reader = requests.reference.read_105 if identifier == 105 else requests.reference.read_113
            self.check_prefixes(reader, raw, len(raw) <= 128 or name.startswith('native-'))
        for _, raw in requests.group_cases():
            self.check_prefixes(requests.reference.read_155, raw, len(raw) <= 128)
        for count in (-0x80000000, -1, 0x7fffffff):
            self.assertRaises(ArchiveError, requests.reference.read_113, struct.pack('<i', count))
        self.assertRaises(ArchiveError, requests.reference.read_155, struct.pack('<i', 1))
        self.bad_version_one_mode()

    def check_prefixes(self, reader: Callable[[bytes], dict], raw: bytes, compact: bool) -> None:
        if compact:
            for end in range(len(raw)):
                self.assertRaises(ArchiveError, reader, raw[:end])
        self.assertRaises(ArchiveError, reader, raw + b'\0')

    def bad_version_one_mode(self) -> None:
        spec = requests.conditional.ItemSpec(version=1, mode=0)
        raw = bytearray(requests.group_raw(requests.conditional.GroupSpec(items=(spec,))))
        mode = next(field for field in requests.group_snapshot(raw).fields if field.path[-1] == 'mode_byte')
        raw[mode.offset] = 1
        self.assertRaises(ArchiveError, requests.reference.read_155, raw)

    def test_all_400_selection_outcomes_preserve_preference_and_fail_without_fallback(self) -> None:
        for _, records, result in requests.selection_cases():
            selected = next((record for identifier in (113, 155) for record in records[::-1] if record[0] == identifier), None)
            if selected is None:
                self.assertEqual(result, dict(completed=True, identifier=None, cause=''))
                continue
            identifier, raw = selected
            valid = raw == b'\0' * 4 if identifier == 113 else len(raw) == 24
            self.assertEqual(result['completed'], valid)
            self.assertEqual(result['identifier'], identifier if valid else None)
            self.assertEqual(bool(result['cause']), not valid)

    def test_packets_and_reference_manifest_regenerate_byte_for_byte(self) -> None:
        with tempfile.TemporaryDirectory() as folder:
            root = Path(folder)
            frozen.freeze(root)
            report = Path('docs/features/flow-legacy-field-containers/baseline-verification.json')
            self.assertEqual((root / report).read_bytes(), (requests.ROOT / report).read_bytes())
            for family, metadata in json.loads((root / report).read_text())['packets'].items():
                path = Path('tests/fixtures/flow-legacy-field-containers') / f'{family}.bin.gz'
                self.assertEqual((root / path).read_bytes(), (requests.ROOT / path).read_bytes())
                raw = gzip.decompress((root / path).read_bytes())
                self.assertLessEqual(len(raw), 16_000_000)
                self.assertEqual(len(raw), metadata['bytes'])
                self.assertEqual(requests.digest(raw), metadata['sha256'])
