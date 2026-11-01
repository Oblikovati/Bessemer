"""Complete C-871 evidence and read-only installed comparator bindings (#166)."""
import json
from typing import cast
from native_carriers import RESEARCH, SOURCE, digest
from dialog_evidence import dialog_baseline
from dialog_bindings import dialog_images, dialog_export, dialog_import
from colorref_import_evidence import colorref_import_reports


def legend_comparison_report() -> dict[str, object]:
    """Retain the original 109-state denominator and all prefix/version qualifications; e.g. legend_comparison_report()."""
    report = cast(dict[str, object], json.loads((RESEARCH / 'cosworks-legend-comparison-native-evidence.json').read_bytes()))
    assert report['evidence'] == 'C-871' and report['target_executed'] is False, report['evidence']
    assert report['coverage_credit'] == 0 and report['model_native_states'] == 109, report['model_native_states']
    dialog_baseline('legend-comparison-c871-reference.json', report)
    return report


def verify_legend_comparison_sources() -> None:
    """Require original pins except the one exact later C-906-qualified color census; e.g. verify_legend_comparison_sources()."""
    report = legend_comparison_report()
    _, latest = colorref_import_reports()
    canonical = {path.replace('\\', '/').split('/inventor-interface/')[-1]: value
                 for path, value in cast(dict[str, str], latest['sources']).items()}
    transitions: list[dict[str, str]] = []
    for path, expected in cast(dict[str, str], report['sources']).items():
        relative = path.replace('\\', '/').split('/inventor-interface/')[-1]
        assert relative.startswith('solidworks/research/'), path
        actual = digest((SOURCE / relative).read_bytes())
        if actual == expected: continue
        assert relative == 'solidworks/research/cosworks-post-color-fields-native-evidence.json', relative
        assert (expected, actual) == ('f11bf2049dbd4bad26a58b496377c5e0c3bd5fb6284fd79f797574225762f643', '97f4f83c3dad71727f9d6edb209acc776753680d74ea770c2b7edc349a1e415d'), (expected, actual)
        assert canonical[relative] == actual, actual
        transitions.append(dict(path=relative, original=expected, current=actual))
    assert len(transitions) == 1, transitions
    dialog_baseline('legend-comparison-source-transitions.json', dict(transitions=transitions))


def verify_legend_comparison_bindings() -> None:
    """Check the entire named comparator, probe, inequality import and decompilation pins; e.g. verify_legend_comparison_bindings()."""
    report = legend_comparison_report()
    method = cast(dict[str, str], report['method'])
    probe = json.loads((RESEARCH / 'cosworks-post-legend-compare-probe.json').read_bytes())[0]
    assert (method['address'], method['end']) == (probe['address'], probe['end']), method
    images = dialog_images()
    try:
        image = images['dgxpost.dll']
        assert digest(image.__data__) == report['module_sha256'], report['module_sha256']
        start, end = int(method['address'], 16), int(method['end'], 16)
        assert digest(image.get_data(start - image.OPTIONAL_HEADER.ImageBase, end - start)) == method['sha256'], method
        dialog_export(image, b'?DiffOGLRender@CDspLegend@@', start)
        dialog_import(image, b'dspogl.dll', b'??9COGLColor@@QEAAHAEBV0@@Z')
        assert digest((RESEARCH / 'decomp/cosworks-post-legend-compare/FUN_1800d9950.c').read_bytes()) == method['decompilation_sha256']
    finally:
        for image in images.values(): image.close()
