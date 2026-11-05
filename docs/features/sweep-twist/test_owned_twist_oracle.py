"""Independent complete-stream admission, aliases, and literal field edits (#211)."""
from pathlib import Path
from typing import cast
import struct
import tempfile
import unittest
import twist_wire_fixtures as fixture
from twist_pointer_sequence import TwistStyle
from owned_twist_oracle import OwnedTwistArchive, OwnedTwistContext, owned_twist_context, owned_twist_stream, read_owned_twist
from swarchive import ArchiveError


class OwnedTwistOracleTests(unittest.TestCase):
    def test_complete_sweeps_and_cuts_keep_aliases_and_exact_edited_bits(self) -> None:
        self.assertIsNotNone(fixture.REFERENCE)
        reference = cast(Path, fixture.REFERENCE)
        document, context = owned_twist_context(reference)
        original = struct.pack('<BQQ', 255, 0x8000000000000000, 0x7FF8000000001234)
        for name in ('moSweep_c', 'moSweepCut_c'):
            for rich in (False, True):
                for style in ('fresh', 'shared', 'cached-class'):
                    payloads = (original,) if style == 'fresh' else (original, bytes(17))
                    raw, slots, sequence = owned_twist_stream(len(context.map)-1, rich, name, cast(TwistStyle, style), payloads)
                    with self.assertRaisesRegex(ArchiveError, fixture.CLASS):
                        read_owned_twist(reference, document, context, raw, False)
                    result = read_owned_twist(reference, document, context, raw, True)
                    self.assertEqual(len(cast(OwnedTwistArchive, result['archive']).map), slots)
                    self.assert_twist_objects(result, sequence.object_slots, payloads, style)
                    self.assert_literal_edits(reference, document, context, rich, name, cast(TwistStyle, style), payloads)

    def test_existing_sweep_records_and_owned_fixtures_remain_byte_identical(self) -> None:
        from freeze_sweeps import freeze_records, WORKSPACE
        from owned_sweep_fixtures import freeze_owned
        reference = cast(Path, fixture.REFERENCE)
        records = freeze_records(reference)
        packet = b'SWF1' + struct.pack('<I', len(records))
        packet += b''.join(struct.pack('<I', row['slots']) for _, row in records)
        packet += b''.join(raw for raw, _ in records)
        self.assertEqual(len(records), 160)
        self.assertEqual(packet, (WORKSPACE / 'tests/fixtures/sweeps/cases.bin').read_bytes())
        with tempfile.TemporaryDirectory(prefix='bessemer-sweep-regression-') as temporary:
            destination = Path(temporary)
            self.assertEqual(len(freeze_owned(reference, destination)), 7)
            for generated in destination.iterdir():
                self.assertEqual(generated.read_bytes(),
                                 (WORKSPACE / 'tests/fixtures/sweeps' / generated.name).read_bytes())

    def test_retained_owned_packet_matches_all_complete_original_reads(self) -> None:
        from owned_twist_cases import owned_twist_cases, write_owned_twists
        reference = cast(Path, fixture.REFERENCE)
        document, context = owned_twist_context(reference)
        records = owned_twist_cases(len(context.map)-1)
        self.assertEqual(len(records), 48)
        for row in records:
            result = read_owned_twist(reference, document, context, row.raw, True)
            self.assertEqual(len(cast(OwnedTwistArchive, result['archive']).map), row.slots)
            style = 'shared' if '-shared-' in row.label else 'cached-class' if '-cached-class-' in row.label else 'fresh'
            self.assert_twist_objects(result, list(row.object_slots), row.payloads, style)
        write_owned_twists(Path(__file__).resolve().parents[3] / 'target/sweep-twist-owned-fixtures', records)

    def assert_twist_objects(self, result: dict[str, object], slots: list[int],
                             payloads: tuple[bytes, ...], style: str) -> None:
        archive = cast(OwnedTwistArchive, result['archive'])
        for slot, expected in zip(slots, payloads):
            twist = cast(dict[str, object], archive.map[slot])
            encoded = struct.pack('<Bdd', twist['raw_08'], twist['raw_10'], twist['raw_18'])
            self.assertEqual(encoded, expected)
        items = cast(list[dict[str, object]], result['items'])
        twists = [cast(dict[str, object], cast(dict[str, object], item['payload'])['data'])['twist_298']
                  for item in items]
        if style == 'shared':
            self.assertIs(twists[0], twists[1])
        if style == 'cached-class':
            self.assertIsNot(twists[0], twists[1])

    def assert_literal_edits(self, reference: Path, document: object, context: OwnedTwistContext, rich: bool,
                             name: str, style: TwistStyle, payloads: tuple[bytes, ...]) -> None:
        for offset in (0, 1, 9):
            changed = bytearray(payloads[0]); changed[offset] ^= 1
            edited = (bytes(changed),) + payloads[1:]
            raw, slots, sequence = owned_twist_stream(len(context.map)-1, rich, name, style, edited)
            result = read_owned_twist(reference, document, context, raw, True)
            self.assertEqual(len(cast(OwnedTwistArchive, result['archive']).map), slots)
            self.assert_twist_objects(result, sequence.object_slots, edited, style)
