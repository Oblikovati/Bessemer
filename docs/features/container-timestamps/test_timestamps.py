"""Independent arithmetic, request diversity and byte-for-byte baseline replay."""
from __future__ import annotations
import gzip
import struct
import tempfile
import unittest
from pathlib import Path
import freeze_timestamps as frozen


class TimestampBaselineTests(unittest.TestCase):
    def test_all_reference_outcomes_match_weighted_unsigned_oracle(self) -> None:
        for request in frozen.requests():
            self.assertEqual(frozen.expected(*request), frozen.oracle(*request))
        self.assertGreater(len(frozen.requests()), 20_000)

    def test_every_input_axis_retains_all_single_bits_and_complements(self) -> None:
        rows = [calendar for calendar, override in frozen.requests() if calendar is not None and not override]
        for axis in range(6):
            observed = {row[axis] for row in rows}
            for bit in range(32):
                self.assertIn(1 << bit, observed)
                self.assertIn(0xffffffff ^ (1 << bit), observed)

    def test_exact_year_boundaries_overrides_and_time_overflow(self) -> None:
        self.assertEqual(frozen.expected((0, 0, 0, 1, 0, 1980), 0), 0xd8210000)
        self.assertEqual(frozen.expected((0, 0, 0, 1, 0, 80), 0), 0xa0210000)
        self.assertEqual(frozen.expected((0xffffffff, 0, 0, 0, 0, 0), 0), 0x7fffffff)
        self.assertEqual(frozen.expected(None, 0), 0)
        for override in frozen.patterns():
            if override:
                self.assertEqual(frozen.expected((0xffffffff,) * 6, override), override)
                self.assertEqual(frozen.expected(None, override), override)

    def test_native_header_intervals_and_reference_names_cover_all_113_streams(self) -> None:
        rows = frozen.native_rows()
        self.assertEqual(len(rows), 113)
        self.assertEqual({row[0] for row in rows}, {0, 1, 2})
        for ordinal, name, local, central, expected in rows:
            raw = (frozen.ROOT / 'tests/fixtures' / frozen.FIXTURES[ordinal]).read_bytes()
            self.assertTrue(name)
            self.assertLess(local, central)
            self.assertEqual(struct.unpack_from('<I', raw, local)[0], expected)
            self.assertEqual(struct.unpack_from('<I', raw, central)[0], expected)

    def test_packet_generation_replays_byte_for_byte_under_an_injected_root(self) -> None:
        with tempfile.TemporaryDirectory(prefix='bessemer-timestamps-') as directory:
            root = Path(directory)
            actual = frozen.freeze(root)
            self.assertEqual(actual['packets']['native']['cases'], 113)
            for name in ('requests', 'native'):
                relative = Path('tests/fixtures/container-timestamps') / (name + '.bin.gz')
                self.assertEqual((root / relative).read_bytes(), (frozen.ROOT / relative).read_bytes())
                raw = gzip.decompress((root / relative).read_bytes())
                self.assertEqual(frozen.digest(raw), actual['packets'][name]['sha256'])

    def test_varied_axis_and_native_packets_preserve_explicit_wire_widths(self) -> None:
        base = (1, 2, 3, 4, 5, 6)
        self.assertEqual(frozen.varied(base, 2, 9), (1, 2, 9, 4, 5, 6))
        self.assertEqual(len(frozen.request_packet((None, 7))), 9)
        self.assertEqual(len(frozen.request_packet((base, 7))), 33)
        self.assertEqual(len(frozen.native_packet((0, 'x', 10, 20, 30))), 26)
