"""Independent closed-form and metadata-directed checks of the overflow fixtures."""
import struct
import unittest
from writer_pages import OverflowImage, image_matrix, make_image, fixture_packet


def key_position(extent: int, length: int) -> tuple[int, int]:
    """Derive the key end without running the writer loop; e.g. key_position(512,495)."""
    first = extent - 18
    if length <= first:
        return 0, 18 + length
    full, residual = divmod(length - first - 1, extent - 8)
    return full + 1, 9 + residual


def reconstruct_key(image: OverflowImage) -> bytes:
    """Read key bytes using closed-form boundaries; e.g. reconstruct_key(make_image(512,495,0))."""
    end_page, end_cursor = key_position(image.extent, len(image.key))
    pieces = [page[18 if index == 0 else 8:end_cursor if index == end_page else image.extent]
              for index, (_, page) in enumerate(image.pages[:end_page + 1])]
    return b''.join(pieces)


def reconstruct_value(image: OverflowImage) -> bytes:
    """Follow stored value metadata separately from key copying; e.g. reconstruct_value(image)."""
    identity, cursor = struct.unpack_from('>QH', image.pages[0][1], 8)
    available = dict(image.pages)
    pieces: list[bytes] = []
    remaining = len(image.value)
    while remaining:
        assert identity in available and 8 <= cursor <= image.extent
        page = available.pop(identity)
        count = min(remaining, image.extent - cursor)
        pieces.append(page[cursor:cursor + count])
        remaining -= count
        identity = struct.unpack_from('>Q', page)[0]
        cursor = 8
    return b''.join(pieces)


class WriterPageTests(unittest.TestCase):
    def test_complete_key_and_value_bytes_match_independent_spans(self) -> None:
        for image in image_matrix():
            self.assertEqual(reconstruct_key(image), image.key)
            if not image.cursor_wrapped:
                self.assertEqual(reconstruct_value(image), image.value)

    def test_value_checkpoint_matches_closed_form_key_end(self) -> None:
        for image in image_matrix():
            index, cursor = key_position(image.extent, len(image.key))
            expected = (image.pages[index][0], cursor & 0xffff)
            self.assertEqual(struct.unpack_from('>QH', image.pages[0][1], 8), expected)

    def test_links_and_bucket_metadata_match_independent_counts(self) -> None:
        for image in image_matrix():
            count = 1 + max(0, (len(image.key) + len(image.value) - (image.extent - 18)
                              + image.extent - 9) // (image.extent - 8))
            self.assertEqual(len(image.pages), count)
            self.assertEqual(struct.unpack_from('>IIQHQ', image.bucket, 20),
                             (0x80000001, len(image.key), len(image.value), 0, image.pages[0][0]))
            for index, (_, page) in enumerate(image.pages):
                following = image.pages[index + 1][0] if index + 1 < count else 0
                self.assertEqual(struct.unpack_from('>Q', page)[0], following)

    def test_first_native_reader_request_differs_from_stored_boundary(self) -> None:
        image = make_image(512, 504, 0)
        self.assertEqual(key_position(512, 504), (1, 18))
        self.assertEqual(len(image.pages[0][1][18:]), 494)
        self.assertEqual(min(504, 512 - 8), 504)
        self.assertEqual(reconstruct_key(image), image.key)

    def test_full_65536_cursor_wrap_cannot_admit_value_at_zero(self) -> None:
        image = make_image(65536, 65518, 1)
        self.assertTrue(image.cursor_wrapped)
        self.assertEqual(key_position(65536, 65518), (0, 65536))
        self.assertEqual(struct.unpack_from('>H', image.pages[0][1], 16)[0], 0)
        with self.assertRaises(AssertionError):
            reconstruct_value(image)

    def test_fixture_packet_is_repeatable_with_complete_record_extents(self) -> None:
        packet = fixture_packet()
        self.assertEqual(packet, fixture_packet())
        self.assertEqual(packet[:8], b'HOV1' + struct.pack('<I', 117))
        cursor = 8
        for _ in image_matrix():
            length = struct.unpack_from('<I', packet, cursor)[0]
            cursor += 4 + length
        self.assertEqual(cursor, len(packet))


if __name__ == '__main__':
    unittest.main()
