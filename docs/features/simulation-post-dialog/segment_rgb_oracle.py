"""Independent C892 ordered normal-flow RGB-read decisions (#166)."""
import struct


def segment_signed_word(bits: int) -> int:
    """Interpret a raw32 index without Rust casts; e.g. segment_signed_word(0xffffffff)."""
    return struct.unpack('<i', struct.pack('<I', bits))[0]


def segment_rgb_oracle(words: list[int]) -> bytes:
    """Return local stage or the first reached unknown, without geometry success; e.g. segment_rgb_oracle(words)."""
    assert len(words) == 13
    mode, first, last, flags = [None if words[index] == 0 else words[index + 1] for index in (0, 2, 4, 6)]
    if mode is None: return segment_rgb_empty(10)
    if ((mode - 3) & 0xffffffff) > 1: return segment_rgb_empty(0)
    if first is None: return segment_rgb_empty(11)
    if segment_signed_word(first) < 1: return segment_rgb_empty(1)
    if last is None: return segment_rgb_empty(12)
    if segment_signed_word(last) < 1: return segment_rgb_empty(2)
    if segment_signed_word(first) > segment_signed_word(last): return segment_rgb_empty(3)
    if flags is None: return segment_rgb_empty(13)
    if flags and (flags // 4) % 2 == 0: return segment_rgb_empty(4)
    if words[8] == 0: return segment_rgb_empty(14)
    return struct.pack('<9I', 5, 1, *words[9:13], *words[9:12])


def segment_rgb_empty(status: int) -> bytes:
    """Serialize explicit read absence independently of zero-valued stored words; e.g. segment_rgb_empty(0)."""
    return struct.pack('<9I', status, *([0] * 8))
