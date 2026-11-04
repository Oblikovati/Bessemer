"""Independent C891 execution-order and COMISS carry-flag oracle (#166)."""
import math
import struct


def plot54_carry_clamp(alpha: int) -> int:
    """Use the reviewed COMISS(1,alpha)/JAE flag decision, not the shared Rust bit classifier; e.g. plot54_carry_clamp(0xffc00042)."""
    value = struct.unpack('<f', struct.pack('<I', alpha))[0]
    carry = math.isnan(value) or 1.0 < value
    return 0x3f800000 if carry else alpha


def plot54_plan_status(words: list[int]) -> int:
    """Require facts only when native control flow reaches them, including copies before helper rejection; e.g. plot54_plan_status(words)."""
    mode_tag, mode, plot_tag, plot, colors = words[:5]
    core, callback = words[13:15]
    if mode_tag == 0: return 10
    if ((mode - 3) & 0xffffffff) > 1: return 0
    if plot_tag == 0: return 11
    if plot != 54: return 1
    if colors == 0: return 12
    if core == 0: return 13
    if core == 1: return 2
    if callback == 0: return 14
    if callback == 1: return 3
    return 4


def plot54_plan_oracle(words: list[int]) -> bytes:
    """Keep raw copied arguments separate from the admitted read and post-comparison color; e.g. plot54_plan_oracle(words)."""
    status = plot54_plan_status(words)
    copied = status in (2, 3, 4)
    read = status == 4
    arguments = words[5:13] if copied else [0] * 8
    original = words[5:9] if read else [0] * 4
    projected = original[:3] + [plot54_carry_clamp(original[3])] if read else [0] * 4
    return struct.pack('<19I', status, int(copied), int(read), *arguments, *original, *projected)
