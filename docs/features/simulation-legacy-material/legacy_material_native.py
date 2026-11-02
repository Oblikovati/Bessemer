"""Fresh complete legacy MaterialList payloads with unchanged failed-reader and candidate outcomes (#175)."""
from pathlib import Path
from typing import cast
import importlib
import json
import re
from native_carriers import digest, recover_sample
from dialog_legacy_carriers import legacy_dialog_versions
from material_reference import actual_material_parser, retained_outcome
from legacy_material_evidence import RESEARCH, legacy_material_reports, legacy_material_baseline
from legacy_material_identifiers import verify_fresh_legacy_identifiers


def legacy_material_inventory() -> list[dict[str, object]]:
    """Select every exact historical carrier while excluding the two other Material families; e.g. legacy_material_inventory()."""
    corpus = json.loads((RESEARCH / 'third-party-storage-corpus.json').read_bytes())
    return [dict(path=sample['path'], stream=row['name'], bytes=row['bytes'], sha256=row['sha256'])
        for sample in corpus['samples'] for row in sample['nested_streams']
        if re.fullmatch(r'Cosworks/[^/]+/MaterialList/[^/]+', row['name'])]


def verify_legacy_material_inventory() -> None:
    """Retain all70 identities/20,426 bytes before considering any parse result; e.g. verify_legacy_material_inventory()."""
    inventory = legacy_material_inventory()
    original = cast(list[dict[str, object]], legacy_material_reports()[0]['rows'])
    expected = [{key: row[key] for key in ('path', 'stream', 'bytes', 'sha256')} for row in original]
    assert inventory == expected, 'Fresh MaterialList identities differ; expected the complete original C838 inventory'
    assert len(inventory) == 70 and sum(cast(int, row['bytes']) for row in inventory) == 20426


def fresh_legacy_material_attempt(raw: bytes, version: int) -> dict[str, object]:
    """Preserve the current reader's exact original failure instead of changing version; e.g. fresh_legacy_material_attempt(raw, 840030)."""
    parsed = actual_material_parser(raw, version)
    assert parsed['success'] is False, f'{version}; expected the original complete-reader failure'
    return retained_outcome(version, parsed) | dict(diagnostic_raw_hex=raw.hex())


def fresh_legacy_material_row(original: dict[str, object], streams: dict[str, bytes], versions: list[int], bindings: list[dict[str, object]]) -> dict[str, object]:
    """Parse freshly recovered bytes under independently recovered retained contexts; e.g. fresh_legacy_material_row(row, streams, versions, bindings)."""
    raw = streams[cast(str, original['stream'])]
    assert len(raw) == original['bytes'] and digest(raw) == original['sha256'], original['stream']
    attempts = [fresh_legacy_material_attempt(raw, version) for version in versions]
    assert attempts == original['attempts'], f'{original["stream"]}; expected every original failure/endpoint/hash'
    header = importlib.import_module('cosworks_legacy_material_header').parse_header(raw)
    assert header == original['header_prefix'] and header['complete'] is False, original['stream']
    candidate = importlib.import_module('cosworks_legacy_material_candidate').inspect_candidate(raw)
    assert candidate['candidate_success'] and candidate['candidate_eof'] and candidate['complete'] is False
    assert candidate['reconstruction_sha256'] == original['sha256'] and candidate['end'] == len(raw)
    assert b''.join(bytes.fromhex(span['raw_hex']) for span in candidate['spans']) == raw
    return {key: original[key] for key in ('path', 'stream', 'bytes', 'sha256')} | dict(wire_hex=raw.hex(),
        retained_versions=versions, version_sources=bindings, header_prefix=header, attempts=attempts,
        current_reader_still_failed=True, candidate=candidate)


def fresh_legacy_material_rows(pin: Path) -> list[dict[str, object]]:
    """Recover each original outer envelope through frozen adapters, then every complete nested payload; e.g. fresh_legacy_material_rows(pin)."""
    original = cast(list[dict[str, object]], legacy_material_reports()[0]['rows'])
    recovered: dict[tuple[str, str], dict[str, object]] = {}
    for document in sorted({cast(str, row['path']) for row in original}):
        streams = recover_sample(document, pin)
        versions, bindings = legacy_dialog_versions(document, streams)
        for row in (row for row in original if row['path'] == document):
            recovered[cast(str, row['path']), cast(str, row['stream'])] = fresh_legacy_material_row(row, streams, versions, bindings)
    return [recovered[cast(str, row['path']), cast(str, row['stream'])] for row in original]


def replay_legacy_material_native(pin: Path) -> None:
    """Require complete original C838/C894/C895 outcomes without new completed-schema credit; e.g. replay_legacy_material_native(pin)."""
    rows = fresh_legacy_material_rows(pin)
    candidate_report = legacy_material_reports()[1]
    actual = [{key: row[key] for key in ('path', 'stream', 'bytes', 'sha256', 'retained_versions', 'current_reader_still_failed', 'candidate')} for row in rows]
    assert actual == candidate_report['rows'], 'Fresh candidates differ; expected the complete original C894 results'
    verify_fresh_legacy_identifiers(rows)
    legacy_material_baseline('native-replay-reference.json', dict(rows=rows, streams=len(rows),
        bytes=sum(cast(int, row['bytes']) for row in rows), header_bytes=sum(cast(dict[str, int], row['header_prefix'])['end'] for row in rows),
        candidate_eof_streams=70, fixed_records=196, nonzero_identifier_suffixes=182,
        complete_reader_successes=0, completed_schema_records=0, target_executed=False))
