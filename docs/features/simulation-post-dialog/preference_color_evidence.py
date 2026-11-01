"""C905 legend initialization and C907 conditional preference-color evidence (#166)."""
from collections import Counter
from typing import cast
import json
import unittest
from collections.abc import Callable
from native_carriers import RESEARCH, SOURCE, digest
from dialog_evidence import dialog_baseline
from dialog_owned import dialog_owned_cases
from stored_style_colors import canonical_style_spans, optional_style_rgba
from local_render_evidence import local_render_report
from style_preferences_projection import raw_style_identity, raw_style_native_contexts

PREFERENCE_COLOR_ROUTES = ((0x220, 0x1578, 0x157c), (0x1bc, 0x1588, None), (0x1cc, 0x158c, None),
    (0x1dc, 0x1590, 0x1594), (0x2ac, 0x15a8, 0x15b0), (0x2bc, 0x15ac, None),
    (0x1f4, 0x15d0, 0x15c8), (0x20c, 0x15cc, 0x15c4))


def preference_color_reports() -> tuple[dict[str, object], dict[str, object]]:
    """Keep both complete original source reports and their separate scope; e.g. preference_color_reports()."""
    reports: list[dict[str, object]] = []
    for evidence, name in (('C-905', 'cosworks-legend-initial-color-evidence.json'), ('C-907', 'cosworks-style-preference-colors-evidence.json')):
        report = cast(dict[str, object], json.loads((RESEARCH / name).read_bytes()))
        assert report['evidence'] == evidence and report['target_executed'] is False and report['coverage_credit'] == 0
        dialog_baseline(name, report)
        reports.append(report)
    legend, preferences = reports
    assert len(cast(list[object], legend['rows'])) == legend['color_count'] == 109
    assert preferences['routes'] == [dict(style_offset=hex(style), packed_preference_offset=hex(packed),
        alpha_preference_offset=None if alpha is None else hex(alpha)) for style, packed, alpha in PREFERENCE_COLOR_ROUTES]
    assert preferences['local_block_skip_type'] == 73 and preferences['legend_call_precedes_guard'] is True
    return legend, preferences


def verify_preference_color_sources() -> None:
    """Require every original dependency unchanged without executing mutating audit entry points; e.g. verify_preference_color_sources()."""
    pins: dict[str, str] = {}
    for report in preference_color_reports():
        for original, expected in cast(dict[str, str], report['sources']).items():
            relative = original.replace('\\', '/').split('/inventor-interface/')[-1]
            assert relative.startswith('solidworks/research/'), original
            assert digest((SOURCE / relative).read_bytes()) == expected, (relative, expected)
            assert relative not in pins or pins[relative] == expected
            pins[relative] = expected
    dialog_baseline('preference-color-source-pins.json', dict(pins=pins, exact_transitions=0))


def preference_color_observation(family: int, version: int, raw: bytes) -> dict[str, str | None]:
    """Retain each named route's actual availability and all four raw words; e.g. preference_color_observation(2, 700005, raw)."""
    spans = canonical_style_spans(family, version, raw)
    observed: dict[str, str | None] = {}
    for member, _, _ in PREFERENCE_COLOR_ROUTES:
        color = optional_style_rgba(spans, raw, f'style.color_{member:x}')
        observed[hex(member)] = None if color is None else color.hex()
    legend = optional_style_rgba(spans, raw, 'legend.color_100')
    observed['legend100'] = None if legend is None else legend.hex()
    return observed


def verify_preference_color_native() -> None:
    """Preserve all276 source identities and109 original legend observations without defaults or invocation claims; e.g. verify_preference_color_native()."""
    legend, _ = preference_color_reports()
    original = cast(list[dict[str, object]], local_render_report()['rows'])
    families = {raw_style_identity(row): 0 if row['kind'] == 'cwPostDlgParam' else 2 for row in original}
    available = {(family, digest(raw), version): preference_color_observation(family, version, raw)
        for family, version, raw in raw_style_native_contexts()}
    expected = Counter(raw_style_identity(row) for row in original
        if available[(families[raw_style_identity(row)], row['sha256'], row['selected_version'])]['legend100'] is not None)
    rows = cast(list[dict[str, object]], legend['rows'])
    assert Counter(raw_style_identity(row) for row in rows) == expected
    for row in rows:
        colors = cast(list[dict[str, object]], row['colors'])
        assert len(colors) == 1 and colors[0]['label'] == 'legend.color_100.component'
        assert colors[0]['raw_hex'] == available[(families[raw_style_identity(row)], row['sha256'], row['selected_version'])]['legend100']
    observed = [available[(families[raw_style_identity(row)], row["sha256"], row["selected_version"])] for row in original]
    preference_native_counts(legend, observed)


def preference_native_counts(legend: dict[str, object], observed: list[dict[str, str | None]]) -> None:
    """Keep white stored counterexamples distinct from the blue initializer; e.g. preference_native_counts(legend, observed)."""
    counts = dict(Counter(row['legend100'] for row in observed if row['legend100'] is not None))
    assert counts == legend['color_values'] == {'00000000000000000000803f0000803f': 101, '0000803f0000803f0000803f0000803f': 8}
    dialog_baseline('preference-color-native-reference.json', dict(native_contexts=276, legend_present=109,
        legend_values=counts, route_presence={hex(member): sum(row[hex(member)] is not None for row in observed) for member, _, _ in PREFERENCE_COLOR_ROUTES},
        actual_preference_bytes_known=False, actual_invocation_observed=False, initialized_value_is_wire_default=False, additional_stored_byte_credit=0))


def verify_preference_color_owned() -> None:
    """Retain every425 complete context and conditional route absence without initializer substitution; e.g. verify_preference_color_owned()."""
    contexts = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    observed = [preference_color_observation(*context) for context in contexts]
    assert len(observed) == 425
    assert sum(all(row[hex(member)] is None for member, _, _ in PREFERENCE_COLOR_ROUTES) for row in observed) == 20
    dialog_baseline('preference-color-owned-reference.json', dict(owned_contexts=425, opaque_legacy=20,
        route_presence={hex(member): sum(row[hex(member)] is not None for row in observed) for member, _, _ in PREFERENCE_COLOR_ROUTES},
        legend_present=sum(row['legend100'] is not None for row in observed), initializer_substitution=False))



def preference_color_functions() -> tuple[Callable[[], None], ...]:
    """Keep all original pins, complete bodies and canonical native/owned observations together; e.g. preference_color_functions()."""
    from preference_color_bindings import verify_preference_color_bindings, verify_preference_color_rea_capture
    return (verify_preference_color_sources, verify_preference_color_bindings, verify_preference_color_rea_capture,
        verify_preference_color_native, verify_preference_color_owned)


def run_preference_color_checks() -> unittest.TestResult:
    """Retain every original catalog/style/color test and allthree preference model tests; e.g. run_preference_color_checks()."""
    import importlib
    from dialog_evidence import verify_dialog_sources
    from dialog_field_evidence import verify_dialog_field_sources
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in (verify_dialog_sources, verify_dialog_field_sources) + preference_color_functions())
    for name in ('test_cosworks_post_style_field_catalog', 'test_cosworks_post_style', 'test_cosworks_post_color_fields', 'test_cosworks_style_preference_colors'):
        suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module(name)))
    return unittest.TextTestRunner(verbosity=2).run(suite)
