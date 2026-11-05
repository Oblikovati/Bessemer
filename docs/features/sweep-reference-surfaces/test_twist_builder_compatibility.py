"""Keep owned pre-twist surface wire unchanged when no writer is supplied (#211)."""
from pathlib import Path
import struct
import unittest
from sweep_surface_bytes import sweep_surface_object

ROOT = Path(__file__).resolve().parents[3]
FIXTURES = ROOT / 'tests/fixtures/sweep-reference-surfaces'


class SurfaceTwistBuilderCompatibilityTests(unittest.TestCase):
    def test_all_162_prior_bare_graphs_keep_literal_bytes_and_slots(self) -> None:
        packet = (FIXTURES / 'cases.bin').read_bytes()
        self.assertEqual(packet[:8], b'SRS1' + struct.pack('<I', 162))
        slots = struct.unpack_from('<162I', packet, 8)
        position = 8 + 162 * 4
        for expected_slots in slots:
            kind, version, length, oracle_length = struct.unpack_from('<BIII', packet, position)
            position += 13
            raw, actual_slots = sweep_surface_object(kind // 6, version, kind % 6)
            self.assertEqual((raw, actual_slots), (packet[position:position+length], expected_slots))
            position += length + oracle_length
        self.assertEqual(position, len(packet))

    def test_all_19_prior_owned_streams_keep_literal_default_bytes(self) -> None:
        for kind in range(3):
            for style in range(6):
                self.assert_owned_surface(kind, style, False)
        self.assert_owned_surface(0, 2, True)

    def assert_owned_surface(self, kind: int, style: int, edited: bool) -> None:
        raw, _ = sweep_surface_object(kind, 19000, style, edited, 96)
        label = f'owned-{kind}-{style}' + ('-edited' if edited else '') + '.bin'
        self.assertEqual(struct.pack('<iH', -123, 1) + raw, (FIXTURES / label).read_bytes())


if __name__ == '__main__':
    unittest.main()
