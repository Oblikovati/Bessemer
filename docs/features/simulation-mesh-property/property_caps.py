"""Actual sufficient-payload acceptance for every fixed Mesh Property count cap (#163)."""
import struct
import sys
from native_carriers import digest
from property_evidence import FEATURE, property_baseline
from property_native import actual_property
from property_projection import property_fields
from property_cap_literals import PropertyCapBuilder, property_cap_builders


def property_cap_field_count(builder: PropertyCapBuilder, count: int) -> int:
    """Count repeated complete entries from independently parsed zero/one controls; e.g. property_cap_field_count(builder, 10000)."""
    sizes = []
    for value in (0, 1):
        version, raw = builder(value)
        parsed = actual_property(raw, version)
        assert parsed['success'] and parsed['end'] == len(raw), (value, parsed)
        sizes.append(len(property_fields(raw, version, parsed)))
    return sizes[0] + count * (sizes[1] - sizes[0])


def verify_property_caps() -> None:
    """Require complete payloads and reconstruction at each cap before rejecting one above; e.g. verify_property_caps()."""
    packet, rows = bytearray(), []
    for name, maximum, builder in property_cap_builders():
        for count in (maximum, maximum + 1):
            version, raw = builder(count)
            parsed = actual_property(raw, version)
            success = bool(parsed['success'])
            assert success == (count == maximum), (name, count, parsed)
            if success: assert parsed['end'] == len(raw) and parsed['reconstruction_sha256'] == digest(raw), name
            fields = property_cap_field_count(builder, count) if success else 0
            packet.extend(struct.pack('<IIIII', 0, version, int(success), fields, len(raw)) + raw)
            rows.append(dict(name=name, count=count, success=success, fields=fields, bytes=len(raw), sha256=digest(raw)))
    path = FEATURE.parents[2] / 'tests/fixtures/simulation-mesh-property/hardcaps.bin'
    if '--write' in sys.argv: path.write_bytes(packet)
    assert path.read_bytes() == packet, f'{path}; expected complete unchanged fixed-cap controls'
    property_baseline('hardcap-reference.json', dict(cases=rows, contexts=len(rows), bytes=len(packet), sha256=digest(packet)))
