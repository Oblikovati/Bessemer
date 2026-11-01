"""Retained masked-exception/DAZ-off color comparison evidence and installed bindings (#166)."""
import json
from typing import cast
from native_carriers import RESEARCH, SOURCE, digest
from dialog_evidence import dialog_baseline
from dialog_bindings import dialog_images, dialog_export
from dialog_field_bindings import verify_field_leaf
from colorref_import_evidence import colorref_import_reports


def color_comparison_report() -> dict[str, object]:
    """Preserve the complete original native census and conditional environment; e.g. color_comparison_report()."""
    report = cast(dict[str, object], json.loads((RESEARCH / 'cosworks-color-comparison-native-evidence.json').read_bytes()))
    assert report['evidence'] == 'C-864' and report['target_executed'] is False, report['evidence']
    assert report['coverage_credit'] == 0 and report['model_environment'] == 'masked_exceptions_daz_off', report['model_environment']
    dialog_baseline('rgba-comparison-c864-reference.json', report)
    return report


def verify_color_comparison_sources() -> None:
    """Require all original pins or the single qualified later color-report transition; e.g. verify_color_comparison_sources()."""
    report = color_comparison_report()
    _, latest = colorref_import_reports()
    pins = {path.replace('\\', '/').split('/inventor-interface/')[-1]: value
            for path, value in cast(dict[str, str], latest['sources']).items()}
    transitions: list[dict[str, str]] = []
    for path, expected in cast(dict[str, str], report['sources']).items():
        relative = path.replace('\\', '/').split('/inventor-interface/')[-1]
        assert relative.startswith('solidworks/research/'), path
        current = digest((SOURCE / relative).read_bytes())
        if current == expected: continue
        assert relative == 'solidworks/research/cosworks-post-color-fields-native-evidence.json', relative
        assert (expected, current) == ('a1a672e78546d9dbddc7ee55b6315916003c8776dfe7acfd7ea1907495ccb67c', '97f4f83c3dad71727f9d6edb209acc776753680d74ea770c2b7edc349a1e415d'), (expected, current)
        assert pins[relative] == current, current
        transitions.append(dict(path=relative, original=expected, current=current))
    assert len(transitions) == 1, transitions
    dialog_baseline('rgba-comparison-source-transitions.json', dict(transitions=transitions))


def verify_color_comparison_bindings() -> None:
    """Bind both full operator leaves and exact exports without executing provider code; e.g. verify_color_comparison_bindings()."""
    report = color_comparison_report()
    images = dialog_images()
    try:
        image = images['dspogl.dll']
        assert digest(image.__data__) == report['module_sha256'], report['module_sha256']
        functions = cast(list[dict[str, str]], report['functions'])
        assert len(functions) == 2, len(functions)
        for function in functions:
            verify_field_leaf(image, function)
            dialog_export(image, function['export'].encode(), int(function['address'], 16))
    finally:
        for image in images.values(): image.close()
