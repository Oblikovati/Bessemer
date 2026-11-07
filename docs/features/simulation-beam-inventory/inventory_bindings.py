"""Exact retained BeamManager identity reconciliation before any fresh payload replay (#146)."""
from pathlib import Path
from typing import TypedDict, cast
import hashlib
import json

RESEARCH = Path(__file__).resolve().parents[4] / 'inventor-interface/solidworks/research'
REPORTS = ('cosworks-manager-property-native-evidence.json', 'cosworks-material-corpus-native-evidence.json',
           'cosworks-beam-property-native-evidence.json', 'cosworks-beam-group-native-evidence.json')


class BoundPayload(TypedDict):
    document: str
    stream: str
    size: int
    sha256: str
    family: int
    expected_joint_count: int | None
    attempts: list[dict[str, object]]


def retained_inventory() -> dict[str, object]:
    """Require every original reconciliation dependency to remain exact; e.g. retained_inventory()."""
    retained = json.loads((RESEARCH / 'cosworks-beam-inventory-reconciliation.json').read_bytes())
    assert retained['evidence'] == 'C-727' and not retained['target_executed']
    for original, expected in retained['sources'].items():
        relative = original.replace('\\', '/').split('/solidworks/research/', 1)[1]
        assert hashlib.sha256((RESEARCH / relative).read_bytes()).hexdigest() == expected
    return cast(dict[str, object], retained)


def family_for(stream: str) -> int:
    """Reject unassigned payload paths instead of silently excluding them; e.g. family_for(stream)."""
    suffix = stream.split('/BeamManager/', 1)[1]
    if suffix == 'Property': return 0
    if suffix.startswith('Beams/') and suffix.endswith('/Property'): return 1
    if suffix.startswith('Beams/') and suffix.endswith('/Material'): return 2
    terminal = suffix.rsplit('/', 1)[-1]
    assert suffix.startswith('JointGroups/') and terminal in ('JointGroup', 'JointList', 'Joint'), stream
    return {'JointGroup': 3, 'JointList': 4, 'Joint': 5}[terminal]


def inventory_identities() -> dict[tuple[str, str], dict[str, object]]:
    """Keep the complete C-590 denominator without overwriting duplicate identities; e.g. inventory_identities()."""
    inventory = json.loads((RESEARCH / 'third-party-storage-corpus.json').read_bytes())
    identities: dict[tuple[str, str], dict[str, object]] = {}
    for sample in inventory['samples']:
        for stream in sample['nested_streams']:
            if '/BeamManager/' not in stream['name']: continue
            identity = (sample['path'], stream['name'])
            assert identity not in identities, identity
            identities[identity] = stream
    assert len(identities) == 1080
    return identities


def bound_payload(row: dict[str, object]) -> BoundPayload:
    """Normalize each persisted identity and family exactly once; e.g. bound_payload(row)."""
    return BoundPayload(document=cast(str, row['path']), stream=cast(str, row['stream']),
                        size=cast(int, row['bytes']), sha256=cast(str, row['sha256']),
                        family=family_for(cast(str, row['stream'])),
                        expected_joint_count=cast(int | None, row.get('expected_joint_count')),
                        attempts=cast(list[dict[str, object]], row['attempts']))


def reconcile() -> list[BoundPayload]:
    """Require one complete binding per original identity and preserve every argument; e.g. reconcile()."""
    retained = retained_inventory()
    identities = inventory_identities()
    bindings: dict[tuple[str, str], dict[str, object]] = {}
    payloads: list[BoundPayload] = []
    for name in REPORTS:
        report = json.loads((RESEARCH / name).read_bytes())
        for row in report['rows']:
            if '/BeamManager/' not in row['stream']: continue
            identity = (row['path'], row['stream'])
            assert identity in identities and identity not in bindings, identity
            original = identities[identity]
            assert (row['bytes'], row['sha256']) == (original['bytes'], original['sha256'])
            bindings[identity] = {key: row[key] for key in ('path', 'stream', 'bytes', 'sha256')}
            bindings[identity]['evidence'] = report['evidence']
            payloads.append(bound_payload(row))
    assert set(bindings) == set(identities)
    assert sorted(bindings.values(), key=lambda row: (row['path'], row['stream'])) == sorted(cast(list[dict[str, object]], retained['bindings']), key=lambda row: (row['path'], row['stream']))
    assert sum(payload['size'] for payload in payloads) == 620654
    return sorted(payloads, key=lambda payload: (payload['document'], payload['stream']))
