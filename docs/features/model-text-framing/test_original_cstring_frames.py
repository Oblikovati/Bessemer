"""Confirm legal count-prefix alternatives through original swarchive (#200)."""
import struct
import unittest
from swarchive import Archive


def literal_cstring_frames() -> list[tuple[str, str, bytes]]:
    """Build six explicit independent frames; e.g. literal_cstring_frames()."""
    frames = []
    for encoding, text, count, payload in [('Latin1', 'ABC', 3, b'ABC'),
            ('Utf16', 'Ω equation 0', 12, 'Ω equation 0'.encode('utf-16-le'))]:
        marker = b'\xff\xfe\xff' if encoding == 'Utf16' else b''
        prefixes = [bytes([count]), b'\xff' + struct.pack('<H', count), b'\xff\xff\xff' + struct.pack('<I', count)]
        frames.extend((f'{encoding}/{width}', text, marker + prefix + payload) for width, prefix in enumerate(prefixes))
    return frames


class OriginalCStringFrameTests(unittest.TestCase):
    def test_every_literal_frame_decodes_exactly_through_eof(self) -> None:
        """Require all Byte/Word/Dword representations in both admitted encodings."""
        for name, expected, frame in literal_cstring_frames():
            with self.subTest(frame=name):
                reader = Archive(frame, 19000)
                self.assertEqual(reader.cstring(), expected)
                self.assertEqual(reader.p, len(frame))

    def test_equal_text_has_distinct_raw_images_for_each_width(self) -> None:
        """Keep semantic text comparison separate from physical field framing."""
        frames = literal_cstring_frames()
        for start in (0, 3):
            selected = frames[start:start + 3]
            self.assertEqual(len({text for _, text, _ in selected}), 1)
            self.assertEqual(len({wire for _, _, wire in selected}), 3)
