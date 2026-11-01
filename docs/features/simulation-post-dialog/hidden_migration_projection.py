"""C856 derived pairs without changing wire absence or earlier/later load semantics (#166)."""
from collections import Counter
from typing import cast
import importlib
import struct
from native_carriers import digest
from dialog_evidence import dialog_baseline
from dialog_field_evidence import dialog_field_reports
from dialog_owned import dialog_owned_cases
from dialog_projection import actual_post_record
from style_preferences_projection import raw_style_native_contexts, raw_style_identity
from local_render_evidence import local_render_report
from legend_word_sources import verify_legend_rejected_identities
from hidden_migration_evidence import hidden_migration_report


def hidden_wire_fields(spans: list[dict[str, object]]) -> list[dict[str, object]]:
    """Preserve actual optional1EC and1F0 raw words with exact physical offsets; e.g. hidden_wire_fields(spans)."""
    fields: list[dict[str, object]] = []
    for span in spans:
        if span['label'] not in ('style.word_1ec', 'style.word_1f0'): continue
        raw = bytes.fromhex(cast(str, span['raw_hex']))
        assert len(raw) == cast(int, span['end']) - cast(int, span['offset']) == 4, (span, 'expected four signed32 style bytes')
        fields.append(dict(label=span['label'], offset=span['offset'], end=span['end'], raw_hex=raw.hex(), signed_value=struct.unpack('<i', raw)[0]))
    return fields


def hidden_migration_observation(family: int, version: int, raw: bytes) -> dict[str, object]:
    """Require complete EOF and retain derived migration separately from absent wire fields; e.g. hidden_migration_observation(2, 700006, raw)."""
    parsed = actual_post_record(family, version, raw)
    assert parsed['success'] and parsed['end'] == len(raw), (family, version, parsed)
    fields = hidden_wire_fields(cast(list[dict[str, object]], parsed['spans']))
    available = {field['label']: field for field in fields}
    for field in fields:
        assert raw[cast(int, field['offset']):cast(int, field['end'])].hex() == field['raw_hex']
    migration: dict[str, int] | None = None
    if 700006 <= version < 840008:
        assert 'style.word_1ec' not in available and 'style.word_1f0' in available, (version, fields)
        migration = importlib.import_module('cosworks_post_style_legacy_hidden_policy').migrate(available['style.word_1f0']['signed_value'], version)
    if version >= 840008:
        assert set(available) == {'style.word_1ec', 'style.word_1f0'}, (version, fields)
    return dict(fields=fields, migration=migration)


def verify_hidden_original_rows(rows: list[dict[str, object]], available: dict[tuple[int, str, int], dict[str, object]]) -> None:
    """Match each original migrated row, including scope, offset and all raw/derived values; e.g. verify_hidden_original_rows(rows, available)."""
    for row in rows:
        family = 0 if cast(str, row['stream']).endswith('/cwPostDlgParam') else 2
        assert row['scope'] == ('prefix_only' if family == 0 else 'strict_eof')
        view = available[(family, cast(str, row['sha256']), cast(int, row['selected_version']))]
        assert view['migration'] == row['migration'], row
        fields = cast(list[dict[str, object]], view['fields'])
        assert len(fields) == 1 and fields[0]['label'] == 'style.word_1f0', fields
        assert all(fields[0][key] == row[key] for key in ('offset', 'end', 'raw_hex')), row


def hidden_native_rows(available: dict[tuple[int, str, int], dict[str, object]]) -> list[dict[str, object]]:
    """Retain all276 original accepted identities, including160 outside the migration interval; e.g. hidden_native_rows(available)."""
    rows = cast(list[dict[str, object]], local_render_report()['rows'])
    result: list[dict[str, object]] = []
    for row in rows:
        family = 0 if cast(str, row['stream']).endswith('/cwPostDlgParam') else 2
        view = available[(family, cast(str, row['sha256']), cast(int, row['selected_version']))]
        identity = {key: row[key] for key in ('path', 'stream', 'sha256', 'selected_version')}
        result.append(dict(**identity, scope='prefix_only' if family == 0 else 'strict_eof', **view))
    return result


def verify_hidden_migration_native() -> None:
    """Preserve116 migrated observations,160 later reads and all eight rejected candidates; e.g. verify_hidden_migration_native()."""
    report = hidden_migration_report()
    available = {(family, digest(raw), version): hidden_migration_observation(family, version, raw)
        for family, version, raw in raw_style_native_contexts()}
    rows = hidden_native_rows(available)
    migrated = [row for row in rows if row['migration'] is not None]
    original = cast(list[dict[str, object]], report['rows'])
    assert len(rows) == 276 and len(migrated) == 116
    assert Counter(map(raw_style_identity, migrated)) == Counter(map(raw_style_identity, original))
    verify_hidden_original_rows(original, available)
    current = cast(list[dict[str, object]], dialog_field_reports()[2]['rows'])
    verify_legend_rejected_identities([row for row in current if not any(attempt['success'] for attempt in row['attempts'])])
    dialog_baseline('hidden-migration-native-reference.json', dict(rows=rows, accepted_contexts=276,
        migrated_contexts=116, later_direct_read_contexts=160, rejected_contexts=8,
        stored_values=report['stored_values'], derived_pairs=report['derived_pairs'],
        additional_stored_byte_credit=0, source_bytes_rewritten=False, nonzero_native_migrations_observed=False))


def verify_hidden_migration_owned() -> None:
    """Keep all425 complete parents and version-gated migration without field/default inference; e.g. verify_hidden_migration_owned()."""
    contexts = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    assert len(contexts) == 425
    views = [hidden_migration_observation(*context) for context in contexts]
    rows = [dict(family=family, version=version, bytes=len(raw), sha256=digest(raw), **view)
        for (family, version, raw), view in zip(contexts, views, strict=True)]
    dialog_baseline('hidden-migration-owned-reference.json', dict(contexts=425, rows=rows,
        migrated_contexts=sum(view['migration'] is not None for view in views),
        unmigrated_contexts=sum(view['migration'] is None for view in views),
        source_bytes_rewritten=False, raw_class_conversion_inferred=False, full_visibility_policy_modeled=False))
