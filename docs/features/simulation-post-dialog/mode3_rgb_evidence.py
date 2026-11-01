"""C893 conditional1DC RGB candidates without observed array update or draw (#166)."""
from collections import Counter
import json
import unittest
from collections.abc import Callable
import struct
from typing import cast
from native_carriers import RESEARCH, SOURCE, digest
from dialog_evidence import dialog_baseline
from dialog_owned import dialog_owned_cases
from local_render_evidence import local_render_report
from stored_style_colors import canonical_style_spans, optional_style_rgba
from style_preferences_projection import raw_style_identity, raw_style_native_contexts


def mode3_rgb_report() -> dict[str, object]:
    """Keep both complete methods, exact thunk and all276 original observations; e.g. mode3_rgb_report()."""
    report = cast(dict[str, object], json.loads((RESEARCH / 'cosworks-mode3-rgb-native-evidence.json').read_bytes()))
    assert report['evidence'] == 'C-893' and report['target_executed'] is False and report['coverage_credit'] == 0
    methods = cast(list[dict[str, object]], report['functions'])
    assert [(method['start'], method['end'], len(cast(list[object], method['instructions']))) for method in methods] == [
        ('0x18015c230', '0x18015c8c3', 371), ('0x18018a870', '0x18018aa78', 133)]
    assert len(cast(list[object], report['rows'])) == 276
    dialog_baseline('mode3-rgb-c893-reference.json', report)
    return report


def verify_mode3_rgb_sources() -> None:
    """Qualify only the exact color-report transition beside complete raw replay; e.g. verify_mode3_rgb_sources()."""
    transitions: list[dict[str, str]] = []
    for original, expected in cast(dict[str, str], mode3_rgb_report()['sources']).items():
        relative = original.replace('\\', '/').split('/inventor-interface/')[-1]
        assert relative.startswith('solidworks/research/'), original
        actual = digest((SOURCE / relative).read_bytes())
        if actual == expected: continue
        assert relative == 'solidworks/research/cosworks-post-color-fields-native-evidence.json'
        assert (expected, actual) == ('b283e2f8a63a5276fcb77d646ae98358b48a682c7d6b740f91a6de7206381880', '97f4f83c3dad71727f9d6edb209acc776753680d74ea770c2b7edc349a1e415d')
        transitions.append(dict(path=relative, original=expected, current=actual))
    assert len(transitions) == 1
    dialog_baseline('mode3-rgb-source-transitions.json', dict(transitions=transitions,
        reason='All276 original stream identities, signed stored modes and complete1DC colors are independently replayed from complete archive bytes.'))


def mode3_rgb_observation(family: int, version: int, raw: bytes) -> dict[str, object] | None:
    """Retain raw1DC, actual signed mode and older absence without inferring face or submission; e.g. mode3_rgb_observation(2, 700006, raw)."""
    spans = canonical_style_spans(family, version, raw)
    rgba = optional_style_rgba(spans, raw, 'style.color_1dc')
    assert (rgba is not None) == (version >= 700006)
    mode_span = spans.get('style.word_1b8')
    if rgba is None:
        assert mode_span is None
        return None
    assert mode_span is not None
    mode_raw = bytes.fromhex(cast(str, mode_span['raw_hex']))
    offset = cast(int, mode_span['offset'])
    assert len(mode_raw) == 4 and raw[offset:offset + 4] == mode_raw
    mode = struct.unpack('<i', mode_raw)[0]
    return dict(mode_1b8=mode, rgba_1dc_raw_hex=rgba.hex(), stored_rgb_candidate_when_mode3=mode == 3,
        actual_runtime_face_code=None, actual_triple_submission=None)


def verify_mode3_rgb_native() -> None:
    """Rematch every original stream and preserve exact conditional counts without invocation credit; e.g. verify_mode3_rgb_native()."""
    report = mode3_rgb_report()
    rows = cast(list[dict[str, object]], report['rows'])
    original = cast(list[dict[str, object]], local_render_report()['rows'])
    identities = {raw_style_identity(row): 0 if row['kind'] == 'cwPostDlgParam' else 2 for row in original}
    assert Counter(raw_style_identity(row) for row in rows) == Counter(raw_style_identity(row) for row in original)
    available = {(family, digest(raw), version): mode3_rgb_observation(family, version, raw)
        for family, version, raw in raw_style_native_contexts()}
    for row in rows:
        assert row['scope'] == 'stored_fields_only'
        actual = available[(identities[raw_style_identity(row)], row['sha256'], row['selected_version'])]
        assert actual is not None and actual == {name: row[name] for name in actual}, row
    modes = dict(Counter(str(row['mode_1b8']) for row in rows))
    colors = dict(Counter(cast(str, row['rgba_1dc_raw_hex'])[:24] for row in rows))
    assert modes == report['mode_counts'] == {'1': 255, '0': 9, '2': 9, '3': 3}
    assert colors == report['rgb_counts'] and len(colors) == 2
    dialog_baseline('mode3-rgb-native-reference.json', dict(native_contexts=276, mode_counts=modes, rgb_counts=colors,
        actual_face_known=False, actual_submission_observed=False, additional_stored_byte_credit=0))


def verify_mode3_rgb_owned() -> None:
    """Retain all425 complete owned contexts, including20 opaque legacy styles; e.g. verify_mode3_rgb_owned()."""
    contexts = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    observed = [mode3_rgb_observation(*context) for context in contexts]
    assert len(observed) == 425 and sum(row is None for row in observed) == 20
    dialog_baseline('mode3-rgb-owned-reference.json', dict(owned_contexts=425, present=405, missing=20,
        mode_counts=dict(Counter(str(row['mode_1b8']) for row in observed if row is not None)),
        alpha_retained=True, actual_face_known=False, actual_submission_observed=False))



def mode3_rgb_functions() -> tuple[Callable[[], None], ...]:
    """Keep complete source/body evidence and all raw stored contexts together; e.g. mode3_rgb_functions()."""
    from mode3_rgb_bindings import verify_mode3_rgb_bindings, verify_mode3_rgb_rea_capture
    return (verify_mode3_rgb_sources, verify_mode3_rgb_bindings, verify_mode3_rgb_rea_capture,
        verify_mode3_rgb_native, verify_mode3_rgb_owned)


def run_mode3_rgb_checks() -> unittest.TestResult:
    """Retain every original catalog, framing and color check beside C893; e.g. run_mode3_rgb_checks()."""
    import importlib
    from dialog_evidence import verify_dialog_sources
    from dialog_field_evidence import verify_dialog_field_sources
    checks = (verify_dialog_sources, verify_dialog_field_sources) + mode3_rgb_functions()
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    for name in ('test_cosworks_post_style_field_catalog', 'test_cosworks_post_style', 'test_cosworks_post_color_fields'):
        suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module(name)))
    return unittest.TextTestRunner(verbosity=2).run(suite)
