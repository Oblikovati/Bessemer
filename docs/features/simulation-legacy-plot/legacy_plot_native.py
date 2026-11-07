"""Replay every named historical plot and keep rejected caller contexts separate (#174)."""
from pathlib import Path
from typing import cast
import importlib
import json
import struct
from operator import itemgetter
from native_carriers import RESEARCH, digest, recover_sample
from dialog_legacy_carriers import legacy_dialog_versions
from legacy_plot_evidence import FEATURE, legacy_plot_baseline, legacy_plot_reports


def is_named_plot_stream(name: str) -> bool:
    """Recognize the complete historical carrier shape once; e.g. is_named_plot_stream('Cosworks/S/C/P/P')."""
    components = name.split('/')
    return len(components) == 5 and components[0] == 'Cosworks' and components[-1] == components[-2]


def legacy_plot_inventory() -> list[dict[str, object]]:
    """Select all sixty original named streams without endpoint filtering; e.g. legacy_plot_inventory()."""
    corpus = json.loads((RESEARCH / 'third-party-storage-corpus.json').read_bytes())
    return [dict(path=sample['path'], stream=row['name'], bytes=row['bytes'], sha256=row['sha256'])
        for sample in corpus['samples'] for row in sample['nested_streams'] if is_named_plot_stream(row['name'])]


def verify_legacy_plot_inventory() -> None:
    """Match every original identity and full stored extent; e.g. verify_legacy_plot_inventory()."""
    inventory = legacy_plot_inventory()
    report, _ = legacy_plot_reports()
    original = cast(list[dict[str, object]], report['rows'])
    assert inventory == [{key: row[key] for key in ('path', 'stream', 'bytes', 'sha256')} for row in original]
    assert len(inventory) == 60 and sum(cast(int, row['bytes']) for row in inventory) == 81744


def replay_legacy_plot_row(row: dict[str, object], streams: dict[str, bytes], versions: list[int], bindings: list[dict[str, object]]) -> dict[str, object]:
    """Try every supplied candidate on the complete original payload; e.g. replay_legacy_plot_row(row, streams, versions, bindings)."""
    raw = streams[cast(str, row['stream'])]
    assert len(raw) == row['bytes'] and digest(raw) == row['sha256'], row['stream']
    parse = importlib.import_module('cosworks_legacy_plot_record').parse
    attempts = [dict(version=version, **parse(raw, version)) for version in versions]
    return dict(**row, wire_hex=raw.hex(), candidate_sources=bindings, attempts=attempts)


def legacy_plot_native_rows(pin: Path) -> list[dict[str, object]]:
    """Recover documents through frozen adapters before checking original outcomes; e.g. legacy_plot_native_rows(pin)."""
    inventory = legacy_plot_inventory()
    rows: list[dict[str, object]] = []
    for document in sorted({cast(str, row['path']) for row in inventory}):
        streams = recover_sample(document, pin)
        versions, bindings = legacy_dialog_versions(document, streams)
        rows.extend(replay_legacy_plot_row(row, streams, versions, bindings) for row in inventory if row['path'] == document)
    report, _ = legacy_plot_reports()
    actual = [{key: row[key] for key in ('path', 'stream', 'bytes', 'sha256', 'attempts')} for row in rows]
    order = itemgetter('path', 'stream')
    assert sorted(actual, key=order) == sorted(cast(list[dict[str, object]], report['rows']), key=order)
    return rows


def legacy_plot_native_packet(rows: list[dict[str, object]]) -> bytes:
    """Retain full payloads and endpoints for every candidate, including failures; e.g. legacy_plot_native_packet(rows)."""
    packet = bytearray()
    for row in rows:
        raw = bytes.fromhex(cast(str, row['wire_hex']))
        for attempt in cast(list[dict[str, object]], row['attempts']):
            packet.extend(struct.pack('<5I', cast(int, attempt['version']), int(cast(bool, attempt['success'])),
                int(cast(bool, attempt['named_stream_dispatch_enabled'])), cast(int, attempt['end']), len(raw)))
            packet.extend(raw)
    return bytes(packet)


def verify_legacy_wide_replay(rows: list[dict[str, object]]) -> None:
    """Require all240 full buffer, prefix and original-suffix views; e.g. verify_legacy_wide_replay(rows)."""
    _, report = legacy_plot_reports()
    model = importlib.import_module('cosworks_legacy_plot_record')
    observed: list[dict[str, object]] = []
    for row in rows:
        successful = [attempt for attempt in cast(list[dict[str, object]], row['attempts'])
            if attempt['success'] and cast(int, attempt['version']) >= 700004]
        if not successful: continue
        attempt = successful[0]
        observed.append({key: row[key] for key in ('path', 'stream', 'sha256')} | dict(version=attempt['version'],
            named_stream_dispatch_enabled=attempt['named_stream_dispatch_enabled'],
            views=model.fixed_wide_views(attempt, version=attempt['version'])))
    order = itemgetter('path', 'stream')
    assert sorted(observed, key=order) == sorted(cast(list[dict[str, object]], report['rows']), key=order)
    assert report['counts'] == dict(buffers=240, terminated=240, nonzero_bytes_after_first_nul=240)


def replay_legacy_plot_native(pin: Path) -> None:
    """Publish only exact retained helper outcomes, never inferred caller success; e.g. replay_legacy_plot_native(pin)."""
    rows = legacy_plot_native_rows(pin)
    verify_legacy_wide_replay(rows)
    packet = legacy_plot_native_packet(rows)
    (FEATURE.parents[2] / 'target/simulation-legacy-plot-native.bin').write_bytes(packet)
    report, _ = legacy_plot_reports()
    legacy_plot_baseline('native-reference.json', dict(summary=report['summary'], rows=rows,
        packet_bytes=len(packet), packet_sha256=digest(packet), target_executed=False))
