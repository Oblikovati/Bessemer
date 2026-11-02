"""Every original Mesh Property candidate, with complete final fields and explicit historical replay (#163)."""
from pathlib import Path
from typing import cast
import json
import struct
from native_carriers import RESEARCH, digest, recover_sample
from solid_inventory_bindings import complete_identity_match
from property_evidence import FEATURE, property_baseline, property_reports


def actual_property(raw: bytes, version: int) -> dict[str, object]:
    """Use the copied complete reader and established scoped embedded callback; e.g. actual_property(raw, 891107)."""
    from cosworks_mesh_layout import parse
    from cosworks_embedded_const_surface import archive_reader
    return cast(dict[str, object], parse(raw, version, 'Mesh', archive_reader=archive_reader))


def historical_property_view(actual: dict[str, object]) -> dict[str, object]:
    """Preserve old native observations while recognizing only the documented empty-set correction; e.g. historical_property_view(actual)."""
    if not actual['success']: return actual
    assert actual.get('newest_keys') in (None, []), actual.get('newest_keys')
    older = dict(actual)
    older.pop('newest_keys')
    spans = []
    for span in cast(list[dict[str, object]], older['spans']):
        if span['label'] == 'mesh.newest_set.keys':
            assert span['offset'] == span['end'] and span['sha256'] == digest(b''), span
            continue
        label = 'mesh.newest_map.count' if span['label'] == 'mesh.newest_set.count' else span['label']
        spans.append(dict(span, label=label))
    older['spans'] = spans
    return older


def verify_property_inventory() -> None:
    """Match every retained Mesh stream identity to every original report; e.g. verify_property_inventory()."""
    corpus = json.loads((RESEARCH / 'third-party-storage-corpus.json').read_bytes())
    identities = [dict(path=sample['path'], stream=row['name'], bytes=row['bytes'], sha256=row['sha256'])
                  for sample in corpus['samples'] for row in sample['nested_streams']
                  if row['name'].endswith('/Mesh/Property')]
    for report in property_reports():
        actual = [dict(path=row['path'], stream=row.get('stream', row.get('name')), bytes=row['bytes'], sha256=row['sha256'])
                  for row in cast(list[dict[str, object]], report['rows'])]
        complete_identity_match(identities, actual)
    assert len(identities) == 357 and sum(row['bytes'] for row in identities) == 94876
    property_baseline('identity-reference.json', identities)


def property_native_row(row: dict[str, object], streams: dict[str, bytes]) -> bytes:
    """Require every complete final field and failed alternative unchanged; e.g. property_native_row(row, streams)."""
    raw = streams[cast(str, row['stream'])]
    assert len(raw) == row['bytes'] and digest(raw) == row['sha256'], row['stream']
    packet = bytearray()
    for expected in cast(list[dict[str, object]], row['attempts']):
        version = cast(int, expected['version'])
        parsed = actual_property(raw, version)
        assert dict(version=version, **parsed) == expected, (row['stream'], version)
        packet.extend(struct.pack('<III', version, int(cast(bool, parsed['success'])), len(raw)) + raw)
    return bytes(packet)


def verify_property_historical_replay() -> None:
    """Preserve both earlier full native reports, including all failed traces; e.g. verify_property_historical_replay()."""
    reports = property_reports()
    latest = cast(list[dict[str, object]], reports[-1]['rows'])
    for report in reports[1:-1]:
        for older, current in zip(cast(list[dict[str, object]], report['rows']), latest, strict=True):
            assert {key: value for key, value in older.items() if key != 'attempts'} == {
                key: value for key, value in current.items() if key != 'attempts'}
            for old, new in zip(cast(list[dict[str, object]], older['attempts']),
                                cast(list[dict[str, object]], current['attempts']), strict=True):
                assert old == historical_property_view(new), (report['evidence'], older['stream'], old['version'])


def verify_property_historical_guards() -> None:
    """Reject populated keys or a nonempty span disguised as the historical empty suffix; e.g. verify_property_historical_guards()."""
    bad_span = dict(label='mesh.newest_set.keys', offset=0, end=4, sha256=digest(b'\0' * 4))
    controls = (dict(success=True, newest_keys=[1]),
                dict(success=True, newest_keys=[], spans=[bad_span]))
    for actual in controls:
        try: historical_property_view(actual)
        except AssertionError: continue
        raise AssertionError(f'{actual}; expected rejection outside the documented empty native suffix')


def replay_property_native(pin: Path) -> None:
    """Recover complete outer/nested carriers before replaying all 391 original arguments; e.g. replay_property_native(pin)."""
    rows = cast(list[dict[str, object]], property_reports()[-1]['rows'])
    packet = bytearray()
    for document in sorted({cast(str, row['path']) for row in rows}):
        streams = recover_sample(document, pin)
        for row in (row for row in rows if row['path'] == document): packet.extend(property_native_row(row, streams))
    attempts = [attempt for row in rows for attempt in cast(list[dict[str, object]], row['attempts'])]
    assert len(attempts) == 391 and sum(cast(bool, attempt['success']) for attempt in attempts) == 357
    (FEATURE.parents[2] / 'target/simulation-mesh-property-native.bin').write_bytes(packet)
    property_baseline('native-reference.json', dict(streams=len(rows), logical_bytes=94876, arguments=len(attempts),
                      accepted=357, failed=34, bytes=len(packet), sha256=digest(packet)))
