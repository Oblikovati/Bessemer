"""Pin export protocol bounds independently of either implementation's writer (#202)."""
from pathlib import Path
import struct
import sys
import unittest

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'docs/features/resolved-root-graph'))
from parameter_packet import ParameterPacket


class ParameterPacketTests(unittest.TestCase):
    def test_literal_numeric_and_text_images_remain_exact(self) -> None:
        """Preserve raw NaN images, signed integers, UTF-8 and original CString frames."""
        cases = [(struct.pack('<IQ', 0, 0x7ff0000000000001), (0, bytes.fromhex('010000000000f07f'))),
                 (struct.pack('<Ii', 1, -2**31), (1, bytes.fromhex('00000080'))),
                 (struct.pack('<II', 2, 3) + b'ABC' + struct.pack('<I', 6) + b'\xff\x03\x00ABC',
                  (2, 'ABC', b'\xff\x03\x00ABC'))]
        for raw, expected in cases:
            packet = ParameterPacket(raw)
            self.assertEqual(packet.value(), expected)
            self.assertEqual(packet.position, len(raw))

    def test_null_name_and_empty_context_have_literal_complete_shapes(self) -> None:
        """Keep null independent from an empty parameter context."""
        packet = ParameterPacket(bytes(28))
        self.assertEqual(packet.name(), dict(tag=(0, None, 0, 0, '', 0), origin=0, text=None))
        self.assertEqual(packet.position, len(packet.raw))
        self.assertEqual(ParameterPacket(bytes(4)).parameters(), [])

    def test_missing_frames_and_invalid_discriminators_or_budgets_are_rejected(self) -> None:
        """Reject malformed exports before reporting successful equality."""
        bad_frames = [bytes(3), struct.pack('<I', 1), struct.pack('<I', 64 * 1024 * 1024 + 1)]
        for raw in bad_frames:
            with self.assertRaises(AssertionError):
                ParameterPacket(raw).frame()
        for method, word in [('present', 2), ('value', 3), ('parameters', 100001)]:
            with self.assertRaises(AssertionError):
                getattr(ParameterPacket(struct.pack('<I', word)), method)()


if __name__ == '__main__':
    unittest.main()
