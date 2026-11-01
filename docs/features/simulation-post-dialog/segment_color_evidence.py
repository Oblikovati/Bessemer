"""C892 complete element-segment RGB evidence without alpha or draw inference (#166)."""
from collections import Counter
import json
import unittest
from collections.abc import Callable
from typing import cast
from native_carriers import RESEARCH, SOURCE, digest
from dialog_evidence import dialog_baseline
from dialog_projection import actual_post_record
from dialog_owned import dialog_owned_cases
from local_render_evidence import local_render_report
from style_preferences_projection import raw_style_identity, raw_style_native_contexts


def segment_color_report() -> dict[str, object]:
    """Keep original three complete bodies and all276 stream observations; e.g. segment_color_report()."""
    report = cast(dict[str, object], json.loads((RESEARCH / 'cosworks-element-segment-color-native-evidence.json').read_bytes()))
    assert report['evidence'] == 'C-892' and report['target_executed'] is False and report['coverage_credit'] == 0
    methods = cast(list[dict[str, object]], report['functions'])
    assert [(method['start'], method['end'], len(cast(list[object], method['instructions']))) for method in methods] == [
        ('0x18015e830', '0x18015ece4', 275), ('0x1800beaa0', '0x1800bed0c', 136), ('0x1800bed10', '0x1800befc8', 145)]
    assert len(cast(list[object], report['rows'])) == 276
    dialog_baseline('segment-color-c892-reference.json', report)
    return report


def verify_segment_color_sources() -> None:
    """Qualify only the exact historical color-report transition with complete archive replay; e.g. verify_segment_color_sources()."""
    transitions: list[dict[str, str]] = []
    for original, expected in cast(dict[str, str], segment_color_report()['sources']).items():
        relative = original.replace('\\', '/').split('/inventor-interface/')[-1]
        assert relative.startswith('solidworks/research/'), original
        actual = digest((SOURCE / relative).read_bytes())
        if actual == expected: continue
        assert relative == 'solidworks/research/cosworks-post-color-fields-native-evidence.json'
        assert (expected, actual) == ('caf6581daa815060ac5086d50eca1d03a1753fa852cf1cfc51c89a8c6ca11eb3', '97f4f83c3dad71727f9d6edb209acc776753680d74ea770c2b7edc349a1e415d')
        transitions.append(dict(path=relative, original=expected, current=actual))
    assert len(transitions) == 1
    dialog_baseline('segment-color-source-transitions.json', dict(transitions=transitions,
        reason='All276 original stream identities and every complete1BC RGBA are independently rematched from complete archive bytes.'))


def segment_color_observation(family: int, version: int, raw: bytes) -> dict[str, object] | None:
    """Read exact four-component storage, retaining the separate twelve-byte consumer view; e.g. segment_color_observation(2, 890620, raw)."""
    parsed = actual_post_record(family, version, raw)
    assert parsed['success'] and parsed['end'] == len(raw)
    spans = {span['label']: span for span in cast(list[dict[str, object]], parsed['spans'])}
    present = [f'style.color_1bc.component_{index}' in spans for index in range(4)]
    assert all(present) or not any(present)
    assert all(present) == (version >= 700006)
    if not all(present): return None
    words: list[bytes] = []
    for index in range(4):
        span = spans[f'style.color_1bc.component_{index}']
        word = bytes.fromhex(cast(str, span['raw_hex']))
        offset = cast(int, span['offset'])
        assert len(word) == 4 and raw[offset:offset + 4] == word
        words.append(word)
    rgba = b''.join(words)
    return dict(rgba_1bc_raw_hex=rgba.hex(), rgb_consumer_bytes_hex=rgba[:12].hex(),
        alpha_purpose=None, actual_runtime_mode_2c68=None, actual_primitive_invocation=None)


def verify_segment_color_native() -> None:
    """Retain stream multiplicity and unknown runtime facts across all276 original contexts; e.g. verify_segment_color_native()."""
    report = segment_color_report()
    rows = cast(list[dict[str, object]], report['rows'])
    original = cast(list[dict[str, object]], local_render_report()['rows'])
    identities = {raw_style_identity(row): 0 if row['kind'] == 'cwPostDlgParam' else 2 for row in original}
    assert Counter(raw_style_identity(row) for row in rows) == Counter(raw_style_identity(row) for row in original)
    available = {(family, digest(raw), version): segment_color_observation(family, version, raw)
        for family, version, raw in raw_style_native_contexts()}
    observed: list[dict[str, object]] = []
    for row in rows:
        assert row['scope'] == 'explicit_stored_fields_only'
        actual = available[(identities[raw_style_identity(row)], row['sha256'], row['selected_version'])]
        assert actual is not None and actual == {name: row[name] for name in actual}, row
        observed.append(actual)
    counts = dict(Counter(cast(str, row['rgb_consumer_bytes_hex']) for row in observed))
    assert counts == report['rgb_counts'] and len(counts) == 4
    dialog_baseline('segment-color-native-reference.json', dict(native_contexts=276, rgb_counts=counts,
        alpha_purpose_bound=False, actual_invocation_observed=False, additional_stored_byte_credit=0))


def verify_segment_color_owned() -> None:
    """Require every complete owned parent/style body and retain all four components; e.g. verify_segment_color_owned()."""
    contexts = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    observed = [segment_color_observation(*context) for context in contexts]
    assert len(observed) == 425 and sum(row is None for row in observed) == 20
    dialog_baseline('segment-color-owned-reference.json', dict(owned_contexts=425, present=405, missing=20, wire_gate='version>=700006',
        rgba_counts=dict(Counter(cast(str, row['rgba_1bc_raw_hex']) for row in observed if row is not None)),
        alpha_purpose_bound=False, actual_invocation_observed=False))



def segment_color_functions() -> tuple[Callable[[], None], ...]:
    """Keep exact original sources, full bodies and complete archive replays together; e.g. segment_color_functions()."""
    from segment_color_bindings import verify_segment_color_bindings, verify_segment_color_rea_capture
    return (verify_segment_color_sources, verify_segment_color_bindings, verify_segment_color_rea_capture,
        verify_segment_color_native, verify_segment_color_owned)


def run_segment_color_checks() -> unittest.TestResult:
    """Retain all original catalog, framing and color unit checks beside C892; e.g. run_segment_color_checks()."""
    import importlib
    from dialog_evidence import verify_dialog_sources
    from dialog_field_evidence import verify_dialog_field_sources
    checks = (verify_dialog_sources, verify_dialog_field_sources) + segment_color_functions()
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    for name in ('test_cosworks_post_style_field_catalog', 'test_cosworks_post_style', 'test_cosworks_post_color_fields'):
        suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module(name)))
    return unittest.TextTestRunner(verbosity=2).run(suite)
