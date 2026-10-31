"""Exact seven-family C-739 identity and original-argument bindings (#154)."""
from pathlib import Path
from typing import TypedDict, cast
import hashlib
import json

RESEARCH = Path(__file__).resolve().parents[4] / 'inventor-interface/solidworks/research'
REPORTS = ('cosworks-solid-manager-native-evidence.json', 'cosworks-solid-component-native-evidence.json',
           'cosworks-solid-body-expanded-native-evidence.json', 'cosworks-sheet-property-final-native-evidence.json',
           'cosworks-material-corpus-native-evidence.json', 'cosworks-surface-body-native-evidence.json',
           'cosworks-component-persistid-native-evidence.json')


class SolidInventoryPayload(TypedDict):
    document: str
    stream: str
    size: int
    sha256: str
    family: int
    attempts: list[dict[str, object]]


def retained_solid_inventory() -> dict[str, object]:
    """Bind the entire original reconciliation and every dependency; e.g. retained_solid_inventory()."""
    report = json.loads((RESEARCH / 'cosworks-solid-inventory-reconciliation.json').read_bytes())
    assert report['evidence'] == 'C-739' and not report['target_executed']
    for original, expected in report['sources'].items():
        relative = original.replace('\\', '/').split('/solidworks/research/', 1)[1]
        assert hashlib.sha256((RESEARCH / relative).read_bytes()).hexdigest() == expected, relative
    return cast(dict[str, object], report)


def solid_payload_family(stream: str) -> int:
    """Reject unassigned storage shapes before any parser dispatch; e.g. solid_payload_family(stream)."""
    parts = stream.split('/SolidManager/', 1)[1].split('/')
    if parts == ['Property']: return 0
    assert parts[0] == 'Solid Components' and len(parts) >= 3, stream
    if len(parts) == 3 and parts[-1] == 'Property': return 1
    if len(parts) == 3 and parts[-1] == 'Component PersistID': return 6
    assert len(parts) == 5 and parts[2] in ('Solid Bodies', 'Sheet Bodies'), stream
    if parts[-1] == 'Material': return 4
    if parts[-1] == 'SurfaceBody': return 5
    assert parts[-1] == 'Property', stream
    return 2 if parts[2] == 'Solid Bodies' else 3


def exact_identity_rows(rows: list[dict[str, object]]) -> dict[tuple[str, str], dict[str, object]]:
    """Reject duplicates before dictionary insertion can hide them; e.g. exact_identity_rows(rows)."""
    identities: dict[tuple[str, str], dict[str, object]] = {}
    for row in rows:
        identity = (cast(str, row['path']), cast(str, row['stream']))
        assert identity not in identities, f'{identity}; expected unique stream identity'
        identities[identity] = row
    return identities


def complete_identity_match(expected: list[dict[str, object]], actual: list[dict[str, object]]) -> None:
    """Require exact names, sizes and hashes without missing/extra substitutions; e.g. complete_identity_match(rows, rows)."""
    expected_rows, actual_rows = exact_identity_rows(expected), exact_identity_rows(actual)
    assert expected_rows == actual_rows, 'inventory identities; expected exact one-to-one metadata equality'


def original_solid_rows() -> list[dict[str, object]]:
    """Preserve all original alternatives across the seven retained reports; e.g. original_solid_rows()."""
    rows: list[dict[str, object]] = []
    for name in REPORTS:
        report = json.loads((RESEARCH / name).read_bytes())
        assert not report['target_executed'], name
        for row in report['rows']:
            if '/SolidManager/' not in row['stream']: continue
            complete = any(attempt['success'] for attempt in row['attempts'])
            rows.append(dict(**row, evidence=report['evidence'], complete=complete))
    return rows


def solid_inventory_payloads() -> list[SolidInventoryPayload]:
    """Normalize the complete retained denominator once without choosing a successful version; e.g. solid_inventory_payloads()."""
    retained = retained_solid_inventory()
    rows = original_solid_rows()
    keys = ('path', 'stream', 'bytes', 'sha256', 'evidence', 'complete')
    complete_identity_match(cast(list[dict[str, object]], retained['rows']), [{key: row[key] for key in keys} for row in rows])
    assert len(rows) == 3439 and sum(cast(int, row['bytes']) for row in rows) == 4329610
    payloads = [SolidInventoryPayload(document=cast(str, row['path']), stream=cast(str, row['stream']),
                size=cast(int, row['bytes']), sha256=cast(str, row['sha256']),
                family=solid_payload_family(cast(str, row['stream'])),
                attempts=cast(list[dict[str, object]], row['attempts'])) for row in rows]
    return sorted(payloads, key=lambda payload: (payload['document'], payload['stream']))
