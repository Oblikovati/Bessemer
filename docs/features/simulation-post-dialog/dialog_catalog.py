"""Exact current PostStyle role catalog and native/probed denominator reconciliation (#166)."""
from typing import cast
from collections import Counter
import importlib
from dialog_field_evidence import dialog_field_reports

CATALOG_PROBES = (700005, 700006, 700329, 700330, 810008, 810009, 810018, 810019,
    820005, 820006, 840007, 840008, 840009, 840010, 890102, 890103, 890701, 890702)


def catalog_probe_presence() -> dict[str, list[int]]:
    """Recompute all eighteen gate probes without inventing native observations; e.g. catalog_probe_presence()."""
    parse = importlib.import_module('cosworks_post_style_layout').parse
    present: dict[str, list[int]] = {}
    for version in CATALOG_PROBES:
        parsed = parse(bytes(2048), version, require_eof=False)
        assert parsed['success'], version
        for span in parsed['spans']:
            present.setdefault(span['label'], []).append(version)
    return present


def catalog_native_statistics(entries: dict[str, dict[str, object]]) -> tuple[Counter[str], Counter[str], Counter[str], int]:
    """Count only the first accepted native interpretation and actual consumed style/legend spans; e.g. catalog_native_statistics(entries)."""
    observations: Counter[str] = Counter()
    byte_counts: Counter[str] = Counter()
    labels: Counter[str] = Counter()
    streams = 0
    for row in cast(list[dict[str, object]], dialog_field_reports()[2]['rows']):
        accepted = next((attempt for attempt in cast(list[dict[str, object]], row['attempts']) if attempt['success']), None)
        if accepted is None: continue
        streams += 1
        spans = cast(list[dict[str, object]], cast(dict[str, object], accepted['result'])['spans'])
        for span in spans:
            label = cast(str, span['label'])
            if not label.startswith(('style.', 'legend.')): continue
            status = cast(str, entries[label]['purpose_status'])
            observations[status] += 1
            byte_counts[status] += cast(int, span['end']) - cast(int, span['offset'])
            labels[label] += 1
    return observations, byte_counts, labels, streams


def verify_dialog_role_catalog() -> None:
    """Require every current role, unknown purpose, probe and native occurrence exactly; e.g. verify_dialog_role_catalog()."""
    report = dialog_field_reports()[3]
    entries = {cast(str, entry['label']): entry for entry in cast(list[dict[str, object]], report['catalog'])}
    assert len(entries) == len(cast(list[object], report['catalog'])) == 118
    presence = catalog_probe_presence()
    assert set(entries) == set(presence), 'expected every actual probed physical label'
    describe = importlib.import_module('cosworks_post_style_field_catalog').describe
    observations, byte_counts, labels, streams = catalog_native_statistics(entries)
    for label, entry in entries.items():
        actual = dict(**describe(label), present_at_probed_versions=presence[label], native_occurrences=labels[label])
        assert actual == entry, label
    assert dict(observations) == report['native_span_status_counts']
    assert dict(byte_counts) == report['native_span_bytes_by_status']
    assert streams == report['native_accepted_streams'] == 276
