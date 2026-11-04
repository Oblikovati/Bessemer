"""Complete staged dialog replay and separate legacy helper/caller outcomes (#166)."""
from pathlib import Path
from typing import cast
import importlib
import json
import re
import struct
from native_carriers import RESEARCH, digest, recover_sample
from solid_inventory_bindings import complete_identity_match
from dialog_evidence import FEATURE, dialog_reports, dialog_baseline

DIALOG_READERS = ('cosworks_post_dialog_prefix', 'cosworks_post_dialog_info_prefix',
    'cosworks_post_dialog_style_prefix', 'cosworks_post_planes_prefix', 'cosworks_post_dialog_layout')


def dialog_path_family(stream: str) -> int:
    """Normalize complete modern or legacy storage shapes once; e.g. dialog_path_family('Cosworks/S/C/P/DspPostInfo')."""
    if re.fullmatch(r'[^/]+/StudyManager/[^/]+/Results/[^/]+/[^/]+/cwPostDlgParam', stream): return 0
    if re.fullmatch(r'Cosworks/[^/]+/[^/]+/[^/]+/cwPostDlgParam', stream): return 3
    match = re.fullmatch(r'Cosworks/[^/]+/[^/]+/[^/]+/(DspPostInfo|DspPostStyle)', stream)
    assert match, f'{stream}; expected complete modern dialog or standalone legacy helper path'
    return 1 if match[1] == 'DspPostInfo' else 2


def verify_dialog_inventory() -> None:
    """Match all 224 modern and 120 distinct legacy identities at every checkpoint; e.g. verify_dialog_inventory()."""
    corpus = json.loads((RESEARCH / 'third-party-storage-corpus.json').read_bytes())
    identities = [dict(path=sample['path'], stream=row['name'], bytes=row['bytes'], sha256=row['sha256'])
                  for sample in corpus['samples'] for row in sample['nested_streams']
                  if row['name'].endswith(('/cwPostDlgParam', '/DspPostInfo', '/DspPostStyle'))]
    modern = [row for row in identities if dialog_path_family(cast(str, row['stream'])) == 0]
    legacy = [row for row in identities if dialog_path_family(cast(str, row['stream'])) in (1, 2)]
    additional = [row for row in identities if dialog_path_family(cast(str, row['stream'])) == 3]
    for index, report in enumerate(dialog_reports()):
        actual = [{key: row[key] for key in ('path', 'stream', 'bytes', 'sha256')} for row in cast(list[dict[str, object]], report['rows'])]
        complete_identity_match(legacy if index == 5 else modern, actual)
    assert (len(modern), sum(cast(int, row['bytes']) for row in modern)) == (224, 496979)
    assert (len(legacy), sum(cast(int, row['bytes']) for row in legacy)) == (120, 44640)
    assert (len(additional), sum(cast(int, row['bytes']) for row in additional)) == (251, 275470)
    dialog_baseline('identity-reference.json', dict(modern=modern, legacy=legacy, additional_legacy_dialogs=additional))


def replay_dialog_row(row: dict[str, object], streams: dict[str, bytes], checkpoint: int) -> bytes:
    """Preserve every original value/span/failure and independently qualify caller completion; e.g. replay_dialog_row(row, streams, 4)."""
    family = dialog_path_family(cast(str, row['stream']))
    raw = streams[cast(str, row['stream'])]
    assert len(raw) == row['bytes'] and digest(raw) == row['sha256'], row['stream']
    reader = DIALOG_READERS[checkpoint] if checkpoint < 5 else ('cosworks_post_info_layout' if family == 1 else 'cosworks_post_style_layout')
    packet = bytearray()
    for expected in cast(list[dict[str, object]], row['attempts']):
        version = cast(int, expected['version'])
        parsed = importlib.import_module(reader).parse(raw, version)
        actual = dict(version=version, **parsed)
        if checkpoint == 5: actual['named_stream_dispatch_enabled'] = version < 700335
        assert actual == expected, (row['stream'], version, checkpoint)
        if checkpoint < 4: continue
        accepted = cast(bool, parsed['success'])
        compatible = accepted and (version >= 700335 if family == 0 else version < 700335)
        packet.extend(struct.pack('<IIIII', family, version, int(accepted), int(compatible), len(raw)) + raw)
    return bytes(packet)


def replay_dialog_document(document: str, reports: list[dict[str, object]], pin: Path) -> bytes:
    """Reuse one recovered immutable envelope across every original checkpoint; e.g. replay_dialog_document(document, reports, pin)."""
    streams = recover_sample(document, pin)
    packet = bytearray()
    for checkpoint, report in enumerate(reports):
        rows = cast(list[dict[str, object]], report['rows'])
        selected = [row for row in rows if row['path'] == document]
        packet.extend(b''.join(replay_dialog_row(row, streams, checkpoint) for row in selected))
    return bytes(packet)


def replay_dialog_native(pin: Path) -> None:
    """Replay complete native dialog and legacy alternatives without merging prefix credit; e.g. replay_dialog_native(pin)."""
    reports = dialog_reports()
    documents = sorted({cast(str, row['path']) for report in reports for row in cast(list[dict[str, object]], report['rows'])})
    packet = b''.join(replay_dialog_document(document, reports, pin) for document in documents)
    (FEATURE.parents[2] / 'target/simulation-post-dialog-native.bin').write_bytes(packet)
    summaries = dialog_native_summaries(reports)
    dialog_baseline('native-reference.json', dict(checkpoints=summaries, bytes=len(packet), sha256=digest(packet)))


def dialog_native_summaries(reports: list[dict[str, object]]) -> list[dict[str, object]]:
    """Keep prefix consumption, exact helpers and caller qualifiers separate; e.g. dialog_native_summaries(reports)."""
    summaries: list[dict[str, object]] = []
    for index, report in enumerate(reports):
        rows = cast(list[dict[str, object]], report['rows'])
        attempts = [attempt for row in rows for attempt in cast(list[dict[str, object]], row['attempts'])]
        accepted = sum(cast(bool, row.get('success', row.get('prefix_success', False))) for row in attempts)
        summaries.append(dict(evidence=report['evidence'], streams=len(rows), arguments=len(attempts), accepted=accepted,
            failed=len(attempts) - accepted, original_summary=report['summary']))
        if index < 5: assert (len(rows), len(attempts), accepted) == (224, 224, 224), report['evidence']
    assert reports[5]['summary'] == {'DspPostInfo': dict(streams=60, bytes=27600, helper_complete=60, caller_compatible_complete=52, caller_compatible_bytes=23920),
        'DspPostStyle': dict(streams=60, bytes=17040, helper_complete=52, caller_compatible_complete=52, caller_compatible_bytes=14768)}
    return summaries


def verify_dialog_legacy_view() -> None:
    """Replay C-867's complete synthetic legacy view without invoking its mutating audit; e.g. verify_dialog_legacy_view()."""
    name = 'cosworks-post-v7-composed-view-evidence.json'
    report = json.loads((RESEARCH / name).read_bytes())
    assert report['evidence'] == 'C-867' and not report['target_executed']
    raw = bytes(6) + bytes(range(256)) + bytes(range(112))
    actual = importlib.import_module('cosworks_post_style_layout').parse(raw, 700005, offset=6)
    assert actual == report['synthetic_result'], 'expected complete C-867 overlapping legacy view and original outer span'
    dialog_baseline(name, report)
