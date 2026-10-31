"""Exact C-746 identities and original argument bindings for the combined replay (#160)."""
from pathlib import Path
from typing import TypedDict, cast
import hashlib
import json
import sys

FEATURE = Path(__file__).resolve().parent
RESEARCH = FEATURE.parents[3] / 'inventor-interface/solidworks/research'
sys.path.insert(0, str(FEATURE.parent / 'simulation-solid-inventory'))
from solid_inventory_bindings import complete_identity_match

REPORTS = ('cosworks-shell-manager-native-evidence.json', 'cosworks-shell-property-native-evidence.json',
           'cosworks-midsurf-property-native-evidence.json', 'cosworks-material-corpus-native-evidence.json',
           'cosworks-composite-native-evidence.json')


class ShellInventoryPayload(TypedDict):
    document: str
    stream: str
    size: int
    sha256: str
    family: int
    attempts: list[dict[str, object]]


def retained_shell_inventory() -> dict[str, object]:
    """Bind the historical reconciliation without claiming fresh replay; e.g. retained_shell_inventory()."""
    report = json.loads((RESEARCH / 'cosworks-shell-inventory-reconciliation.json').read_bytes())
    assert report['evidence'] == 'C-746' and not report['target_executed'], report['evidence']
    for original, expected in report['sources'].items():
        relative = original.replace('\\', '/').split('/solidworks/research/', 1)[1]
        actual = hashlib.sha256((RESEARCH / relative).read_bytes()).hexdigest()
        assert actual == expected, f'{relative}: {actual}; expected retained hash {expected}'
    return cast(dict[str, object], report)


def shell_payload_family(stream: str) -> int:
    """Normalize only explicit persisted storage shapes; e.g. shell_payload_family('x/ShellManager/Property')."""
    assert stream.count('/ShellManager/') == 1, f'{stream}; expected one ShellManager storage'
    parts = stream.split('/ShellManager/', 1)[1].split('/')
    if parts == ['Property']: return 0
    if parts == ['ShellManagerNew', 'ShellManagerGroup']: return 1
    assert len(parts) == 3 and parts[0] in ('Shells', 'MidSurfShells') and parts[1], stream
    if parts[-1] == 'Property': return 2 if parts[0] == 'Shells' else 3
    if parts[-1] == 'Material': return 4
    assert parts[0] == 'Shells' and parts[-1] in ('CompositeFlipTurn', 'CompositeExtra'), stream
    return 5 if parts[-1] == 'CompositeFlipTurn' else 6


def original_shell_rows() -> list[dict[str, object]]:
    """Retain failed alternatives alongside accepted profiles; e.g. original_shell_rows()."""
    rows: list[dict[str, object]] = []
    for name in REPORTS:
        report = json.loads((RESEARCH / name).read_bytes())
        assert not report['target_executed'], f'{name}; expected retained static evidence'
        for row in report['rows']:
            if '/ShellManager/' not in row['stream']: continue
            successes = [attempt for attempt in row['attempts'] if attempt['success']]
            rows.append(dict(**row, evidence=report['evidence'], complete=bool(successes),
                             successful_profiles=sorted({attempt.get('wire_profile', 'reader') for attempt in successes})))
    return rows


def shell_inventory_payloads() -> list[ShellInventoryPayload]:
    """Normalize once without selecting a convenient successful attempt; e.g. shell_inventory_payloads()."""
    retained = retained_shell_inventory()
    rows = original_shell_rows()
    keys = ('path', 'stream', 'bytes', 'sha256', 'evidence', 'complete', 'successful_profiles')
    complete_identity_match(cast(list[dict[str, object]], retained['rows']), [{key: row[key] for key in keys} for row in rows])
    assert len(rows) == 936 and sum(cast(int, row['bytes']) for row in rows) == 1648966
    payloads = [ShellInventoryPayload(document=cast(str, row['path']), stream=cast(str, row['stream']),
                size=cast(int, row['bytes']), sha256=cast(str, row['sha256']),
                family=shell_payload_family(cast(str, row['stream'])),
                attempts=cast(list[dict[str, object]], row['attempts'])) for row in rows]
    return sorted(payloads, key=lambda payload: (payload['document'], payload['stream']))


def shell_inventory_identity(payload: ShellInventoryPayload) -> dict[str, object]:
    """Keep spelling and complete content identity unchanged; e.g. shell_inventory_identity(payload)."""
    return dict(path=payload['document'], stream=payload['stream'], bytes=payload['size'], sha256=payload['sha256'])


def shell_inventory_denominator() -> list[dict[str, object]]:
    """Require all original C-590 identities before replay; e.g. shell_inventory_denominator()."""
    corpus = json.loads((RESEARCH / 'third-party-storage-corpus.json').read_bytes())
    identities = [dict(path=sample['path'], stream=row['name'], bytes=row['bytes'], sha256=row['sha256'])
                  for sample in corpus['samples'] for row in sample['nested_streams'] if '/ShellManager/' in row['name']]
    complete_identity_match(identities, [shell_inventory_identity(payload) for payload in shell_inventory_payloads()])
    return identities
