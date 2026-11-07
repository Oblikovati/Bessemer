"""Complete extra legacy-location dialog inventory with explicit root version evidence (#166)."""
from pathlib import Path
from typing import cast
import importlib
import json
import struct
from native_carriers import RESEARCH, digest, recover_sample
from dialog_evidence import FEATURE, dialog_baseline
from dialog_native import dialog_path_family


def legacy_dialog_inventory() -> list[dict[str, object]]:
    """Retain all 251 additional carriers outside C-770's modern denominator; e.g. legacy_dialog_inventory()."""
    corpus = json.loads((RESEARCH / 'third-party-storage-corpus.json').read_bytes())
    return [dict(path=sample['path'], stream=row['name'], bytes=row['bytes'], sha256=row['sha256'])
            for sample in corpus['samples'] for row in sample['nested_streams']
            if row['name'].endswith('/cwPostDlgParam') and dialog_path_family(row['name']) == 3]


def legacy_dialog_versions(document: str, streams: dict[str, bytes]) -> tuple[list[int], list[dict[str, object]]]:
    """Read only hash-bound primary/coexisting root candidates without selecting one to fit; e.g. legacy_dialog_versions(document, streams)."""
    from cosworkspersist import read_text
    corpus = json.loads((RESEARCH / 'third-party-storage-corpus.json').read_bytes())
    sample = next(row for row in corpus['samples'] if row['path'] == document)
    versions: set[int] = set()
    bindings: list[dict[str, object]] = []
    for suffix in ('CosworksVersoin', 'CosworksVersoinNumber'):
        name = 'Cosworks/' + suffix
        if name not in streams: continue
        raw = streams[name]
        expected = next(row for row in sample['nested_streams'] if row['name'] == name)
        assert len(raw) == expected['bytes'] and digest(raw) == expected['sha256'], (document, name)
        text = read_text(raw).text if suffix == 'CosworksVersoin' else raw[:12].decode('utf-16le')
        assert text and text.isascii() and text.isdigit(), (document, name, text)
        versions.add(int(text))
        bindings.append(dict(stream=name, sha256=digest(raw), version=int(text)))
    assert versions, f'{document}; expected explicit retained root version candidates'
    return sorted(versions), bindings


def legacy_dialog_attempt(raw: bytes, version: int) -> dict[str, object]:
    """Keep disabled/minimum-version contexts distinct from complete helper parsing; e.g. legacy_dialog_attempt(raw, 700338)."""
    parse = importlib.import_module('cosworks_post_dialog_layout').parse
    if version >= 700335: return dict(version=version, named_dialog_dispatch_enabled=True, **parse(raw, version))
    try: parse(raw, version)
    except ValueError as error:
        return dict(version=version, named_dialog_dispatch_enabled=False, success=False, complete=False,
                    end=0, failure_class='unsupported_reader_version', error=str(error))
    raise AssertionError(f'{version}; expected explicit dialog minimum-version rejection')


def replay_legacy_dialog_row(row: dict[str, object], streams: dict[str, bytes], versions: list[int], bindings: list[dict[str, object]]) -> dict[str, object]:
    """Observe every complete original payload under every retained candidate; e.g. replay_legacy_dialog_row(row, streams, versions, bindings)."""
    raw = streams[cast(str, row['stream'])]
    assert len(raw) == row['bytes'] and digest(raw) == row['sha256'], row['stream']
    attempts = [legacy_dialog_attempt(raw, version) for version in versions]
    return dict(**row, wire_hex=raw.hex(), candidate_sources=bindings, attempts=attempts)


def replay_additional_legacy_dialogs(pin: Path) -> None:
    """Recover all additional carriers without merging them into C-770's original inventory; e.g. replay_additional_legacy_dialogs(pin)."""
    inventory = legacy_dialog_inventory()
    rows: list[dict[str, object]] = []
    for document in sorted({cast(str, row['path']) for row in inventory}):
        streams = recover_sample(document, pin)
        versions, bindings = legacy_dialog_versions(document, streams)
        rows.extend(replay_legacy_dialog_row(row, streams, versions, bindings) for row in inventory if row['path'] == document)
    assert len(rows) == 251 and sum(cast(int, row['bytes']) for row in rows) == 275470
    packet = legacy_dialog_packet(rows)
    (FEATURE.parents[2] / 'target/simulation-post-dialog-additional-legacy.bin').write_bytes(packet)
    accepted = sum(any(cast(bool, attempt['success']) for attempt in cast(list[dict[str, object]], row['attempts'])) for row in rows)
    dialog_baseline('additional-legacy-reference.json', dict(streams=len(rows), logical_bytes=275470,
        helper_complete_streams=accepted, incomplete_streams=len(rows) - accepted, rows=rows, bytes=len(packet), sha256=digest(packet)))


def legacy_dialog_packet(rows: list[dict[str, object]]) -> bytes:
    """Keep full payloads and both failed/successful candidates in deterministic order; e.g. legacy_dialog_packet(rows)."""
    packet = bytearray()
    for row in rows:
        raw = bytes.fromhex(cast(str, row['wire_hex']))
        assert len(raw) == row['bytes'] and digest(raw) == row['sha256'], row['stream']
        for attempt in cast(list[dict[str, object]], row['attempts']):
            accepted = int(cast(bool, attempt['success']))
            compatible = int(bool(accepted and attempt['named_dialog_dispatch_enabled']))
            packet.extend(struct.pack('<IIIII', 3, cast(int, attempt['version']), accepted, compatible, len(raw)) + raw)
    return bytes(packet)
