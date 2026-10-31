"""Exact Results Property inventory and fresh original candidate replay by hierarchy level (#164)."""
from pathlib import Path
from typing import cast
import importlib
import json
import struct
from native_carriers import RESEARCH, digest, recover_sample
from solid_inventory_bindings import complete_identity_match
from results_evidence import FEATURE, READERS, results_baseline, results_reports, results_level


def actual_results(raw: bytes, version: int, level: int) -> dict[str, object]:
    """Dispatch only a normalized explicit level to its copied standalone reader; e.g. actual_results(raw, 890605, 0)."""
    assert type(level) is int and 0 <= level < 3, f'{level}; expected Results level 0..2'
    module = importlib.import_module(READERS[level])
    return cast(dict[str, object], module.parse(raw, version))


def verify_results_inventory() -> None:
    """Match all 450 original occurrences without merging distinct Property levels; e.g. verify_results_inventory()."""
    corpus = json.loads((RESEARCH / 'third-party-storage-corpus.json').read_bytes())
    identities = [dict(path=sample['path'], stream=row['name'], bytes=row['bytes'], sha256=row['sha256'])
                  for sample in corpus['samples'] for row in sample['nested_streams']
                  if '/Results/' in row['name'] and row['name'].endswith('/Property')]
    for level, report in enumerate(results_reports()):
        expected = [row for row in identities if results_level(row['stream']) == level]
        actual = [{key: row[key] for key in ('path', 'stream', 'bytes', 'sha256')}
                  for row in cast(list[dict[str, object]], report['rows'])]
        complete_identity_match(expected, actual)
    assert len(identities) == 450 and sum(row['bytes'] for row in identities) == 10956
    results_baseline('identity-reference.json', identities)


def results_native_row(row: dict[str, object], streams: dict[str, bytes], level: int) -> bytes:
    """Require every original full outcome, including failure traces and values; e.g. results_native_row(row, streams, 1)."""
    assert results_level(cast(str, row['stream'])) == level, row['stream']
    raw = streams[cast(str, row['stream'])]
    assert len(raw) == row['bytes'] and digest(raw) == row['sha256'], row['stream']
    packet = bytearray()
    for expected in cast(list[dict[str, object]], row['attempts']):
        version = cast(int, expected['version'])
        parsed = actual_results(raw, version, level)
        assert dict(version=version, **parsed) == expected, (row['stream'], version)
        packet.extend(struct.pack('<IIII', level, version, int(cast(bool, parsed['success'])), len(raw)) + raw)
    return bytes(packet)


def replay_results_native(pin: Path) -> None:
    """Recover each complete source envelope before independent level-specific replay; e.g. replay_results_native(pin)."""
    reports = results_reports()
    packets = [bytearray(), bytearray(), bytearray()]
    documents = sorted({cast(str, row['path']) for report in reports for row in cast(list[dict[str, object]], report['rows'])})
    for document in documents:
        streams = recover_sample(document, pin)
        for level, report in enumerate(reports):
            for row in cast(list[dict[str, object]], report['rows']):
                if row['path'] == document: packets[level].extend(results_native_row(row, streams, level))
    summaries = results_native_summaries(reports, packets)
    packet = b''.join(packets)
    (FEATURE.parents[2] / 'target/simulation-results-headers-native.bin').write_bytes(packet)
    results_baseline('native-reference.json', dict(levels=summaries, bytes=len(packet), sha256=digest(packet)))


def results_native_summaries(reports: list[dict[str, object]], packets: list[bytearray]) -> list[dict[str, object]]:
    """Retain each denominator and failed candidate rather than only successful versions; e.g. results_native_summaries(reports, packets)."""
    summaries = []
    expected = ((53, 2192, 58, 53, 5), (185, 1652, 196, 185, 11), (212, 7112, 227, 212, 15))
    for level, (report, packet) in enumerate(zip(reports, packets, strict=True)):
        rows = cast(list[dict[str, object]], report['rows'])
        attempts = [attempt for row in rows for attempt in cast(list[dict[str, object]], row['attempts'])]
        accepted = sum(cast(bool, attempt['success']) for attempt in attempts)
        assert (len(rows), sum(cast(int, row['bytes']) for row in rows), len(attempts), accepted, len(attempts) - accepted) == expected[level]
        summaries.append(dict(level=level, streams=len(rows), logical_bytes=expected[level][1],
            arguments=len(attempts), accepted=accepted, failed=len(attempts) - accepted,
            bytes=len(packet), sha256=digest(packet)))
    return summaries
