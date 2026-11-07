"""Exact graph inventory and full original native argument replay (#165)."""
from pathlib import Path
from typing import cast
import importlib
import json
import re
import struct
from native_carriers import RESEARCH, digest, recover_sample
from solid_inventory_bindings import complete_identity_match
from graph_evidence import FEATURE, READER, graph_report, graph_baseline


def graph_stream_guard(stream: str) -> None:
    """Require the complete graph storage shape before parser selection; e.g. graph_stream_guard('Cosworks/StudyManager/S/Results/C/P/cwPostGraphParam')."""
    expected = r'[^/]+/StudyManager/[^/]+/Results/[^/]+/[^/]+/cwPostGraphParam'
    assert re.fullmatch(expected, stream), f'{stream}; expected complete Results category/plot graph path'


def verify_graph_inventory() -> None:
    """Match every original occurrence with complete independent corpus metadata; e.g. verify_graph_inventory()."""
    corpus = json.loads((RESEARCH / 'third-party-storage-corpus.json').read_bytes())
    expected = [dict(path=sample['path'], stream=row['name'], bytes=row['bytes'], sha256=row['sha256'])
                for sample in corpus['samples'] for row in sample['nested_streams']
                if '/Results/' in row['name'] and row['name'].endswith('/cwPostGraphParam')]
    rows = cast(list[dict[str, object]], graph_report()['rows'])
    actual = [{key: row[key] for key in ('path', 'stream', 'bytes', 'sha256')} for row in rows]
    complete_identity_match(expected, actual)
    for row in rows: graph_stream_guard(cast(str, row['stream']))
    assert len(rows) == 224 and sum(cast(int, row['bytes']) for row in rows) == 83328
    graph_baseline('identity-reference.json', actual)


def graph_native_row(row: dict[str, object], streams: dict[str, bytes]) -> bytes:
    """Require every complete original outcome, including text bytes and failed traces; e.g. graph_native_row(row, streams)."""
    raw = streams[cast(str, row['stream'])]
    assert len(raw) == row['bytes'] and digest(raw) == row['sha256'], row['stream']
    reader = importlib.import_module(READER)
    packet = bytearray()
    for expected in cast(list[dict[str, object]], row['attempts']):
        version = cast(int, expected['version'])
        result = reader.parse(raw, version)
        assert dict(version=version, **result) == expected, (row['stream'], version)
        packet.extend(struct.pack('<III', version, int(result['success']), len(raw)) + raw)
    return bytes(packet)


def replay_graph_native(pin: Path) -> None:
    """Recover all source envelopes before complete version-candidate replay; e.g. replay_graph_native(pin)."""
    rows = cast(list[dict[str, object]], graph_report()['rows'])
    packet = bytearray()
    for document in sorted({cast(str, row['path']) for row in rows}):
        streams = recover_sample(document, pin)
        for row in rows:
            if row['path'] == document: packet.extend(graph_native_row(row, streams))
    attempts = [attempt for row in rows for attempt in cast(list[dict[str, object]], row['attempts'])]
    accepted = sum(cast(bool, attempt['success']) for attempt in attempts)
    (FEATURE.parents[2] / 'target/simulation-post-graph-native.bin').write_bytes(packet)
    graph_baseline('native-reference.json', dict(streams=len(rows), logical_bytes=83328,
        arguments=len(attempts), accepted=accepted, failed=len(attempts) - accepted,
        bytes=len(packet), sha256=digest(packet)))


def verify_graph_path_guards() -> None:
    """Keep named studies and repeated Results names without basename guessing; e.g. verify_graph_path_guards()."""
    graph_stream_guard('Cosworks/StudyManager/SimStudy-7/Results/Results/Results/cwPostGraphParam')
    for stream in ('cwPostGraphParam', 'Cosworks/StudyManager/S/Results/cwPostGraphParam',
                   'Cosworks/StudyManager/S/Results/C/P/Q/cwPostGraphParam'):
        try: graph_stream_guard(stream)
        except AssertionError: continue
        raise AssertionError(f'{stream}; expected rejection outside complete plot graph shape')
