"""Sufficient-payload controls on both sides of all five counted-record grammar caps (#161)."""
from collections.abc import Callable
import struct
import sys
from native_carriers import digest
from owned_shell_property import empty_shell_base
from shell_property_caps import reference_cap_literal, blob_cap_literal, curve_cap_literal, optional_cap_literal, shell_null_prefix, shell_null_tail
from mesh_control_evidence import FEATURE, mesh_baseline
from owned_mesh_records import mesh_record_suffix
from mesh_record_packets import actual_mesh_records
from mesh_record_projection import mesh_record_fields

BaseCapBuilder = Callable[[int], tuple[int, bytes]]


def mesh_cap_from_base(builder: BaseCapBuilder, count: int) -> tuple[int, bytes]:
    """Reuse independently built complete base payloads without retaining the Shell continuation; e.g. mesh_cap_from_base(blob_cap_literal, 1000000)."""
    version, shell = builder(count)
    suffix = shell_null_prefix(version)[len(empty_shell_base(version)):] + shell_null_tail(version)
    assert suffix and shell.endswith(suffix), f'{version}; expected the known complete Shell continuation'
    base = shell[:-len(suffix)]
    return version, struct.pack('<i', 1) + base + mesh_record_suffix(version, 0)


def mesh_record_count_cap(count: int) -> tuple[int, bytes]:
    """Supply every declared complete direct record, including one above the cap; e.g. mesh_record_count_cap(10000)."""
    record = empty_shell_base(0) + mesh_record_suffix(0, 0)
    return 0, struct.pack('<i', count) + record * count


def mesh_cap_builders() -> tuple[tuple[str, int, BaseCapBuilder], ...]:
    """Keep root count separate from inherited reference/blob/pair/curve caps; e.g. mesh_cap_builders()."""
    from functools import partial
    return (('records', 10000, mesh_record_count_cap),
            ('references', 10000, partial(mesh_cap_from_base, reference_cap_literal)),
            ('blob', 1000000, partial(mesh_cap_from_base, blob_cap_literal)),
            ('curves', 50, partial(mesh_cap_from_base, curve_cap_literal)),
            ('optional_pairs', 100000, partial(mesh_cap_from_base, optional_cap_literal)))


def mesh_cap_field_count(name: str, version: int, raw: bytes, parsed: dict[str, object], count: int) -> int:
    """Count repeated records from a separately decoded single-record control; e.g. mesh_cap_field_count(name, version, raw, parsed, count)."""
    if name != 'records': return len(mesh_record_fields(raw, parsed))
    _, single = mesh_record_count_cap(1)
    first = actual_mesh_records(single, version)
    assert first['success'], first
    return 1 + count * (len(mesh_record_fields(single, first)) - 1)


def verify_mesh_record_caps() -> None:
    """Require sufficient complete payload and permissive caller budgets for each cap boundary; e.g. verify_mesh_record_caps()."""
    packet, rows = bytearray(), []
    for name, maximum, builder in mesh_cap_builders():
        for count in (maximum, maximum + 1):
            version, raw = builder(count)
            parsed = actual_mesh_records(raw, version)
            success = bool(parsed['success'])
            assert success == (count == maximum), (name, count, parsed)
            fields = mesh_cap_field_count(name, version, raw, parsed, count) if success else 0
            packet.extend(struct.pack('<IIIII', 0, version, int(success), fields, len(raw)) + raw)
            rows.append(dict(name=name, count=count, success=success, fields=fields, bytes=len(raw), sha256=digest(raw)))
    path = FEATURE.parents[2] / 'tests/fixtures/simulation-mesh-control/records-hardcaps.bin'
    if '--write' in sys.argv: path.write_bytes(packet)
    assert path.read_bytes() == packet, f'{path}; expected complete sufficient-payload controls'
    mesh_baseline('records-hardcap-reference.json', dict(cases=rows, contexts=len(rows), bytes=len(packet), sha256=digest(packet)))
