"""Check complete owned archive boundaries using the actual additive callback (#134)."""
import struct
from owned_embedded import owned_embedded_cases


def verify_direct_boundaries(case: dict[str, int | str]) -> None:
    """Locate both transforms with the actual direct codec; e.g. verify_direct_boundaries(case)."""
    from mfcobjects import ObjectArchive
    from dl_ui import read_mgXform_c
    raw = bytes.fromhex(str(case['raw_hex']))
    archive = ObjectArchive(raw, int(case['version']), int(case['wrapper_end']))
    first = read_mgXform_c(archive)
    assert archive.p == case['first_transform_end']
    assert first['flags'] == case['flags']
    assert ('matrix' in first) == (case['matrix_presence'] != 0)
    second = read_mgXform_c(archive)
    assert archive.p == len(raw) and 'matrix' in second
    assert second['flags'] == int(case['flags']) ^ 0x0f


def verify_owned_embedded() -> list[dict[str, object]]:
    """Verify populated matrices and both direct suffixes; e.g. verify_owned_embedded()."""
    from cosworks_binary_thermal_transform_audit import archive_reader, TRACE
    outcomes: list[dict[str, object]] = []
    for case in owned_embedded_cases():
        verify_direct_boundaries(dict(case))
        raw = bytes.fromhex(case['raw_hex'])
        TRACE.clear()
        end, details = archive_reader(raw + b'TAIL', 0)
        assert end == len(raw) and details['object_class'] == 'moCompXFormWrapper_c'
        assert details['classes'] == {'moCompXFormWrapper_c': 1}
        assert details['body_counts']['wrapper_bodies'] == 1
        assert len(TRACE) == 1 and TRACE[0]['offset'] == 10 + len(b'moCompXFormWrapper_c')
        event = TRACE[0]
        assert event['end'] == len(raw)
        assert event['transform_bytes_hex'] == raw[case['wrapper_end']:].hex()
        outcomes.append(dict(**case, archive=details, transform_trace=dict(event)))
    assert len(outcomes) == 198
    return outcomes


def verify_embedded_rejections() -> None:
    """Require actual errors for every populated prefix and unsupported version; e.g. verify_embedded_rejections()."""
    from cosworks_binary_thermal_transform_audit import archive_reader
    from swarchive import ArchiveError
    import unittest
    case = next(case for case in owned_embedded_cases() if case['matrix_presence'] == 255 and case['flags'] == 0x4b)
    raw = bytes.fromhex(case['raw_hex'])
    assertion = unittest.TestCase()
    for end in range(len(raw)):
        with assertion.assertRaises((ArchiveError, UnicodeError)):
            archive_reader(raw[:end], 0)
    for version in (0, 4281, 4306, 19001, 0xffffffff):
        with assertion.assertRaises(ArchiveError):
            archive_reader(struct.pack('<I', version) + raw[4:], 0)
