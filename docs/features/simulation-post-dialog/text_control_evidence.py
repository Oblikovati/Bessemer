"""C890 full text-control replay preserves the earlier C855 assignment qualifications (#166)."""
from typing import cast
from collections import Counter
import importlib
import json
import struct
from native_carriers import RESEARCH, SOURCE, digest
from dialog_evidence import dialog_baseline
from dialog_projection import actual_post_record
from dialog_owned import dialog_owned_cases
from dialog_field_evidence import dialog_field_reports
from local_render_evidence import local_render_report
from style_preferences_projection import raw_style_identity, raw_style_native_contexts


def text_control_report() -> dict[str, object]:
    """Retain the complete original consumer and every accepted native identity; e.g. text_control_report()."""
    report = cast(dict[str, object], json.loads((RESEARCH / 'cosworks-style-text-controls-native-evidence.json').read_bytes()))
    assert report['evidence'] == 'C-890' and report['target_executed'] is False and report['coverage_credit'] == 0
    assert len(cast(list[object], report['rows'])) == 276
    consumer = cast(dict[str, object], report['consumer'])
    assert (consumer['start'], consumer['end']) == ('0x18010aa00', '0x18010af52')
    assert len(cast(list[object], consumer['instructions'])) == 315
    dialog_baseline('text-control-c890-reference.json', report)
    return report


def text_assignment_report() -> dict[str, object]:
    """Preserve all284 earlier accepted/rejected rows and their original prefix scopes; e.g. text_assignment_report()."""
    report = cast(dict[str, object], json.loads((RESEARCH / 'cosworks-post-style-text-view-native-evidence.json').read_bytes()))
    assert report['evidence'] == 'C-855' and report['native_assignments'] == 0
    assert report['target_executed'] is False and report['coverage_credit'] == 0
    assert len(cast(list[object], report['rows'])) == 284
    dialog_baseline('text-control-c855-reference.json', report)
    return report


def text_source_transitions(report: dict[str, object], historical: str) -> list[dict[str, object]]:
    """Permit only one exact color-report hash transition with complete text replay beside it; e.g. text_source_transitions(report, historical)."""
    transitions: list[dict[str, object]] = []
    for original, expected in cast(dict[str, str], report['sources']).items():
        relative = original.replace('\\', '/').split('/inventor-interface/')[-1]
        assert relative.startswith('solidworks/research/'), original
        actual = digest((SOURCE / relative).read_bytes())
        if actual == expected: continue
        assert relative == 'solidworks/research/cosworks-post-color-fields-native-evidence.json'
        assert expected == historical and actual == '97f4f83c3dad71727f9d6edb209acc776753680d74ea770c2b7edc349a1e415d', (expected, actual)
        transitions.append(dict(evidence=report['evidence'], path=relative, original=expected, current=actual))
    assert len(transitions) == 1
    return transitions


def verify_text_control_sources() -> None:
    """Verify every C890/C855 dependency and preserve both exact historical transitions; e.g. verify_text_control_sources()."""
    transitions = text_source_transitions(text_control_report(), '12a07b5b61debc27b8f104d843e9c0a6d13a80fb2fc256362b01a3af4ce15164')
    transitions += text_source_transitions(text_assignment_report(), 'a1a672e78546d9dbddc7ee55b6315916003c8776dfe7acfd7ea1907495ccb67c')
    dialog_baseline('text-control-source-transitions.json', dict(transitions=transitions,
        reason='Replay all276 accepted text inputs from complete bytes and preserve all8 original unaccepted identities with empty views.'))


def text_control_observation(family: int, version: int, raw: bytes) -> dict[str, object] | None:
    """Require full EOF and actual controls before projecting text or source selection; e.g. text_control_observation(0, 890620, raw)."""
    parsed = actual_post_record(family, version, raw)
    assert parsed['success'] and parsed['end'] == len(raw)
    spans = {span['label']: span for span in cast(list[dict[str, object]], parsed['spans'])}
    if 'style.word_23c' not in spans:
        assert 'style.word_240' not in spans
        return None
    paths = ('style.word_23c', 'style.word_240', 'style.character_count', 'style.utf16')
    for path in paths:
        offset, end = cast(int, spans[path]['offset']), cast(int, spans[path]['end'])
        assert raw[offset:end].hex() == spans[path]['raw_hex']
    return text_fields_observation(spans)


def text_fields_observation(spans: dict[str, dict[str, object]]) -> dict[str, object]:
    """Keep raw counted bytes separate from the assignment prefix and actual runtime text; e.g. text_fields_observation(spans)."""
    primary, lines, count = [bytes.fromhex(cast(str, spans[path]['raw_hex']))
        for path in ('style.word_23c', 'style.word_240', 'style.character_count')]
    assert all(len(word) == 4 for word in (primary, lines, count))
    enabled, grouped, characters = [struct.unpack('<i', word)[0] for word in (primary, lines, count)]
    wire = bytes.fromhex(cast(str, spans['style.utf16']['raw_hex']))
    assignment = importlib.import_module('cosworks_post_style_text_view').decode_view(wire)
    assert assignment['code_unit_count'] == characters
    offset, end = spans['style.utf16']['offset'], spans['style.utf16']['end']
    return dict(primary_control_raw_hex=primary.hex(), group_control_raw_hex=lines.hex(), character_count=characters,
        wire_text_raw_hex=wire.hex(), primary_selected_source='runtime_cstring_248' if enabled else 'empty_literal',
        group_selected_source='runtime_seven_cstrings_250_280' if grouped else 'seven_empty_literals',
        actual_runtime_group_text=None, assignment_view=dict(offset=offset, end=end, **assignment))


def verify_text_control_native() -> None:
    """Replay every original C890 identity and selection without inventing runtime contents; e.g. verify_text_control_native()."""
    report = text_control_report()
    rows = cast(list[dict[str, object]], report['rows'])
    identities = {raw_style_identity(row): 0 if row['kind'] == 'cwPostDlgParam' else 2
        for row in cast(list[dict[str, object]], local_render_report()['rows'])}
    assert Counter(raw_style_identity(row) for row in rows) == Counter(identities.keys())
    available = {(family, digest(raw), version): text_control_observation(family, version, raw)
        for family, version, raw in raw_style_native_contexts()}
    for row in rows:
        assert row['scope'] == 'explicit_stored_fields_only'
        observed = available[(identities[raw_style_identity(row)], row['sha256'], row['selected_version'])]
        assert observed is not None
        assert {name: value for name, value in observed.items() if name != 'assignment_view'} == {
            name: row[name] for name in observed if name != 'assignment_view'}, row
    verify_text_assignment_native(identities, available)
    assert report['native_counts'] == text_native_counts(identities, available) == {'primary_False_group_True_count_0': 276}
    dialog_baseline('text-control-native-reference.json', dict(native_contexts=276,
        native_counts=report['native_counts'], actual_runtime_contents_observed=False, additional_stored_byte_credit=0))


def verify_text_assignment_native(identities: dict[tuple[object, ...], int],
                                  available: dict[tuple[object, ...], dict[str, object] | None]) -> None:
    """Preserve all earlier prefix observations and all eight failed explicit interpretations; e.g. verify_text_assignment_native(identities, available)."""
    rows = cast(list[dict[str, object]], text_assignment_report()['rows'])
    accepted = [row for row in rows if row['selected_version'] is not None]
    assert Counter(raw_style_identity(row) for row in accepted) == Counter(identities.keys())
    for row in accepted:
        key = identities[raw_style_identity(row)], row['sha256'], row['selected_version']
        observed = available[key]
        assert observed is not None and row['views'] == [observed['assignment_view']], row
        assert row['scope'] == ('prefix_only' if key[0] == 0 else 'strict_eof')
    rejected = [row for row in rows if row['selected_version'] is None]
    current = cast(list[dict[str, object]], dialog_field_reports()[2]['rows'])
    failed = [row for row in current if not any(attempt['success'] for attempt in cast(list[dict[str, object]], row['attempts']))]
    assert len(rejected) == len(failed) == 8
    assert Counter(tuple(row[name] for name in ('path', 'stream', 'sha256')) for row in rejected) == Counter(
        tuple(row[name] for name in ('path', 'stream', 'sha256')) for row in failed)
    assert all(row['scope'] == 'strict_eof' and row['views'] == [] for row in rejected)


def verify_text_control_owned() -> None:
    """Keep every actual owned version group without inventing missing controls; e.g. verify_text_control_owned()."""
    contexts = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    observed = [text_control_observation(*case) for case in contexts]
    assert len(observed) == 425
    counts = Counter('missing' if row is None else 'present' for row in observed)
    assert counts == dict(present=405, missing=20)
    dialog_baseline('text-control-owned-reference.json', dict(owned_contexts=425, presence=dict(counts),
        character_counts=dict(Counter(str(row['character_count']) for row in observed if row is not None)),
        missing_values_defaulted=False, additional_stored_byte_credit=0))


def text_native_counts(identities: dict[tuple[object, ...], int],
                       observations: dict[tuple[object, ...], dict[str, object] | None]) -> dict[str, int]:
    """Preserve stream multiplicity when identical complete bytes share one decoded observation; e.g. text_native_counts(identities, observations)."""
    rows = [observations[(family, identity[2], identity[3])] for identity, family in identities.items()]
    return dict(Counter(f'primary_{row["primary_control_raw_hex"] != "00000000"}_group_{row["group_control_raw_hex"] != "00000000"}_count_{row["character_count"]}'
        for row in rows if row is not None))
