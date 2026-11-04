"""Exact shared historical color-report adoption for stored legend-word views (#166)."""
from typing import cast
from native_carriers import SOURCE, digest
from dialog_field_evidence import dialog_field_reports


def legend_word_source_transitions(report: dict[str, object], historical: str) -> list[dict[str, str]]:
    """Require all original pins except one explicit replay-input transition; e.g. legend_word_source_transitions(report, old_hash)."""
    transitions: list[dict[str, str]] = []
    for original, expected in cast(dict[str, str], report['sources']).items():
        relative = original.replace('\\', '/').split('/inventor-interface/')[-1]
        assert relative.startswith('solidworks/research/'), original
        actual = digest((SOURCE / relative).read_bytes())
        if actual == expected: continue
        assert relative == 'solidworks/research/cosworks-post-color-fields-native-evidence.json', relative
        assert (expected, actual) == (historical, '97f4f83c3dad71727f9d6edb209acc776753680d74ea770c2b7edc349a1e415d'), (expected, actual)
        transitions.append(dict(path=relative, original=expected, current=actual))
    assert len(transitions) == 1, transitions
    return transitions


def verify_legend_rejected_identities(rejected: list[dict[str, object]]) -> None:
    """Match all eight original rejected identities to complete retained failed attempts; e.g. verify_legend_rejected_identities(rows)."""
    current = cast(list[dict[str, object]], dialog_field_reports()[2]['rows'])
    available = {(row['path'], row['stream'], row['sha256']): row for row in current}
    assert len(rejected) == 8, len(rejected)
    for row in rejected:
        attempts = cast(list[dict[str, object]], available[(row['path'], row['stream'], row['sha256'])]['attempts'])
        assert attempts and not any(attempt['success'] for attempt in attempts), (row, attempts)
