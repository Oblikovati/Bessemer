"""Owned/native counted-record boundaries and complete operation packets (#161)."""
from pathlib import Path
from typing import cast
import struct
import sys
from native_carriers import digest, recover_sample
from mesh_control_evidence import FEATURE, mesh_baseline, mesh_reports
from owned_mesh_records import OwnedMeshRecords, owned_mesh_records, mesh_collection_literal, mesh_empty_curve_mode
from mesh_record_projection import mesh_record_projection, mesh_record_span_shape, mesh_span_owner
from mesh_record_history import mesh_record_history
from mesh_record_metrics import mesh_record_budget_packet

ROOT = FEATURE.parents[2]


def actual_mesh_records(raw: bytes, version: int) -> dict[str, object]:
    """Use the copied reader with its exact scoped embedded callback; e.g. actual_mesh_records(raw, 840002)."""
    from cosworks_mesh_control_layout import parse
    from cosworks_embedded_const_surface import archive_reader
    return cast(dict[str, object], parse(raw, version, 'MeshControl', archive_reader=archive_reader))


def mesh_record_operation_packets(version: int, raw: bytes, parsed: dict[str, object]) -> tuple[bytes, bytes, bytes, bytes]:
    """Keep complete input, primitive and independently reread history expectations together; e.g. mesh_record_operation_packets(version, raw, parsed)."""
    assert parsed['success'] and parsed['end'] == len(raw), parsed
    return (struct.pack('<III', version, 1, len(raw)) + raw,
            mesh_record_projection(version, raw, parsed), mesh_record_history(version, raw, parsed), mesh_record_budget_packet(version, raw, parsed))


def require_owned_mesh_record_boundaries(case: OwnedMeshRecords, parsed: dict[str, object]) -> None:
    """Check independently constructed extents and every source span without byte gaps; e.g. require_owned_mesh_record_boundaries(case, parsed)."""
    assert parsed['success'] and parsed['end'] == len(case['raw']), (case['version'], case['profile'], parsed)
    assert parsed['records'] == case['records'], (case['version'], case['profile'], parsed['records'])
    offset = 0
    for span in cast(list[dict[str, object]], parsed['spans']):
        start, end = cast(int, span['offset']), cast(int, span['end'])
        assert start == offset and start <= end <= len(case['raw']), span
        assert digest(case['raw'][start:end]) == span['sha256'], span
        offset = end
    assert offset == len(case['raw']), f'{offset}; expected complete {len(case["raw"])} byte coverage'
    assert parsed['reconstruction_sha256'] == digest(case['raw'])


def verify_owned_mesh_record_packets() -> None:
    """Freeze every empty/single/mixed/repeated context with complete operation oracles; e.g. verify_owned_mesh_record_packets()."""
    packets = [bytearray(), bytearray(), bytearray(), bytearray()]
    cases = owned_mesh_records()
    for case in cases:
        parsed = actual_mesh_records(case['raw'], case['version'])
        require_owned_mesh_record_boundaries(case, parsed)
        fragments = mesh_record_operation_packets(case['version'], case['raw'], parsed)
        for packet, fragment in zip(packets, fragments, strict=True): packet.extend(fragment)
    references = save_owned_record_packets(packets)
    mesh_baseline('records-owned-reference.json', dict(cases=len(cases), versions=len({case['version'] for case in cases}),
                  records=sum(len(case['records']) for case in cases), empty_collections=sum(not case['records'] for case in cases),
                  logical_bytes=sum(len(case['raw']) for case in cases), packets=references))


def save_owned_record_packets(packets: list[bytearray]) -> dict[str, object]:
    """Preserve all reviewed owned packet bytes in ordinary runs; e.g. save_owned_record_packets(packets)."""
    folder = ROOT / 'tests/fixtures/simulation-mesh-control'
    references: dict[str, object] = {}
    for name, raw in zip(('input', 'projection', 'history', 'budgets'), packets, strict=True):
        path = folder / f'records-{name}.bin'
        if '--write' in sys.argv:
            folder.mkdir(parents=True, exist_ok=True)
            path.write_bytes(raw)
        assert path.read_bytes() == raw, f'{path}; expected complete unchanged owned record packet'
        references[name] = dict(bytes=len(raw), sha256=digest(raw))
    return references


def native_mesh_record_row(row: dict[str, object], streams: dict[str, bytes]) -> tuple[bytes, bytes, bytes, bytes]:
    """Keep every original record candidate and scoped occurrence; e.g. native_mesh_record_row(row, streams)."""
    raw = streams[cast(str, row['stream'])]
    assert len(raw) == row['bytes'] and digest(raw) == row['sha256'], row['stream']
    packets = [bytearray(), bytearray(), bytearray(), bytearray()]
    for expected in cast(list[dict[str, object]], row['attempts']):
        version = cast(int, expected['version'])
        parsed = actual_mesh_records(raw, version)
        assert dict(version=version, **parsed) == expected, (row['stream'], version)
        assert parsed['success'], (row['stream'], version, parsed)
        fragments = mesh_record_operation_packets(version, raw, parsed)
        for packet, fragment in zip(packets, fragments, strict=True): packet.extend(fragment)
    return bytes(packets[0]), bytes(packets[1]), bytes(packets[2]), bytes(packets[3])


def replay_native_mesh_record_packets(pin: Path) -> None:
    """Recover all twelve native streams and every edited/reference parent; e.g. replay_native_mesh_record_packets(pin)."""
    rows = cast(list[dict[str, object]], mesh_reports()[1]['rows'])
    packets = [bytearray(), bytearray(), bytearray(), bytearray()]
    for document in sorted({cast(str, row['path']) for row in rows}):
        streams = recover_sample(document, pin)
        fragments = [native_mesh_record_row(row, streams) for row in rows if row['path'] == document]
        for index, packet in enumerate(packets): packet.extend(b''.join(fragment[index] for fragment in fragments))
    references: dict[str, object] = {}
    for name, raw in zip(('input', 'projection', 'history', 'budgets'), packets, strict=True):
        (ROOT / f'target/simulation-mesh-control-native-records-{name}.bin').write_bytes(raw)
        references[name] = dict(bytes=len(raw), sha256=digest(raw))
    mesh_baseline('records-native-packets.json', dict(streams=12, arguments=12, records=16, packets=references))


def verify_mesh_record_factory_rejections() -> None:
    """Reject unknown profiles before constructing a body; e.g. verify_mesh_record_factory_rejections()."""
    for profile in (-1, 5, 2147483647):
        try: mesh_collection_literal(840002, profile)
        except AssertionError: continue
        raise AssertionError(f'{profile}; expected an owned MeshControl profile rejection')


def verify_mesh_record_scope_rejections() -> None:
    """Reject unknown physical shapes, wrong widths and missing/overlapping owners; e.g. verify_mesh_record_scope_rejections()."""
    for label, size in (('mesh.unknown', 4), ('mesh.two_doubles_three_words', 27), ('base.six_doubles', 47)):
        try: mesh_record_span_shape(label, size, 'record0')
        except AssertionError: continue
        raise AssertionError(f'{label}/{size}; expected a documented complete primitive shape')
    for records in ([], [dict(index=0, start=4, end=12), dict(index=1, start=8, end=16)]):
        try: mesh_span_owner(9, records)
        except AssertionError: continue
        raise AssertionError(f'{records}; expected exactly one physical owner')
    for version, mode in ((890010, 0), (890111, 2147483648), (890111, -2147483649)):
        try: mesh_empty_curve_mode(version, mode)
        except AssertionError: continue
        raise AssertionError(f'{version}/{mode}; expected a valid table version and signed mode')
