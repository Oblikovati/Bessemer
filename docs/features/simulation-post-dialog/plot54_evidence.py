"""C891 numeric plot54 color-read roles without inferred invocation or2BC purpose (#166)."""
from typing import cast
from collections import Counter
import json
import struct
from native_carriers import RESEARCH, SOURCE, digest
from dialog_evidence import dialog_baseline
from dialog_projection import actual_post_record
from dialog_owned import dialog_owned_cases
from local_render_evidence import local_render_report
from style_preferences_projection import raw_style_identity, raw_style_native_contexts

PLOT54_COLORS = tuple(f'style.color_{member}.component_{index}' for member in ('2ac', '2bc') for index in range(4))


def plot54_report() -> dict[str, object]:
    """Retain all original stored pairs and both complete native methods; e.g. plot54_report()."""
    report = cast(dict[str, object], json.loads((RESEARCH / 'cosworks-plot54-color-native-evidence.json').read_bytes()))
    assert report['evidence'] == 'C-891' and report['target_executed'] is False and report['coverage_credit'] == 0
    assert len(cast(list[object], report['rows'])) == 208
    methods = cast(list[dict[str, object]], report['functions'])
    assert [(method['start'], method['end'], len(cast(list[object], method['instructions']))) for method in methods] == [
        ('0x180159560', '0x1801595d4', 29), ('0x18015a820', '0x18015ad8b', 294)]
    dialog_baseline('plot54-c891-reference.json', report)
    return report


def verify_plot54_sources() -> None:
    """Qualify only the exact later color-report transition beside independent complete-byte replay; e.g. verify_plot54_sources()."""
    transitions: list[dict[str, str]] = []
    for original, expected in cast(dict[str, str], plot54_report()['sources']).items():
        relative = original.replace('\\', '/').split('/inventor-interface/')[-1]
        assert relative.startswith('solidworks/research/'), original
        actual = digest((SOURCE / relative).read_bytes())
        if actual == expected: continue
        assert relative == 'solidworks/research/cosworks-post-color-fields-native-evidence.json'
        assert (expected, actual) == ('f11bf2049dbd4bad26a58b496377c5e0c3bd5fb6284fd79f797574225762f643', '97f4f83c3dad71727f9d6edb209acc776753680d74ea770c2b7edc349a1e415d')
        transitions.append(dict(path=relative, original=expected, current=actual))
    assert len(transitions) == 1
    dialog_baseline('plot54-source-transitions.json', dict(transitions=transitions,
        reason='Every original208 identity, supplied plot code and both complete RGBA groups are replayed from complete archive bytes.'))


def plot54_observation(family: int, version: int, raw: bytes) -> dict[str, object] | None:
    """Preserve conditional pair absence and never infer a standalone style's parent plot code; e.g. plot54_observation(2, 820006, raw)."""
    parsed = actual_post_record(family, version, raw)
    assert parsed['success'] and parsed['end'] == len(raw)
    spans = {span['label']: span for span in cast(list[dict[str, object]], parsed['spans'])}
    present = [path in spans for path in PLOT54_COLORS]
    assert all(present) or not any(present)
    assert all(present) == (version > 820005)
    if not all(present): return None
    words: list[bytes] = []
    for path in PLOT54_COLORS:
        word = bytes.fromhex(cast(str, spans[path]['raw_hex']))
        offset = cast(int, spans[path]['offset'])
        assert len(word) == 4 and raw[offset:offset + 4] == word
        words.append(word)
    return plot54_stored_observation(family, parsed, b''.join(words))


def plot54_stored_observation(family: int, parsed: dict[str, object], colors: bytes) -> dict[str, object]:
    """Retain the supplied parent code and both original colors while runtime facts stay unknown; e.g. plot54_stored_observation(0, parsed, colors)."""
    plot: int | None = None
    if family == 0:
        spans = cast(list[dict[str, object]], parsed['spans'])
        header = bytes.fromhex(cast(str, spans[0]['raw_hex']))
        assert len(header) == 56
        plot = struct.unpack('<14i', header)[1]
    return dict(plot_type_raw_code=plot, plot54_requirement_met=None if plot is None else plot == 54,
        rgba_2ac_raw_hex=colors[:16].hex(), rgba_2bc_raw_hex=colors[16:].hex(),
        actual_runtime_mode_2c68=None, actual_helper_invocation=None)


def verify_plot54_native() -> None:
    """Replay all208 original occurrences, including multiplicity and the single stored54, without invocation credit; e.g. verify_plot54_native()."""
    report = plot54_report()
    rows = cast(list[dict[str, object]], report['rows'])
    original = cast(list[dict[str, object]], local_render_report()['rows'])
    identities = {raw_style_identity(row): 0 if row['kind'] == 'cwPostDlgParam' else 2 for row in original}
    available = {(family, digest(raw), version): plot54_observation(family, version, raw) for family, version, raw in raw_style_native_contexts()}
    expected = Counter(raw_style_identity(row) for row in original
        if available[(identities[raw_style_identity(row)], row['sha256'], row['selected_version'])] is not None)
    assert Counter(raw_style_identity(row) for row in rows) == expected
    observed: list[dict[str, object]] = []
    for row in rows:
        assert row['scope'] == 'stored_fields_only'
        actual = available[(identities[raw_style_identity(row)], row['sha256'], row['selected_version'])]
        assert actual is not None and actual == {name: row[name] for name in actual}, row
        observed.append(actual)
    plot54_native_counts(report, observed)


def plot54_native_counts(report: dict[str, object], observed: list[dict[str, object]]) -> None:
    """Recompute frequency tables per original stream, preserving duplicate complete content; e.g. plot54_native_counts(report, observed)."""
    colors = dict(Counter(cast(str, row['rgba_2ac_raw_hex']) for row in observed))
    plots = dict(Counter(str(row['plot_type_raw_code']) for row in observed))
    assert colors == report['rgba_2ac_counts'] and len(colors) == 3
    assert plots == report['plot_type_counts'] and plots['54'] == 1
    dialog_baseline('plot54-native-reference.json', dict(native_contexts=276, native_present=208, native_missing=68,
        rgba_2ac_counts=colors, plot_type_counts=plots, actual_runtime_mode_known=False,
        actual_invocation_observed=False, forwarded_2bc_purpose_bound=False, additional_stored_byte_credit=0))


def verify_plot54_owned() -> None:
    """Check the exact820005 boundary across every complete owned parent/style context; e.g. verify_plot54_owned()."""
    contexts = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    observed = [plot54_observation(*case) for case in contexts]
    assert len(contexts) == 425
    expected = sum(version > 820005 for _, version, _ in contexts)
    counts = Counter('missing' if row is None else 'present' for row in observed)
    assert counts['present'] == expected and counts['missing'] == 425 - expected
    dialog_baseline('plot54-owned-reference.json', dict(owned_contexts=425, presence=dict(counts),
        standalone_parent_plot_code_inferred=False, missing_values_defaulted=False, additional_stored_byte_credit=0))
