"""Native physical scans and owned modern index recognition/adoption/fallback cases."""
from __future__ import annotations
import json
import struct
import sys
from pathlib import Path
import replay_wire as wire
sys.path.insert(0, str(wire.SOURCE))
import flowindex as reference
from swarchive import ArchiveError
ROOT, SOURCE = wire.ROOT, wire.SOURCE


def normalize(value: dict) -> dict:
    return json.loads(json.dumps(wire.controls.definitions.objects.normalize(value)))


def reference_cases() -> tuple[list[tuple[str, str, bytes]], dict]:
    folder = SOURCE / 'research'
    evidence = json.loads((folder / 'flow-index-reader-native-evidence.json').read_text(encoding='utf-8'))
    paths = ['flow-index-reader-corpus.json', 'flow-index-reader-synthetic.json']
    rows = []
    for name, digest_key in zip(paths, ('corpus_sha256', 'synthetic_sha256')):
        path = folder / name
        assert wire.controls.definitions.objects.digest(path.read_bytes()) == evidence[digest_key]
        rows.extend(authenticate_rows(json.loads(path.read_text(encoding='utf-8'))))
    proof = dict(evidence_sha256=wire.controls.definitions.objects.digest((folder / 'flow-index-reader-native-evidence.json').read_bytes()),
                 corpus_sha256=evidence['corpus_sha256'], synthetic_sha256=evidence['synthetic_sha256'], limitations=evidence['limitations'])
    return rows, proof


def authenticate_rows(entries: list[dict]) -> list[tuple[str, str, bytes]]:
    rows = []
    for entry in entries:
        native = 'path' in entry
        raw = Path(entry['path']).read_bytes() if native else bytes.fromhex(entry['bytes_hex'])
        digest = entry['source_sha256'] if native else entry['sha256']
        assert wire.controls.definitions.objects.digest(raw) == digest
        assert normalize(reference.read(raw)) == entry['projection']
        name = Path(entry['path']).name if native else entry['name']
        rows.append((f'reference-{name}', 'native' if native else 'terminal', raw))
    return rows


def scan_cases(version: int) -> list[tuple[str, str, bytes]]:
    rows = []
    for count in (0, 1, 2, 3, 16, 64, 257, 4096, 65536):
        for kind in ('unique', 'duplicates', 'deleted'):
            records = scan_records(kind, count)
            rows.append((f'scan-{version}-{kind}-{count}', f'scan{version}', wire.carrier(records, version)))
    for identifier in wire.controls.definitions.PATTERNS:
        raw = wire.carrier([(identifier, b'old'), (identifier, b''), (wire.DELETED, b'x'), (identifier, b'last')], version)
        rows.append((f'scan-{version}-identifier-{identifier:08x}', f'scan{version}', raw))
    records = [(wire.INDEX, b'bad'), (wire.SAVED, b'bad'), (wire.DELETED, b'x'), (wire.INDEX, b''), (wire.SAVED, b'latest')]
    rows.append((f'scan-{version}-opaque-controls', f'scan{version}', wire.carrier(records, version)))
    return rows


def scan_records(kind: str, count: int) -> list[tuple[int, bytes]]:
    if kind == 'unique':
        return [(ordinal, bytes([ordinal % 256])) for ordinal in range(count)]
    if kind == 'duplicates':
        return [(ordinal % 3, bytes([ordinal % 256])) for ordinal in range(count)]
    assert kind == 'deleted'
    return [(wire.DELETED, bytes([ordinal % 256])) for ordinal in range(count)]


def terminal_cases() -> list[tuple[str, str, bytes]]:
    rows = []
    for version in (7, 8):
        for count in (0, 1, 2, 3, 16, 64, 257, 4096, 65536):
            entries = [(ordinal, ordinal * 7, ordinal % 5) for ordinal in range(count)]
            rows.append((f'terminal-{version}-unique-{count}', 'terminal', wire.terminal([(7, b'A')], entries, version)))
        entries = [(999, 900, 1), (999, 901, 2), (wire.INDEX, 0xffffffffffffffff, 0xffffffffffffffff), (wire.DELETED, 0, 0)]
        rows.append((f'terminal-{version}-duplicates-self-reserved', 'terminal', wire.terminal([(7, b'A')], entries, version)))
    for identifier in wire.controls.definitions.PATTERNS:
        rows.append((f'terminal-id-{identifier:08x}', 'terminal', wire.terminal([(7, b'A')], [(identifier, 0xffffffffffffffff, 0)])))
    for position in (1, 2):
        for pattern in wire.controls.wides.RAW_PATTERNS:
            entry = [999, 0, 0]
            entry[position] = pattern
            rows.append((f'terminal-wide-{position}-{pattern:016x}', 'terminal', wire.terminal([], [tuple(entry)])))
    return rows


def saved_cases() -> list[tuple[str, str, bytes]]:
    rows = []
    for count in (0, 1, 2, 3, 16, 64, 257, 4096, 65536):
        entries = [(ordinal, ordinal ^ 0x8000000000000000) for ordinal in range(count)]
        rows.append((f'saved-map-{count}', 'terminal', indexed_saved(entries)))
    for position in (0, 1):
        for pattern in wire.controls.wides.RAW_PATTERNS:
            entry = [7, 0]
            entry[position] = pattern
            rows.append((f'saved-wide-{position}-{pattern:016x}', 'terminal', indexed_saved([tuple(entry)])))
    rows.append(('saved-duplicate-last-wins', 'terminal', indexed_saved([(100, 3), (100, 4), (0, 1)])))
    rows.extend(saved_selection_cases())
    return rows


def indexed_saved(entries: list[tuple[int, int]]) -> bytes:
    payload = wire.saved(entries)
    span = len(wire.record(wire.SAVED, payload))
    return wire.terminal([(wire.SAVED, payload)], [(wire.SAVED, 2, span)])


def saved_selection_cases() -> list[tuple[str, str, bytes]]:
    first, second = wire.saved([(7, 1)]), wire.saved([(7, 2)])
    first_span, second_span = len(wire.record(wire.SAVED, first)), len(wire.record(wire.SAVED, second))
    records = [(wire.SAVED, first), (wire.SAVED, second)]
    old, new = (wire.SAVED, 2, first_span), (wire.SAVED, 2 + first_span, second_span)
    rows = [('saved-select-earlier', 'terminal', wire.terminal(records, [old])),
            ('saved-select-later', 'terminal', wire.terminal(records, [new])),
            ('saved-directive-last-earlier', 'terminal', wire.terminal(records, [new, old])),
            ('saved-directive-last-later', 'terminal', wire.terminal(records, [old, new]))]
    for start, span in ((2, first_span - 1), (3, first_span), (0xffffffffffffffff, first_span), (2, 0)):
        rows.append((f'saved-mismatch-{start}-{span}', 'terminal', wire.terminal(records, [(wire.SAVED, start, span)])))
    rows.append(('saved-missing', 'terminal', wire.terminal([(7, b'A')], [(wire.SAVED, 2, 13)])))
    rows.append(('saved-malformed-selected', 'terminal', wire.terminal([(wire.SAVED, b'bad')], [(wire.SAVED, 2, 15)])))
    return rows


def recognition_cases() -> list[tuple[str, str, bytes]]:
    raw = wire.terminal([(7, b'A')], [(999, 900, 1)])
    rows = []
    for distance in (-1, -(1 << 63), 0, 1, len(raw) - 12, len(raw) - 11, len(raw) - 13, len(raw) - 14):
        rows.append((f'fallback-distance-{distance}', 'terminal', wire.footer_patch(raw, distance=distance)))
    for identifier in wire.controls.definitions.PATTERNS:
        rows.append((f'fallback-footer-id-{identifier:08x}', 'terminal', wire.footer_patch(raw, identifier=identifier)))
    rows.extend(marker_cases(raw))
    earlier = wire.carrier([(wire.INDEX, b'bad'), (7, b'A'), (wire.INDEX, wire.controls.index_stored(wire.controls.index_body([]), 8))])
    rows.append(('fallback-earlier-index-not-at-eof', 'terminal', wire.footer_patch(earlier, distance=len(earlier) - 14)))
    rows.append(('matched-truncated-marker', 'terminal', wire.carrier([(wire.INDEX, struct.pack('<I', wire.INDEX))])))
    rows.append(('fallback-four-byte-writer-footer', 'terminal', wire.carrier([(wire.INDEX, wire.controls.index_stored(wire.controls.index_body([]), 4))])))
    return rows


def marker_cases(raw: bytes) -> list[tuple[str, str, bytes]]:
    document = wire.controls.definitions.objects.flowresults.read(raw)
    start = document['records'][-1]['start'] + 12
    rows = []
    for pattern in wire.controls.wides.RAW_PATTERNS:
        if pattern == 0:
            continue
        changed = bytearray(raw)
        changed[start:start + 8] = struct.pack('<Q', pattern)
        rows.append((f'fallback-marker-{pattern:016x}', 'terminal', bytes(changed)))
    return rows


def matched_failure_cases() -> list[tuple[str, str, bytes]]:
    rows = []
    for ordinal, decoded in enumerate((b'', struct.pack('<i', -1), struct.pack('<i', 1), struct.pack('<i', 0) + b'x')):
        rows.append((f'matched-bad-body-{ordinal}', 'terminal', wire.matched_payload(decoded)))
    decoded = wire.controls.index_body([])
    compressed = wire.controls.zlib.compress(decoded)
    corrupt = bytearray(compressed)
    corrupt[-1] ^= 1
    streams = [compressed[:-1], bytes(corrupt), compressed + compressed, compressed + b'x',
               wire.controls.zlib.compress(decoded, wbits=-15), wire.controls.zlib.compress(decoded, wbits=31)]
    for ordinal, stream in enumerate(streams):
        rows.append((f'matched-bad-stream-{ordinal}', 'terminal', wire.matched_payload(decoded, stream)))
    return rows


def cases() -> list[tuple[str, str, bytes]]:
    observed, _ = reference_cases()
    return observed + scan_cases(7) + scan_cases(8) + terminal_cases() + saved_cases() + recognition_cases() + matched_failure_cases()


def outcome(raw: bytes, forced: bool) -> dict:
    try:
        return dict(completed=True, projection=reference.read(raw, force_scan=forced))
    except ArchiveError as error:
        return dict(completed=False, cause=str(error))
