"""Complete reference-surface streams with declared twist bytes (#211)."""
from itertools import product
from typing import cast
import struct
from owned_twist_cases import OwnedTwistCase
from sweep_surface_bytes import sweep_surface_object
from twist_pointer_sequence import TwistPointerSequence, TwistStyle


def surface_twist_cases(first_slot: int) -> list[OwnedTwistCase]:
    """Cover six surface forms and repeated-pointer variants; e.g. surface_twist_cases(96)."""
    shapes = [(style, 'fresh') for style in range(6)]
    shapes += list(product((2, 5), ('shared', 'cached-class')))
    return [record for style, pointer in shapes
            for record in surface_twist_variants(first_slot, style, cast(TwistStyle, pointer))]


def surface_twist_variants(first_slot: int, style: int, pointer: TwistStyle) -> list[OwnedTwistCase]:
    """Preserve independent surface suffixes around each twist; e.g. surface_twist_variants(96,2,'shared')."""
    original = struct.pack('<BQQ', 255, 0x8000000000000000, 0x7FF8000000001234)
    records: list[OwnedTwistCase] = []
    for offset in (-1, 0, 1, 9):
        payload = bytearray(original)
        if offset >= 0:
            payload[offset] ^= 1
        payloads = (bytes(payload), bytes(17)) if style in (2, 5) else (bytes(payload),)
        sequence = TwistPointerSequence(pointer, payloads)
        feature, slots = sweep_surface_object(0, 19000, style, first_slot=first_slot, twist=sequence)
        raw = struct.pack('<iH', -123, 1) + feature + bytes(4)
        label = f'surface-{style}-{pointer}-{offset}'
        records.append(OwnedTwistCase(label, raw, slots, tuple(sequence.object_slots), payloads))
    return records
