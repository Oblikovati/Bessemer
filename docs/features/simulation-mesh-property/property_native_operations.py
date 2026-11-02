"""Every accepted native Mesh Property operation, retaining original failed alternatives separately (#163)."""
from pathlib import Path
from typing import cast
from native_carriers import digest, recover_sample
from property_evidence import FEATURE, property_baseline, property_reports
from property_native import actual_property
from property_projection import property_projection
from property_history import property_history
from property_metrics import property_budget_packet


def native_property_operations(row: dict[str, object], raw: bytes) -> tuple[bytes, bytes, bytes]:
    """Project every original accepted argument without silently replacing failed candidates; e.g. native_property_operations(row, raw)."""
    assert len(raw) == row['bytes'] and digest(raw) == row['sha256'], row['stream']
    projection, history, budgets = bytearray(), bytearray(), bytearray()
    for expected in cast(list[dict[str, object]], row['attempts']):
        version = cast(int, expected['version'])
        parsed = actual_property(raw, version)
        assert dict(version=version, **parsed) == expected, (row['stream'], version)
        if not parsed['success']: continue
        projection.extend(property_projection(version, raw, parsed))
        history.extend(property_history(version, raw))
        budgets.extend(property_budget_packet(version, raw, parsed))
    return bytes(projection), bytes(history), bytes(budgets)


def replay_native_property_operations(pin: Path) -> None:
    """Recover full carriers again before independent primitive/history expansion; e.g. replay_native_property_operations(pin)."""
    rows = cast(list[dict[str, object]], property_reports()[-1]['rows'])
    projection, history, budgets = bytearray(), bytearray(), bytearray()
    for document in sorted({cast(str, row['path']) for row in rows}):
        streams = recover_sample(document, pin)
        for row in (row for row in rows if row['path'] == document):
            fields, edits, bounds = native_property_operations(row, streams[cast(str, row['stream'])])
            projection.extend(fields)
            history.extend(edits)
            budgets.extend(bounds)
    references: dict[str, object] = {}
    for name, packet in (('projection', projection), ('history', history), ('budgets', budgets)):
        (FEATURE.parents[2] / f'target/simulation-mesh-property-native-{name}.bin').write_bytes(packet)
        references[name] = dict(bytes=len(packet), sha256=digest(packet))
    property_baseline('native-operations-reference.json', dict(streams=357, accepted_arguments=357,
                      original_arguments=391, original_failed_arguments=34, packets=references))
