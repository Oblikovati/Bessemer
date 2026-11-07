"""Detach complete native embedded roots using the actual C-716 archive callback (#134)."""
from typing import cast
import unittest
import struct
from owned_chains import tagged_component


def detached_archive_packet(raw: bytes, attempt: dict[str, object]) -> bytes:
    """Check independent archive framing and exact full-registry replays; e.g. detached_archive_packet(raw, attempt)."""
    from cosworks_binary_thermal_transform_audit import archive_reader
    packet = bytearray()
    for retained in cast(list[dict[str, object]], attempt['archive_records']):
        start, end = cast(int, retained['offset']), cast(int, retained['end'])
        image = raw[start:end]
        expected = dict(retained, offset=0, end=len(image))
        actual_end, actual = archive_reader(image, 0)
        assert actual_end == len(image) and actual == expected
        packet.extend(len(image).to_bytes(4, 'little') + image)
    return bytes(packet)


def component_guard_rejections() -> None:
    """Check actual named owner, unit and count guards independently; e.g. component_guard_rejections()."""
    from cosworks_binary_thermal_transform_audit import archive_reader
    from swarchive import ArchiveError
    bodies = [('moExtFeatureList_c', struct.pack('<i', count)) for count in (-1, 10001, 0x7fffffff)]
    bodies += [('moUnitComponent_c', bytes((0, 0, flag))) for flag in (1, 255)]
    owner = tagged_component('moExtFeatureList_c', struct.pack('<i', 0))
    bodies += [(name, owner) for name in ('moUnitComponent_c', 'moChainComponent_c')]
    assertion = unittest.TestCase()
    for name, body in bodies:
        with assertion.assertRaises(ArchiveError):
            archive_reader(struct.pack('<I', 19000) + tagged_component(name, body), 0)
