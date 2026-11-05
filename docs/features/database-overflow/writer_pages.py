"""Independent owned storage images derived from retained writer assembly (#229)."""
from dataclasses import dataclass
import struct


@dataclass(frozen=True)
class OverflowImage:
    """Keep original payloads independent of the page image; e.g. image.key."""
    extent: int
    key: bytes
    value: bytes
    bucket: bytes
    pages: tuple[tuple[int, bytes], ...]
    cursor_wrapped: bool


def payload_bytes(length: int, salt: int) -> bytes:
    """Generate nontext payloads with distinct page segments; e.g. payload_bytes(512, 3)."""
    return bytes((index * 37 + index // 251 + salt) % 256 for index in range(length))


def copy_payload(pages: list[bytearray], extent: int, cursor: int, payload: bytes) -> int:
    """Model bounded physical copies, independent of the key reader; e.g. copy_payload(p,512,18,k)."""
    consumed = 0
    while consumed < len(payload):
        if cursor == extent:
            pages.append(bytearray([0xa5]) * extent)
            cursor = 8
        count = min(extent - cursor, len(payload) - consumed)
        pages[-1][cursor:cursor + count] = payload[consumed:consumed + count]
        consumed += count
        cursor += count
    return cursor


def page_identifiers(count: int) -> tuple[int, ...]:
    """Use noncontiguous full-width IDs rather than file indexes; e.g. page_identifiers(2)."""
    return tuple(0xffffffffffffffff - index * 0x100000003 for index in range(count))


def frame_pages(pages: list[bytearray], value_page: int, cursor: int) -> tuple[tuple[int, bytes], ...]:
    """Store BE links and first-page value metadata; e.g. frame_pages(pages,0,18)."""
    identifiers = page_identifiers(len(pages))
    struct.pack_into('>QH', pages[0], 8, identifiers[value_page], cursor & 0xffff)
    for index, page in enumerate(pages):
        following = identifiers[index + 1] if index + 1 < len(pages) else 0
        struct.pack_into('>Q', page, 0, following)
    return tuple((identity, bytes(page)) for identity, page in zip(identifiers, pages))


def make_image(extent: int, key_length: int, value_length: int) -> OverflowImage:
    """Construct the selected writer storage, including cursor truncation; e.g. make_image(512,495,4)."""
    assert 18 < extent <= 65536
    key = payload_bytes(key_length, 0)
    value = payload_bytes(value_length, 173)
    pages = [bytearray([0xa5]) * extent]
    cursor = copy_payload(pages, extent, 18, key)
    value_page = len(pages) - 1
    copy_payload(pages, extent, cursor, value)
    framed = frame_pages(pages, value_page, cursor)
    bucket = bytearray([0xa5]) * extent
    struct.pack_into('>HHQ', bucket, 0, 20, 0, 0)
    struct.pack_into('>IIQHQ', bucket, 20, 0x80000001, key_length, value_length, 0, framed[0][0])
    return OverflowImage(extent, key, value, bytes(bucket), framed, cursor > 65535)


def image_matrix() -> tuple[OverflowImage, ...]:
    """Cover exact first/continuation boundaries and a wrapped cursor; e.g. image_matrix()."""
    lengths = (0, 1, 493, 494, 495, 997, 998, 999)
    values = (0, 1, 17, 494, 495, 503, 504, 505, 1009)
    small = tuple(make_image(512, key, value) for key in lengths for value in values)
    large = tuple(make_image(extent, key, value)
        for extent in (1024, 4096, 65536)
        for key in (0, extent - 19, extent - 18, extent - 17, 2 * extent - 26)
        for value in (0, 1, extent - 8))
    return small + large


def encode_image(image: OverflowImage) -> bytes:
    """Encode one self-contained oracle record; e.g. encode_image(make_image(512,1,2))."""
    header = struct.pack('<IIQBI', image.extent, len(image.key), len(image.value),
                         image.cursor_wrapped, len(image.pages))
    pages = b''.join(struct.pack('<Q', identity) + page for identity, page in image.pages)
    return header + image.bucket + pages + image.key + image.value


def fixture_packet() -> bytes:
    """Build the HOV1 packet with explicit record extents; e.g. fixture_packet()."""
    records = tuple(encode_image(image) for image in image_matrix())
    return b'HOV1' + struct.pack('<I', len(records)) + b''.join(
        struct.pack('<I', len(record)) + record for record in records)
